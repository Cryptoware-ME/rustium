// use crate::{
//     database::Database,
//     datastore::{
//         repositories::base::{create, delete, get, list, update},
//         surreal_dal::IdThing,
//     },
//     domain::{
//         dto::sale_dto::{CreateSaleDTO, SaleDTO, SaleFilter},
//         sale::Sale,
//     },
//     object::TakeX,
//     prelude::*,
// };
// use modql::ListOptions;
// use std::sync::Arc;
// use surrealdb::sql::Object;

// // region: Implementations

// impl TryFrom<Object> for Sale {
//     type Error = Error;
//     fn try_from(mut val: Object) -> Result<Sale> {
//         let obj = Sale {
//             id: val.take_x_val("id")?,
//             item_id: val.take_x_val("item_id")?,
//             source: val.take_x_val("source")?,
//             tx_date: val.take_x_val("tx_date")?,
//             amount: val.take_x_val("amount")?,
//             currency: val.take_x_val("currency")?,
//         };

//         Ok(obj)
//     }
// }
// // endregion: Implementations

// // region: Public Functions

// pub async fn get_sale(ctx: &Arc<Database>, id: &str) -> Result<Sale> {
//     get::<Sale>(ctx, id).await
// }

// pub async fn create_sale(ctx: &Arc<Database>, data: CreateSaleDTO) -> Result<IdThing> {
//     create(ctx, "sales".into(), data).await
// }

// pub async fn update_sale(ctx: &Arc<Database>, id: &str, data: SaleDTO) -> Result<IdThing> {
//     update(ctx, id, data).await
// }

// pub async fn delete_sale(ctx: &Arc<Database>, id: &str) -> Result<String> {
//     delete(ctx, id).await
// }

// pub async fn query_sales(
//     ctx: &Arc<Database>,
//     filter: SaleFilter,
//     opts: ListOptions,
// ) -> Result<Vec<Sale>> {
//     list(ctx, "sales", Some(filter), opts).await
// }
// // endregion: Public Functions
