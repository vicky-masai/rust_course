# Docker in Production

## Interview Question

What are the key considerations for running Docker containers in production?

## Interview Answer

Running Docker in production requires careful attention to logging, monitoring, resource management, orchestration, and high availability. Containers should be stateless with data persisted to volumes, have health checks configured, resource limits enforced, and be deployed through an orchestrator like Kubernetes or Docker Swarm. For Rust backends, production readiness also includes structured logging, graceful shutdown handling, and proper signal management within containers.

---

## Follow-up Questions & Answers

### Q1. How do you configure logging for production Docker containers?

**Interview Answer**

Configure Docker logging drivers to send container stdout/stderr to centralized systems. Use `json-file` driver with `max-size` and `max-file` options to prevent disk exhaustion. For production, use `fluentd`, `syslog`, or `awslogs` drivers to ship logs to aggregation services. For Rust applications, use structured logging with `tracing` or `slog` crates, outputting JSON to stdout. This integrates seamlessly with Docker's log collection pipeline and enables searching and alerting on log patterns.

---

### Q2. How do you set resource limits for Docker containers?

**Interview Answer**

Use `--memory` and `--cpus` flags at runtime or `deploy.resources.limits` in Compose. For example, `docker run --memory=512m --cpus=1.0 my-rust-app` caps memory at 512MB and uses one CPU core. Set `--oom-kill-disable` carefully — it prevents the OOM killer from terminating the container but can cause host instability. For Rust applications, profile memory usage first to set realistic limits. Rust's predictable memory patterns make resource estimation straightforward compared to garbage-collected languages.

---

### Q3. How do you handle graceful shutdown in Docker containers?

**Interview Answer**

Docker sends `SIGTERM` when stopping a container, then `SIGKILL` after the stop timeout (default 10 seconds). In Rust, use `tokio::signal::ctrl_c()` or `signal-hook` crate to catch `SIGTERM` and drain in-flight requests. Set `stop_grace_period` in Compose or `--stop-timeout` in Docker to give the application enough time. For a Rust web server, register signal handlers that stop accepting new connections, wait for existing requests to complete, and close database connections cleanly.

---

### Q4. What is Docker Swarm and when would you use it?

**Interview Answer**

Docker Swarm is Docker's native orchestration mode for clustering and managing containers across multiple hosts. It provides service discovery, load balancing, rolling updates, and secret management. Use Swarm for simpler deployments where Kubernetes would be overkill. For Rust microservices, Swarm handles scaling, health-check-based restarts, and overlay networking across nodes. However, Kubernetes has become the industry standard for larger deployments due to its richer feature set and ecosystem.

---

### Q5. How do you implement rolling updates with Docker?

**Interview Answer**

In Swarm, use `docker service update --image <new-image> <service>` for rolling updates. In Kubernetes, Deployments handle rolling updates automatically with `maxSurge` and `maxUnavailable` parameters. The key is ensuring new containers pass health checks before old ones are terminated. For Rust backends, this means handling in-flight requests during shutdown and ensuring the new container can start serving within the startup timeout. Blue-green deployments provide zero-downtime but require double the resources.

---

### Q6. How do you monitor Docker containers in production?

**Interview Answer**

Use Prometheus with cAdvisor or Docker metrics API to collect container metrics (CPU, memory, network, disk I/O). Set up alerts for high memory usage, restart loops, and unhealthy containers. For Rust applications, expose application-level metrics (request latency, error rates) via a `/metrics` endpoint using crates like `prometheus`. Combine container metrics with application metrics for full observability. Tools like Grafana dashboards provide visualization across your container fleet.

---

### Q7. How do you handle secrets in production Docker environments?

**Interview Answer**

Use Docker secrets in Swarm mode or Kubernetes Secrets. Mount secrets as files in `/run/secrets/` rather than using environment variables. For non-orchestrated Docker, use external secret managers like HashiCorp Vault, AWS Secrets Manager, or SOPS. Never bake secrets into images or commit them to version control. For Rust apps, read secrets from files at startup and validate they're present before accepting traffic. Rotate secrets regularly and audit access.

---

### Q8. How do you manage container image lifecycle in production?

**Interview Answer**

Tag images with semantic versions and Git SHAs, never use `latest` in production. Store images in private registries with access controls. Implement image scanning in CI/CD pipelines and block deployments with critical vulnerabilities. Purge old images from registries to save storage. For Rust backends, build images in CI, scan them, push to registry, and deploy by specific tag. Use immutable tags — once pushed, a tag should never be overwritten.

---

### Q9. How do you handle persistent storage in production Docker?

**Interview Answer**

Use named volumes for stateful data like databases and file storage. Choose volume drivers based on your infrastructure: local for single-node, NFS for shared storage, cloud-provider drivers (EBS, EFS) for cloud deployments. Back up volumes regularly and test restore procedures. For Rust applications with databases, coordinate volume management with database backup strategies. Consider using external databases (RDS, Cloud SQL) instead of containerized databases for critical production data.

---

### Q10. What is the difference between running Docker in Swarm vs Kubernetes?

**Interview Answer**

Docker Swarm is simpler to set up and manage, with native Docker CLI integration. Kubernetes is more complex but offers richer scheduling, auto-scaling, and a vast ecosystem of tools. Swarm uses Docker's built-in overlay networks, while Kubernetes has its own CNI (Container Network Interface) plugins. For Rust backends, both work well, but Kubernetes is preferred for production due to better tooling (Helm, ArgoCD), community support, and features like Horizontal Pod Autoscaling. Swarm remains suitable for small teams and simpler deployments.
