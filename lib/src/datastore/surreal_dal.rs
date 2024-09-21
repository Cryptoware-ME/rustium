//! Data Access layer for SurrealDB

use modql::{
    filter::ListOptions,
    filter::{FilterGroups, IntoFilterNodes},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::BTreeMap;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::{thing, Datetime, Object, Thing, Value},
    Surreal,
};

use crate::{
    datastore::{
        object::{map, TakeX},
        query_builder::surreal_query_builder,
    },
    prelude::*,
    settings::database::DatabaseSettings,
};
// region: Traits

/// Marker traits for types that can be used for query

pub trait Creatable: Into<Value> {}
pub trait Patchable: Into<Value> {}
pub trait Deletable: Into<Value> {}
pub trait Filterable: IntoFilterNodes {}
// endregionL Traits

// region: Structs

/// Store struct normalizing CRUD SurrealDB application calls
pub struct SurrealDAL(Surreal<Client>);

#[derive(Deserialize, Serialize, Clone)]
pub struct IdThing(pub String);
// endregion: Structs

// region: Implementation

impl SurrealDAL {
    pub async fn new(conf: DatabaseSettings) -> RustiumResult<Self> {
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
        Ok(SurrealDAL(connection))
    }

    pub async fn exec_get<T: DeserializeOwned>(&self, tid: IdThing) -> RustiumResult<T> {
        let sql = "SELECT * FROM $th";

        let vars: BTreeMap<String, Thing> = map!["th".into() => thing(&tid.0)?];

        let mut ress = self.0.query(sql).bind(vars).await?;

        match ress.take(0)? {
            Some(object) => Ok(object),
            None => Err(RustiumError::NotFound(String::from("Object not found"))),
        }
    }

    pub async fn exec_create<T: Creatable>(&self, tb: &str, data: T) -> RustiumResult<IdThing> {
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

        let mut ress = self.0.query(sql).bind(vars).await?;

        let val: Option<Object> = ress.take(0)?;

        match val {
            Some(mut object) => object.take_x_val("id").map_err(|ex| {
                RustiumError::StoreFailToCreate(f!("exec_create failed for {tb} :: {ex}"))
            }),
            None => Err(RustiumError::StoreFailToCreate(f!(
                "exec_create {tb}, nothing returned."
            ))),
        }
    }

    pub async fn exec_merge<T: Patchable>(&self, tid: &str, data: T) -> RustiumResult<IdThing> {
        let sql = "UPDATE $th MERGE $data RETURN id";

        let vars: BTreeMap<String, Value> = map![
			"th".into() => thing(tid)?.into(),
			"data".into() => data.into()];

        let mut ress = self.0.query(sql).bind(vars).await?;

        let val: Option<Object> = ress.take(0)?;

        match val {
            Some(mut object) => object.take_x_val("id"),
            None => Err(RustiumError::StoreFailToCreate(f!(
                "exec_merge {tid}, nothing returned."
            ))),
        }
    }

    pub async fn exec_delete(&self, tid: IdThing) -> RustiumResult<bool> {
        let sql = "DELETE $th";

        let vars: BTreeMap<String, Thing> = map!["th".into() => thing(&tid.0)?];

        let mut ress = self.0.query(sql).bind(vars).await?;

        let val: Option<Object> = ress.take(0)?;

        match val {
            Some(_) => Ok(true),
            None => Err(RustiumError::NotFound(String::from("Could not delete"))),
        }
    }

    pub async fn exec_select<O: Into<FilterGroups>>(
        &self,
        tb: &str,
        filter_groups: Option<O>,
        list_options: ListOptions,
    ) -> RustiumResult<Vec<Object>> {
        let filter_or_groups = filter_groups.map(|v| v.into());

        let (sql, vars) = surreal_query_builder(tb, filter_or_groups, list_options)?;

        let mut ress = self.0.query(sql).bind(vars).await?;

        let array: Vec<Value> = ress.take(0)?;

        array
            .into_iter()
            .map(|value| Wrap(value).try_into())
            .collect()
    }
}
// endregion: Implementation
