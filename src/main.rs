pub mod db;
use crate::db::create_pool;
pub mod schema;
pub mod model;

fn main() {
   let pool = create_pool();
}
   