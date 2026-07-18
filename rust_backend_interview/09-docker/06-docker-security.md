# Docker Security

## Interview Question

What are Docker security best practices for production deployments?

## Interview Answer

Docker security spans image security, container runtime security, and orchestration security. Key practices include running containers as non-root users, using minimal base images to reduce attack surface, scanning images for vulnerabilities, never storing secrets in images, using read-only filesystems, and applying resource limits. For Rust backends, these practices are especially effective since Rust's memory safety guarantees combined with proper containerization create highly secure deployment stacks.

---

## Follow-up Questions & Answers

### Q1. Why should you run containers as a non-root user?

**Interview Answer**

Running as root gives the container process full privileges inside the container, and misconfigurations (like `--privileged` or capability escapes) can lead to host compromise. A non-root user limits the blast radius of any vulnerability in the application. In a Dockerfile, create a user with `RUN useradd -r -s /bin/false appuser` and use `USER appuser` before `CMD`. For Rust binaries, ensure the binary can read any necessary files and write to designated directories without root permissions.

---

### Q2. How do you scan Docker images for vulnerabilities?

**Interview Answer**

Use tools like Docker Scout, Trivy, Snyk, or Grype to scan image layers for known CVEs. Run `docker scout cves <image>` or `trivy image <image>` to get a vulnerability report. Scan images in CI/CD pipelines to catch issues before deployment. For Rust images, the attack surface is typically small since Rust compiles to a static binary — focus scanning on the base OS layers. Address critical and high-severity vulnerabilities by updating base images and dependencies.

---

### Q3. What are Docker secrets and how do they differ from environment variables?

**Interview Answer**

Docker secrets are encrypted at rest and in transit, accessible only to services that have been granted access. They're mounted as files in `/run/secrets/<secret_name>` inside the container. Environment variables are visible in `docker inspect` output and in `/proc/1/environ` inside the container. For production Rust backends, use Docker secrets or a secrets manager (like HashiCorp Vault) for database credentials and API keys instead of environment variables.

---

### Q4. How do you use a read-only filesystem in Docker?

**Interview Answer**

Mount the container's root filesystem as read-only with `--read-only` flag. The application writes only to explicitly mounted volumes or tmpfs mounts. This prevents attackers from modifying binaries or planting malware even if they gain code execution. For Rust binaries, this works well since they typically only need to read configuration and write logs or data to specific directories. Add `tmpfs` mounts for temporary directories the application needs.

---

### Q5. What Linux capabilities should you drop from Docker containers?

**Interview Answer**

Linux capabilities break root privileges into fine-grained units. Drop all capabilities with `cap_drop: [ALL]` and add back only what's needed with `cap_add`. Most Rust web servers only need `NET_BIND_SERVICE` to bind to ports below 1024. Drop `SYS_ADMIN`, `NET_RAW`, and `SYS_PTRACE` which are commonly exploited. Use `--security-opt=no-new-privileges` to prevent processes from gaining additional privileges through setuid binaries.

---

### Q6. How do you verify the integrity of Docker images?

**Interview Answer**

Use Docker Content Trust (DCT) to sign and verify images with `DOCKER_CONTENT_TRUST=1`. This ensures images haven't been tampered with between build and deployment. For production Rust images, sign them in your CI pipeline and verify signatures during deployment. You can also pin images by SHA256 digest instead of tags to prevent tag mutation attacks. This guarantees you're running exactly the image that was built and approved.

---

### Q7. What is the principle of least privilege in Docker context?

**Interview Answer**

Give each container only the permissions it absolutely needs to function. This means running as non-root, dropping all Linux capabilities, using read-only filesystems, setting resource limits, and restricting network access. For a Rust API server, it needs network access to receive requests, volume access for data storage, and nothing else. Audit containers regularly to ensure permissions haven't crept beyond what's necessary, and remove any `--privileged` flags.

---

### Q8. How do you secure Docker in production environments?

**Interview Answer**

Keep Docker and the host OS updated, enable user namespaces, restrict container-to-container communication, use TLS for Docker daemon communication, and enable audit logging. In production, never expose the Docker socket to containers as it gives root-equivalent access to the host. For Rust deployments, combine container security with application-level security — TLS termination, authentication, input validation — for defense in depth.

---

### Q9. How do you prevent containers from escalating privileges?

**Interview Answer**

Use `--security-opt=no-new-privileges` to prevent setuid/setgid binaries from granting additional privileges. Drop all Linux capabilities and only add back what's needed. Avoid running containers in privileged mode. Use AppArmor or SELinux profiles to restrict system calls. For Rust binaries, since they're statically compiled and don't require special capabilities, you can apply very restrictive security profiles without breaking functionality.

---

### Q10. How do you handle logging and auditing for container security?

**Interview Answer**

Configure Docker's logging drivers to send container logs to centralized systems like ELK, Fluentd, or CloudWatch. Enable Docker audit logging to track container lifecycle events. For Rust applications, use structured logging crates like `tracing` or `log` with JSON output to integrate with container log aggregators. Monitor container resource usage and set alerts for anomalous behavior. Regularly review Docker image scan reports and apply security patches promptly.
