# Kubernetes Network Policies

## Interview Question

What are Kubernetes NetworkPolicies, how do they work, and what role do service meshes play in Kubernetes networking?

## Interview Answer

Kubernetes NetworkPolicies are firewall rules that control Pod-to-Pod and namespace-to-namespace traffic using label selectors, similar to how Services select Pods. By default, all Pods can communicate with all other Pods — NetworkPolicies restrict this by specifying ingress (incoming) and egress (outgoing) rules. A default-deny policy (select all Pods but allow no traffic) followed by specific allow rules is the security best practice. NetworkPolicies require a CNI plugin that supports them (Calico, Cilium, Weave) — Flannel does not. Service meshes like Istio and Linkerd add a sidecar proxy (Envoy or Linkerd2-proxy) to each Pod, providing mTLS encryption between services, advanced traffic management (canary routing, circuit breaking), and detailed observability without application code changes. For Rust services, a service mesh gives you mutual TLS between your Axum/Actix services automatically, plus distributed tracing and traffic splitting for canary deployments. The trade-off is added complexity, resource overhead from sidecars, and debugging difficulty when the mesh intercepts traffic.

---

## Follow-up Questions & Answers

### Q1. How do you create a default-deny NetworkPolicy?

**Interview Answer**

Create a NetworkPolicy with an empty `ingress` and `egress` selector that applies to all Pods in the namespace. The YAML specifies `podSelector: {}` (all Pods) with empty `ingress: []` and `egress: []` rules, which denies all traffic by default. Then create additional NetworkPolicies to explicitly allow required traffic — for example, allow ingress from the Ingress controller to your Rust service, and allow egress from your Rust service to the database. This defense-in-depth approach ensures that even if a Pod is compromised, it cannot communicate with other Pods unless explicitly permitted. Apply the default-deny policy to all namespaces (including kube-system) and test that your Rust services still function correctly by verifying that required communication paths are explicitly allowed.

---

### Q2. What are the differences between Calico, Cilium, and Flannel?

**Interview Answer**

Calico is a mature, policy-focused CNI that supports NetworkPolicies, BGP networking, and IP-in-IP or VXLAN encapsulation — it's widely used in production and offers strong network policy enforcement. Cilium is eBPF-based, providing superior performance (kernel-level packet processing without iptables), advanced observability (Hubble UI for network flow visualization), and L7 policy support (allow/block based on HTTP headers, not just IPs). Flannel is the simplest CNI but lacks NetworkPolicy support — use it only for development or non-security-sensitive environments. For production Rust services, Cilium is increasingly preferred because eBPF eliminates kube-proxy overhead, provides better debugging with network flow logs, and supports identity-based security (policies based on service account, not just IPs).

---

### Q3. How do you restrict traffic to a specific namespace using NetworkPolicies?

**Interview Answer**

Use the `namespaceSelector` field in a NetworkPolicy to match namespaces by labels. Create a policy with `ingress.from` containing a `namespaceSelector` that matches the namespace label (e.g., `kubernetes.io/metadata.name: production`) and a `podSelector` that matches specific Pods. For example, a policy that only allows traffic from the `ingress-nginx` namespace to your Rust service: `ingress: [{from: [{namespaceSelector: {matchLabels: {kubernetes.io/metadata.name: ingress-nginx}}}], port: [{port: 8080}]}]`. Combine namespace and Pod selectors for granular control: only traffic from specific Pods in specific namespaces. For production Rust services, this isolates your API from other namespaces while allowing traffic from the Ingress controller namespace, implementing defense-in-depth at the namespace level.

---

### Q4. What is mTLS and how does a service mesh implement it?

**Interview Answer**

mTLS (mutual TLS) encrypts traffic between services and authenticates both the client and server using certificates — unlike one-way TLS where only the server proves its identity. Service meshes like Istio and Linkerd implement mTLS by injecting sidecar proxies into each Pod that automatically handle certificate issuance, rotation, and verification without application code changes. When your Rust service calls another service, the sidecar intercepts the connection, establishes mTLS with the target's sidecar, and forwards decrypted traffic to the target Pod via localhost. This means your Rust code connects to `localhost` while the sidecar handles all encryption transparently. For production, mTLS prevents man-in-the-middle attacks between services and enables zero-trust networking where every service-to-service connection is encrypted and authenticated.

---

### Q5. How do you implement egress NetworkPolicies?

**Interview Answer**

