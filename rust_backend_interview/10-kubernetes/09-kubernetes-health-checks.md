# Kubernetes Health Checks

## Interview Question

Explain the three types of health check probes in Kubernetes, how they differ, and how to implement them for a Rust backend service.

## Interview Answer

Kubernetes provides three probe types to monitor container health: liveness probes determine if a container is running correctly — if it fails, kubelet restarts the container. Readiness probes determine if a container is ready to accept traffic — if it fails, the Pod is removed from Service endpoints and stops receiving requests. Startup probes protect slow-starting containers by running only at startup, delaying liveness and readiness checks until the application is fully initialized. For a Rust Axum or Actix service, you'd implement a `/health/live` endpoint (simple 200 OK to confirm the process is alive), a `/health/ready` endpoint (checks database connectivity and downstream dependencies), and optionally a startup probe that hits `/health/live` with a longer timeout for services with heavy initialization (loading ML models, establishing connection pools). Incorrect probe configuration is a leading cause of production incidents — too aggressive liveness probes cause unnecessary restarts, while missing readiness probes send traffic to unready Pods.

---

## Follow-up Questions & Answers

### Q1. What is a liveness probe and when does it restart a container?

**Interview Answer**

A liveness probe checks if a container is still alive and functioning — if the probe fails (returns non-200 or times out), kubelet kills the container and restarts it according to the Pod's restartPolicy. Use it to detect deadlocks, infinite loops, or other unrecoverable states where the process is running but not making progress. For Rust services, a simple `/health/live` endpoint that returns 200 OK without checking dependencies is sufficient because restarting the container won't fix a database outage. Set `failureThreshold: 3` to avoid restarting on transient failures, and `periodSeconds: 10-15` to check frequently enough to detect issues without excessive overhead. A common mistake is making the liveness probe too aggressive — if your Rust service takes 5 seconds under load to respond, a 1-second timeout causes unnecessary restarts.

---

### Q2. What is a readiness probe and when does it remove a Pod from Service endpoints?

**Interview Answer**

A readiness probe determines if a Pod is ready to serve traffic — when it fails, Kubernetes removes the Pod from the Service's endpoint list, preventing new requests from being routed to it. This is essential during startup (don't send traffic until the server is ready), during graceful shutdown (drain connections before terminating), and when dependencies are temporarily unavailable (remove from load balancing until database reconnects). For Rust services, the readiness probe should check actual dependencies: test database connectivity, verify cache availability, and confirm the server is listening on the expected port. Unlike liveness probe failures, readiness probe failures do NOT restart the container — the Pod stays running but receives no traffic until the probe succeeds again.

---

### Q3. What is a startup probe and when do you need one?

**Interview Answer**

A startup probe runs only when a container starts, delaying liveness and readiness probes until the application is fully initialized. It's essential for slow-starting containers — like a Rust service that loads large datasets, compiles Lua scripts, or warms up caches — where the normal liveness probe would kill the container before it finishes starting. Configure it with `failureThreshold` high enough to allow full startup (e.g., 30 attempts × 2 seconds = 60 seconds max startup time) and a simple endpoint like `/health/live`. Once the startup probe succeeds, liveness and readiness probes take over. For most Rust services with fast startup times (under 1 second), startup probes aren't needed, but they're critical for services with heavy initialization or for languages with slower startup (Java, JVM-based services).

---

### Q4. How do you implement health check endpoints in Actix-web?

**Interview Answer**

In Actix-web, create a health check handler that returns `HttpResponse::Ok()` for liveness and checks dependencies for readiness. For liveness: `async fn liveness() -> HttpResponse { HttpResponse::Ok().finish() }`. For readiness, test your database pool: `async fn readiness(pool: web::Data<PgPool>) -> HttpResponse { match sqlx::query("SELECT 1").execute(pool.get_ref()).await { Ok(_) => HttpResponse::Ok().finish(), Err(_) => HttpResponse::ServiceUnavailable().finish() } }`. Register routes with `.route("/health/live", web::get().to(liveness))` and `.route("/health/ready", web::get().to(readiness))`. The readiness check should have a short timeout (2-3 seconds) so it fails fast if the database is unreachable. Use `actix_web::web::Data` for dependency injection of database pools and other shared state.

---

### Q5. How do you implement health check endpoints in Axum?

**Interview Answer**

