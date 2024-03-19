mod utils;

use log::warn;

fn main() {
    env_logger::init();
    if color_eyre::install().is_err() {
        warn!("Failed to install color_eyre");
    }
}
