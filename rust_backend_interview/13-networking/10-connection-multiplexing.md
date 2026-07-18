# Connection Multiplexing

## Interview Question

What is connection multiplexing in HTTP/2 and how does it improve performance over HTTP/1.1?

## Interview Answer

Connection multiplexing is the ability to send multiple independent HTTP requests and responses simultaneously over a single TCP connection. In HTTP/1.1, each request occupies the connection until the response completes — browsers work around this by opening 6 parallel connections per origin. HTTP/2 introduces streams, where each request/response pair is a stream identified by a unique ID, and frames from multiple streams are interleaved on the single TCP connection. This eliminates the overhead of establishing and maintaining multiple TCP connections, reduces TLS handshake overhead, and allows the server to prioritize responses. In Rust backends, hyper handles HTTP/2 multiplexing automatically, and connection pooling with multiplexing means you need far fewer connections to downstream services.

---

## Follow-up Questions & Answers

### Q1. How does HTTP/2 multiplexing work at the frame level?

**Interview Answer**

HTTP/2 splits messages into binary frames, each with a 9-byte header containing frame type, flags, stream identifier, and payload. DATA frames carry body content, HEADERS frames carry headers, and control frames (SETTINGS, PING, GOAWAY) manage the connection. Multiple streams' frames are interleaved on the single TCP connection. The receiver reassembles frames by stream ID. Flow control operates per-stream and per-connection — a SETTINGS frame negotiates the initial window size, and WINDOW_UPDATE frames adjust flow control. This means a large file download on stream 1 does not block a small API response on stream 3.

---

### Q2. What is head-of-line blocking and how does multiplexing address it?

**Interview Answer**

Head-of-line blocking in HTTP/1.1 means the second request on a connection must wait for the first response to complete. HTTP/2 multiplexing eliminates HTTP-layer head-of-line blocking by allowing concurrent streams. However, HTTP/2 still has TCP-layer head-of-line blocking — if one TCP packet is lost, all streams on that connection are blocked until the packet is retransmitted. This is because TCP guarantees ordered delivery of bytes, and the HTTP/2 framing layer cannot process frames until the lost segment arrives. HTTP/3 (QUIC) eliminates both layers by using independent streams that do not block each other.

---

### Q3. How does connection pooling relate to multiplexing?

**Interview Answer**

Connection pooling maintains a set of reusable TCP connections to downstream services. With HTTP/1.1, each pooled connection handles one request at a time, so you need many connections for high concurrency. With HTTP/2 multiplexing, a single pooled connection handles many concurrent requests, reducing the pool size needed. In Rust, hyper's connection pool automatically reuses HTTP/2 connections. For example, instead of 50 HTTP/1.1 connections to a database proxy, you might need only 2-3 HTTP/2 connections because each handles hundreds of concurrent streams. This reduces memory usage, TCP overhead, and TLS handshake costs.

---

### Q4. How does HTTP/2 stream prioritization work?

**Interview Answer**

HTTP/2 allows clients to assign priority and dependency to streams using PRIORITY frames. A stream can depend on another stream and be assigned a weight (1-256). This tells the server which responses to prioritize when bandwidth is limited. For example, a browser might prioritize the HTML response (to start rendering) over image responses. Servers use this information to schedule frame transmission — sending HEADERS and early DATA frames for high-priority streams first. In practice, prioritization is complex and many servers (including Nginx) implement simplified versions. For Rust backends, the impact is minimal since responses are typically small.

---

### Q5. What is connection coalescing in HTTP/2?

**Interview Answer**

Connection coalescing allows HTTP/2 clients to reuse a single connection for requests to different hostnames if the server's TLS certificate covers both domains (via SAN entries). For example, if api.example.com and cdn.example.com share a wildcard certificate, a client can send requests for both on the same HTTP/2 connection. This reduces the number of connections needed when a Rust backend serves multiple domains behind the same Nginx instance. Browsers implement this automatically, and HTTP clients like reqwest can be configured to coalesce connections when certificates allow.

---

### Q6. How do you implement connection multiplexing in a Rust HTTP client?

**Interview Answer**

Using reqwest with HTTP/2, connection multiplexing is automatic — reqwest maintains a connection pool and reuses HTTP/2 connections for requests to the same host. You configure it with Client::builder().http2_prior_knowledge() or by using a client with TLS (which negotiates HTTP/2 via ALPN). For custom implementations, hyper provides fine-grained control over connection pooling and HTTP/2 settings. The key settings to tune are max_concurrent_streams (how many streams per connection) and initial_window_size (flow control per stream). In production, monitor connection pool metrics to ensure you are not creating unnecessary connections.

---

### Q7. What are the limitations of HTTP/2 multiplexing?

**Interview Answer**

The primary limitation is TCP-level head-of-line blocking — a single lost packet blocks all streams on the connection. This is particularly impactful on unreliable networks (mobile, WiFi). Another limitation is that multiplexing increases CPU usage at the server because it must process frames from many streams concurrently. Server-side flow control can also cause issues if one stream consumes all available window space. Additionally, some middleboxes (firewalls, load balancers) may not properly handle HTTP/2 multiplexing, causing connection resets. HTTP/3 addresses the TCP head-of-line blocking issue through QUIC's independent streams.

---

### Q8. How does multiplexing affect load balancing decisions?

**Interview Answer**

With HTTP/2 multiplexing, all streams on a connection go to the same backend server. This means load balancing decisions are made per-connection, not per-request. If a client opens one HTTP/2 connection and sends 100 requests on different streams, all 100 go to the same server. This can cause uneven load if one client sends many more requests than others. Nginx handles this by load-balancing at the connection level and using the least_connections algorithm to distribute connections evenly. For fine-grained load balancing, some implementations split HTTP/2 connections at the proxy level, but this is complex and uncommon.

---

### Q9. How does HTTP/2 server push relate to multiplexing?

**Interview Answer**

HTTP/2 server push allows the server to proactively send resources the client has not yet requested, using the same multiplexed connection. For example, when serving an HTML page, the server can push the CSS and JavaScript files on separate streams before the client parses the HTML and discovers them. Server push uses PUSH_PROMISE frames to notify the client. However, server push has been largely deprecated in practice because it is difficult to predict what the client needs (browser caches,CDN caches), and it can waste bandwidth by pushing resources the client already has. Most browsers have removed HTTP/2 push support in favor of 103 Early Hints.

---

### Q10. How do you benchmark and monitor connection multiplexing performance?

**Interview Answer**

Key metrics to monitor include: streams per connection (should approach max_concurrent_streams), connection reuse rate (higher is better), time-to-first-byte per stream, and frame processing latency. Tools like h2load benchmark HTTP/2 multiplexing performance, measuring requests per second and latency under concurrent load. In production, monitor connection pool metrics in Rust (active connections, idle connections, connection reuse ratio) using metrics crates. Nginx's stub_status or the nginx-vts module provides upstream connection metrics. A well-configured HTTP/2 setup shows high stream counts per connection and low connection churn.
