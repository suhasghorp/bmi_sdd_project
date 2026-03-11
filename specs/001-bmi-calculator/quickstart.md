# Quickstart: BMI Calculator Web Service

**Feature**: 001-bmi-calculator | **Date**: 2026-03-11

## Prerequisites

- Rust toolchain (1.85.0+ for edition 2024) — install via [rustup](https://rustup.rs/)
- `cargo` (included with Rust)

## Build

```bash
cargo build
```

## Run

```bash
# Default: binds to 127.0.0.1:3000
cargo run

# Custom port:
cargo run -- --port 8080

# With logging:
RUST_LOG=info cargo run

# Heroku-style (PORT env var takes precedence):
PORT=8080 cargo run

# Stop server: Press CTRL+C for graceful shutdown
```

**Note**: The server supports graceful shutdown via CTRL+C (SIGINT) or SIGTERM signals. Active connections will complete before shutdown.

## Test

```bash
cargo test
```

Runs both unit tests (domain logic) and integration tests (API endpoints via Reqwest).

## Usage

### Web UI

Open `http://127.0.0.1:3000/` in a browser. Enter weight (kg) and height (m), submit the form.

### API

```bash
# Calculate BMI
curl -X POST http://127.0.0.1:3000/api/bmi \
  -H "Content-Type: application/json" \
  -d '{"weight_kg": 70.0, "height_m": 1.75}'

# Response: {"bmi": 22.9, "category": "Normal"}

# Health check
curl http://127.0.0.1:3000/health
# Response: 200 OK
```

### Invalid Input

```bash
curl -X POST http://127.0.0.1:3000/api/bmi \
  -H "Content-Type: application/json" \
  -d '{"weight_kg": 0.0, "height_m": 1.75}'

# Response: 422 {"error": "weight_kg must be positive"}
```

## Deploy to Heroku

```bash
heroku create --buildpack emk/rust
git push heroku 001-bmi-calculator:main
```

The `Procfile` should contain: `web: ./target/release/bmi_sdd`
