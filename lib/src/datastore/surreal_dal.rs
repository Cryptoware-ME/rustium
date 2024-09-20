//! Data Access layer for SurrealDB

use crate::{
    datastore::{
        object::{map, TakeX},
        query_builder::surreal_query_builder,
    },
    prelude::*,
};
use modql::{
    filter::ListOptions,
    filter::{FilterGroups, IntoFilterNodes},
};
use serde::{Deserialize, Serialize};
use surrealdb::{
    dbs::Session,
    kvs::Datastore,
    sql::{thing, Array, Datetime, Object, Value},
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
pub struct SurrealDAL {
    store: Datastore,
    session: Session,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct IdThing(pub String);
// endregion: Structs

// region: Implementation

impl SurrealDAL {
    pub async fn new(store: &str, namespace: &str, collection: &str) -> Result<Self> {
        let store = Datastore::new(store).await?;
        let session = Session::default().with_ns(namespace).with_db(collection);
        Ok(SurrealDAL { store, session })
    }

    pub async fn exec_get(&self, tid: &str) -> Result<Object> {
        let sql = "SELECT * FROM $th";

        let vars = map!["th".into() => thing(tid)?.into()];

        let ress = self.store.execute(sql, &self.session, Some(vars)).await?;

        let first_res = ress.into_iter().next().expect("Did not get a response");

        Wrap(first_res.result?.first()).try_into()
    }

    pub async fn exec_create<T: Creatable>(&self, tb: &str, data: T) -> Result<IdThing> {
        let sql = "CREATE type::table($tb) CONTENT $data RETURN id";

        let mut data: Object = Wrap(data.into()).try_into()?;
        match Datetime::default().timestamp_nanos_opt() {
            Some(now) => {
                data.insert("created_at".into(), now.into());
            }
            None => {
                return Err(RustiumError::CreateTableError(String::from(
                    "Error creating table",
                )));
            }
        };

        let vars = map![
			"tb".into() => tb.into(),
			"data".into() => Value::from(data)];

        let ress = self.store.execute(sql, &self.session, Some(vars)).await?;
        let first_val = ress
            .into_iter()
            .next()
            .map(|r| r.result)
            .expect("id not returned")?;

        if let Value::Object(mut val) = first_val.first() {
            val.take_x_val("id")
                .map_err(|ex| RustiumError::StoreFailToCreate(f!("exec_create {tb} {ex}")))
        } else {
            Err(RustiumError::StoreFailToCreate(f!(
                "exec_create {tb}, nothing returned."
            )))
        }
    }

    pub async fn exec_merge<T: Patchable>(&self, tid: &str, data: T) -> Result<IdThing> {
        let sql = "UPDATE $th MERGE $data RETURN id";

        let vars = map![
			"th".into() => thing(tid)?.into(),
			"data".into() => data.into()];

        let ress = self.store.execute(sql, &self.session, Some(vars)).await?;

        let first_res = ress.into_iter().next().expect("id not returned");

        let result = first_res.result?;

        if let Value::Object(mut val) = result.first() {
            val.take_x_val("id")
        } else {
            Err(RustiumError::StoreFailToCreate(f!(
                "exec_merge {tid}, nothing returned."
            )))
        }
    }

    pub async fn exec_delete(&self, tid: &str) -> Result<String> {
        let sql = "DELETE $th";

        let vars = map!["th".into() => thing(tid)?.into()];

        let ress = self.store.execute(sql, &self.session, Some(vars)).await?;

        let first_res = ress.into_iter().next().expect("Did not get a response");

        first_res.result?;

        Ok(tid.to_string())
    }

    pub async fn exec_select<O: Into<FilterGroups>>(
        &self,
        tb: &str,
        filter_groups: Option<O>,
        list_options: ListOptions,
    ) -> Result<Vec<Object>> {
        let filter_or_groups = filter_groups.map(|v| v.into());
        let (sql, vars) = surreal_query_builder(tb, filter_or_groups, list_options)?;

        let ress = self.store.execute(&sql, &self.session, Some(vars)).await?;

        let first_res = ress.into_iter().next().expect("Did not get a response");

        let array: Array = Wrap(first_res.result?).try_into()?;

        array
            .into_iter()
            .map(|value| Wrap(value).try_into())
            .collect()
    }
}
// endregion: Implementation
