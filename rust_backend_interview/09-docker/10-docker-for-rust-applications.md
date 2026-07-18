# Docker for Rust Applications

## Interview Question

How do you optimize Docker for building and deploying Rust applications?

## Interview Answer

Optimizing Docker for Rust involves leveraging multi-stage builds to separate the heavy build toolchain from the minimal runtime, caching Cargo dependencies aggressively to avoid recompilation, using `cargo chef` or similar tools for deterministic dependency builds, choosing appropriate base images (distroless or slim), and producing small final images. Rust binaries are ideal for Docker because they compile to native code without runtime dependencies, enabling extremely minimal images that start in milliseconds and have minimal attack surface.

---

## Follow-up Questions & Answers

### Q1. What is `cargo chef` and how does it improve Docker builds?

**Interview Answer**

`cargo chef` is a tool that generates a dependency recipe from your `Cargo.toml` and `Cargo.lock`, then pre-builds all dependencies in a separate Docker layer. The recipe step creates a dummy project matching your dependencies, builds it, and caches the result. When your source code changes, only your code recompiles — all dependencies remain cached. This reduces incremental Rust Docker builds from 5-10 minutes to under 30 seconds for code-only changes.

---

### Q2. Show an optimized Rust Dockerfile using cargo chef.

**Interview Answer**

```dockerfile
FROM rust:1.75-bookworm AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/my_server /usr/local/bin/
EXPOSE 8080
CMD ["my_server"]
```

This separates dependency building from code compilation for maximum cache efficiency.

---

### Q3. How do you build static Rust binaries for Docker?

**Interview Answer**

Install the musl target with `rustup target add x86_64-unknown-linux-musl` and build with `cargo build --release --target x86_64-unknown-linux-musl`. This produces a fully static binary that works on scratch or distroless images. You may need `musl-tools` and `pkg-config` for C dependencies. For pure Rust projects without C dependencies, this works seamlessly. For projects using OpenSSL, consider switching to `rustls` to avoid musl compatibility issues.

---

### Q4. How do you handle Cargo workspace builds in Docker?

**Interview Answer**

Copy the entire workspace's `Cargo.toml` files and `Cargo.lock` first, create dummy `lib.rs`/`main.rs` files, and build to cache all workspace dependencies. Then copy the real source and rebuild. With `cargo chef`, this is handled automatically — it detects workspace members and generates recipes for the entire workspace. The key insight is that Docker layer caching works on file changes, so dependency layers must be invalidated only when `Cargo.toml` or `Cargo.lock` change.

---

### Q5. How do you minimize the final Rust Docker image size?

**Interview Answer**

Use multi-stage builds with a slim runtime image. Strip the binary with `strip --strip-all` or compile with `strip = true` in `Cargo.toml` profile. Use `opt-level = "z"` for size optimization if binary size matters more than performance. Avoid dynamic linking by targeting musl. Remove debug symbols with `cargo build --release` (no debug info by default). A typical Rust web server in a distroless image can be under 30MB total, compared to 1GB+ for the build stage.

---

### Q6. How do you handle Rust crates with C dependencies in Docker?

**Interview Answer**

C dependencies like OpenSSL, libpq, or zlib require system packages in the builder stage. Install them with `apt-get install -y libssl-dev pkg-config` (builder) and `apt-get install -y libssl3` (runtime) if dynamically linked. For static linking, use musl and install musl-compatible versions. Consider switching to pure-Rust alternatives: `rustls` instead of `openssl`, `tokio-postgres` instead of `libpq-sys`. This eliminates C dependency complexity and enables scratch/distroless runtime images.

---

### Q7. How do you handle runtime configuration in Dockerized Rust apps?

**Interview Answer**

Use environment variables for runtime configuration — Rust reads them with `std::env::var` or crates like `config` and `figment`. Mount configuration files as volumes for complex configs. For secrets, use Docker secrets or mounted files rather than environment variables. The `dotenv` crate can load `.env` files in development. In production, inject configuration via orchestrator environment settings, Docker Compose `env_file`, or Kubernetes ConfigMaps and Secrets.

---

### Q8. How do you handle database migrations for Rust apps in Docker?

**Interview Answer**

Run migrations as an init container or a Docker Compose service that starts before the main application. Use crates like `sqlx` or `diesel` for migration management. In Docker Compose, define a migration service with `depends_on` ordering to ensure the database is healthy before migrations run. Never run migrations inside the main application container in production — if two instances start simultaneously, they could run concurrent migrations. Use `sqlx migrate run` as a separate Docker command.

---

### Q9. How do you set up CI/CD for Dockerized Rust applications?

**Interview Answer**

Use GitHub Actions, GitLab CI, or similar to build, test, and push Docker images. Cache Rust dependencies using `actions/cache` on `~/.cargo` and `target/` directories. Build Docker images with `docker buildx` for multi-platform support. Scan images with Trivy or Docker Scout before pushing. Tag images with Git SHA and semantic version. For Rust, run `cargo test` and `cargo clippy` before building the Docker image to catch issues early. Push to a private registry and deploy via the orchestrator.

---

### Q10. How do you handle Rust application graceful shutdown in Docker?

**Interview Answer**

Docker sends `SIGTERM` when stopping a container. In Rust with Actix-web or Axum, use `tokio::signal::ctrl_c()` or the `signal-hook` crate to catch the signal. Implement a shutdown handler that stops accepting new connections, waits for in-flight requests to complete, and closes database pool connections. Set `stop_grace_period` in Docker Compose to give the application enough time. For Actix-web, use `HttpServer::workers()` with a shutdown signal; for Axum, use `axum::Server::with_graceful_shutdown()`.

```rust
let shutdown_signal = async {
    tokio::signal::ctrl_c().await.expect("Failed to listen for shutdown signal");
};

axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .with_graceful_shutdown(shutdown_signal)
    .await
    .unwrap();
```
