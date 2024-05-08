use essentials::{debug, error};

mod app;
mod config;
mod env;
mod utils;

#[tokio::main]
async fn main() {
    essentials::install();
    let env = match env::Env::new() {
        Ok(env) => {
            debug!("Environment: {:?}", env);
            env
        }
        Err(err) => {
            error!("Failed to parse environment: {}", err);
            return;
        }
    };
    match app::build(env).await {
        Ok(server) => server.run().await,
        Err(err) => {
            error!("Failed to build the server: {}", err);
        }
    }
}
