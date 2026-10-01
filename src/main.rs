pub mod db;
use axum::Router;

use crate::db::create_pool;
pub mod schema;
pub mod model;

#[tokio::main]
async fn main() {
   let router = Router::new().route("get",get_order);
   println!("Let's start the order service");
   let pool = create_pool();
}
   