# Implementation Plan: BMI Calculator Web Service

**Branch**: `001-bmi-calculator` | **Date**: 2026-03-11 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-bmi-calculator/spec.md`

## Summary

Build a stateless BMI calculator web service in Rust using Axum, with three cleanly separated layers: a pure domain layer for BMI calculation and WHO category classification, an API layer exposing a JSON REST endpoint, and a UI layer serving an embedded Bootstrap HTML form. The server is CLI-configured via Clap with tracing-based observability, deployed to Heroku.

## Technical Context

**Language/Version**: Rust edition 2024 (per constitution v1.0.1 and Cargo.toml)
**Primary Dependencies**: Axum, Tokio, Serde + serde_json, thiserror, anyhow, tracing, tracing-subscriber, Clap, Reqwest (dev)
**Storage**: N/A (stateless — no database, no persistence)
**Testing**: `cargo test` — unit tests for domain, integration tests with Reqwest for API endpoints
**Target Platform**: Linux server (Heroku via Rust buildpack with Procfile)
**Project Type**: Web service (single binary)
**Performance Goals**: <1s response time (SC-001), server ready in <3s (SC-005)
**Constraints**: No database, no persistence, no sessions, no external JS/CSS assets except Bootstrap CDN
**Scale/Scope**: Single-purpose BMI calculator, single binary deployment

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Domain Purity | PASS | Domain module contains only pure `calculate_bmi` function and `BmiCategory` classification. No I/O, no Serde, no framework deps. |
| II. Clean Layer Separation | PASS | Three layers: `domain` (pure calc), `api` (Axum handlers + Serde types), `ui` (embedded HTML). No cross-layer leakage. |
| III. Test-First Development | PASS | Unit tests for domain (boundary values, edge cases), integration tests for API (valid/invalid requests). All via `cargo test`. |
| IV. Stateless Design | PASS | Every request independently computable. No storage, no caching, no sessions. |
| V. Observability | PASS | `tracing` + `tracing-subscriber` with CLI-configurable log level via Clap. Structured logging with input context on errors. |
| VI. Simplicity | PASS | Flat module structure. No abstractions beyond what spec requires. No input range clamping, no imperial units, no history. |

**Gate Result**: ALL PASS — proceed to Phase 0.

## Project Structure

### Documentation (this feature)

```text
specs/001-bmi-calculator/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── api.md           # REST API contract
└── tasks.md             # Phase 2 output (created by /speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── main.rs              # Entry point: Clap CLI parsing, tracing setup, Axum server
├── domain.rs            # Pure BMI calculation and WHO category classification
├── api.rs               # Axum handlers, JSON request/response Serde types, error mapping
└── ui.rs                # Embedded HTML string, Axum handler serving the Bootstrap page

tests/
└── api_integration.rs   # Reqwest-based integration tests against running server

Cargo.toml               # Dependencies
Procfile                  # Heroku deployment
```

**Structure Decision**: Flat module structure (single `src/` directory with one file per layer) chosen per Simplicity principle. The project is small enough that nested module directories add no value.

## Constitution Re-Check (Post Phase 1 Design)

| Principle | Status | Post-Design Notes |
|-----------|--------|-------------------|
| I. Domain Purity | PASS | `data-model.md` confirms: `BmiCategory`, `BmiResult`, `BmiError` have no Serde derives. `calculate_bmi` is a pure function with no I/O. |
| II. Clean Layer Separation | PASS | API types (`BmiRequest`, `BmiResponse`, `ErrorResponse`) have Serde derives and live in `api.rs`. Domain types are converted at the API boundary. |
| III. Test-First Development | PASS | Test strategy defined: unit tests for domain boundary values (18.5, 25.0, 30.0) and edge cases; integration tests with Reqwest for full HTTP stack. |
| IV. Stateless Design | PASS | No state in data model. No entity lifecycle. Every request is independently computable. |
| V. Observability | PASS | `tracing` spans can wrap handler execution. Structured error logging includes invalid input values per research.md. |
| VI. Simplicity | PASS | 4 source files, 3 entities, 1 function, 3 HTTP routes. No abstractions beyond what spec requires. |

**Post-Design Gate Result**: ALL PASS — ready for Phase 2 task generation.

## Complexity Tracking

No constitution violations — table not applicable.
