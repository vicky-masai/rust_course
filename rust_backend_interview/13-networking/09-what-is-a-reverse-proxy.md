# What is a Reverse Proxy?

## Interview Question

What is a reverse proxy and how does it differ from a forward proxy in a production Rust backend architecture?

## Interview Answer

A reverse proxy is an intermediary server that sits in front of backend application servers, receiving client requests and forwarding them to the appropriate backend. Unlike a forward proxy (which sits in front of clients and hides the client's identity from servers), a reverse proxy hides the backend server's identity from clients. In a Rust backend architecture, Nginx or Traefik typically acts as a reverse proxy, providing SSL termination, load balancing, request buffering, rate limiting, and security headers before requests reach the Axum application. The reverse proxy is the single entry point for all traffic, enabling centralized management of security, monitoring, and traffic routing without modifying application code.

---

## Follow-up Questions & Answers

### Q1. What are the main benefits of using a reverse proxy?

**Interview Answer**

Reverse proxies provide SSL/TLS termination (encrypting client traffic without burdening backend servers), load balancing across multiple instances, request buffering (protecting backends from slow clients), rate limiting and DDoS protection, static file serving (offloading asset delivery from the application), security header injection, gzip/brotli compression, and request logging and monitoring. They also hide backend infrastructure details from clients, making it harder for attackers to target specific servers. For a Rust backend, using Nginx as a reverse proxy means Axum only needs to handle business logic, not infrastructure concerns.

---

### Q2. How does a reverse proxy differ from a forward proxy?

**Interview Answer**

A forward proxy sits in front of clients, sending requests to servers on behalf of the client. It hides the client's identity from the server (e.g., a corporate proxy that anonymizes employee browsing). A reverse proxy sits in front of servers, receiving requests from clients on behalf of the server. It hides the server's identity from the client (e.g., Nginx distributing traffic across multiple Axum instances). The forward proxy is client-configured; the reverse proxy is server-configured. Both proxy requests, but they serve opposite purposes in the network topology.

---

### Q3. What is the difference between a reverse proxy and a load balancer?

**Interview Answer**

A load balancer specifically distributes traffic across multiple servers, while a reverse proxy provides load balancing plus many additional features. All reverse proxies can act as load balancers, but not all load balancers are reverse proxies. A dedicated Layer 4 load balancer (like HAProxy in TCP mode) forwards packets without inspecting HTTP content, while a reverse proxy (like Nginx) provides L7 load balancing with HTTP-aware routing, header manipulation, caching, and security features. In practice, Nginx serves as both a reverse proxy and load balancer for Rust backends.

---

### Q4. What are common use cases for reverse proxies in microservice architectures?

**Interview Answer**

Reverse proxies serve as API gateways routing to multiple microservices, providing a single entry point for authentication, rate limiting, and logging. They handle service mesh ingress, routing external traffic to the correct service. They implement canary deployments by splitting traffic between service versions. They provide circuit breaking to protect against cascading failures. They handle protocol translation (HTTP/2 to HTTP/1.1). In a Rust microservice architecture, Nginx or Envoy at the edge, and Linkerd or Istio service mesh internally, provide these reverse proxy capabilities.

---

### Q5. How does Nginx compare to other reverse proxies like Envoy, Traefik, or Caddy?

**Interview Answer**

Nginx is the most mature and widely deployed, with a vast module ecosystem and proven performance. Envoy (used by Istio) provides advanced observability, dynamic configuration through xDS APIs, and native support for service mesh, making it ideal for complex microservice architectures. Traefik integrates natively with Docker and Kubernetes, providing automatic service discovery and Let's Encrypt certificate management. Caddy offers the simplest configuration with automatic HTTPS. For Rust backends, Nginx is the default for most deployments, Envoy for Kubernetes service meshes, and Caddy for simpler setups where automatic TLS is valuable.

---

### Q6. What is request buffering and why is it important?

**Interview Answer**

Request buffering means the reverse proxy reads the entire client request before forwarding it to the backend. This protects backends from slow client attacks where attackers send headers or body data extremely slowly to tie up backend resources. With buffering, Nginx absorbs the slow upload, and the backend only receives complete requests. The proxy_buffering and client_body_timeout directives control this behavior. For file uploads, buffering ensures the backend processes the complete file rather than streaming partial data. The trade-off is increased memory usage at the proxy layer for large requests.

---

### Q7. How do you configure a reverse proxy for WebSocket support?

**Interview Answer**

WebSocket connections require the reverse proxy to forward the HTTP Upgrade headers. In Nginx, configure proxy_set_header Upgrade $http_upgrade and proxy_set_header Connection "upgrade" in the location block. You also need proxy_http_version 1.1 (WebSockets require HTTP/1.1 or higher) and proxy_read_timeout set to a high value to prevent idle WebSocket connections from being closed. Without these headers, Nginx treats WebSocket requests as regular HTTP and closes them after the default timeout. For Traefik, WebSocket support is automatic when using the HTTP router.

---

### Q8. What is the impact of reverse proxy on request latency?

**Interview Answer**

A reverse proxy adds a small amount of latency per request — typically 0.1-1ms for local Nginx proxying due to the overhead of receiving the request, parsing headers, and forwarding to the backend. This is negligible compared to typical API response times. However, for extremely latency-sensitive applications (high-frequency trading), even this small overhead matters. In those cases, you can use Nginx's raw TCP proxying (L4) instead of HTTP proxying (L7) to reduce overhead. For most Rust backends, the benefits of a reverse proxy (security, load balancing, monitoring) far outweigh the minimal latency increase.

---

### Q9. How do reverse proxies handle certificate management for HTTPS?

**Interview Answer**

The reverse proxy manages TLS certificates centrally. With Nginx, you specify the certificate and key paths in the server block configuration. Certificates from Let's Encrypt are automatically renewed using certbot or acme.sh, and Nginx is reloaded to pick up new certificates. For multiple domains, you configure separate server blocks with different certificates, or use a wildcard certificate. The backend application receives plain HTTP, so it does not need to know about certificates. This centralization simplifies certificate management — you update one file instead of redeploying every backend instance.

---

### Q10. What is the role of a reverse proxy in defense-in-depth security?

**Interview Answer**

A reverse proxy is a critical layer in defense-in-depth. It provides the first line of defense by filtering malicious requests before they reach the application. It can block known attack patterns (SQL injection, path traversal) through WAF modules, enforce rate limiting to prevent brute-force attacks, strip internal server headers to avoid information disclosure, implement IP allowlisting/denylisting, and enforce TLS policies. For a Rust backend, even if Axum has proper input validation, the reverse proxy provides an additional security layer that catches attacks early and reduces the attack surface exposed to the application.
