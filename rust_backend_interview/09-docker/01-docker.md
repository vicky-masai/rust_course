# Docker

## Interview Question

What is Docker and why is it used in backend development?

## Interview Answer

Docker is a containerization platform that packages applications and their dependencies into isolated, portable containers. It solves the "it works on my machine" problem by ensuring consistent environments across development, testing, and production. Docker uses OS-level virtualization, sharing the host kernel while keeping the filesystem, networking, and process space isolated. For Rust backends, Docker enables reproducible builds and simplifies deployment since the compiled binary and its runtime dependencies are bundled together.

---

## Follow-up Questions & Answers

### Q1. What is the difference between a Docker image and a Docker container?

**Interview Answer**

A Docker image is a read-only template containing the application code, runtime, libraries, and filesystem layers. A container is a runnable instance of that image — it adds a writable layer on top and runs as an isolated process. You create multiple containers from one image, each with its own state. Images are stored in registries like Docker Hub, while containers run on the Docker engine.

---

### Q2. What is a Dockerfile and what does each instruction do?

**Interview Answer**

A Dockerfile is a text file with sequential instructions that Docker reads to build an image. Common instructions include `FROM` (base image), `COPY` (add files), `RUN` (execute commands), `WORKDIR` (set directory), `EXPOSE` (document ports), and `CMD` or `ENTRYPOINT` (define startup command). Each instruction creates a new layer in the image, which Docker caches for faster rebuilds. For Rust, a typical Dockerfile copies source code, runs `cargo build --release`, and then copies the final binary into a minimal base image.

---

### Q3. How does Docker achieve container isolation?

**Interview Answer**

Docker uses Linux namespaces to isolate PID, network, mount, and user spaces so each container has its own view of the system. Control groups (cgroups) limit CPU, memory, and I/O resources each container can consume. The copy-on-write filesystem (using overlay2 or similar drivers) gives each container its own writable layer. These mechanisms together prevent containers from interfering with each other or the host OS without the overhead of full virtual machines.

---

### Q4. What is Docker Hub and why would you use a private registry?

**Interview Answer**

Docker Hub is the default public registry for storing and sharing Docker images. Organizations often use private registries like AWS ECR, GitHub Container Registry, or self-hosted registries to keep proprietary images secure and control access. Private registries integrate with CI/CD pipelines and IAM policies, ensuring only authorized deployments pull images. For Rust backends, you'd push your release image to a private registry and pull it during deployment.

---

### Q5. What is the difference between `CMD` and `ENTRYPOINT` in a Dockerfile?

**Interview Answer**

`CMD` provides default arguments for the container's entrypoint and can be overridden at runtime. `ENTRYPOINT` defines the fixed executable that always runs, and arguments are appended to it. A common pattern is using `ENTRYPOINT` for the application binary and `CMD` for default flags. For a Rust binary, you might use `ENTRYPOINT ["/app/my_server"]` and `CMD ["--port", "8080"]`.

---

### Q6. How does Docker handle networking between containers?

**Interview Answer**

Docker creates virtual networks that containers join when they run. The default bridge network allows containers to communicate via IP addresses, while a user-defined bridge network enables DNS-based service discovery by container name. Host networking removes isolation entirely, sharing the host's network stack. For multi-container Rust applications, user-defined bridge networks let services discover each other by name without hardcoding IPs.

---

### Q7. What are Docker layers and how does layer caching work?

**Interview Answer**

Each Dockerfile instruction creates a read-only layer stacked on top of the previous one. Docker caches these layers during builds — if nothing changes in a layer or below it, Docker reuses the cached version instead of rebuilding. This is why you put infrequently changing instructions (like installing system dependencies) before frequently changing ones (like copying source code). For Rust projects, caching the `cargo build` layer separately from dependency fetching drastically speeds up rebuilds.

---

### Q8. How would you containerize a Rust backend application?

**Interview Answer**

Use a multi-stage Dockerfile: in the builder stage, use a Rust image to compile the binary with `cargo build --release`. In the runtime stage, copy only the compiled binary into a minimal base image like `debian:slim` or `alpine`. Set `WORKDIR`, `EXPOSE` the port, and define `CMD` with the binary path. This approach produces a small image without compilers or source code, often under 50MB for a Rust binary compared to 1GB+ for the build stage.

---

### Q9. What is a `.dockerignore` file and why is it important?

**Interview Answer**

A `.dockerignore` file excludes files and directories from the Docker build context — the set of files sent to the Docker daemon. Without it, you might send `target/`, `.git/`, and other unnecessary files, making builds slower and images larger. For Rust projects, excluding `target/` alone can save gigabytes from the build context. It works similarly to `.gitignore` syntax and is essential for both build speed and image security.

---

### Q10. What is the difference between `docker build`, `docker pull`, and `docker run`?

**Interview Answer**

`docker build` creates an image from a Dockerfile and build context. `docker run` creates and starts a container from an image, applying runtime options like port mappings and environment variables. `docker pull` downloads a pre-built image from a registry to your local machine. In a typical workflow, you `pull` or `build` an image first, then `run` it to start the containerized application.
