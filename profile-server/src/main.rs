use log::info;
use server_shared::database::db_pool;
use shared::endpoints::CreateUserProfileEndpoint;
use warp::path;

use crate::handlers::create_user;
//use server_shared::{route, serve_and_run};

mod handlers;

#[tokio::main]
pub async fn main() {
    env_logger::init();
    info!("Logger initialized!");

    /*let pool = db_pool().await;
    let me_ulr = "simplechat-official.com";

    let x = warp::path!("").;

    let routes = route(
        path!("users"),
        CreateUserProfileEndpoint,
        create_user,
        pool.clone(),
        String::from(me_ulr),
    );

    serve_and_run(routes).await;*/
}
