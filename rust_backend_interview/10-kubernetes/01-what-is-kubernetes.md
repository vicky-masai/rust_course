# What is Kubernetes

## Interview Question

Explain what Kubernetes is, why it exists, and how it differs from running containers directly on a host.

## Interview Answer

Kubernetes (K8s) is an open-source container orchestration platform that automates the deployment, scaling, and management of containerized applications. It exists because managing hundreds or thousands of containers across multiple hosts manually is impractical — you need automated scheduling, self-healing, service discovery, and rolling updates out of the box. Unlike running containers directly on a host with Docker, Kubernetes provides a cluster abstraction where containers are scheduled across a pool of nodes, with built-in health checks, load balancing, and secret management. The fundamental unit in Kubernetes is the Pod, which wraps one or more containers that share networking and storage, rather than managing individual containers. For a Rust backend developer, Kubernetes means your Actix or Axum services run as scalable, resilient deployments without custom orchestration code.

---

## Follow-up Questions & Answers

### Q1. What is a container and how does it differ from a virtual machine?

**Interview Answer**

A container is an isolated process that shares the host OS kernel while packaging its own dependencies, making it lightweight and fast to start — typically milliseconds versus minutes for VMs. A virtual machine includes an entire guest OS and hypervisor layer, consuming significantly more resources. Containers achieve isolation through Linux namespaces and cgroups rather than hardware virtualization. In production, you run multiple containers per node efficiently, whereas VMs each consume dedicated CPU and memory allocations.

---

### Q2. What problems does Kubernetes solve that Docker alone cannot?

**Interview Answer**

Docker handles building and running individual containers but does not natively solve cross-host networking, automated failover when a node dies, or zero-downtime rolling deployments across a cluster. Kubernetes adds a control plane that continuously reconciles desired state with actual state — if a node crashes, Pods are rescheduled elsewhere automatically. It also provides built-in service discovery, config/secrets management, and autoscaling, which Docker Swarm attempted but never achieved the same ecosystem adoption for. For production Rust services, Kubernetes gives you the operational maturity Docker alone lacks.

---

### Q3. What is a Pod in Kubernetes?

**Interview Answer**

A Pod is the smallest deployable unit in Kubernetes, representing one or more co-located containers that share a network namespace (same IP address) and can share storage volumes. Most Pods run a single container — for example, your Rust Axum server — but sidecar patterns use multi-container Pods for logging or proxying. Pods are ephemeral: Kubernetes can kill and replace them at any time, so your Rust services must be stateless and handle graceful shutdown via SIGTERM. Understanding Pod lifecycle is essential before working with higher-level resources like Deployments.

---

### Q4. What is the difference between Docker Swarm and Kubernetes?

**Interview Answer**

Docker Swarm is simpler to set up and uses Docker-native commands, making it suitable for small teams or simple deployments. Kubernetes has a steeper learning curve but offers far richer features: custom scheduling, RBAC, network policies, operator patterns, and a massive ecosystem of Helm charts and CNCF projects. Swarm uses Docker's built-in overlay networking, while Kubernetes has its own CNI (Container Network Interface) plugin system supporting Calico, Cilium, and others. In practice, Kubernetes has won the orchestration war, and most cloud providers offer managed Kubernetes (EKS, GKE, AKS) with no Swarm equivalent.

---

### Q5. How does Kubernetes handle service discovery?

**Interview Answer**

Kubernetes provides DNS-based service discovery through CoreDNS, which runs as a cluster add-on and automatically creates DNS entries for Services. When you create a Service named `api-service`, other Pods can reach it at `api-service.default.svc.cluster.local` without knowing which specific Pod IP handles the request. This decouples your Rust backend from specific Pod IPs that change frequently. Environment variables and DNS SRV records provide additional discovery mechanisms for legacy applications.

---

### Q6. What are the main components of a Kubernetes cluster?

**Interview Answer**

A Kubernetes cluster consists of a control plane (API server, etcd, scheduler, controller-manager) and worker nodes where your Pods actually run. The API server is the central entry point for all operations, etcd stores cluster state as a key-value store, and the scheduler assigns Pods to nodes based on resource constraints. Worker nodes run kubelet (the node agent), kube-proxy (network rules), and your container runtime (containerd or CRI-O). Understanding this architecture is critical for debugging issues — for example, if the API server is unreachable, no deployments can proceed.

---

### Q7. What is etcd and why is it critical?

**Interview Answer**

etcd is a distributed, consistent key-value store that serves as the single source of truth for all Kubernetes cluster state — every Service, Pod, ConfigMap, and Secret is persisted there. It uses the Raft consensus algorithm to maintain consistency across multiple nodes, which is why production clusters run 3 or 5 etcd instances. If etcd becomes unavailable, the control plane cannot make scheduling decisions or record state changes, effectively freezing the cluster. Backing up etcd regularly is one of the most important operational tasks for any Kubernetes administrator.

---

### Q8. How would you deploy a Rust backend service to Kubernetes?

**Interview Answer**

First, write a multi-stage Dockerfile that builds your Rust binary in a builder stage with all build dependencies, then copies only the binary into a minimal runtime image like `gcr.io/distroless/cc-debian12`. Create a Kubernetes Deployment manifest specifying the image, resource requests/limits, and environment variables, then expose it via a Service (ClusterIP for internal, LoadBalancer for external). Add a liveness probe on a `/health` endpoint so Kubernetes restarts your Rust process if it hangs, and a readiness probe to prevent traffic until the server is ready. Use a ConfigMap for non-sensitive configuration and a Secret for database credentials, mounting them as environment variables or files in the Pod.

---

### Q9. What is the Kubernetes control plane?

**Interview Answer**

The control plane is the brain of the Kubernetes cluster, responsible for making global decisions like scheduling, detecting and responding to cluster events, and maintaining desired state. It comprises the API server (handles all REST operations), etcd (persists state), the scheduler (assigns Pods to nodes), and controller managers (run reconciliation loops for Deployments, ReplicaSets, etc.). In managed Kubernetes like EKS, the cloud provider runs and maintains the control plane, while you manage worker nodes. Understanding control plane logs is essential for debugging issues like failed scheduling due to insufficient resources.

---

### Q10. What are Kubernetes namespaces and when should you use them?

**Interview Answer**

Namespaces are a way to partition a single Kubernetes cluster into multiple virtual clusters, allowing different teams or environments (dev, staging, prod) to share the same physical infrastructure with isolation. They scope resource names, RBAC policies, and network policies — for example, a `dev` namespace can restrict Pod-to-Pod communication while `prod` allows it. You should use namespaces when multiple teams share a cluster, when you need per-environment access controls, or when you want to apply resource quotas to prevent one team from consuming all cluster resources. However, namespaces do not provide security isolation — for that, you need Pod Security Standards or dedicated clusters.