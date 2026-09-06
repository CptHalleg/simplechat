use log::info;

mod handlers;
mod websocket;
use tokio::signal::unix::{SignalKind, signal};
use warp::Filter;

#[tokio::main]
pub async fn main() {
    env_logger::init();
    info!("Logger initialized!");
    let routes = warp::path::end()
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(websocket::user_connected));

    warp::serve(routes)
        .bind(([127, 0, 0, 1], 6767))
        .await
        .graceful(async move {
            let mut terminate =
                signal(SignalKind::terminate()).expect("failed to listen to shutdown signal");
            terminate.recv().await;
        })
        .run()
        .await;
}
