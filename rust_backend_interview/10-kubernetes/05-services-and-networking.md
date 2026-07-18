# Services and Networking

## Interview Question

Explain the different types of Kubernetes Services and how Ingress works for external traffic routing.

## Interview Answer

Kubernetes Services provide stable networking for ephemeral Pods, assigning a consistent DNS name and IP that load balances traffic across matching Pods. ClusterIP (default) exposes the Service internally within the cluster, NodePort opens a specific port on every node for external access, and LoadBalancer provisions a cloud load balancer (like AWS NLB or GCP LB) for external traffic. For production Rust backends, you typically use ClusterIP for internal services and LoadBalancer or Ingress for external-facing APIs. Ingress is a separate resource that routes external HTTP/HTTPS traffic to Services based on hostnames and paths, handling TLS termination and path-based routing — for example, routing `/api/v1/users` to your user service and `/api/v1/orders` to your order service. Ingress requires an Ingress controller (NGINX, Traefik, or cloud-specific) to actually implement the routing rules.

---

## Follow-up Questions & Answers

### Q1. What is the difference between ClusterIP, NodePort, and LoadBalancer?

**Interview Answer**

ClusterIP creates a virtual IP accessible only within the cluster — other Pods reach it via DNS (e.g., `my-service.default.svc.cluster.local`). NodePort exposes the Service on a static port (30000-32767) on every node's IP, allowing external traffic to reach it via `<NodeIP>:<NodePort>`, but this is not recommended for production due to limited port range and security concerns. LoadBalancer provisions an external cloud load balancer that routes to the Service's ClusterIP, providing a public IP and proper load balancing. For Rust services, use ClusterIP for internal microservices, LoadBalancer for user-facing APIs, and avoid NodePort except for testing. The LoadBalancer type is the most common for production deployments because it integrates with cloud provider health checks and auto-scaling.

---

### Q2. What is an Ingress and how does it differ from a LoadBalancer Service?

**Interview Answer**

An Ingress is a Kubernetes resource that defines HTTP/HTTPS routing rules (hostnames, paths, TLS) and is implemented by an Ingress controller (NGINX, Traefik, AWS ALB Ingress Controller). A LoadBalancer Service creates a one-to-one mapping between an external IP and a Service — every LoadBalancer Service gets its own IP, which becomes expensive at scale. Ingress consolidates routing for multiple Services behind a single load balancer, enabling path-based and host-based routing (e.g., `api.example.com/users` → user Service, `api.example.com/orders` → order Service). For Rust microservices, Ingress is the standard way to expose multiple APIs through a single entry point, handling TLS termination and request routing without requiring a separate load balancer for each service.

---

### Q3. How does DNS-based service discovery work in Kubernetes?

**Interview Answer**

CoreDNS runs as a cluster add-on and automatically creates DNS A records for every Service. A Service named `user-api` in namespace `production` gets the DNS name `user-api.production.svc.cluster.local`, and other Pods can resolve it by simply using `user-api` (within the same namespace) or `user-api.production` (across namespaces). Headless Services (with `clusterIP: None`) return individual Pod IPs instead of a single virtual IP, enabling direct connection to specific Pods. For Rust services, this means your database connection string can be `postgres://db-service:5432` instead of hardcoding Pod IPs that change constantly. DNS caching in application code or service meshes can affect discovery — ensure your Rust HTTP client respects DNS TTL or re-resolves periodically.

---

### Q4. What is a Headless Service and when would you use it?

**Interview Answer**

A Headless Service (created with `clusterIP: None`) skips assigning a virtual IP and instead returns individual Pod IPs in DNS SRV records, allowing clients to connect directly to specific Pods. Use cases include StatefulSets where you need stable network identities (e.g., `pod-0.database-service`), databases that need direct peer connections (Cassandra, Kafka), and scenarios where client-side load balancing is preferred. For Rust services, headless services are useful when your application implements its own connection pooling or service mesh-like routing. The main trade-off is that you lose Kubernetes' server-side load balancing — your Rust client must handle load balancing logic itself, which adds complexity but gives more control over connection behavior.

---

### Q5. What is a NetworkPolicy and how does it control traffic?

**Interview Answer**

