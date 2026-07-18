# DNS Resolution

## Interview Question

Explain the DNS resolution process and how it affects backend service connectivity.

## Interview Answer

DNS (Domain Name System) translates domain names to IP addresses through a hierarchical resolution process. When your Rust backend resolves a hostname, the OS first checks its local cache, then the hosts file, then queries the configured resolver. The resolver checks its cache, then queries root servers (which direct to TLD servers like .com), then the TLD server directs to the authoritative nameserver for the domain, which returns the final IP address. DNS supports multiple record types: A (IPv4), AAAA (IPv6), CNAME (alias), MX (mail), TXT (text), and SRV (service). DNS resolution latency and caching directly impact backend performance — a cold DNS lookup can add 50-500ms to the first request to a new service.

---

## Follow-up Questions & Answers

### Q1. What are the different DNS record types and when is each used?

**Interview Answer**

A records map a domain to an IPv4 address, AAAA records to IPv6. CNAME records alias one domain to another (e.g., www.example.com to example.com). SRV records specify host and port for a service (used by Kubernetes and Consul for service discovery). MX records define mail servers. TXT records store arbitrary text (used for SPF, DKIM, and domain verification). PTR records map IPs to domains (used for reverse DNS). In Rust backends, you typically resolve A and AAAA records for external services and SRV records for service discovery in microservice architectures.

---

### Q2. How does DNS caching work and where do caches exist?

**Interview Answer**

DNS caching exists at multiple levels: the application may cache (or the OS resolver may cache), the OS maintains a DNS cache (systemd-resolved on Linux, mDNSResponder on macOS), the configured DNS resolver caches responses, and intermediate DNS servers cache along the resolution path. Each record has a TTL (Time To Live) in seconds that specifies how long it can be cached. Short TTLs (60-300 seconds) allow faster updates but increase DNS query load. Long TTLs (3600-86400 seconds) reduce DNS load but delay propagation of changes. When deploying a Rust backend, you must consider DNS TTL when changing server IPs.

---

### Q3. How does DNS resolution affect connection establishment in a Rust backend?

**Interview Answer**

When your Rust backend makes an outbound HTTP request (e.g., calling an external API or another microservice), the OS resolves the hostname before establishing the TCP connection. If DNS resolution is slow (e.g., the resolver is unreachable or the cache is cold), the first request to a new host is delayed. The trust-dns-resolver crate provides async DNS resolution in Tokio, allowing non-blocking resolution. For performance-critical paths, you can resolve DNS once, cache the result, and use the IP address directly. Connection pooling to resolved IPs avoids repeated DNS lookups.

---

### Q4. What is DNS load balancing and how does it compare to other approaches?

**Interview Answer**

DNS load balancing returns multiple A records for a domain, and the client connects to one of them. It is simple but has significant limitations: clients typically only use the first IP returned, TTL-based caching means changes propagate slowly, and there is no health checking — a down server continues receiving traffic until the DNS record is updated. Compared to Nginx load balancing (which has health checks, weighted routing, and instant failover), DNS load balancing is less sophisticated. It is commonly used as a first layer of load balancing (DNS returns IPs of multiple Nginx instances) with application-level load balancing as the second layer.

---

### Q5. How do you implement DNS resolution in Rust using trust-dns?

**Interview Answer**

Using the trust-dns-resolver crate (now hickory-resolver), you create an AsyncResolver with system configuration or custom nameservers, then call resolver.lookup_ip(hostname).await to get IP addresses. The resolver handles caching, retries, and follow-up queries automatically. For example: let resolver = AsyncResolver::tokio_from_system_conf().unwrap(); let response = resolver.lookup_ip("api.example.com").await.unwrap(); You can then iterate over the response to get IPv4 or IPv6 addresses. This integrates naturally with Tokio for non-blocking DNS resolution in async code.

---

### Q6. What is DNS round-robin and what are its limitations?

**Interview Answer**

DNS round-robin returns multiple A records in rotation, cycling the order on each query to distribute traffic across servers. Its limitations include: no health checking (dead servers continue receiving traffic), sticky caching (clients may cache and reuse a single IP), uneven distribution (different clients cache differently), and slow propagation (TTL delays changes). For these reasons, DNS round-robin alone is insufficient for production load balancing. It is better used as a coarse-grained distribution layer with Nginx or application-level load balancing providing the fine-grained traffic management.

---

### Q7. How does DNS resolution work in containerized environments like Docker and Kubernetes?

**Interview Answer**

In Docker, containers use Docker's built-in DNS resolver which resolves container names and service names within a Docker network. In Kubernetes, CoreDNS provides cluster-internal DNS, resolving Service names to ClusterIPs and Pod names to Pod IPs. For a Rust backend in Kubernetes, when you connect to a service like my-service.my-namespace.svc.cluster.local, CoreDNS resolves it to the service's ClusterIP, and kube-proxy handles load balancing to backend pods. The search domain configuration allows short names to be resolved automatically, so my-service resolves to my-service.my-namespace.svc.cluster.local.

---

### Q8. What is DNS-based service discovery and how does it differ from other methods?

**Interview Answer**

DNS-based service discovery uses DNS records (typically SRV records) to advertise service instances. Consul, for example, registers services as DNS entries, and clients query DNS to discover available instances. This is simple and works with any language without special libraries. However, it has limitations: DNS caching can serve stale records, health checking is limited, and there is no real-time notification of changes. Alternative approaches like Consul's HTTP API, etcd, or Kubernetes service APIs provide real-time updates, health checking, and richer metadata. In Rust, you can use DNS discovery with crates like trust-dns or integrate with Consul's HTTP API for more features.

---

### Q9. How do you handle DNS resolution failures in a Rust backend?

**Interview Answer**

DNS failures should be handled gracefully with retries, fallbacks, and circuit breakers. When trust-dns-resolver fails to resolve a hostname, it returns an error that should be caught and handled. Common strategies include: retrying with exponential backoff, falling back to a cached IP if available, using a secondary DNS resolver, and implementing a circuit breaker to stop retrying after repeated failures. In production, you should monitor DNS resolution latency and failure rates. Setting appropriate timeouts (e.g., 5 seconds for DNS resolution) prevents your backend from hanging on unreachable DNS servers.

---

### Q10. What is DNS over HTTPS (DoH) and should you use it in a Rust backend?

**Interview Answer**

DNS over HTTPS (DoH) encrypts DNS queries by sending them to a resolver over HTTPS (port 443), preventing eavesdropping and manipulation of DNS traffic. The rustls and reqwest crates can be used to implement DoH in Rust. DoH improves privacy and security but adds latency due to the HTTPS overhead compared to traditional UDP DNS. It is more relevant for client applications than backend servers, as backend DNS queries typically occur within trusted network infrastructure. However, for Rust backends in environments where DNS privacy matters (e.g., multi-tenant cloud), DoH with a trusted resolver like Cloudflare (1.1.1.1) or Google (8.8.8.8) is a reasonable choice.
