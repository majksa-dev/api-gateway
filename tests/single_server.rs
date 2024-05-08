use serde_json::json;
use std::net::SocketAddr;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn single_server_config(ports: &Vec<u16>) -> serde_json::Value {
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
                        "path": "^/hello$",
                        "id": "hello",
                        "method": "GET",
                        "headers": []
                    }
                ]
            }
        }
    })
}

async fn before_each(ctx: &helper::Context) -> MockServer {
    let listener =
        std::net::TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], ctx.services[0]))).unwrap();
    let mock_server = MockServer::builder().listener(listener).start().await;
    Mock::given(method("GET"))
        .and(path("/hello"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    mock_server
}

#[helper::test(servers = 1, config = single_server_config)]
async fn should_succeed(ctx: helper::Context) {
    let _server = before_each(&ctx).await;
    let status = surf::get(format!("http://127.0.0.1:{}/hello", &ctx.app))
        .header("X-Real-IP", "1.2.3.4")
        .header("X-Api-Token", "token")
        .header("Origin", "http://localhost:3000")
        .header("Host", "app")
        .await
        .unwrap()
        .status();
    assert_eq!(status as u16, 200);
}

#[helper::test(servers = 1, config = single_server_config)]
async fn should_fail_when_calling_without_host(ctx: helper::Context) {
    let _server = before_each(&ctx).await;
    let status = surf::get(format!("http://127.0.0.1:{}/hello", &ctx.app))
        .await
        .unwrap()
        .status();
    assert_eq!(status as u16, 502);
}

#[helper::test(servers = 1, config = single_server_config)]
async fn should_fail_when_calling_valid_endpoint_without_token(ctx: helper::Context) {
    let _server = before_each(&ctx).await;
    let status = surf::get(format!("http://127.0.0.1:{}/hello", &ctx.app))
        .header("X-Real-IP", "1.2.3.4")
        .header("Origin", "http://localhost:3000")
        .header("Host", "app")
        .await
        .unwrap()
        .status();
    assert_eq!(status as u16, 401);
}

#[helper::test(servers = 1, config = single_server_config)]
async fn should_fail_when_calling_valid_endpoint_without_ip(ctx: helper::Context) {
    let _server = before_each(&ctx).await;
    let status = surf::get(format!("http://127.0.0.1:{}/hello", &ctx.app))
        .header("X-Api-Token", "token")
        .header("Origin", "http://localhost:3000")
        .header("Host", "app")
        .await
        .unwrap()
        .status();
    assert_eq!(status as u16, 200);
}

#[helper::test(servers = 1, config = single_server_config)]
async fn should_fail_when_calling_valid_endpoint_without_origin(ctx: helper::Context) {
    let _server = before_each(&ctx).await;
    let status = surf::get(format!("http://127.0.0.1:{}/hello", &ctx.app))
        .header("X-Real-IP", "1.2.3.4")
        .header("X-Api-Token", "token")
        .header("Host", "app")
        .await
        .unwrap()
        .status();
    assert_eq!(status as u16, 401);
}

#[helper::test(servers = 1, config = single_server_config)]
async fn should_fail_when_calling_invalid_endpoint(ctx: helper::Context) {
    let _server = before_each(&ctx).await;
    let status = surf::get(format!("http://127.0.0.1:{}/unknown", &ctx.app))
        .header("X-Real-IP", "1.2.3.4")
        .header("X-Api-Token", "token")
        .header("Origin", "http://localhost:3000")
        .header("Host", "app")
        .await
        .unwrap()
        .status();
    assert_eq!(status as u16, 404);
}
