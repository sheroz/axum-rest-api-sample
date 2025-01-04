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
- `.env` based configuration parsing
- `tracing` based logs
- `Docker` based configurations
  - `PostgreSQL` and `Redis` services
  - Building a full stack: API + `PostgreSQL` + `Redis`
- Tests
  - `Docker` based end-to-end tests
  - GitHub CI configuration for running tests

## REST API

Please check for available API: [tests/endpoints.http](/tests/endpoints.http)

Visual Studio Code [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) is helpfull to play with API.

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

Running the service in test configuration:

```shell
ENV_TEST=1 cargo run
```

## Tests

REST API tests: [/tests](/tests)

Running the API tests:

```shell
docker-compose up -d
cargo test
```

## Logging

Setting the `RUST_LOG` - logging level on the launch:

```shell
RUST_LOG=info,hyper=debug,axum_web=trace cargo run
```

## Project Stage

**Development**: this project is under development.
