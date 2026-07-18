# Load Balancing Algorithms

## Interview Question

Explain the different load balancing algorithms and when to use each in a production Rust backend.

## Interview Answer

Load balancing distributes incoming traffic across multiple backend servers to maximize throughput, minimize latency, and ensure high availability. The main algorithms are: Round Robin (distributes requests sequentially), Least Connections (routes to the server with fewest active connections), IP Hash (routes a client to the same server consistently), Weighted Round Robin (proportional distribution based on server capacity), and Consistent Hashing (minimizes redistribution when servers are added or removed). In a Rust backend architecture, Nginx typically handles load balancing at the edge, while application-level load balancing (like service mesh) handles internal traffic. The choice depends on whether requests are stateless (round robin works), stateful (sticky sessions needed), or have variable processing times (least connections preferred).

---

## Follow-up Questions & Answers

### Q1. How does Round Robin load balancing work and when is it appropriate?

**Interview Answer**

Round Robin assigns each incoming request to the next server in a circular order: server 1, server 2, server 3, server 1, server 2, and so on. It is simple, stateless, and requires no knowledge of server load. It is appropriate when all backend servers have equal capacity and requests have similar processing times. However, it does not account for varying server loads or request complexity — a server handling a CPU-intensive query receives the same traffic as one handling a simple cache hit. In Nginx, round robin is the default algorithm configured with the upstream directive.

---

### Q2. When should you use Least Connections over Round Robin?

**Interview Answer**

Least Connections routes each request to the server with the fewest active connections, making it ideal when request processing times vary significantly. If one request takes 5 seconds and another takes 50ms, round robin might send both to the same server, while least connections distributes them based on actual load. It is particularly effective for long-lived connections like WebSockets or streaming endpoints. In Nginx, enable it with the least_conn directive in the upstream block. The trade-off is that Nginx must track connection counts, adding slight overhead, but this is negligible for most workloads.

---

### Q3. What is IP Hash load balancing and when would you use it?

**Interview Answer**

IP Hash uses a hash of the client's IP address to consistently route requests from the same client to the same server. This provides session affinity without client-side state — a user's requests always hit the same backend, which is useful for in-memory sessions, user-specific caches, or WebSocket connections that must maintain state. In Nginx, configure it with ip_hash in the upstream block. The downside is uneven distribution if traffic comes from a few large NAT gateways (corporate networks, mobile carriers), and it breaks when servers are added or removed because the hash distribution changes.

---

### Q4. What is Consistent Hashing and why is it important for load balancing?

**Interview Answer**

Consistent Hashing maps both servers and requests to positions on a hash ring. When a request arrives, it is routed to the nearest server clockwise on the ring. When a server is added or removed, only the requests that mapped to the affected segment of the ring are redistributed, rather than all requests being reshuffled. This minimizes disruption when scaling. It is essential for distributed caches (Redis, Memcached) and content delivery networks where cache invalidation on server changes would be catastrophic. In Rust, you can implement consistent hashing using the hashring crate or use it through a service mesh like Linkerd.

---

### Q5. What is Weighted Round Robin and when is it needed?

**Interview Answer**

Weighted Round Robin assigns each server a weight proportional to its capacity, and distributes traffic accordingly. A server with weight 3 receives three times the traffic of a server with weight 1. This is needed when backend servers have different capacities — a beefy dedicated server might get weight 5 while a smaller container gets weight 1. In Nginx, configure it with the weight parameter in the upstream server directive: server 10.0.0.1:8080 weight=5;. This ensures high-capacity servers handle more traffic without overloading smaller instances.

---

### Q6. How do health checks work with load balancing?

**Interview Answer**

Health checks detect unhealthy servers and remove them from the load balancing pool. Active health checks periodically send requests to a health endpoint (e.g., GET /health) and mark servers as down if they fail. Passive health checks monitor actual traffic responses and remove servers that return errors or timeouts. In Nginx, active health checks are available in Nginx Plus (with health_check directive), while the open-source version can use the third-party nginx_upstream_check_module. For Rust backends, implement a /health endpoint that checks database connectivity, disk space, and other dependencies, returning 200 when healthy and 503 when not.

---

### Q7. What is a load balancing algorithm's impact on connection pooling?

**Interview Answer**

The load balancing algorithm affects how connections are distributed across backends, which impacts connection pool efficiency. Round Robin distributes connections evenly, making simple connection pools work well. Least Connections may concentrate connections on servers with fewer connections, potentially causing pool exhaustion on lightly-loaded servers. Consistent Hashing may create hotspots if certain clients generate disproportionate traffic. In Rust, when using hyper's connection pool with Nginx in front, the load balancer manages which backend receives each connection, so your application-level connection pool should be sized based on the maximum connections any single backend will receive.

---

### Q8. How do you implement load balancing in a Rust microservice architecture?

**Interview Answer**

In a Rust microservice architecture, load balancing happens at multiple layers: Nginx at the edge for external traffic, Kubernetes Services or a service mesh (Linkerd, Istio) for internal traffic, and application-level load balancing for database connections and external API calls. For internal Rust-to-Rust communication, you can use tower::load_balance for application-level balancing, or rely on Kubernetes headless services with client-side DNS resolution. For external HTTP calls, reqwest with a connection pool handles the connection lifecycle. Consistent hashing is useful when calling distributed cache layers, ensuring requests for the same key always hit the same cache shard.

---

### Q9. What is session affinity and how does it relate to load balancing?

**Interview Answer**

Session affinity (sticky sessions) ensures that all requests from a specific client are routed to the same backend server. This is necessary when the backend stores session state in memory — if a user logs in on server 1 and their next request goes to server 2, the session is lost. Affinity is typically implemented via cookies (Nginx's ip_hash or sticky cookie directives) or IP hash. However, session affinity reduces load balancing effectiveness and makes scaling harder. The preferred approach is to externalize session state to Redis or a database, making backends stateless and enabling any load balancing algorithm.

---

### Q10. What is the difference between L4 and L7 load balancing?

**Interview Answer**

L4 (Layer 4) load balancing operates at the TCP/UDP level, forwarding packets based on IP address and port without inspecting application content. It is fast but cannot make routing decisions based on HTTP headers, URLs, or content. L7 (Layer 7) load balancing operates at the HTTP level, routing based on URL path, headers, cookies, or content type. Nginx provides L7 load balancing, allowing you to route /api/* to one upstream and /static/* to another, or route based on Authorization headers. L7 is more flexible but adds overhead from HTTP parsing. For most Rust backends, L7 load balancing provides the flexibility needed for modern API routing.
