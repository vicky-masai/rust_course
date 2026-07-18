# Deployments and ReplicaSets

## Interview Question

Explain how Deployments and ReplicaSets work together, including rolling updates, rollbacks, and scaling strategies.

## Interview Answer

A Deployment creates and manages ReplicaSets, which in turn ensure the desired number of Pod replicas are running at any time. When you update the Pod template in a Deployment (e.g., changing the image tag for your Rust service), the Deployment creates a new ReplicaSet with the updated template and gradually shifts traffic from the old ReplicaSet to the new one via rolling updates. The `strategy` field controls how this happens — `RollingUpdate` (default) replaces Pods incrementally with configurable `maxSurge` and `maxUnavailable` parameters, while `Recreate` kills all old Pods before creating new ones. Rollbacks are instant because the Deployment retains previous ReplicaSet history — running `kubectl rollout undo deployment/my-rust-api` reverts to the previous revision. Scaling is as simple as updating the `replicas` field or running `kubectl scale`, and the Deployment controller adjusts the ReplicaSet to match. Understanding this layered abstraction is essential for safe production deployments of Rust backends.

---

## Follow-up Questions & Answers

### Q1. What is a ReplicaSet and how does it differ from a ReplicationController?

**Interview Answer**

A ReplicaSet is the modern evolution of ReplicationController, supporting both equality-based and set-based label selectors (ReplicationController only supports equality-based). Both ensure a specified number of Pod replicas are running, but ReplicaSets are the backing mechanism for Deployments — you rarely create ReplicaSets directly. When you run `kubectl get rs`, you see the ReplicaSets managed by your Deployments, each corresponding to a revision. The distinction matters for understanding Deployment history: each revision is a separate ReplicaSet with its own Pod template, allowing instant rollbacks by scaling up a previous ReplicaSet.

---

### Q2. What are maxSurge and maxUnavailable in a rolling update?

**Interview Answer**

`maxSurge` controls how many extra Pods can be created above the desired replica count during a rolling update, while `maxUnavailable` controls how many Pods can be unavailable (not ready) during the update. For example, with `replicas: 4, maxSurge: 1, maxUnavailable: 0`, Kubernetes creates at most 5 Pods total (4 desired + 1 surge) and ensures all 4 are always available. This means your Rust service maintains full capacity during updates, but the update takes longer because each new Pod must pass readiness checks before the next old Pod is terminated. For zero-downtime deployments of Rust services, always set `maxUnavailable: 0` and tune `maxSurge` based on your cluster's spare capacity.

---

### Q3. How do you perform a rollback in Kubernetes?

**Interview Answer**

Kubernetes Deployments maintain revision history (default: 10 revisions), and rolling back is done with `kubectl rollout undo deployment/my-rust-api` (reverts to previous revision) or `kubectl rollout undo deployment/my-rust-api --to-revision=N` (reverts to a specific revision). The undo command scales up the old ReplicaSet and scales down the new one, essentially reversing the rolling update. You can view rollout history with `kubectl rollout history deployment/my-rust-api` and check rollout status with `kubectl rollout status deployment/my-rust-api`. For production Rust services, always test rollbacks in staging first, and ensure your database migrations are backward-compatible so rollbacks don't break the schema.

---

### Q4. How do you scale a Deployment?

**Interview Answer**

Scale a Deployment with `kubectl scale deployment/my-rust-api --replicas=6` or by editing the manifest with `kubectl edit deployment/my-rust-api` and changing the `replicas` field. For autoscaling, create a HorizontalPodAutoscaler that adjusts replicas based on CPU utilization, memory usage, or custom metrics like request latency. The Deployment controller immediately creates or terminates Pods through the ReplicaSet to match the new replica count. When scaling down, Kubernetes terminates Pods with the longest uptime first and respects PodDisruptionBudgets. For Rust services, ensure your application can handle rapid scaling — stateless services behind a LoadBalancer scale seamlessly, while services holding in-memory state need careful design.

---

### Q5. What is the difference between RollingUpdate and Recreate deployment strategies?