In Axum, create async handler functions for each probe type. Liveness is trivial: `async fn liveness() -> StatusCode { StatusCode::OK }`. Readiness checks dependencies: `async fn readiness(State(db): State<PgPool>) -> StatusCode { match db.execute("SELECT 1").await { Ok(_) => StatusCode::OK, Err(_) => StatusCode::SERVICE_UNAVAILABLE } }`. Register routes with `Router::new().route("/health/live", get(liveness)).route("/health/ready", get(readiness)).with_state(db_pool)`. For production Rust services, add a timeout to the readiness check using `tokio::time::timeout(Duration::from_secs(3), check_dependencies())` to ensure the probe fails fast rather than hanging indefinitely if a dependency is down. Return structured JSON with dependency status in development environments for easier debugging.

---

### Q6. What are the common mistakes when configuring health probes?

**Interview Answer**

Common mistakes include: making liveness probes too aggressive (short timeoutSeconds, low failureThreshold) causing unnecessary restarts during traffic spikes, not setting readiness probes so traffic hits unready Pods causing errors, making readiness probes too strict (checking every dependency) causing the Pod to be removed during brief dependency blips, not setting startup probes for slow-starting containers causing them to be killed before initialization completes, and using the same endpoint for liveness and readiness (the liveness check should be simple, the readiness check should verify dependencies). For Rust services, another common mistake is having the health endpoint check the database on every request without caching — under high traffic, this creates unnecessary database load. Instead, cache dependency health status and check periodically.

---

### Q7. How do health probes interact with graceful shutdown?

**Interview Answer**

When Kubernetes decides to terminate a Pod (during scaling, rolling updates, or node drains), it first sends SIGTERM to your process and simultaneously removes the Pod from Service endpoints. Your readiness probe should fail immediately during shutdown so Kubernetes stops routing traffic, while your Rust service continues processing in-flight requests. Implement this by having your readiness probe check a shared `AtomicBool` flag that's set to false in your SIGTERM handler. Your Rust service should then drain in-flight requests (typically 30 seconds — matching `terminationGracePeriodSeconds`) before exiting. Without this coordination, your Rust service might receive new requests after SIGTERM, causing 502 errors. The sequence is: SIGTERM received → readiness probe starts failing → Pod removed from endpoints → in-flight requests complete → process exits.

---

### Q8. What is the difference between probe results and container state?

**Interview Answer**

Container state reflects kubelet's observation of the container process: Running, Waiting (pulling images, creating containers), or Terminated (exited with a code). Probe results are separate health signals: Success, Failure, or Unknown. A container can be Running but failing its liveness probe (detected as unhealthy, will be restarted), or Running and passing all probes (healthy, receiving traffic). A Terminated container has already exited and probe results are no longer relevant. For debugging Rust services, check both: `kubectl describe pod` shows container state and last probe results, while `kubectl logs --previous` shows logs from the last terminated container. Understanding this distinction helps diagnose whether an issue is a crash (container terminated), a health check failure (container running but probe failing), or a readiness issue (container healthy but not receiving traffic).

---

### Q9. How do you test health check endpoints before deploying to Kubernetes?

**Interview Answer**

Test health endpoints locally by running your Rust service and hitting them with curl: `curl http://localhost:8080/health/live` should return 200, and `curl http://localhost:8080/health/ready` should return 200 only when dependencies are available. Write unit tests for the health handlers using `actix_web::test` or `axum::test` to verify correct status codes under various conditions (healthy, degraded, unhealthy). Use integration tests that spin up the service with test dependencies (test database, mock cache) to verify readiness checks work end-to-end. Before deploying to Kubernetes, run your Docker image locally with Docker Compose and test health endpoints in a container environment. For Rust services, also test that your health endpoints respond quickly under load — if the readiness check takes 5 seconds, your Pod won't receive traffic for 5 seconds after each restart.

---

### Q10. What is a Pod lifecycle hook and how does it relate to health probes?

**Interview Answer**

Pod lifecycle hooks (`postStart` and `preStop`) execute commands or HTTP requests at specific points in a container's lifecycle: `postStart` runs after the container is created but before it's marked as started, and `preStop` runs before the container receives SIGTERM. Unlike health probes which check ongoing health, lifecycle hooks are one-time events that perform setup or cleanup. For Rust services, `preStop` hooks are critical for graceful shutdown — use `preStop.httpGet` on a `/shutdown` endpoint that starts draining connections, or `preStop.exec` with a command like `sleep 15` to give load balancers time to stop routing traffic. The sequence is: `preStop` hook executes → SIGTERM sent → container drains and exits. Without `preStop` hooks, there's a race condition where SIGTERM arrives before load balancers have updated their endpoint lists, causing dropped requests.