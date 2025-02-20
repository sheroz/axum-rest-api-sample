# Getting started with REST API Web Services in Rust using Axum, JWT, SQLx, PostgreSQL, and Redis

[![build & test](https://github.com/sheroz/axum-web/actions/workflows/ci.yml/badge.svg)](https://github.com/sheroz/axum-web/actions/workflows/ci.yml)
[![MIT](https://img.shields.io/github/license/sheroz/axum-web)](https://github.com/sheroz/axum-web/tree/main/LICENSE)

This project demonstrates how to build a REST API web server in Rust using `axum`, `JSON Web Tokens (JWT)`, `SQLx`, `PostgreSQL`, and `Redis`

The REST API web server supports JWT-based authentication and authorization, asynchronous database operations for user and account models, a basic transaction example that transfers money between accounts, and detailed API error handling in a structured format.

The brief description: [rust-axum-rest-api-postgres-redis-jwt-docker.html](https://sheroz.com/pages/blog/rust-axum-rest-api-postgres-redis-jwt-docker.html)

## Covers

- REST API web server based on [axum](https://github.com/tokio-rs/axum)
  - Routing and request handling
  - API versioning
  - API Error handling using structured format
  - Cross-Origin Resource Sharing (CORS)
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
- Using `PostgreSQL`database with `SQLx`
  - Database migrations
  - Async connection pooling
  - Async CRUD operations and transactions
- Using `Redis` in-memory storage
  - Async `Redis` operations
- Configuration settings
  - Loading and parsing `.env` file
  - Using environment variables
- Logs
  - `tracing` based logs
- Tests
  - End-to-end API tests
  - Database isolation in tests
- Using `Docker`
  - Running `PostgreSQL` and `Redis` services
  - Building the application using the official `Rust` image
  - Running the full stack: API + `PostgreSQL` + `Redis`
- GitHub CI configuration
  - Running `cargo deny` to check for security vulnerabilities and licenses
  - Running `cargo fmt` to check for the Rust code format according to style guidelines
  - Running `cargo clippy` to catch common mistakes and improving the Rust code
  - Running tests
  - Building the application

## REST API Endpoints

- List of available API endpoints: [docs/api-docs.md](/docs/api-docs.md)
- API request samples in the format RFC 2616: [tests/endpoints.http](/tests/endpoints.http)

## API Request Samples

- Using [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) for Visual Studio Code,
supports RFC 2616, request samples: [tests/endpoints.http](/tests/endpoints.http).
- Using [curl](https://curl.se/):

  Health check

  ```shell
  curl -i http://127.0.0.1:8080/v1/health
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

## Running end-to-end API tests

REST API tests: [/tests](/tests)

```shell
docker-compose up -d
cargo test
```

## Running the service (debug build)

```shell
docker-compose up -d
cargo run
```

## Running the service in test configuration

```shell
ENV_TEST=1 cargo run
```

## Running the service at a specific log level

Setting the `RUST_LOG` - logging level on the launch:

```shell
RUST_LOG=info,hyper=debug,axum_web=trace cargo run
```

## Running the Docker based full stack build

```shell
docker-compose -f docker-compose.full.yml up -d
```

## Project Stage

**Development**: this project is under development.
