//! TakeXImpl implementations for the surrealdb Object types.

use std::time::Duration;
use surrealdb::sql::{Object, Uuid};

use crate::{
    datastore::{idb_dal::IdThing, object::TakeXImpl},
    prelude::*,
};
// region: Implementations

impl TakeXImpl<String> for Object {
    fn take_x_impl(&mut self, k: &str) -> RustiumResult<Option<String>> {
        let v = self.remove(k).map(|v| Wrap(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(ex)) => Err(ex),
        }
    }
}

impl TakeXImpl<f64> for Object {
    fn take_x_impl(&mut self, k: &str) -> RustiumResult<Option<f64>> {
        let v = self.remove(k).map(|v| Wrap(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(ex)) => Err(ex),
        }
    }
}

impl TakeXImpl<i64> for Object {
    fn take_x_impl(&mut self, k: &str) -> RustiumResult<Option<i64>> {
        let v = self.remove(k).map(|v| Wrap(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(ex)) => Err(ex),
        }
    }
}

impl TakeXImpl<bool> for Object {
    fn take_x_impl(&mut self, k: &str) -> RustiumResult<Option<bool>> {
        Ok(self.remove(k).map(|v| v.is_true()))
    }
}

impl TakeXImpl<Uuid> for Object {
    fn take_x_impl(&mut self, k: &str) -> RustiumResult<Option<Uuid>> {
        let v = self.remove(k).map(|v| Wrap(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(e)) => Err(e),
        }
    }
}

impl TakeXImpl<Duration> for Object {
    fn take_x_impl(&mut self, k: &str) -> RustiumResult<Option<Duration>> {
        let v = self.remove(k).map(|v| Wrap(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(e)) => Err(e),
        }
    }
}

impl TakeXImpl<IdThing> for Object {
    fn take_x_impl(&mut self, k: &str) -> RustiumResult<Option<IdThing>> {
        let v = self.remove(k).map(|v| Wrap(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(e)) => Err(e),
        }
    }
}
// endregion: Implementations
