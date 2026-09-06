use log::info;
use tokio::signal::unix::SignalKind;
use tokio::signal::unix::signal;
use warp::Filter;

pub mod database;
pub mod routes;

use warp::Reply;

pub async fn serve_and_run<F>(filter: F)
where
    F: Filter + Clone + Send + Sync + 'static,
    F::Extract: Reply + Send,
    F::Error: Send + Sync + 'static,
{
    warp::serve(filter)
        .bind(([127, 0, 0, 1], 6767))
        .await
        .graceful(async move {
            let mut terminate =
                signal(SignalKind::terminate()).expect("failed to listen to shutdown signal");
            terminate.recv().await;
            info!("Shutdown signal recieved, shutting down");
        })
        .run()
        .await;
}
