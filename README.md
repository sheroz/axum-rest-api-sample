# Getting started with REST API Web Services in Rust

[![build & test](https://github.com/sheroz/axum-web/actions/workflows/ci.yml/badge.svg)](https://github.com/sheroz/axum-web/actions/workflows/ci.yml)
[![MIT](https://img.shields.io/github/license/sheroz/axum-web)](https://github.com/sheroz/axum-web/tree/main/LICENSE)

Kick-start template for building REST API Web service in Rust using `axum`, `JSON Web Tokens (JWT)`, `PostgreSQL`, and `Redis`

Covers:

- REST API based on [axum](https://github.com/tokio-rs/axum)
  - routing
  - api versioning
  - CORS settings
  - basic error handling
  - graceful shutdown
- `JSON Web Tokens (JWT)` based authentication & authorization
  - login, logout, refresh, and revoking operations
  - role based authorization
  - generating and validating of access and refresh tokens
  - setting the tokens expiry time (based on configuration)
  - using the refresh tokens rotation technique
  - revoking the issued tokens by using `Redis` (based on configuration)
    - revoke all tokens issued until the current time
    - revoke tokens belonging to a user issued until the current time
    - cleanup of revoked tokens
- `PostgreSQL`database with `SQLx` driver
  - database migrations
  - async connection pooling
  - async CRUD operations
- `Redis` in-memory storage
  - async operations
- Configuration settings
  - loading and parsing `.env` file
  - using environment variables
- Logs
  - `tracing` based logs
- Tests
  - `Docker` based end-to-end tests
  - GitHub CI configuration for running tests
- `Docker` based configurations
  - `PostgreSQL` and `Redis` services
  - building a full stack services: API + `PostgreSQL` + `Redis`

## Run

Running the REST API service (debug release):

```shell
docker-compose up -d
cargo run
```

Running the Docker based full stack build: [docker-compose.full.yml](docker-compose.full.yml)

```shell
docker-compose -f docker-compose.full.yml up -d
```

## REST API endpoints

Please check for the list of available REST API endpoints: [tests/endpoints.http](/tests/endpoints.http)

REST API endpoints can be easily tested using following tools:

- [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) for Visual Studio Code.
- [curl](https://curl.se/) samples:

  Heartbeat

  ```shell
  curl -i http://127.0.0.1:8080/v1/heartbeat/1
  ```

  Login

  ```shell
  curl -i http://127.0.0.1:8080/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password_hash":"7c44575b741f02d49c3e988ba7aa95a8fb6d90c0ef63a97236fa54bfcfbd9d51"}'
  ```

  List of users

  ```shell
  curl -i http://127.0.0.1:8080/v1/users \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJkNTFlNjE4Ny1jYmFjLTQ0ZmEtOWE5NS04ZjFkZWJkYmFlZWEiLCJqdGkiOiIwN2Y3OWE0OC1kMWFhLTQ1ZjItOWE5NS05Y2M5MGZiY2UyYTciLCJpYXQiOjE3MzYwMTA3MjIsImV4cCI6MTczNjAxNDMyMiwidHlwIjowLCJyb2xlcyI6ImFkbWluIn0.3f2c_5PyPXMhgu0FIX4--SGjnSDW1GLxL0ba6gSImfM"
  ```

## Tests

REST API tests: [/tests](/tests)

Running tests:

```shell
docker-compose up -d
cargo test
```

Running the service in test configuration:

```shell
ENV_TEST=1 cargo run
```

## Logging

Setting the `RUST_LOG` - logging level on the launch:

```shell
RUST_LOG=info,hyper=debug,axum_web=trace cargo run
```

## Project Stage

**Development**: this project is under development.
