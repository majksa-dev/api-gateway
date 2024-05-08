use serde_json::json;

fn single_server_config(ports: &Vec<u16>) -> serde_json::Value {
    json!(
        {
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
        }
    )
}

#[helper::test(servers = 1, config = single_server_config)]
fn single_server(_ctx: helper::Context) {}

fn main() {}
