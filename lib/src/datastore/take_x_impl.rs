//! TakeXImpl implementations for the surrealdb Object types.

use crate::{
    datastore::{object::TakeXImpl, surreal_dal::IdThing},
    prelude::*,
};
use std::time::Duration;
use surrealdb::sql::{Object, Uuid};

// region: Implementations

impl TakeXImpl<String> for Object {
    fn take_x_impl(&mut self, k: &str) -> Result<Option<String>> {
        let v = self.remove(k).map(|v| Wrap(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(ex)) => Err(ex),
        }
    }
}

impl TakeXImpl<f64> for Object {
    fn take_x_impl(&mut self, k: &str) -> Result<Option<f64>> {
        let v = self.remove(k).map(|v| Wrap(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(ex)) => Err(ex),
        }
    }
}

impl TakeXImpl<i64> for Object {
    fn take_x_impl(&mut self, k: &str) -> Result<Option<i64>> {
        let v = self.remove(k).map(|v| Wrap(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(ex)) => Err(ex),
        }
    }
}

impl TakeXImpl<bool> for Object {
    fn take_x_impl(&mut self, k: &str) -> Result<Option<bool>> {
        Ok(self.remove(k).map(|v| v.is_true()))
    }
}

impl TakeXImpl<Uuid> for Object {
    fn take_x_impl(&mut self, k: &str) -> Result<Option<Uuid>> {
        let v = self.remove(k).map(|v| Wrap(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(e)) => Err(e),
        }
    }
}

impl TakeXImpl<Duration> for Object {
    fn take_x_impl(&mut self, k: &str) -> Result<Option<Duration>> {
        let v = self.remove(k).map(|v| Wrap(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(e)) => Err(e),
        }
    }
}

impl TakeXImpl<IdThing> for Object {
    fn take_x_impl(&mut self, k: &str) -> Result<Option<IdThing>> {
        let v = self.remove(k).map(|v| Wrap(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(e)) => Err(e),
        }
    }
}
// endregion: Implementations