NetworkPolicies are Kubernetes resources that control Pod-to-Pod and namespace-to-namespace traffic using label selectors, similar to how Services select Pods. By default, all Pods can communicate with all other Pods — NetworkPolicies restrict this by specifying ingress (incoming) and egress (outgoing) rules. For example, you can create a policy that only allows traffic to your Rust backend Pods from the Ingress controller Pods, blocking all other sources. NetworkPolicies require a CNI plugin that supports them (Calico, Cilium, Weave) — Flannel does not. For production Rust services, default-deny policies (block all traffic, then explicitly allow) are a security best practice to prevent lateral movement in case of a breach.

---

### Q6. How do you implement TLS termination in Kubernetes?

**Interview Answer**

TLS termination in Kubernetes is typically handled at the Ingress level using a TLS certificate stored in a Kubernetes Secret. You create a Secret with `kubectl create secret tls my-tls-secret --cert=cert.pem --key=key.pem`, then reference it in the Ingress spec under `tls[].secretName`. The Ingress controller (NGINX, Traefik) terminates TLS and forwards decrypted HTTP traffic to your Rust backend Pods. For automatic certificate management, use cert-manager with Let's Encrypt — it watches Ingress resources with specific annotations and automatically provisions and renews certificates. For Rust services, this means your Actix/Axum server receives plain HTTP internally, simplifying your code while the Ingress layer handles all TLS complexity.

---

### Q7. What is a Service ExternalName and when is it useful?

**Interview Answer**

An ExternalName Service maps a Kubernetes DNS name to an external CNAME record, allowing Pods to access external services using the same DNS pattern as internal Services. For example, creating an ExternalName Service named `external-db` pointing to `db.example.com` lets your Rust service connect using `external-db` as the hostname, even though the database runs outside the cluster. This is useful during migrations when some dependencies are not yet containerized, or for managed services (AWS RDS, Cloud SQL) that your Pods need to access. The limitation is that ExternalName Services only support DNS CNAME resolution — they don't support IP-based endpoints or port remapping, which requires a Service with explicit endpoints.

---

### Q8. What is kube-proxy and how does it implement Service networking?

**Interview Answer**

kube-proxy runs on each node and maintains network rules that enable Service-based load balancing to backend Pods. In iptables mode (default), it creates NAT rules that intercept traffic to Service IPs and randomly distribute it across healthy backend Pod IPs. In IPVS mode, it uses the kernel's IP Virtual Server for better performance at scale (supporting more backends and more efficient load balancing algorithms like round-robin and least-connections). kube-proxy also handles NodePort rules and externalTrafficPolicy for LoadBalancer Services. For Rust services, the mode affects performance minimally at small scale, but IPVS is preferred for high-traffic services because iptables rules are evaluated sequentially (O(n) complexity) while IPVS uses hash tables (O(1)).

---

### Q9. What is externalTrafficPolicy and when should you change it?

**Interview Answer**

`externalTrafficPolicy` on a Service controls how external traffic (from LoadBalancer or NodePort) reaches Pods. The default `Cluster` mode allows traffic to be load-balanced across all Pods in the cluster, even on other nodes, which adds a network hop but provides better distribution. `Local` mode only routes to Pods on the node that received the traffic, preserving the client's source IP (no SNAT) and avoiding unnecessary cross-node traffic, but requires your Pods to be distributed across nodes. For Rust services requiring client IP logging or WebSocket connections where source IP matters, use `Local` policy. The trade-off is that `Local` can cause uneven load distribution if Pods are not evenly spread across nodes.

---

### Q10. How do you debug networking issues in Kubernetes?

**Interview Answer**

Start with `kubectl get endpoints <service>` to verify that your Service has healthy Pod endpoints — an empty endpoint list means no Pods match the Service selector. Use `kubectl exec -it <pod> -- nslookup <service-name>` to test DNS resolution from within a Pod, and `kubectl exec -it <pod> -- curl <service-ip>:<port>` to test connectivity. Check NetworkPolicies with `kubectl get networkpolicies` to see if traffic is being blocked, and inspect kube-proxy logs or iptables rules (`iptables -t nat -L`) for Service routing issues. For Rust services, also verify that your application is listening on the correct port (0.0.0.0, not 127.0.0.1) and that readiness probes are passing — a non-ready Pod won't receive traffic from the Service. Use `kubectl logs <pod> --previous` to see logs from crashed containers that might indicate network connection failures.