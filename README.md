# api-gateway

A Rust API Gateway built on top of pingora

## Running the gateway

`docker run --rm -v ./config.json:/etc/gateway/config.json -p 80:80 ghcr.io/majksa-dev/api-gateway:0`

All containers you wish to be accessible through the gateway must be on the same network with the gateway.
Configuration is done using environment variables.
Please keep in mind that it is expected that this gateway is expected to be hidden behind a reverse proxy. That reverse proxy **must** forward client's IP using the `X-Real-IP` header.

## Gateway Configuration

| **ENV**          | **Description**                                              | **Default**              |
| ---------------- | ------------------------------------------------------------ | ------------------------ |
| PORT             | HTTP port that the gateway will be exposed on.               | 80                       |
| HEALTHCHECK_PORT | HTTP port that gateway healthcheck endpoint is available on. | 9000                     |
| CONFIG_FILE      | Path to the configuration file                               | /etc/gateway/config.json |
| REDIS_URL        | Connection URL for redis database                            | redis://localhost:6379   |

## Configuration file reference

Json schema is available at: [./config.schema.json](https://raw.githubusercontent.com/majksa-dev/api-gateway/main/config.schema.json)

## Example configuration

```json
{
  "$schema": "https://raw.githubusercontent.com/majksa-dev/api-gateway/main/config.schema.json",
  "apps": {
    "app": {
      "upstream": {
        "host": "localhost",
        "port": 3005
      },
      "auth": [
        {
          "token": "hello",
          "origins": ["http://localhost:9000", "http://localhost:5500"],
          "quota": {
            "total": {
              "amount": 1000,
              "interval": {
                "amount": 1,
                "unit": "minutes"
              }
            },
            "user": {
              "amount": 10,
              "interval": {
                "amount": 1,
                "unit": "minutes"
              }
            }
          }
        }
      ],
      "endpoints": [
        {
          "path": "^/api/hello$",
          "id": "hello",
          "method": "GET",
          "headers": [],
          "rate-limit": {
            "amount": 100,
            "interval": {
              "amount": 1,
              "unit": "minutes"
            }
          },
          "websocket": false
        },
        {
          "path": "^/api/update/\\w+$",
          "id": "update",
          "method": "POST",
          "headers": [],
          "rate-limit": {
            "amount": 1,
            "interval": {
              "amount": 1,
              "unit": "minutes"
            }
          },
          "websocket": true
        }
      ]
    }
  }
}
```
