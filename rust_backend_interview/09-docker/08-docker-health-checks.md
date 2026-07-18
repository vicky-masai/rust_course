# Docker Health Checks

## Interview Question

How do Docker health checks work and why are they important?

## Interview Answer

Docker health checks periodically test whether a container is functioning correctly. The `HEALTHCHECK` instruction in a Dockerfile (or `healthcheck` in Compose) defines a command to run inside the container. Docker marks the container as `healthy`, `unhealthy`, or `starting` based on the command's exit code. For Rust backends, health checks enable orchestrators to automatically restart failed services and route traffic only to healthy instances.

---

## Follow-up Questions & Answers

### Q1. How do you define a HEALTHCHECK in a Dockerfile?

**Interview Answer**

```dockerfile
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1
```

`--interval` is how often to check, `--timeout` is the max time per check, `--start-period` gives the app time to initialize, and `--retries` is how many failures before marking unhealthy. The `CMD` must exit 0 for healthy, 1 for unhealthy. For Rust apps, you can use `wget` instead of `curl` to avoid installing curl in minimal images.

---

### Q2. What are the different health check states and their meanings?

**Interview Answer**

`starting` means the container recently started and the start period hasn't elapsed. `healthy` means the last check succeeded. `unhealthy` means the check has failed consecutively beyond the retry count. Docker doesn't automatically restart unhealthy containers — orchestrators like Swarm or Kubernetes handle restarts. For a Rust API server, `starting` prevents premature traffic routing while the server initializes connections and loads configuration.

---

### Q3. How do you implement a health check endpoint in Rust?

**Interview Answer**

```rust
use actix_web::get;

#[get("/health")]
async fn health_check() -> impl actix_web::Responder {
    // Check database connection, cache connectivity, etc.
    actix_web::HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}
```

The endpoint should verify critical dependencies: database connectivity, cache availability, and external service reachability. Return 200 if all dependencies respond, 503 if any are down. Keep the check lightweight — avoid heavy queries or operations that would fail under load.

---

### Q4. How do you handle health checks when the application takes time to start?

**Interview Answer**

Use the `--start-period` flag to define a grace period where failures don't count toward the unhealthy threshold. For Rust binaries that compile routes, establish database connections, or load configuration, set a generous start period (10-30 seconds). During this period, Docker marks the container as `starting` rather than unhealthy. Combine with a retry count so transient connection failures during startup don't trigger restarts.

---

### Q5. How do you use health checks with Docker Compose?

**Interview Answer**

```yaml
services:
  backend:
    build: .
    healthcheck:
      test: ["CMD", "wget", "--no-verbose", "--tries=1", "--spider", "http://localhost:8080/health"]
      interval: 30s
      timeout: 5s
      retries: 3
      start_period: 10s
  db:
    image: postgres:16
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 10s
      timeout: 5s
      retries: 5
```

Combine with `depends_on: { db: { condition: service_healthy } }` to ensure the database is ready before the Rust backend starts.

---

### Q6. How do you test health checks without installing curl?

**Interview Answer**

For minimal images without curl, use `wget --spider` which doesn't download content. Alternatively, use a custom health check binary or a simple TCP check. In Rust, you could build a small `health-check` binary that connects to the port. For distroless images, use Docker's `--health-cmd` at runtime instead of in the Dockerfile. You can also use `CMD` with a Rust binary's own health check mode: `CMD ["/usr/local/bin/my_server", "--health-check"]`.

---

### Q7. What is the impact of health checks on container performance?

**Interview Answer**

Health checks consume resources each time they run — spawning a process, making a network request, and processing the response. Frequent checks (every 5 seconds) on many containers add measurable overhead. Use appropriate intervals: 30 seconds for most production services, 10 seconds for critical path services. The health endpoint itself should be extremely lightweight, returning a simple status without expensive database queries or computations.

---

### Q8. How do health checks interact with load balancers and orchestrators?

**Interview Answer**

Kubernetes uses liveness, readiness, and startup probes (similar to Docker health checks) to manage pod lifecycle. Swarm mode uses Docker health checks to determine service routing. When a container becomes unhealthy, the orchestrator stops sending traffic and may restart it. For Rust backends behind a load balancer, health checks ensure requests only reach instances that can serve them. Configure your load balancer to also check the `/health` endpoint.

---

### Q9. How do you handle health checks in multi-stage builds?

**Interview Answer**

The `HEALTHCHECK` instruction must be in the final runtime stage since that's the image that runs. Don't install health check tools in the runtime stage if you can avoid it — use binaries already present or build a lightweight check tool in an intermediate stage. For Rust, `wget` is typically available in Debian-based images. If using scratch or distroless, implement the health check in the application binary itself rather than relying on external tools.

---

### Q10. How do you debug health check failures?

**Interview Answer**

Run the health check command manually inside the container: `docker exec <container> wget --spider http://localhost:8080/health`. Check container logs with `docker logs <container>` for errors during startup. Inspect the health check history with `docker inspect --format='{{json .State.Health}}' <container>`. Common issues include: the application hasn't started yet (increase start period), the port is wrong, the application crashed, or external dependencies are unavailable. For Rust apps, check `RUST_LOG` output for panic messages or initialization errors.
