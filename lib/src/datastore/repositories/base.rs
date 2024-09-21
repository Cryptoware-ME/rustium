//! Base data repository

use modql::filter::ListOptions;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use surrealdb::sql::Object;

use crate::{
    datastore::surreal_dal::{Creatable, Filterable, IdThing, Patchable, SurrealDAL},
    prelude::*,
};

// region: Public Base Repo

pub async fn get<E: DeserializeOwned>(dal: &Arc<SurrealDAL>, id: &str) -> RustiumResult<E>
where
    E: TryFrom<Object, Error = RustiumError>,
{
    dal.exec_get(IdThing(id.into())).await
}

pub async fn create<D>(
    dal: &Arc<SurrealDAL>,
    entity: &'static str,
    data: D,
) -> RustiumResult<IdThing>
where
    D: Creatable,
{
    dal.exec_create(entity, data).await
}

pub async fn update<D>(dal: &Arc<SurrealDAL>, id: &str, data: D) -> RustiumResult<IdThing>
where
    D: Patchable,
{
    dal.exec_merge(id, data).await
}

pub async fn delete(dal: &Arc<SurrealDAL>, id: &str) -> RustiumResult<bool> {
    dal.exec_delete(IdThing(id.into())).await
}

pub async fn list<E, F>(
    dal: &Arc<SurrealDAL>,
    entity: &'static str,
    filter: Option<F>,
    opts: ListOptions,
) -> RustiumResult<Vec<E>>
where
    E: TryFrom<Object, Error = RustiumError>,
    F: Filterable + std::fmt::Debug,
{
    // query for the Surreal Objects
    let objects = dal
        .exec_select(entity, filter.map(|f| f.filter_nodes(None)), opts)
        .await?;

    // then get the entities
    objects
        .into_iter()
        .map(|o| o.try_into())
        .collect::<RustiumResult<_>>()
}
// endregion: Public Base Repo
