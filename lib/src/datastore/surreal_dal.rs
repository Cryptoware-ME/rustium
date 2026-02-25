//! Data Access layer for SurrealDB
use axum::async_trait;
use di::{injectable, Ref};
use modql::filter::{FilterGroups, ListOptions};
use std::collections::BTreeMap;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::{Datetime, Object, Thing, Value},
    Surreal,
};

use crate::{
    datastore::{
        idb::{IRustiumDb, IdThing},
        query_builder::surreal_query_builder,
    },
    map,
    prelude::*,
    service::RustiumService,
    settings::interface::IRustiumSettings,
};

// region: Structs
pub struct SurrealDAL {
    settings: Ref<dyn IRustiumSettings>,
    db: Option<Surreal<Client>>,
}

// endregion: Structs

// region: Implementation
#[injectable(IRustiumDb)]
impl SurrealDAL {
    fn new(inj_settings: Ref<dyn IRustiumSettings>) -> Self {
        let mut this = Self {
            settings: inj_settings,
            db: None,
        };
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(this.init())
        })
        .expect("DB should be available & credentials should be valid");
        this
    }
}

#[async_trait]
impl RustiumService for SurrealDAL {
    async fn init(&mut self) -> RustiumResult<()> {
        let conf = self.settings.get_database()?;
        let connection = Surreal::new::<Ws>(&conf.uri).await?;
        connection
            .signin(Root {
                username: &conf.username,
                password: &conf.password,
            })
            .await?;
        connection
            .use_ns(&conf.namespace)
            .use_db(&conf.dbname)
            .await?;
        self.db = Some(connection);
        Ok(())
    }

    async fn run(&self) -> RustiumResult<()> {
        Ok(())
    }

    fn as_rustium(&self) -> RustiumResult<Option<Box<&dyn RustiumService>>> {
        Ok(Some(Box::new(self)))
    }
}

#[async_trait]
impl IRustiumDb for SurrealDAL {
    async fn exec_get(&self, tid: IdThing) -> RustiumResult<Object> {
        let sql = "SELECT * FROM $th";

        let vars: BTreeMap<String, Thing> = map!["th".into() => tid.0];

        match &self.db {
            Some(db) => match db.query(sql).bind(vars).await?.take(0)? {
                Some(object) => Ok(object),
                None => Err(RustiumError::NotFound(String::from("Object not found"))),
            },
            None => Err(RustiumError::Unresolved(String::from(
                "DB service not initialized",
            ))),
        }
    }

    async fn exec_create(&self, tb: &str, mut data: Object) -> RustiumResult<IdThing> {
        let sql = "CREATE type::table($tb) CONTENT $data RETURN id";

        match Datetime::default().timestamp_nanos_opt() {
            Some(now) => {
                data.insert("created_at".into(), now.into());
                data.insert("updated_at".into(), now.into());
            }
            None => {
                return Err(RustiumError::CreateTableError(String::from(
                    "Error creating table record",
                )));
            }
        };

        let vars: BTreeMap<String, Value> = map![
			"tb".into() => tb.into(),
			"data".into() => Value::from(data)];

        match &self.db {
            Some(db) => {
                let ress: Option<Thing> = db.query(sql).bind(vars).await?.take("id")?;
                match ress {
                    Some(id) => Ok(IdThing(id)),
                    None => Err(RustiumError::StoreFailToCreate(f!(
                        "exec_create {tb}, nothing returned."
                    ))),
                }
            }
            None => Err(RustiumError::Unresolved(String::from(
                "DB service not initialized",
            ))),
        }
    }

    async fn exec_merge(&self, tid: IdThing, mut data: Object) -> RustiumResult<IdThing> {
        let sql = "UPDATE $th MERGE $data RETURN id";

        match Datetime::default().timestamp_nanos_opt() {
            Some(now) => {
                data.insert("updated_at".into(), now.into());
            }
            None => {
                return Err(RustiumError::CreateTableError(String::from(
                    "Error updating table record",
                )));
            }
        };

        let vars: BTreeMap<String, Value> = map![
			"th".into() => tid.0.into(),
			"data".into() => Value::from(data)];

        match &self.db {
            Some(db) => {
                let ress: Option<Thing> = db.query(sql).bind(vars).await?.take("id")?;
                match ress {
                    Some(id) => Ok(IdThing(id)),
                    None => Err(RustiumError::StoreFailToCreate(f!(
                        "exec_merge, nothing returned."
                    ))),
                }
            }
            None => Err(RustiumError::Unresolved(String::from(
                "DB service not initialized",
            ))),
        }
    }

    async fn exec_delete(&self, tid: IdThing) -> RustiumResult<bool> {
        let sql = "DELETE $th";

        let vars: BTreeMap<String, Thing> = map!["th".into() => tid.0];

        match &self.db {
            Some(db) => {
                let ress: Option<Thing> = db.query(sql).bind(vars).await?.take("id")?;
                match ress {
                    Some(_) => Ok(true),
                    None => Err(RustiumError::StoreFailToCreate(f!(
                        "exec_delete, nothing returned."
                    ))),
                }
            }
            None => Err(RustiumError::Unresolved(String::from(
                "DB service not initialized",
            ))),
        }
    }

    async fn exec_select(
        &self,
        tb: &str,
        filter_groups: Option<FilterGroups>,
        list_options: ListOptions,
    ) -> RustiumResult<Vec<Object>> {
        let filter_or_groups = filter_groups.map(|v| v.into());

        let (sql, vars) = surreal_query_builder(tb, filter_or_groups, list_options)?;

        let ress: Vec<Object> = match &self.db {
            Some(db) => match db.query(sql).bind(vars).await?.take(0)? {
                Some(object) => object,
                None => {
                    return Err(RustiumError::StoreFailToCreate(f!(
                        "exec_merge {tb}, nothing returned."
                    )))
                }
            },
            None => {
                return Err(RustiumError::Unresolved(String::from(
                    "DB service not initialized",
                )))
            }
        };

        Ok(ress)
    }
}
// endregion: Implementation
