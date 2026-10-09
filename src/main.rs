pub mod db;
use axum::{Router, serve};
use tokio::net::TcpListener;
pub mod order;
use axum::middleware;
pub mod auth;
use crate::{auth::jwt::JwtService, db::{AppState,create_pool}};
use crate::order::api::get_order;
pub mod schema;
use axum::{routing::{get}};
use crate::auth::middleware::auth_middleware;
pub mod model;

#[tokio::main]
async fn main() {
   let pool = create_pool();
   let decoding_key = std::env::var("JwtService").expect("Secret Not Found");
   let jwt_service = JwtService::new(&decoding_key);
   let state = AppState {
         db_pool: pool,
         jwt_service: jwt_service
      };

   let router = Router::new()
    .route("/health", get(get_order))
    .layer(middleware::from_fn_with_state( 
            state.clone(),
            auth_middleware,
        ));
   println!("Let's start the order service");

   let port: u16 = std::env::var("PORT").ok().and_then(|p| p.parse().ok())
        .unwrap_or(3000);

   let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await
           .unwrap_or_else(|e| panic!("failed to bind to port {port}: {e}"));

   serve(listener, router).await.unwrap()
}
   