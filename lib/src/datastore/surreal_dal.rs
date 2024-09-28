//! Data Access layer for SurrealDB

use axum::async_trait;
use di::{injectable, Ref};
use modql::{filter::FilterGroups, filter::ListOptions};
use std::collections::BTreeMap;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::{thing, Datetime, Object, Thing, Value},
    Surreal,
};

use crate::{
    datastore::{
        idb_dal::{IDbDal, IdThing},
        object::{map, TakeX},
        query_builder::surreal_query_builder,
    },
    prelude::*,
    service::RustiumService,
    settings::RustiumSettings,
};

// region: Structs
#[injectable(IDbDal)]
pub struct SurrealDAL {
    settings: Ref<RustiumSettings>,
    db: Option<Surreal<Client>>,
}
// endregion: Structs

impl Default for SurrealDAL {
    fn default() -> Self {
        Self {
            db: Option::None,
            settings: Ref::default(),
        }
    }
}
// region: Implementation
#[async_trait]
impl RustiumService for SurrealDAL {
    async fn init(&mut self) -> RustiumResult<()> {
        let conf = self.settings.database.clone();
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

    async fn run(&mut self) -> RustiumResult<()> {
        Ok(())
    }

    fn as_rustium(&mut self) -> RustiumResult<Option<Box<&mut dyn RustiumService>>> {
        Ok(Some(Box::new(self)))
    }
}

#[async_trait]
impl IDbDal for SurrealDAL {
    async fn exec_get(&self, tid: IdThing) -> RustiumResult<Object> {
        let sql = "SELECT * FROM $th";

        let vars: BTreeMap<String, Thing> = map!["th".into() => thing(&tid.0)?];

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

    async fn exec_create(&self, tb: &str, data: Object) -> RustiumResult<IdThing> {
        let sql = "CREATE type::table($tb) CONTENT $data RETURN id";

        let mut data: Object = Wrap(data.into()).try_into()?;

        match Datetime::default().timestamp_nanos_opt() {
            Some(now) => {
                data.insert("created_at".into(), now.into());
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
                let ress: Option<Object> = db.query(sql).bind(vars).await?.take(0)?;
                match ress {
                    Some(mut object) => {
                        let id = object.take_x_val("id").map_err(|ex| {
                            RustiumError::StoreFailToCreate(f!(
                                "exec_create failed for {tb} :: {ex}"
                            ))
                        })?;
                        Ok(IdThing(id))
                    }
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

    async fn exec_merge(&self, tid: IdThing, data: Object) -> RustiumResult<IdThing> {
        let sql = "UPDATE $th MERGE $data RETURN id";

        let vars: BTreeMap<String, Value> = map![
			"th".into() => thing(&tid.0)?.into(),
			"data".into() => data.into()];

        match &self.db {
            Some(db) => {
                let ress: Option<Object> = db.query(sql).bind(vars).await?.take(0)?;
                match ress {
                    Some(mut object) => {
                        let id = object.take_x_val("id").map_err(|ex| {
                            RustiumError::StoreFailToCreate(f!(
                                "exec_merge failed for {tid} :: {ex}"
                            ))
                        })?;
                        Ok(IdThing(id))
                    }
                    None => Err(RustiumError::StoreFailToCreate(f!(
                        "exec_merge {tid}, nothing returned."
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

        let vars: BTreeMap<String, Thing> = map!["th".into() => thing(&tid.0)?];

        match &self.db {
            Some(db) => {
                let ress: Option<Object> = db.query(sql).bind(vars).await?.take(0)?;
                match ress {
                    Some(_) => Ok(true),
                    None => Err(RustiumError::StoreFailToCreate(f!(
                        "exec_delete {tid}, nothing returned."
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

        let array: Vec<Value> = match &self.db {
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

        array
            .into_iter()
            .map(|value| Wrap(value).try_into())
            .collect()
    }
}
// endregion: Implementation
