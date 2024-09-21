pub mod authentication;
pub mod datastore;
pub mod error;
pub mod logging;
pub mod prelude;
pub mod response;
pub mod result;
pub mod settings;

use crate::prelude::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        ()
    }
}
