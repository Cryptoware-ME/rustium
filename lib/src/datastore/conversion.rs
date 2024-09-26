//! TryFrom implementations for wrapped store-related types

use std::time::Duration;
use surrealdb::sql::{Array, Object, Uuid, Value};

use crate::{datastore::idb_dal::IdThing, prelude::*};

// region: Implementations

impl TryFrom<Wrap<Value>> for Object {
    type Error = RustiumError;
    fn try_from(val: Wrap<Value>) -> RustiumResult<Object> {
        match val.0 {
            Value::Object(obj) => Ok(obj),
            _ => Err(Self::Error::PropertyNotFound(String::from("Object"))),
        }
    }
}

impl TryFrom<Wrap<Value>> for Array {
    type Error = RustiumError;
    fn try_from(val: Wrap<Value>) -> RustiumResult<Array> {
        match val.0 {
            Value::Array(obj) => Ok(obj),
            _ => Err(Self::Error::PropertyNotFound(String::from("Array"))),
        }
    }
}

impl TryFrom<Wrap<Value>> for f64 {
    type Error = RustiumError;
    fn try_from(val: Wrap<Value>) -> RustiumResult<f64> {
        match val.0 {
            Value::Strand(obj) => Ok(obj.as_str().parse::<f64>().unwrap()),
            Value::Number(obj) => Ok(obj.as_float()),
            _ => Err(Self::Error::PropertyNotFound(String::from("f64"))),
        }
    }
}

impl TryFrom<Wrap<Value>> for Duration {
    type Error = RustiumError;
    fn try_from(val: Wrap<Value>) -> RustiumResult<Duration> {
        match val.0 {
            Value::Number(obj) => Ok(Duration::from_millis(obj.to_int() as u64)),
            _ => Err(Self::Error::PropertyNotFound(String::from("Duration"))),
        }
    }
}

impl TryFrom<Wrap<Value>> for i64 {
    type Error = RustiumError;
    fn try_from(val: Wrap<Value>) -> RustiumResult<i64> {
        match val.0 {
            Value::Strand(obj) => Ok(obj.as_str().parse::<i64>().unwrap()),
            Value::Number(obj) => Ok(obj.as_int()),
            _ => Err(Self::Error::PropertyNotFound(String::from("i64"))),
        }
    }
}

impl TryFrom<Wrap<Value>> for bool {
    type Error = RustiumError;
    fn try_from(val: Wrap<Value>) -> RustiumResult<bool> {
        match val.0 {
            Value::Bool(false) => Ok(false),
            Value::Bool(true) => Ok(true),
            _ => Err(Self::Error::PropertyNotFound(String::from("bool"))),
        }
    }
}

impl TryFrom<Wrap<Value>> for String {
    type Error = RustiumError;
    fn try_from(val: Wrap<Value>) -> RustiumResult<String> {
        match val.0 {
            Value::Strand(strand) => Ok(strand.as_string()),
            Value::Thing(thing) => Ok(thing.to_string()),
            _ => Err(Self::Error::PropertyNotFound(String::from("String"))),
        }
    }
}

impl TryFrom<Wrap<Value>> for Uuid {
    type Error = RustiumError;
    fn try_from(val: Wrap<Value>) -> RustiumResult<Uuid> {
        match val.0 {
            Value::Uuid(id) => Ok(id),
            _ => Err(Self::Error::PropertyNotFound(String::from("Uuid"))),
        }
    }
}

impl TryFrom<Wrap<Value>> for IdThing {
    type Error = RustiumError;
    fn try_from(val: Wrap<Value>) -> RustiumResult<IdThing> {
        match val.0 {
            Value::Thing(thing) => Ok(IdThing(thing.id.to_string())),
            _ => Err(Self::Error::PropertyNotFound(String::from("IdThing"))),
        }
    }
}
// endregion: Implementations
