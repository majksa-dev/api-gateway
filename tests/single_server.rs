use essentials::{debug, info};
use pretty_assertions::assert_eq;
use serde_json::json;
use std::{
    env,
    net::{SocketAddr, TcpListener},
    process::Child,
};
use testing_utils::{
    fs::{
        self,
        fixture::{FileTouch, FileWriteStr, PathChild},
    },
    get_random_ports, macros as utils, server_cmd, surf,
    testcontainers::{
        core::{ContainerPort, WaitFor},
        runners::AsyncRunner,
        ContainerAsync, GenericImage,
    },
};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

fn single_server_config(ports: &[u16]) -> serde_json::Value {
    json!({
        "apps": {
            "app": {
                "upstream": {
                    "host": "127.0.0.1",
                    "port": ports[0]
                },
                "auth": [
                    {
                        "token": "token",
                        "origins": ["http://localhost:3000"]
                    }
                ],
                "endpoints": [
                    {
                        "path": "/hello",
                        "id": "hello",
                        "method": "GET"
                    }
                ]
            }
        }
    })
}

struct Context {
    cmd: Child,
    app: u16,
    redis_cache: ContainerAsync<GenericImage>,
    redis_rate_limiter: ContainerAsync<GenericImage>,
    _mock_server: MockServer,
}

async fn before_each() -> Context {
    env::set_var("RUST_LOG", "debug");
    essentials::install();
    let redis_cache: ContainerAsync<GenericImage> = GenericImage::new("redis", "7.2.4")
        .with_exposed_port(ContainerPort::Tcp(6379))
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        .start()
        .await
        .expect("Redis started");
    let redis_cache_port = redis_cache.get_host_port_ipv4(6379).await.unwrap();
    let redis_rate_limiter: ContainerAsync<GenericImage> = GenericImage::new("redis", "7.2.4")
        .with_exposed_port(ContainerPort::Tcp(6379))
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        .start()
        .await
        .expect("Redis started");
    let redis_rate_limiter_port = redis_rate_limiter.get_host_port_ipv4(6379).await.unwrap();
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0))).unwrap();
    let origin = listener.local_addr().unwrap().port();
    let mock_server = MockServer::builder().listener(listener).start().await;
    Mock::given(method("GET"))
        .and(path("/hello"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let ports = get_random_ports(2);
    let config = single_server_config(&[origin]);
    let temp = fs::TempDir::new().unwrap();
    let input_file = temp.child("config.json");
    input_file.touch().unwrap();
    input_file.write_str(&config.to_string()).unwrap();
    debug!("Provided config: {}", config.to_string());
    let app = server_cmd()
        .env("RUST_BACKTRACE", "full")
        .env("RUST_LOG", "debug")
        .env("PORT", ports[0].to_string())
        .env("HEALTHCHECK_PORT", ports[1].to_string())
        .env("CONFIG_FILE", input_file.path())
        .env(
            "REDIS_RATE_LIMITER_URL",
            format!("redis://localhost:{}", redis_rate_limiter_port),
        )
        .env(
            "REDIS_CACHE_URL",
            format!("redis://localhost:{}", redis_cache_port),
        )
        .spawn()
        .unwrap();
    for _ in 0..20 {
        if let Ok(status) = surf::get(format!("http://localhost:{}", &ports[1].to_string()))
            .await
            .map(|res| res.status())
        {
            if status == 200 {
                info!("Server started on port {}", ports[0].to_string());
                return Context {
                    cmd: app,
                    app: ports[0],
                    redis_cache,
                    redis_rate_limiter,
                    _mock_server: mock_server,
                };
            }
        }
        // Sleep for 5 seconds
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
    panic!("Could not start the server");
}

async fn after_each(mut ctx: Context) {
    ctx.cmd.kill().unwrap();
    ctx.cmd.wait().unwrap();
    ctx.redis_cache
        .stop()
        .await
        .expect("Redis could not be stopped");
    ctx.redis_rate_limiter
        .stop()
        .await
        .expect("Redis could not be stopped");
}

#[utils::test(setup = before_each, teardown = after_each)]
async fn should_succeed(ctx: Context) -> Context {
    let status = surf::get(format!("http://127.0.0.1:{}/hello", &ctx.app))
        .header("X-Real-IP", "1.2.3.4")
        .header("X-Api-Token", "token")
        .header("Origin", "http://localhost:3000")
        .header("Host", "app")
        .await
        .unwrap()
        .status();
    assert_eq!(status as u16, 200);
    ctx
}

#[utils::test(setup = before_each, teardown = after_each)]
async fn should_fail_when_calling_without_host(ctx: Context) -> Context {
    let status = surf::get(format!("http://127.0.0.1:{}/hello", &ctx.app))
        .await
        .unwrap()
        .status();
    assert_eq!(status as u16, 502);
    ctx
}

#[utils::test(setup = before_each, teardown = after_each)]
async fn should_fail_when_calling_valid_endpoint_without_token(ctx: Context) -> Context {
    let status = surf::get(format!("http://127.0.0.1:{}/hello", &ctx.app))
        .header("X-Real-IP", "1.2.3.4")
        .header("Origin", "http://localhost:3000")
        .header("Host", "app")
        .await
        .unwrap()
        .status();
    assert_eq!(status as u16, 401);
    ctx
}

#[utils::test(setup = before_each, teardown = after_each)]
async fn should_fail_when_calling_valid_endpoint_without_ip(ctx: Context) -> Context {
    let status = surf::get(format!("http://127.0.0.1:{}/hello", &ctx.app))
        .header("X-Api-Token", "token")
        .header("Origin", "http://localhost:3000")
        .header("Host", "app")
        .await
        .unwrap()
        .status();
    assert_eq!(status as u16, 400);
    ctx
}

#[utils::test(setup = before_each, teardown = after_each)]
async fn should_fail_when_calling_valid_endpoint_without_origin(ctx: Context) -> Context {
    let status = surf::get(format!("http://127.0.0.1:{}/hello", &ctx.app))
        .header("X-Real-IP", "1.2.3.4")
        .header("X-Api-Token", "token")
        .header("Host", "app")
        .await
        .unwrap()
        .status();
    assert_eq!(status as u16, 400);
    ctx
}

#[utils::test(setup = before_each, teardown = after_each)]
async fn should_fail_when_calling_invalid_endpoint(ctx: Context) -> Context {
    let status = surf::get(format!("http://127.0.0.1:{}/unknown", &ctx.app))
        .header("X-Real-IP", "1.2.3.4")
        .header("X-Api-Token", "token")
        .header("Origin", "http://localhost:3000")
        .header("Host", "app")
        .await
        .unwrap()
        .status();
    assert_eq!(status as u16, 403);
    ctx
}
