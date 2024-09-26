//! Base data repository

use modql::filter::{FilterGroups, ListOptions};
use serde::de::DeserializeOwned;
use surrealdb::sql::Object;

use crate::{
    datastore::idb_dal::{Creatable, IDbDal, IdThing, Patchable},
    prelude::*,
};

// region: Public Base Repo

pub async fn get<E, S>(dal: S, id: &str) -> RustiumResult<E>
where
    E: TryFrom<Object, Error = RustiumError> + DeserializeOwned + Send + Sync,
    S: IDbDal,
{
    Ok(E::try_from(dal.exec_get(IdThing(id.into())).await?)?)
}

pub async fn create<D, S>(dal: S, entity: &'static str, data: D) -> RustiumResult<IdThing>
where
    D: TryInto<Object, Error = RustiumError> + Creatable + Send + Sync,
    S: IDbDal,
{
    dal.exec_create(entity, data.try_into()?).await
}

pub async fn update<D, S>(dal: S, id: &str, data: D) -> RustiumResult<IdThing>
where
    D: TryInto<Object, Error = RustiumError> + Patchable + Send + Sync,
    S: IDbDal,
{
    dal.exec_merge(IdThing(id.into()), data.try_into()?).await
}

pub async fn delete<S>(dal: S, id: &str) -> RustiumResult<bool>
where
    S: IDbDal,
{
    dal.exec_delete(IdThing(id.into())).await
}

pub async fn list<E, S>(
    dal: S,
    entity: &'static str,
    filter: Option<FilterGroups>,
    opts: ListOptions,
) -> RustiumResult<Vec<E>>
where
    E: TryFrom<Object, Error = RustiumError>,
    S: IDbDal,
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
