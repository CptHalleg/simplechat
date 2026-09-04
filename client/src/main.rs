use log::{debug, info};

mod app;
mod connection;
mod core;
mod login;

fn main() {
    env_logger::init();
    info!("Logger initialized!");
    app::run();
    debug!("exiting app");
}
