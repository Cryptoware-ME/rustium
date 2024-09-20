pub mod datastore;
pub mod error;
pub mod logging;
pub mod prelude;
pub mod response;
pub mod result;

use crate::prelude::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        ()
    }
}
