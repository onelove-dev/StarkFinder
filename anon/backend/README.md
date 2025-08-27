# 🦀 StarkFinder Backend API

This is the backend service for StarkFinder, built with Rust, Axum, and Tokio. It provides a simple API with health check and request tracing capabilities.

---

## 🚀 Features

- **Health Check Endpoint**: `GET /healthz` returns `{"ok": true}`
- **Structured Logging**: JSON logs with unique request IDs via `tracing`
- **Environment Configuration**: Uses `.env` and `RUST_LOG` for runtime config
- **Graceful Shutdown**: Handles `Ctrl+C` and `termination signals` for clean shutdowns.
- **Modular Workspace**: Organized with a `cargo` workspace (`api/` crate)

---

## 📦 Dependencies

Key crates used:

| Crate               | Purpose                          |
|---------------------|----------------------------------|
| `axum`              | Web framework                    |
| `tokio`             | Async runtime                    |
| `tracing`           | Structured logging               |
| `tracing-subscriber`| JSON log formatting              |
| `dotenvy`           | Load `.env` configuration        |
| `uuid`              | Generate request IDs             |
| `serde_json`        | JSON serialization               |
| `hyper`             | HTTP server backend              |

---

## Getting Started
Prerequisites

 **Rust (latest stable)
 **Cargo
**dotenvy for environment variable management