Egress policies control outbound traffic from Pods — use them to prevent compromised Pods from contacting external services or other namespaces. Define `egress` rules with `to` (destination) and `ports` fields: for example, allow your Rust service to reach only the database namespace on port 5432 and the external API on port 443. Create a default-deny egress policy first (`egress: []`), then add specific allow rules. Without egress policies, a compromised Rust Pod could exfiltrate data to any external endpoint. Egress policies are applied at the Pod level using label selectors, and destination labels can include `ipBlock` for external IPs, `namespaceSelector` for cross-namespace traffic, and `podSelector` for intra-namespace traffic. Test egress policies thoroughly to ensure your Rust services can reach all required dependencies.

---

### Q6. What is a service mesh and what problems does it solve?

**Interview Answer**

A service mesh is an infrastructure layer (Istio, Linkerd, Consul Connect) that handles service-to-service communication concerns — mTLS, load balancing, retries, circuit breaking, traffic splitting, and observability — without changing application code. It injects sidecar proxies into each Pod that intercept all inbound and outbound traffic. For Rust services, a service mesh provides distributed tracing (automatic span propagation), canary deployments (traffic splitting by percentage), and resilience patterns (automatic retries, timeouts, circuit breaking) that you'd otherwise implement in application code. The main drawbacks are added latency (each request passes through two sidecars), resource overhead (sidecar memory/CPU), debugging complexity (traffic flows through the mesh, not directly), and operational overhead (managing mesh configuration alongside application configuration). Evaluate whether the benefits justify the complexity for your specific use case.

---

### Q7. What are the performance implications of NetworkPolicies?

**Interview Answer**

NetworkPolicies add minimal performance overhead when implemented efficiently — Calico evaluates rules using iptables (O(n) rule evaluation, slight latency per rule) or eBPF (O(1) evaluation, negligible overhead). Cilium's eBPF implementation is particularly efficient because it processes packets in the kernel without traversing iptables chains. For most Rust services, the performance impact is negligible compared to the security benefits. However, in extremely high-throughput scenarios (millions of requests per second), the per-packet evaluation overhead can become measurable. NetworkPolicies also add operational complexity — debugging blocked traffic requires examining policy logs (Calico's `calicoctl` or Cilium's Hubble) and understanding label selectors. Always test NetworkPolicy changes in staging before applying to production to verify that legitimate traffic is not blocked.

---

### Q8. How do you debug NetworkPolicy issues in Kubernetes?

**Interview Answer**

Start by checking if NetworkPolicies exist in the namespace with `kubectl get networkpolicies`, then describe each policy to understand the rules. Use `kubectl exec -it <pod> -- curl <target>` to test connectivity directly, and check if traffic is being blocked by comparing the result with and without NetworkPolicies. For Calico, use `calicoctl node status` and `calicoctl policy stats` to see policy hit counts. For Cilium, use `hubble observe` to see real-time network flows and which policies are allowing or denying traffic. Common issues include: policies with label selectors that don't match the intended Pods, missing egress rules blocking DNS resolution (CoreDNS needs egress access), and namespace selector mismatches. For Rust services, if your service can't reach the database, first verify DNS resolution (`nslookup db-service`), then check egress policies, then check ingress policies on the database Pods.

---

### Q9. How do service meshes handle traffic splitting for canary deployments?

**Interview Answer**

Service meshes like Istio use VirtualService resources to split traffic between service versions by percentage — for example, 90% to stable and 10% to canary. Linkerd uses TrafficSplit CRDs for similar functionality. The sidecar proxy in each Pod enforces the split rules, routing requests to the appropriate backend version based on the configured percentages. This is more sophisticated than Kubernetes-native canary deployments (which use replica counts) because it provides precise traffic control, header-based routing, and automatic rollback based on error rates. For Rust services, you'd create a VirtualService targeting your service with traffic rules splitting between version labels. The mesh also provides metrics on each version's performance, enabling data-driven decisions about promoting the canary. This eliminates the need for external load balancers or ingress-based canary configurations.

---

### Q10. What is the future of Kubernetes networking with eBPF?

**Interview Answer**

eBPF is transforming Kubernetes networking by replacing traditional iptables-based networking and service mesh sidecars with kernel-level packet processing. Cilium uses eBPF for NetworkPolicies, load balancing (replacing kube-proxy), and observability — all without user-space proxies. Tetragon (from Cilium) provides security observability and runtime enforcement using eBPF, detecting suspicious system calls in real-time. For Rust services, eBPF-based networking means lower latency (no sidecar overhead), better observability (kernel-level flow logs), and stronger security (runtime threat detection). The trade-off is that eBPF requires Linux kernel 4.19+ and is more complex to debug than traditional networking. As the ecosystem matures, eBPF will likely become the default for production Kubernetes clusters, especially for performance-sensitive Rust services where sidecar overhead is unacceptable.