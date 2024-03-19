use std::sync::Arc;

use log::{error, info};
use structopt::StructOpt;

use pingora::proxy::http_proxy_service;
use pingora::server::configuration::Opt;
use pingora::server::Server;

use crate::app::gateway::Gateway;
use crate::env::Env;

use super::health_check::HealthCheck;

fn prepare() -> Option<(Server, Gateway, Arc<Env>)> {
    let env = Arc::new(
        Env::new()
            .map_err(|e| {
                error!("Could not load environment variables: {}", e);
                0
            })
            .ok()?,
    );

    let gateway = Gateway::new(&env)
        .map_err(|e| {
            error!("Could not create gateway: {}", e);
            0
        })
        .ok()?;

    let opt = Opt::from_args();
    let my_server = Server::new(Some(opt))
        .map_err(|e| {
            error!("Could not create server: {}", e);
            0
        })
        .ok()?;
    Some((my_server, gateway, env))
}

pub fn create_server() -> Option<Server> {
    let (mut my_server, gateway, env) = match prepare() {
        Some(data) => data,
        None => return None,
    };

    my_server.bootstrap();

    {
        let mut service = http_proxy_service(&my_server.configuration, gateway);
        let server = format!("127.0.0.1:{}", env.port);
        service.add_tcp(server.as_str());
        info!("Listening on {}", server);
        my_server.add_service(service);
    }

    {
        let mut service = http_proxy_service(&my_server.configuration, HealthCheck);
        let server = format!("127.0.0.1:{}", env.health_check_port);
        service.add_tcp(server.as_str());
        info!("Healthcheck listening on {}", server);
        my_server.add_service(service);
    }
    Some(my_server)
}
