use assert_fs::prelude::*;
use futures_util::Future;
use log::{debug, info};
use serde_json::Value;

pub use test_macros::test;

mod bin;
mod redis;
mod utils;

#[derive(Clone)]
pub struct Context {
    pub app: u16,
    pub health_check: u16,
    pub redis: u16,
    pub services: Vec<u16>,
}

struct Processes {
    pub app: std::process::Child,
    pub redis: std::process::Child,
}

async fn setup(config: Value) -> (Processes, Context) {
    utils::setup_env();
    let ports_vec = utils::get_random_ports(3);
    let ports = Context {
        app: ports_vec[0],
        health_check: ports_vec[1],
        redis: ports_vec[2],
        services: vec![],
    };
    let temp = assert_fs::TempDir::new().unwrap();
    let input_file = temp.child("config.json");
    input_file.touch().unwrap();
    input_file.write_str(&config.to_string()).unwrap();
    debug!("Provided config: {}", config.to_string());
    info!("Starting redis on port {}", ports.redis.to_string());
    let redis = redis::start_redis(ports.redis).spawn().unwrap();
    let app = bin::server_cmd()
        .env("RUST_BACKTRACE", "full")
        .env("RUST_LOG", "debug")
        .env("PORT", ports.app.to_string())
        .env("HEALTHCHECK_PORT", ports.health_check.to_string())
        .env("CONFIG_FILE", input_file.path())
        .env(
            "REDIS_URL",
            format!("redis://localhost:{}", ports.redis.to_string()),
        )
        .spawn()
        .unwrap();
    for _ in 0..10 {
        if let Ok(status) = surf::get(format!(
            "http://localhost:{}",
            &ports.health_check.to_string()
        ))
        .await
        .and_then(|res| Ok(res.status()))
        {
            if status == 200 {
                info!("Server started on port {}", ports.app.to_string());
                return (Processes { app, redis }, ports);
            }
        }
        // Sleep for 5 seconds
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    panic!("Could not start the server");
}

async fn teardown(ctx: &mut Processes) {
    ctx.app.kill().unwrap();
    ctx.redis.kill().unwrap();
    ctx.app.wait().unwrap();
    ctx.redis.wait().unwrap();
}

pub async fn test<T, C, Fut>(servers: u16, config: C, test: T)
where
    C: Fn(&Vec<u16>) -> Value,
    T: FnOnce(Context) -> Fut,
    Fut: Future<Output = ()>,
{
    let apps = utils::random_listeners(servers);
    let services_ports: Vec<_> = apps
        .iter()
        .map(|listener| listener.local_addr().unwrap().port())
        .collect();
    let config = config(&services_ports);
    let (mut ctx, mut ports) = setup(config).await;
    drop(apps); // Drop the listeners
    ports.services = services_ports;
    test(ports.clone()).await;
    teardown(&mut ctx).await;
}
