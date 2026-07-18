# Docker Multi-Stage Builds

## Interview Question

What are multi-stage builds in Docker and why are they important?

## Interview Answer

Multi-stage builds use multiple `FROM` statements in a single Dockerfile, where each stage creates an independent image. You can copy artifacts from one stage to another using the `COPY --from` instruction. This separates the build environment (compilers, build tools, source code) from the runtime environment (minimal OS, only the compiled binary). For Rust, multi-stage builds are critical because the Rust toolchain is over 1GB while the final binary might be 20-50MB.

---

## Follow-up Questions & Answers

### Q1. Show a multi-stage Dockerfile for a Rust application.

**Interview Answer**

```dockerfile
# Builder stage
FROM rust:1.75-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/my_server /usr/local/bin/
EXPOSE 8080
CMD ["my_server"]
```

This produces a final image under 50MB compared to 1GB+ for the builder stage alone.

---

### Q2. How do you leverage Docker cache for Rust dependency builds?

**Interview Answer**

Copy only `Cargo.toml` and `Cargo.lock` first, create a dummy `main.rs`, and run `cargo build --release`. This caches all dependency compilation. When source code changes, only the final `cargo build` recompiles your code, not all dependencies. The `touch src/main.rs` after copying real source forces Docker to invalidate the cache from that point onward. This technique reduces Rust Docker build times from 10+ minutes to under a minute for code-only changes.

---

### Q3. What is a scratch image and when should you use it?

**Interview Answer**

`FROM scratch` starts from a completely empty image — no shell, no libraries, nothing. It's the ultimate minimal base for statically linked binaries. Rust binaries compiled with `target=x86_64-unknown-linux-musl` produce fully static executables that work on scratch. Use scratch for maximum security (smallest attack surface) and minimal image size. The downside is no debugging tools inside the container, so you need external observability.

---

### Q4. What is the distroless base image?

**Interview Answer**

Google's distroless images contain only the application runtime and its dependencies — no shell, package managers, or other utilities. For Rust, use `gcr.io/distroless/cc-debian12` which includes glibc and necessary system libraries. It's a middle ground between full Debian and scratch: smaller attack surface than Debian but still supports dynamic linking. Combined with multi-stage builds, distroless produces production-ready images that are both small and functional.

---

### Q5. How do you copy build artifacts between stages?

**Interview Answer**

Use `COPY --from=<stage-name> <source-path> <dest-path>` to copy files from a previous build stage. You can reference stages by name (e.g., `--from=builder`) or by index (e.g., `--from=0`). For Rust, copy the compiled binary from the builder stage: `COPY --from=builder /app/target/release/my_server /usr/local/bin/`. You can also copy specific configuration files or static assets from intermediate stages.

---

### Q6. How do you handle build arguments across multiple stages?

**Interview Answer**

Define `ARG` instructions at the top of each stage where they're needed — build args don't persist between stages by default. Re-declare the same `ARG` in each stage that needs it, or pass it during build with `--build-arg`. For Rust, you might pass a `RUST_VERSION` arg to control the toolchain version in the builder stage and a `DEBIAN_VERSION` in the runtime stage. Using `ARG` keeps your Dockerfile flexible for different build configurations.

---

### Q7. What are the common pitfalls of multi-stage builds for Rust?

**Interview Answer**

The biggest pitfall is incorrect dependency caching — if you copy all source code before building, dependency changes invalidate the cache. Another issue is forgetting `RUN touch src/main.rs` after copying real source, which might skip recompilation if timestamps match. Also, ensure the runtime stage has all required shared libraries — if your Rust binary dynamically links to OpenSSL, the runtime image needs `libssl3`. Test the final image thoroughly since it differs significantly from the build environment.

---

### Q8. How do you build for multiple architectures with multi-stage builds?

**Interview Answer**

Use `docker buildx build --platform linux/amd64,linux/arm64` to build multi-architecture images. Each stage must support the target architecture — the Rust builder stage can cross-compile using `rustup target add` and `cargo build --target`. For ARM targets on an AMD64 host, cross-compilation is faster than emulation. The final runtime stage should use architecture-appropriate base images, which Docker automatically selects based on the target platform.

---

### Q9. How do you optimize the builder stage specifically for Rust?

**Interview Answer**

Install only the necessary components: `rustup component add rust-src` for building. Use `cargo chef` for advanced dependency caching that handles workspace builds. Remove the entire toolchain from the builder stage by only copying the binary to the runtime stage. Consider using `sccache` or `cargo-nextest` to speed up builds within the builder stage. For workspaces, `cargo chef` generates a recipe that builds all dependencies first, then only recompiles your code.

---

### Q10. How do you debug issues in a multi-stage final image?

**Interview Answer**

Use `docker run --rm -it <image> /bin/sh` to get a shell in the final image for basic debugging. If using scratch or distroless (no shell), use `docker cp` to extract files or build a temporary debug image with shell included. Add `RUN ldd /usr/local/bin/my_server` in the Dockerfile to verify all shared library dependencies are present. For Rust binaries, use `cargo build --release` with debug info and copy the debug symbols to the runtime stage for profiling without bloating the binary itself.
