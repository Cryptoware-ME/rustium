//! TakeX trait for taking a value from an object for a given key.
//! The trait to implement for a type is the `TakeXImpl` which has only one function.
//!
//! `take_x_impl(&mut self, k: &str) -> Result<Option<T>>`
//!
//! Then, TakeX is a blanket implementation with
//! - `take_x` that returns a `Result<Option<T>>`
//! - `take_x_val` that returns `Result<T>` and fails if no value for key
//!
//! Also includes a convenient map! macro

use crate::prelude::*;

// region: Macros

macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
		let mut m = ::std::collections::BTreeMap::new();
        $(m.insert($k, $v);)+
        m
    }};
  }
pub(crate) use map;
// endregion: Macros

// region: Traits

pub trait TakeXImpl<T> {
    fn take_x_impl(&mut self, k: &str) -> Result<Option<T>>;
}

pub trait TakeX {
    fn take_x<T>(&mut self, k: &str) -> Result<Option<T>>
    where
        Self: TakeXImpl<T>;

    fn take_x_val<T>(&mut self, k: &str) -> Result<T>
    where
        Self: TakeXImpl<T>;
}
// endregion: Traits

// region: Implementations

/// Blanket Implementation
impl<O> TakeX for O {
    fn take_x<T>(&mut self, k: &str) -> Result<Option<T>>
    where
        Self: TakeXImpl<T>,
    {
        TakeXImpl::take_x_impl(self, k)
    }

    fn take_x_val<T>(&mut self, k: &str) -> Result<T>
    where
        Self: TakeXImpl<T>,
    {
        let val: Option<T> = TakeXImpl::take_x_impl(self, k)?;
        val.ok_or_else(|| RustiumError::PropertyNotFound(k.to_string()))
    }
}
// endregion: Implementations
