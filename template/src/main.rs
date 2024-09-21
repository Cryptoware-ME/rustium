use rustium::datastore::surreal_dal::SurrealDAL;
use std::sync::Arc;
use tokio::sync::OnceCell;

// Tokio ONE CELL for context
static DB_ONCE: OnceCell<Arc<SurrealDAL>> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    // // Init DB Context
    // let db = DB_ONCE
    //     .get_or_init(|| async { Arc::new(SurrealDAL::new().await) })
    //     .await;
}
