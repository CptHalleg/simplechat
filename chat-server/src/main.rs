use log::info;

mod handlers;
mod websocket;
use server_shared::serve_and_run;
use warp::Filter;

#[tokio::main]
pub async fn main() {
    env_logger::init();
    info!("Logger initialized!");
    let routes = warp::path::end()
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(websocket::user_connected));

    serve_and_run(routes).await;
}
