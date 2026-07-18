# Nginx

## Interview Question

What is Nginx and how does it function in a production Rust backend architecture?

## Interview Answer

Nginx is a high-performance web server and reverse proxy that sits in front of backend application servers. It handles SSL/TLS termination, load balancing across multiple backend instances, static file serving, request buffering, and rate limiting. In a Rust backend architecture, Nginx typically receives all incoming HTTP/HTTPS traffic and forwards dynamic requests to one or more Axum or Actix-web servers running on localhost. Nginx's event-driven, non-blocking architecture allows it to handle tens of thousands of concurrent connections with minimal memory usage, making it ideal for absorbing traffic spikes before they reach the application layer. It also provides security benefits by hiding backend server details, preventing direct internet exposure of the application process.

---

## Follow-up Questions & Answers

### Q1. Why use Nginx in front of a Rust backend instead of serving directly from Axum?

**Interview Answer**

Serving directly from Axum exposes the application process to the public internet, which is a security risk. Nginx provides SSL/TLS termination, so your Rust application only handles plain HTTP internally. It also handles static assets directly without burdening the application, provides request buffering to protect against slow client attacks, and offers mature load balancing. Additionally, Nginx can gracefully handle connection limits, serve as a buffer during deployments, and provides battle-tested security headers that are simpler to configure than implementing them in application code.

---

### Q2. How does Nginx load balancing work with multiple Rust backend instances?

**Interview Answer**

Nginx uses an upstream block to define a pool of backend servers, then distributes incoming requests across them using algorithms like round-robin (default), least connections, IP hash, or weighted round-robin. For example, an Axum API running on three ports (8080, 8081, 8082) would be configured in the upstream block, and Nginx would distribute requests across them. Least connections is often preferred for backends with variable request processing times, as it routes new requests to the server with fewest active connections. Health checks can be configured to automatically remove unhealthy backends from rotation.

---

### Q3. What is the difference between a forward proxy and a reverse proxy?

**Interview Answer**

A forward proxy sits in front of clients and sends requests on their behalf to servers — it hides the client's identity from the server (e.g., a VPN or corporate proxy). A reverse proxy sits in front of servers and receives requests on their behalf — it hides the server's identity from the client. Nginx acts as a reverse proxy: clients connect to Nginx thinking it is the origin server, while Nginx forwards the request to a backend. This is essential for security, load balancing, and SSL termination in production architectures.

---

### Q4. How do you configure Nginx as a reverse proxy for an Axum application?

**Interview Answer**

In the Nginx configuration, you define an upstream block listing your Axum server addresses, then configure a server block that proxies requests to that upstream. Key directives include proxy_pass to the upstream, proxy_set_header to forward client IP and host information, proxy_http_version 1.1 for keep-alive support, and proxy_buffering to prevent slow client issues. You also set proxy_connect_timeout and proxy_read_timeout to prevent hung connections. For WebSocket support, you add Upgrade and Connection headers with proxy_set_header.

---

### Q5. What is SSL/TLS termination and why does Nginx handle it?

**Interview Answer**

SSL/TLS termination means the reverse proxy decrypts incoming HTTPS traffic and forwards plain HTTP to the backend. Nginx handles this because it is optimized for cryptographic operations and can offload this CPU-intensive work from the application servers. Certificates are managed in one place rather than duplicated across every backend instance. This also simplifies certificate rotation — you update one Nginx configuration instead of redeploying every backend. In production, Nginx with OpenSSL can handle thousands of TLS handshakes per second using hardware acceleration.

---

### Q6. How does Nginx handle WebSocket connections for real-time features?

**Interview Answer**

WebSocket connections require an HTTP upgrade handshake. Nginx proxies this by setting the Upgrade and Connection headers and using proxy_set_header to pass them through to the backend. You also set proxy_read_timeout to a high value (or disable it) so Nginx does not close idle WebSocket connections. The key configuration is proxy_http_version 1.1 and the Upgrade/Connection header directives in the location block. Without these, Nginx will treat WebSocket connections as regular HTTP and close them prematurely.

---

### Q7. What rate-limiting features does Nginx provide?

**Interview Answer**

Nginx provides the limit_req and limit_conn modules for rate limiting. The limit_req_zone directive defines a shared memory zone keyed by client IP or other variables, and limit_req applies a rate limit to a location block with configurable burst and delay parameters. For example, you can allow 10 requests per second per IP with a burst of 20. The limit_conn module limits concurrent connections per client. These features protect backend servers from abuse, DDoS attempts, and resource exhaustion without requiring custom rate-limiting code in the Rust application.

---

### Q8. How do you configure Nginx to handle slow client attacks?

**Interview Answer**

Slow client attacks involve sending HTTP headers or body data extremely slowly to keep connections open. Nginx mitigates this with client_header_timeout and client_body_timeout directives, which close connections that do not send headers or body within a configured time. proxy_buffering ensures Nginx reads the full request before forwarding to the backend, preventing slow clients from tying up backend worker threads. Large client_max_body_size limits prevent memory exhaustion from oversized uploads. These layers of defense ensure backend resources are spent only on legitimate, complete requests.

---

### Q9. What is Nginx's role in a blue-green or canary deployment strategy?

**Interview Answer**

In blue-green deployment, Nginx serves as the traffic router that switches between the old (blue) and new (green) backend versions. You update the upstream configuration to point to the new backend and reload Nginx, causing zero-downtime traffic cutover. For canary deployments, you can use weighted upstream directives to send a percentage of traffic to the new version while monitoring for errors. Nginx Plus offers native canary features, while the open-source version achieves this through weighted upstream blocks and conditional routing based on headers or cookies.

---

### Q10. How does Nginx compare to alternative reverse proxies like Caddy or Traefik?

**Interview Answer**

Nginx is the most mature and widely deployed reverse proxy with extensive documentation and community support. Caddy offers automatic HTTPS with Let's Encrypt out of the box and a simpler configuration format, making it appealing for smaller deployments. Traefik integrates natively with container orchestration platforms like Docker and Kubernetes, providing automatic service discovery and certificate management. For Rust backends in production, Nginx remains the default choice due to its proven performance, extensive module ecosystem, and battle-tested stability, though Caddy is gaining traction for its ease of configuration.
