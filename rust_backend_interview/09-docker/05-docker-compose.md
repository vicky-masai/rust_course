# Docker Compose

## Interview Question

What is Docker Compose and how do you define multi-container applications with it?

## Interview Answer

Docker Compose is a tool for defining and running multi-container Docker applications using a YAML configuration file (`docker-compose.yml`). It declares services, networks, and volumes in a single file, then creates and manages all resources with one command. Each service defines its own image or Dockerfile, ports, environment variables, and dependencies. For Rust backends, Compose orchestrates the API server alongside databases, caches, and message brokers during development and testing.

---

## Follow-up Questions & Answers

### Q1. What is the structure of a `docker-compose.yml` file?

**Interview Answer**

A Compose file has top-level keys: `version` (spec file version), `services` (container definitions), `networks` (custom network definitions), and `volumes` (persistent storage definitions). Each service specifies `image` or `build`, `ports`, `environment`, `volumes`, `depends_on`, and `restart` policies. For a Rust backend, a typical Compose file defines the API service with a build context pointing to the Dockerfile, a PostgreSQL database service, and a Redis cache service with shared networks.

---

### Q2. How do you define a service that builds from a Dockerfile?

**Interview Answer**

Use the `build` key with a `context` (path to build directory) and optional `dockerfile` (specific Dockerfile name). For example, `build: { context: ./backend, dockerfile: Dockerfile }` tells Compose to build from the `./backend` directory. The `args` key passes build-time variables. You can combine `build` and `image` to tag the built image. For Rust projects, set the build context to the Cargo workspace root so the Dockerfile has access to `Cargo.toml` and `Cargo.lock`.

---

### Q3. How does `depends_on` work and what are its limitations?

**Interview Answer**

`depends_on` controls startup order — services listed under `depends_on` start before the dependent service. However, it only waits for the container to start, not for the application inside to be ready. A Rust API server starting before PostgreSQL is fully accepting connections will fail. Use `depends_on` with `condition: service_healthy` and a `healthcheck` definition on the database service. This ensures the dependency is actually ready before the dependent service starts.

---

### Q4. How do you override Compose settings for different environments?

**Interview Answer**

Use multiple Compose files: `docker-compose.yml` for base configuration and `docker-compose.override.yml` for development overrides. Docker automatically merges them. For different environments, use `docker-compose -f docker-compose.yml -f docker-compose.prod.yml up`. Environment variables can be set in `.env` files and referenced with `${VARIABLE}` in the YAML. For Rust backends, override environment variables like `DATABASE_URL` and `RUST_LOG` per environment.

---

### Q5. How do you manage environment variables in Docker Compose?

**Interview Answer**

Environment variables can be defined inline under the `environment` key, loaded from a `.env` file in the project root, or referenced from a `env_file` pointing to a specific file. The `.env` file sets variables for the Compose file itself, while `env_file` passes variables into the container. For Rust applications, use `env_file` to load database credentials and application secrets without hardcoding them in version control. Never commit `.env` files with real secrets.

---

### Q6. What is the `profiles` feature in Docker Compose?

**Interview Answer**

Profiles let you selectively start services based on named groups. Services with a `profiles` key only start when that profile is activated with `--profile`. For example, mark monitoring tools and test utilities under a `debug` profile, keeping the default profile lean. Run `docker compose --profile debug up` to include debug services. For Rust development, you might have a `test` profile that includes test databases and a `production` profile with resource limits.

---

### Q7. How do you define custom networks in Docker Compose?

**Interview Answer**

Define networks under the top-level `networks` key with a driver (default: bridge). Services are attached to specific networks using `networks` within the service definition. Compose creates a default network for all services, but custom networks provide isolation between service groups. For a Rust microservices setup, create separate `frontend` and `backend` networks so only the API gateway connects to both, while internal services only communicate on the backend network.

---

### Q8. How do you handle volumes in Docker Compose for development?

**Interview Answer**

Define named volumes in the top-level `volumes` key and mount them in services. For development, use bind mounts to enable live code reloading: mount `./src:/app/src` so Rust `cargo watch` or `cargo run` reflects changes immediately. Named volumes persist between `docker compose down` and `docker compose up` runs. Use `docker compose down -v` to also remove volumes when you need a clean slate.

---

### Q9. What is the difference between `docker compose up` and `docker compose run`?

**Interview Answer**

`docker compose up` starts all services defined in the Compose file, respecting `depends_on` ordering. `docker compose run` starts a one-off command in a specific service container, useful for running migrations or CLI tools. `run` also allows overriding the command: `docker compose run backend cargo test`. For Rust backends, use `up` for the full application stack and `run` for tasks like database migrations, test execution, or one-time scripts.

---

### Q10. How do you configure resource limits in Docker Compose?

**Interview Answer**

Use the `deploy.resources` section (Compose spec v3) to set `limits` and `reservations` for CPU and memory. For example, `resources: { limits: { memory: 512M, cpus: '0.5' } }` prevents the Rust backend from consuming more than 512MB RAM. In Docker Compose v2, use `mem_limit` and `cpus` directly on the service. Always set limits in production to prevent a single container from exhausting host resources. For Rust applications, memory limits are especially important due to the language's efficient but potentially large allocations.
