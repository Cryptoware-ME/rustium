//! Base data repository

use crate::{
    datastore::surreal_dal::{Creatable, Filterable, IdThing, Patchable, SurrealDAL},
    prelude::*,
};
use modql::filter::ListOptions;
use std::sync::Arc;
use surrealdb::sql::Object;

// region: Public Base Repo

pub async fn get<E>(dal: &Arc<SurrealDAL>, id: &str) -> Result<E>
where
    E: TryFrom<Object, Error = RustiumError>,
{
    dal.exec_get(id).await?.try_into()
}

pub async fn create<D>(dal: &Arc<SurrealDAL>, entity: &'static str, data: D) -> Result<IdThing>
where
    D: Creatable,
{
    Ok(dal.exec_create(entity, data).await?)
}

pub async fn update<D>(dal: &Arc<SurrealDAL>, id: &str, data: D) -> Result<IdThing>
where
    D: Patchable,
{
    Ok(dal.exec_merge(id, data).await?)
}

pub async fn delete(dal: &Arc<SurrealDAL>, id: &str) -> Result<String> {
    Ok(dal.exec_delete(id).await?)
}

pub async fn list<E, F>(
    dal: &Arc<SurrealDAL>,
    entity: &'static str,
    filter: Option<F>,
    opts: ListOptions,
) -> Result<Vec<E>>
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
        .collect::<Result<_>>()
}
// endregion: Public Base Repo
