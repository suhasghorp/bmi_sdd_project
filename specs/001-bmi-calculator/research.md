# Research: BMI Calculator Web Service

**Feature**: 001-bmi-calculator | **Date**: 2026-03-11

## Research Tasks

### R-001: Rust Edition 2024 vs Constitution's Edition 2021

**Decision**: Use Rust edition 2024 as configured in Cargo.toml.

**Rationale**: Rust edition 2024 (stabilized in Rust 1.85.0, Feb 2025) is a superset of edition 2021. It introduces improvements like `gen` keyword reservation, lifetime capture rules changes, and `unsafe_op_in_unsafe_fn` lint. All edition 2021 code compiles under 2024. The constitution's reference to "edition 2021" predates the project setup; the Cargo.toml reflects the actual project configuration.

**Alternatives considered**: Downgrading to edition 2021 — rejected because edition 2024 is already configured and offers improvements with no drawbacks for this project.

**Action**: Note discrepancy in plan.md. Constitution amendment (patch-level) recommended as follow-up, not blocking.

### R-002: Axum Best Practices for Combined JSON API + Embedded HTML

**Decision**: Use Axum Router with separate route groups — API routes returning `Json<T>`, UI routes returning `Html<String>`.

**Rationale**: Axum's type-safe extractors and response types make this natural. `Json<BmiRequest>` for the API extractor, `Json<BmiResponse>` or `(StatusCode, Json<ErrorResponse>)` for API responses. `Html<&'static str>` for the embedded HTML page. The `axum::Router` composes these cleanly with `.route()`.

**Alternatives considered**:
- Separate Axum apps merged via `nest` — rejected, unnecessary complexity for 3 routes.
- Actix-web instead of Axum — rejected, constitution mandates Axum.

### R-003: Bootstrap CDN Embedding Approach

**Decision**: Embed a complete HTML string as a `&'static str` constant in `ui.rs`, referencing Bootstrap CSS and JS via CDN links (`https://cdn.jsdelivr.net/npm/bootstrap@5`).

**Rationale**: The spec requires "self-contained single-page interface with no external CSS/JS dependencies" at the asset level (no local files to serve), but the constitution explicitly specifies "Bootstrap via CDN." The HTML form uses `fetch()` to POST to `/api/bmi` and displays results inline. No build step needed.

**Alternatives considered**:
- Tera/Askama templating — rejected, overkill for a single static page (Simplicity principle).
- Include HTML via `include_str!` from a separate file — viable but adds a file for a single constant. Could be adopted later if HTML grows.

### R-004: Heroku Rust Buildpack Setup

**Decision**: Use `emk/rust` buildpack with a `Procfile` containing `web: ./target/release/bmi_sdd`.

**Rationale**: Heroku sets the `PORT` environment variable. Per constitution, PORT env var takes precedence over `--port` CLI flag. The Clap config should check `std::env::var("PORT")` first, then fall back to the CLI flag default.

**Alternatives considered**:
- Docker-based deployment — rejected, buildpack is simpler and sufficient.
- Shuttle.rs — rejected, constitution specifies Heroku.

### R-005: Error Handling Strategy

**Decision**: Use `thiserror` for domain errors (`BmiError` enum with `InvalidWeight` and `InvalidHeight` variants). Use Axum's `IntoResponse` to map domain errors to HTTP 422 responses with JSON `{"error": "..."}` body. Use `anyhow` only if needed at the application level (e.g., server startup failures).

**Rationale**: Constitution specifies `thiserror` for domain, `anyhow` for application. Domain errors are a closed set (only two variants), making `thiserror` ideal. The API layer implements `IntoResponse` for the domain error type to produce 422 JSON responses.

**Alternatives considered**:
- Custom error types without thiserror — rejected, more boilerplate for no benefit.
- Single error enum for everything — rejected, violates layer separation.

### R-006: Integration Test Approach with Reqwest

**Decision**: Integration tests start a real Axum server on a random port using `TcpListener::bind("127.0.0.1:0")`, then use Reqwest to send HTTP requests and assert on responses.

**Rationale**: This tests the full HTTP stack including serialization, routing, and status codes. Binding to port 0 lets the OS assign a free port, avoiding conflicts in CI. Each test gets its own server instance.

**Alternatives considered**:
- `axum::test` / `tower::ServiceExt` for handler-level testing — viable for unit-level handler tests but doesn't test the full HTTP stack. Could complement integration tests.
- Shared test server with `lazy_static` — rejected, port conflicts and test ordering issues.

## All NEEDS CLARIFICATION: Resolved

No unresolved items remain. All technical decisions are informed by the constitution and spec.
