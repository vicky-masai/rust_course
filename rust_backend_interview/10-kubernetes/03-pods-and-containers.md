# Pods and Containers

## Interview Question

Explain the Pod lifecycle, init containers, and the sidecar pattern in Kubernetes.

## Interview Answer

A Pod progresses through several lifecycle phases: Pending (scheduled but containers not yet created), Running (at least one container is running), Succeeded (all containers terminated successfully), or Failed (at least one container terminated with failure). Init containers run sequentially before the main containers start and must complete successfully — they're useful for database migrations, waiting for dependencies, or generating configuration files. The sidecar pattern places a helper container alongside the main application container in the same Pod, sharing network and storage, commonly used for log forwarding (Fluentd), proxying (Envoy), or monitoring agents. For a Rust backend, your main container runs the Actix/Axum server while a sidecar might handle log shipping to a central system. Understanding this lifecycle is critical for writing robust health checks and handling graceful shutdown correctly.

---

## Follow-up Questions & Answers

### Q1. What happens when a Pod is created?

**Interview Answer**

When you submit a Pod manifest, the API server validates and stores it in etcd, then the scheduler assigns it to a node based on resource requirements and constraints. On the assigned node, kubelet pulls the container images (if not already present), creates the containers in order (init containers first, then main containers), configures networking and volumes, and starts executing the containers' commands. kubelet continuously monitors container health via configured probes and reports Pod status back to the API server. If any init container fails, the Pod stays in Pending and kubelet retries according to restartPolicy.

---

### Q2. What are init containers and when should you use them?

**Interview Answer**

Init containers are specialized containers that run to completion before the main application containers start, and they execute sequentially in the order defined in the Pod spec. Use them for tasks like waiting for a database to be reachable, running database migrations, downloading configuration from an external service, or building generated code. For Rust projects, an init container might run `diesel migration run` before your Axum server starts, ensuring the schema is up to date. The key constraint is that init containers must exit successfully (exit code 0) — if one fails, Kubernetes restarts it according to the Pod's restart policy before proceeding.

---

### Q3. What is the sidecar pattern and how is it implemented in Kubernetes?

**Interview Answer**

The sidecar pattern places a secondary container in the same Pod as your primary application, sharing the same network namespace (localhost) and optionally sharing volumes. Common use cases include log forwarding (a Fluentd sidecar reads application logs from a shared volume and ships them to Elasticsearch), service mesh proxies (Envoy sidecars intercept all network traffic for mTLS and observability), and file watchers (a sidecar regenerates configuration when ConfigMaps change). For a Rust backend, a sidecar might run a Prometheus exporter that scrapes your `/metrics` endpoint. The limitation is that sidecars share Pod resources — if your Rust service needs 512Mi, the sidecar's memory must fit within the Pod's total resource limit.

---

### Q4. What are the possible states a Pod can be in?

**Interview Answer**

Pods progress through these phases: Pending (accepted by the API server but not yet scheduled or containers still pulling images), Running (at least one container is running, or is in the process of restarting), Succeeded (all containers exited with code 0), Failed (all containers terminated and at least one exited with non-zero code), and Unknown (kubelet stopped reporting status, often due to node failure). Within Running, individual containers can be in Waiting (pulling images or running init containers), Running (actively executing), or Terminated (exited with an exit code). Understanding these states helps debug stuck Pods — for example, a Pod stuck in Pending often indicates insufficient cluster resources or unsatisfied node selectors.

---

### Q5. How does Kubernetes handle container restarts?

**Interview Answer**

When a container exits, kubelet checks the Pod's restartPolicy: Always (default, always restart), OnFailure (restart only if exit code is non-zero), or Never (never restart). The restart delay follows an exponential backoff: 10 seconds, 20 seconds, 40 seconds, up to a maximum of 5 minutes, resetting after 10 minutes of successful execution. This backoff prevents a crash-looping container from overwhelming the node with rapid restart attempts. For your Rust services, if your binary panics on startup due to a missing environment variable, the container will crash-loop with increasing delays — kubelet logs will show the exit code and you can use `kubectl describe pod` to see the restart history.

---

### Q6. What are ephemeral containers and when would you use them?

**Interview Answer**

Ephemeral containers are a special container type that can be added to a running Pod for debugging purposes — they share the Pod's namespaces and are useful when your main container image lacks debugging tools like `curl`, `strace`, or `tcpdump`. You add them with `kubectl debug -it <pod> --image=busybox --target=<container>` without restarting the Pod. This is invaluable for debugging production Rust services where you can't add debugging tools to your minimal distroless runtime image. Ephemeral containers cannot be restarted, have no probes, and are not shown in `kubectl get pods` — you must use `kubectl describe pod` or `kubectl logs --previous` for inspection.

---

### Q7. How do you share data between containers in the same Pod?

**Interview Answer**

Containers in the same Pod can share data through shared volumes (emptyDir, configMap, secret, or PVC volumes mounted at specific paths) or through the shared network namespace (localhost communication). For example, a sidecar container writing log files to a shared emptyDir volume makes those logs accessible to your Rust main container. You can also use the Pod's localhost network — if your Rust service exposes metrics on port 9090, a monitoring sidecar can scrape `localhost:9090` directly. The `downwardAPI` volume type allows containers to share Pod metadata like labels and annotations. For production Rust services, shared volumes are commonly used for log aggregation and certificate rotation.

---

### Q8. What is the difference between a Pod and a Deployment?

**Interview Answer**

A Pod is a single instance of your application, while a Deployment is a higher-level resource that manages ReplicaSets, which in turn manage Pods. When you create a Deployment, it creates a ReplicaSet that ensures the desired number of Pod replicas are running, and it manages rolling updates when you change the Pod template. You should almost never create bare Pods in production because they lack self-healing — if a node fails, the Pod is gone permanently. A Deployment with `replicas: 3` ensures three copies of your Rust service are always running, handles zero-downtime updates, and provides rollback capability. The only time bare Pods make sense is for batch jobs or one-off tasks.

---

### Q9. How does resource limiting work for containers in a Pod?

**Interview Answer**

Each container in a Pod can specify resource requests (guaranteed resources the scheduler uses for placement) and limits (maximum resources the container can consume). For Rust services, you might set `resources.requests.cpu: "250m", memory: "256Mi"` and `resources.limits.cpu: "1000m", memory: "512Mi"`. If a container exceeds its memory limit, kubelet kills it with an OOMKilled exit code. CPU limits throttle the container if it tries to exceed the specified amount, which can hurt latency-sensitive Rust services — many teams set CPU requests without limits, relying on node-level scheduling instead. Understanding the relationship between requests and limits is critical for right-sizing your Rust services and avoiding unnecessary OOM kills or throttling.

---

### Q10. What is a PodDisruptionBudget and why is it important?

**Interview Answer**

A PodDisruptionBudget (PDB) limits the number of Pods from a workload that can be simultaneously disrupted by voluntary disruptions like node drains, cluster upgrades, or autoscaling events. For example, a PDB specifying `minAvailable: 2` for your Rust Deployment ensures at least 2 replicas are always running even during node maintenance. PDBs do not protect against involuntary disruptions (node crashes, OOM kills) — they only govern voluntary disruptions initiated by the cluster operator or Kubernetes itself. Without PDBs, draining a node could take down all replicas of your service at once, causing downtime. Always create PDBs for production workloads to ensure safe rolling operations.