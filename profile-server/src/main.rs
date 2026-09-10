use axum::{Router, routing::get};
use log::info;
use server_shared::{database::db_pool, routes::RouteBuilder};
use shared::endpoints::{
    CreateUserProfileEndpoint, GetUserParams, GetUserProfileEndpoint, TestProfileEndpoint,
};
use warp::path;

use crate::handlers::{create_user, get_test, get_user};
//use server_shared::{route, serve_and_run};

mod handlers;

#[tokio::main]
pub async fn main() {
    env_logger::init();
    info!("Logger initialized!");

    let pool = db_pool().await;
    let app = RouteBuilder::new(pool, String::new())
        .route(CreateUserProfileEndpoint, create_user)
        .route(GetUserProfileEndpoint, get_user)
        .route(TestProfileEndpoint, get_test)
        .build();

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