**Interview Answer**

`RollingUpdate` (default) gradually replaces old Pods with new ones, maintaining availability throughout the update — ideal for most production services including Rust backends. `Recreate` terminates all existing Pods before creating new ones, causing downtime but avoiding version skew between old and new Pods. Use `Recreate` only when your application cannot run two versions simultaneously (e.g., due to database schema changes that are not backward-compatible). For Rust services, `RollingUpdate` is almost always preferred because it provides zero-downtime deployments, and Rust's fast startup time (typically under 100ms) means new Pods become ready quickly during the rolling process.

---

### Q6. How does the Deployment controller handle failed updates?

**Interview Answer**

If a new ReplicaSet's Pods fail to become ready (crash loops, failed health checks), the Deployment controller pauses the rolling update at the current surge level, leaving the failed Pods in a non-ready state. You can set `progressDeadlineSeconds` (default: 600s) to tell the controller how long to wait before marking the rollout as "Progressing=False" — this triggers events you can alert on. The old ReplicaSet's Pods continue serving traffic because they remain ready. To recover, fix the issue and apply a new revision, or roll back with `kubectl rollout undo`. For Rust services, common causes include missing environment variables, failed database connections at startup, or binary incompatibilities with the container image.

---

### Q7. What are Deployment annotations and how are they useful?

**Interview Answer**

Annotations are key-value metadata attached to Deployments (and other resources) that don't affect the actual behavior but provide operational context — for example, `kubectl.kubernetes.io/last-applied-configuration` stores the manifest for `kubectl diff` comparisons. Common annotations include `deployment.kubernetes.io/revision` tracking the current rollout version, `prometheus.io/scrape` enabling metrics collection, and custom annotations like `team/owner` for cost attribution. For Rust services, annotations can store build commit hashes, deployment timestamps, or links to runbooks. They're also used by tools like ArgoCD and Flux to track Git sync status, and by admission controllers to implement deployment policies.

---

### Q8. How do you handle zero-downtime deployments for Rust services?

**Interview Answer**

Zero-downtime deployments require coordinated work across Kubernetes configuration and application code. In your Deployment, set `maxUnavailable: 0` and a reasonable `maxSurge`, add a `preStop` hook with a small sleep to let load balancers drain connections, and configure readiness probes that check your Rust server's actual readiness. In your Rust code, handle SIGTERM signals gracefully — when kubelet sends SIGTERM, stop accepting new connections, wait for in-flight requests to complete (typically 30 seconds before SIGKILL), then exit cleanly. Use `tokio::signal::ctrl_c()` or Actix's `GracefulShutdown` to implement this. Also ensure your database connections and other resources are cleaned up in the shutdown handler to avoid connection pool exhaustion during rolling updates.

---

### Q9. What is a Deployment revision history limit?

**Interview Answer**

The `revisionHistoryLimit` field (default: 10) controls how many old ReplicaSets Kubernetes retains for rollback purposes. When you exceed this limit, the oldest ReplicaSets are garbage collected — you can no longer roll back to those revisions. For production Rust services, 10 is usually sufficient, but if you deploy very frequently (many times per day), you might increase it or decrease it based on your rollback needs. If you set it to 0, no old ReplicaSets are kept and rollbacks are impossible. The retained ReplicaSets have zero replicas (all their Pods are terminated) but their Pod templates are preserved for potential rollback.

---

### Q10. How do you perform canary deployments with Kubernetes native resources?

**Interview Answer**

Kubernetes doesn't have native canary support, but you can implement it by creating two Deployments — one for the stable version and one for the canary — with the same Pod labels but different replica counts. For example, stable has 9 replicas and canary has 1, both behind the same Service, so ~10% of traffic hits the canary. You monitor the canary's metrics (error rates, latency) before gradually shifting more replicas to the new version. For more sophisticated canary routing, use a service mesh like Istio or Linkerd with VirtualService traffic splitting, or an Ingress controller like NGINX with canary annotations. For Rust services, canary deployments are valuable because Rust's performance characteristics make subtle regressions easier to detect in latency metrics.