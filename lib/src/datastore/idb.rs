use axum::async_trait;
use modql::filter::{FilterGroups, ListOptions};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use surrealdb::sql::Object;

use crate::{
    prelude::*,
    service::{RustiumService, RustiumThreadSafe},
};

#[derive(Deserialize, Serialize, Clone)]
pub struct IdThing(pub String);

/// Marker traits for types that can be used for query
pub trait Creatable: TryInto<Object> {}
pub trait Patchable: TryInto<Object> {}
pub trait Deletable: TryInto<Object> {}

#[async_trait]
pub trait IRustiumDb: RustiumService {
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
