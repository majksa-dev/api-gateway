mod app;
mod env;
mod http;
mod utils;

use http::create_server;
use log::{error, warn};

fn main() {
    println!("Hello, world!");
    env_logger::init();
    if color_eyre::install().is_err() {
        warn!("Failed to install color_eyre");
    }

    if let Some(mut server) = create_server() {
        server.run_forever();
    } else {
        error!("Something went wrong, exiting...");
    }
}
