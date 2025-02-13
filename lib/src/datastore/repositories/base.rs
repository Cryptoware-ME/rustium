//! Base data repository
use modql::filter::{FilterGroups, ListOptions};
use std::sync::Arc;
use surrealdb::sql::{thing, Object};

use crate::{
    datastore::idb::{Creatable, IRustiumDb, IdThing, Patchable},
    prelude::*,
};

// region: Public Base Repo

pub async fn get<E, S>(dal: Arc<S>, id: &str) -> RustiumResult<E>
where
    E: TryFrom<Object, Error = RustiumError> + Send + Sync,
    S: IRustiumDb + ?Sized,
{
    Ok(E::try_from(dal.exec_get(IdThing(thing(id)?)).await?)?)
}

pub async fn create<D, S>(dal: Arc<S>, entity: &'static str, data: D) -> RustiumResult<IdThing>
where
    D: TryInto<Object, Error = RustiumError> + Creatable + Send + Sync,
    S: IRustiumDb + ?Sized,
{
    dal.exec_create(entity, data.try_into()?).await
}

pub async fn update<D, S>(dal: Arc<S>, id: &str, data: D) -> RustiumResult<IdThing>
where
    D: TryInto<Object, Error = RustiumError> + Patchable + Send + Sync,
    S: IRustiumDb + ?Sized,
{
    dal.exec_merge(IdThing(thing(id)?), data.try_into()?).await
}

pub async fn delete<S>(dal: Arc<S>, id: &str) -> RustiumResult<bool>
where
    S: IRustiumDb + ?Sized,
{
    dal.exec_delete(IdThing(thing(id)?)).await
}

pub async fn list<E, S>(
    dal: Arc<S>,
    entity: &'static str,
    filter: Option<FilterGroups>,
    opts: ListOptions,
) -> RustiumResult<Vec<E>>
where
    E: TryFrom<Object, Error = RustiumError>,
    S: IRustiumDb + ?Sized,
{
    // query for the Surreal Objects
    let objects = dal.exec_select(entity, filter, opts).await?;

    // then get the entities
    objects
        .into_iter()
        .map(|o| o.try_into())
        .collect::<RustiumResult<_>>()
}
// endregion: Public Base Repo
