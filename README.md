# api-gateway

A Rust API Gateway built on top of pingora

Configuration:
| **ENV** | **Description** | **Default** |
|------------------|--------------------------------------------------------------|--------------------------|
| PORT | HTTP port that the gateway will be exposed on. | 80 |
| HEALTHCHECK_PORT | HTTP port that gateway healthcheck endpoint is available on. | 9000 |
| CONFIG_FILE | Path to the configuration file | /etc/gateway/config.json |
| REDIS_URL | Connection URL for redis database | redis://localhost:6379 |
