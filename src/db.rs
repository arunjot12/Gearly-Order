use std::env;
use deadpool_diesel::mysql::{Manager, Pool};

pub fn create_pool() -> Pool {
    dotenvy::dotenv().ok();
    let database = env::var("DATABASE_URL").expect("no database found");
    let manager = Manager::new(database, deadpool_diesel::Runtime::Tokio1);
    Pool::builder(manager).max_size(10).build().expect("Unable to setup")
}