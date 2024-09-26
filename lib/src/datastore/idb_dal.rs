use axum::async_trait;
use modql::filter::{FilterGroups, ListOptions};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use surrealdb::sql::{Object, Value};

use crate::{prelude::*, service::RustiumThreadSafe};

#[derive(Deserialize, Serialize, Clone)]
pub struct IdThing(pub String);

/// Marker traits for types that can be used for query
pub trait Creatable: Into<Value> {}
pub trait Patchable: Into<Value> {}
pub trait Deletable: Into<Value> {}

#[async_trait]
pub trait IDbDal {
    async fn exec_get(&self, tid: IdThing) -> RustiumResult<Object>;
    async fn exec_create(&self, tb: &str, data: Object) -> RustiumResult<IdThing>;
    async fn exec_merge(&self, tid: IdThing, data: Object) -> RustiumResult<IdThing>;
    async fn exec_delete(&self, tid: IdThing) -> RustiumResult<bool>;
    async fn exec_select(
        &self,
        tb: &str,
        filter_groups: Option<FilterGroups>,
        list_options: ListOptions,
    ) -> RustiumResult<Vec<Object>>;
}

impl RustiumThreadSafe for IdThing {}
impl Display for IdThing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
