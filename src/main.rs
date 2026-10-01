pub mod db;
use axum::Router;
use tokio::net::TcpListener;
pub mod order;
use crate::{db::{AppState,create_pool}};
use crate::order::api::get_order;
pub mod schema;
use axum::{routing::{get}};
pub mod model;

#[tokio::main]
async fn main() {

   let pool = create_pool();
   let state = AppState {
         db_pool: pool,
      };

   let router = Router::new()
    .route("/health", get(get_order)).with_state(state);

   println!("Let's start the order service");

   let port: u16 = std::env::var("PORT").ok().and_then(|p| p.parse().ok())
        .unwrap_or(3000);

   let port = TcpListener::bind(format!("0.0.0.0:{port}")).await;
}
   