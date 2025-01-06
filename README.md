# Getting started with REST API Web Services in Rust using Axum, PostgreSQL, Redis, and JWT

[![build & test](https://github.com/sheroz/axum-web/actions/workflows/ci.yml/badge.svg)](https://github.com/sheroz/axum-web/actions/workflows/ci.yml)
[![MIT](https://img.shields.io/github/license/sheroz/axum-web)](https://github.com/sheroz/axum-web/tree/main/LICENSE)

A sample starter project for building REST API Web service in Rust using `axum`, `JSON Web Tokens (JWT)`, `PostgreSQL`, and `Redis`

Covers:

- REST API based on [axum](https://github.com/tokio-rs/axum)
  - Routing
  - API versioning
  - CORS settings
  - Error handling (basic)
  - Graceful shutdown
- Authentication & authorization using `JSON Web Tokens (JWT)`
  - Login, logout, refresh, and revoking operations
  - Role based authorization
  - Generating and validating access and refresh tokens
  - Setting tokens expiry time (based on configuration)
  - Using refresh tokens rotation technique
  - Revoking issued tokens by using Redis (based on configuration)
    - Revoke all tokens issued until the current time
    - Revoke tokens belonging to the user issued until the current time
    - Cleanup of revoked tokens
- Using `PostgreSQL`database with `SQLx` driver
  - Database migrations
  - Async connection pooling
  - Async CRUD operations
- Using `Redis` in-memory storage
  - Async `Redis` operations
- Configuration settings
  - Loading and parsing `.env` file
  - Using environment variables
- Logs
  - `tracing` based logs
- Tests
  - `Docker` based end-to-end tests
  - GitHub CI configuration for running tests
- Using `Docker` for running services
  - `PostgreSQL` and `Redis`
  - Building the application using the official `Rust` image
  - Running the full stack: API + `PostgreSQL` + `Redis`

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
