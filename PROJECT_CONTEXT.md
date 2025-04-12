# Axum REST API Sample — Project Context

- **Rust (2024), Axum, async/Tokio**
- **Purpose:** Modular, production-ready REST API template
- **Features:** JWT auth (refresh, revoke), SQLx/Postgres, Redis, error handling, API versioning, Docker, CI/CD, E2E tests

**Structure:**
- `src/api/`: HTTP, routes, handlers, error, version
- `src/application/`: business logic, services, security, repo, config, state
- `src/domain/models/`: User, Account, Transaction
- `src/infrastructure/`: Postgres (migrations, queries), Redis
- Entry: `src/main.rs` → `application::app::run()`
- Exports: `src/lib.rs`

**API:** (see `docs/api-docs.md`)
- `/v1/` endpoints: auth (login, refresh, logout, revoke, cleanup), users (CRUD), accounts (CRUD), transactions (transfer, get), health, version
- JWT (access/refresh), roles in claims, RBAC
- Structured JSON errors (code, kind, trace, doc_url)

**Security:**
- JWT revocation (Redis), refresh rotation, RBAC, CORS, error hygiene, graceful shutdown

**Testing:**
- `tests/`: auth, user, account, transaction, health, version, E2E HTTP, test DB, isolation, `tests/common/` utils, `endpoints.http` samples

**Dev/Deploy:**
- Local: `docker-compose up -d`, `cargo run`, `.env`
- Test: `cargo test`
- Full stack: `docker-compose -f docker-compose.full.yml up -d`
- CI: GitHub Actions (lint, audit, test, build)

**Quick Reference:**
- Main: `src/main.rs`
- Routes: `src/api/routes/`
- Auth: `src/application/security/`
- Errors: `src/api/error.rs`
- DB: `src/infrastructure/database/postgres/migrations/`
- Redis: `src/infrastructure/redis/`
- Docs: `docs/api-docs.md`
- Tests: `tests/`
- Config: `.env`
