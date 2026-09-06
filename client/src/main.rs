use log::debug;
use log::info;

mod app;
mod core;
mod login;
mod websocket;

fn main() {
    env_logger::init();
    info!("Logger initialized!");
    app::run();
    debug!("exiting app");
}
