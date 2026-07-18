# Kubernetes Architecture

## Interview Question

Describe the architecture of a Kubernetes cluster, including the control plane components and what runs on worker nodes.

## Interview Answer

A Kubernetes cluster is split into two planes: the control plane that makes decisions and the worker nodes that run actual workloads. The control plane consists of the API server (the single entry point for all operations), etcd (distributed key-value store holding cluster state), the scheduler (assigns Pods to nodes based on resource availability), and controller managers (run reconciliation loops to maintain desired state). Each worker node runs kubelet (agent communicating with the API server), kube-proxy (manages network rules for Services), and a container runtime like containerd. This separation means the control plane can be highly available across multiple masters while worker nodes scale horizontally. Understanding this architecture is critical for debugging — when a Pod fails to start, the root cause is often in the scheduler, kubelet, or container runtime rather than your application code.

---

## Follow-up Questions & Answers

### Q1. What is the role of the Kubernetes API server?

**Interview Answer**

The API server (kube-apiserver) is the central management entity that processes all REST operations — kubectl commands, controller reconciliations, and kubelet heartbeats all go through it. It validates and processes requests, persists state to etcd, and serves as the single coordination point for the entire cluster. The API server supports authentication (tokens, certificates), authorization (RBAC), and admission control (webhooks that mutate or validate resources). If the API server goes down, no new deployments, scaling, or state changes can happen, though existing Pods continue running.

---

### Q2. How does the Kubernetes scheduler decide which node to place a Pod on?

**Interview Answer**

The scheduler watches for Pods with no assigned node and evaluates each node through a multi-stage pipeline: filtering (removing nodes that violate constraints like taints, resource requests, or node selectors) and scoring (ranking remaining nodes by factors like resource balance, affinity preferences, and data locality). It then assigns the Pod to the highest-scoring node. For your Rust services, you can influence scheduling with resource requests (which the scheduler uses for capacity planning), node selectors, and affinity/anti-affinity rules to spread replicas across failure domains.

---

### Q3. What is kubelet and what does it do on each worker node?

**Interview Answer**

kubelet is the node agent that runs on every worker node and ensures the containers described in PodSpecs are running and healthy. It communicates with the container runtime via CRI (Container Runtime Interface) to pull images, start/stop containers, and reports node status back to the API server. kubelet also runs liveness, readiness, and startup probes, and manages Pod lifecycle including volume mounts and secret/configmap injection. When kubelet detects a container crash, it restarts the container (respecting restartPolicy) and notifies the API server if the Pod needs rescheduling.

---

### Q4. What is kube-proxy and how does it implement Services?

**Interview Answer**

kube-proxy runs on each node and maintains network rules that enable Service-based load balancing to Pods. It has three modes: iptables (default, uses netfilter rules to distribute traffic across backend Pods), IPVS (better performance for large clusters using kernel-level load balancing), and userspace (legacy, proxying through kube-proxy process). When a Pod connects to a Service DNS name, kube-proxy rules intercept the connection and route it to a healthy backend Pod's IP. For Rust services exposed via Services, kube-proxy handles the load balancing transparently without your application code needing to know about individual Pod IPs.

---

### Q5. What is etcd and how does Kubernetes use it?

**Interview Answer**

etcd is a distributed, strongly-consistent key-value store that serves as the backing store for all Kubernetes cluster data — every object from Pods to Secrets is persisted there. Kubernetes uses etcd's watch mechanism for efficient change detection, allowing controllers and the API server to react to state changes immediately. In production, etcd runs on dedicated nodes with fast SSD storage and runs as a 3 or 5 node cluster using Raft consensus for high availability. Losing etcd means losing all cluster state, making regular etcd snapshots and encrypted backups a critical operational practice.

---

### Q6. What are controller-manager controllers?

**Interview Answer**

The controller-manager runs multiple controllers, each watching a specific resource type and reconciling actual state with desired state. The ReplicaSet controller ensures the specified number of Pod replicas exist, the Deployment controller manages rolling updates and rollbacks, and the Node controller handles node heartbeat timeouts and Pod eviction. Each controller runs a simple loop: list resources, compare to desired state, take corrective action (create/delete Pods, update status). This controller pattern is the core of Kubernetes' self-healing architecture — you declare what you want, and controllers continuously work to make it real.

---

### Q7. What is a CNI plugin and which ones are commonly used?

**Interview Answer**

CNI (Container Network Interface) plugins provide networking for Pods in a Kubernetes cluster, assigning IP addresses and enabling Pod-to-Pod communication across nodes. Common plugins include Calico (policy-based networking with network policy support), Cilium (eBPF-based, offering superior performance and observability), Flannel (simple overlay networking, limited network policy support), and Weave Net (encryption and multicast support). For production Rust backends, Cilium is increasingly preferred because eBPF provides kernel-level load balancing and network policy enforcement without kube-proxy overhead. The choice affects Pod IP allocation, network policy capabilities, and debugging tooling.

---

### Q8. What are admission controllers in Kubernetes?

**Interview Answer**

Admission controllers are plugins that intercept requests to the Kubernetes API server after authentication and authorization but before the object is persisted to etcd. Mutating admission controllers can modify incoming resources (adding labels, injecting sidecars), while validating admission controllers can reject resources that violate policies (requiring resource limits, enforcing naming conventions). They run as a pipeline — mutating controllers first, then the API server persists, then validating controllers check the result. Custom admission webhooks allow you to implement organization-specific policies, such as requiring all Rust deployment images come from a trusted registry.

---

### Q9. How does high availability work in Kubernetes?

**Interview Answer**

HA in Kubernetes means running multiple control plane nodes (typically 3 or 5) with a load balancer in front of the API server, and ensuring etcd runs across all control plane nodes with quorum. Managed Kubernetes services like EKS handle control plane HA automatically, while self-managed clusters require careful setup of keepalived or cloud load balancers. Worker node HA is achieved by running multiple replicas of your Rust service across nodes, with PodDisruptionBudgets ensuring at least a minimum number remain available during node maintenance. For stateful workloads, use PodAntiAffinity to spread replicas across failure domains (different zones or racks).

---

### Q10. What is the Container Runtime Interface (CRI)?

**Interview Answer**

CRI is the standard interface between kubelet and container runtimes, allowing Kubernetes to support different runtimes without code changes. Before CRI, kubelet used Docker-specific APIs, but CRI standardized this with runtime classes like containerd (default in most clusters) and CRI-O (purpose-built for Kubernetes). You can even use lightweight VM runtimes like kata-containers or gVisor for stronger isolation of security-sensitive workloads. For Rust services, CRI is transparent — your Dockerfile builds an image, and whichever CRI-compatible runtime the cluster uses will run it identically. Understanding CRI helps when debugging image pull failures or runtime-level issues in production.