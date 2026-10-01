pub mod db;
use axum::{Router, serve};
use tokio::net::TcpListener;
pub mod order;
pub mod auth;
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

   let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await
           .unwrap_or_else(|e| panic!("failed to bind to port {port}: {e}"));

   serve(listener, router).await.unwrap()
}
   