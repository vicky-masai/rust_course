# Dockerfile Best Practices

## Interview Question

What are Dockerfile best practices for building efficient and secure images?

## Interview Answer

Dockerfile best practices focus on minimizing image size, improving build cache efficiency, and enhancing security. Key practices include using minimal base images, combining `RUN` commands to reduce layers, leveraging multi-stage builds to separate build and runtime environments, ordering instructions from least to most frequently changing, and using `.dockerignore` to exclude unnecessary files. For Rust applications, these practices can reduce a 2GB build image to under 50MB while keeping builds fast through aggressive layer caching.

---

## Follow-up Questions & Answers

### Q1. Why should you order Dockerfile instructions from least to most frequently changing?

**Interview Answer**

Docker rebuilds every layer from the first changed instruction downward. By placing stable instructions like system dependency installation early and source code copying later, you maximize cache hits. For a Rust Dockerfile, installing `build-essential` and `libssl-dev` rarely changes, so it stays cached across rebuilds. Only the `COPY src/` and `cargo build` layers rebuild when code changes, cutting build times from minutes to seconds.

---

### Q2. How do you combine `RUN` commands to reduce layers?

**Interview Answer**

Each `RUN` instruction creates a new layer. Combining related commands with `&&` and using backslashes `\` for line continuation reduces the number of layers and total image size. You should also clean up temporary files in the same `RUN` command so they don't persist in earlier layers. For example, installing Rust dependencies, building, and cleaning up the cargo cache should happen in one `RUN` block.

---

### Q3. What is the purpose of using `COPY --chown` and `COPY --chmod`?

**Interview Answer**

`COPY --chown` sets file ownership during the copy, avoiding a separate `RUN chown` layer. `COPY --chmod` sets file permissions in a single instruction. Both reduce the number of layers and keep images smaller. For Rust binaries, setting `--chmod=+x` on the compiled binary eliminates the need for a separate permission-setting layer. The `--chown` flag is especially important when running containers as non-root users.

---

### Q4. How does `.dockerignore` improve build performance and security?

**Interview Answer**

The `.dockerignore` file filters the build context before sending it to the Docker daemon. Excluding `target/`, `.git/`, and `node_modules/` can reduce context size from gigabytes to megabytes, speeding up builds. From a security perspective, it prevents accidentally including secrets, `.env` files, or private keys in the image. For Rust projects, always ignore `target/`, `*.pdb`, and any IDE configuration files.

---

### Q5. Why should you avoid using `latest` tags for base images?

**Interview Answer**

The `latest` tag is mutable and can change without notice, leading to non-reproducible builds. A Dockerfile that worked yesterday might break today if the base image updates. Always pin to specific versions like `rust:1.75-bookworm` or use SHA256 digests for absolute reproducibility. For production Rust images, pinning ensures that every build produces identical results regardless of upstream changes.

---

### Q6. What is the difference between `alpine` and `debian-slim` base images?

**Interview Answer**

Alpine is approximately 5MB and uses musl libc, while Debian slim is around 30MB and uses glibc. Rust binaries on Alpine may have compatibility issues with C libraries compiled against glibc. Debian slim is generally safer for Rust deployments since most crates assume glibc. Alpine requires adding a build stage to compile with musl, adding complexity. For most Rust backends, `debian:slim` or `distroless` images are the better choice.

---

### Q7. How do you handle secrets in a Dockerfile without embedding them in the image?

**Interview Answer**

Never `COPY` or `RUN` secret values directly — they become permanent layers visible in the image history. Instead, use `--mount=type=secret` in `RUN` instructions to access secrets at build time without persisting them. At runtime, pass secrets via environment variables, Docker secrets, or mounted files. For Rust apps connecting to databases, pass the connection string as an environment variable at `docker run` time, not baked into the image.

---

### Q8. What is the `SHELL` instruction and when would you use it?

**Interview Answer**

The `SHELL` instruction changes the shell used for `RUN` commands from the default `/bin/sh -c`. On Windows containers, you'd use `SHELL ["powershell", "-Command"]`. On Linux, it's useful for enabling `pipefail` with bash: `SHELL ["/bin/bash", "-o", "pipefail", "-c"]`. This ensures that piped commands like `curl ... | tar -x` fail correctly instead of silently succeeding when the first command fails.

---

### Q9. How do you minimize the final image size for a Rust application?

**Interview Answer**

Use multi-stage builds to compile in a full Rust image and copy only the binary into a minimal runtime image. Strip debug symbols with `strip` or `cargo build --release`. Use `cargo build --release` with `opt-level=z` for size optimization if binary size matters more than speed. Avoid including documentation, tests, and development tools in the runtime image. A typical Rust web server can fit in a 20-50MB `distroless` image.

---

### Q10. What is the `ARG` instruction and how does it differ from `ENV`?

**Interview Answer**

`ARG` defines build-time variables available only during `docker build`, not in the running container. `ENV` sets environment variables available both during build and at runtime. `ARG` values can be overridden with `--build-arg` flags. For Rust, you might use `ARG RUST_VERSION=1.75` to parameterize the Rust toolchain version without hardcoding it. Sensitive values should never use `ENV` since they're visible in the image metadata.
