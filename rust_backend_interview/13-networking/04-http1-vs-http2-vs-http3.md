# HTTP/1.1 vs HTTP/2 vs HTTP/3

## Interview Question

Explain the evolution from HTTP/1.1 to HTTP/2 to HTTP/3 and the key differences between each version.

## Interview Answer

HTTP/1.1 uses text-based messages over TCP with one request per connection (or pipelining with head-of-line blocking). HTTP/2 introduced binary framing, header compression (HPACK), and multiplexing — multiple requests and responses can share a single TCP connection simultaneously, eliminating head-of-line blocking at the HTTP layer. HTTP/3 replaces TCP with QUIC (built on UDP), eliminating head-of-line blocking at the transport layer entirely, adding 0-RTT connection setup, and improving performance on unreliable networks. In a Rust backend, Axum serves HTTP/1.1 and HTTP/2 natively through hyper, and HTTP/3 support is emerging through the quinn crate. The evolution addresses fundamental performance limitations at each layer.

---

## Follow-up Questions & Answers

### Q1. What is head-of-line blocking and how does each HTTP version address it?

**Interview Answer**

Head-of-line blocking occurs when a request must wait for previous requests to complete before being processed. HTTP/1.1 has strict head-of-line blocking — only one request can be in-flight per connection at a time. HTTP/2 eliminates HTTP-layer head-of-line blocking through multiplexing, allowing multiple requests to be in-flight simultaneously on one connection. However, HTTP/2 still suffers from TCP-level head-of-line blocking — if one TCP segment is lost, all streams on that connection are blocked until it is retransmitted. HTTP/3 eliminates this entirely by using QUIC, where each stream operates independently and packet loss on one stream does not block others.

---

### Q2. How does HTTP/2 multiplexing work?

**Interview Answer**

HTTP/2 splits messages into binary frames, each tagged with a stream identifier. Multiple streams can interleave frames on a single TCP connection, and the receiver reassembles frames by stream ID. This means a large response and several small responses can be transmitted simultaneously. The flow control is per-stream and per-connection, preventing any single stream from monopolizing bandwidth. In Rust, hyper handles HTTP/2 framing transparently — when you write an Axum handler response, hyper frames it into the correct stream. Browsers typically open one HTTP/2 connection per origin and multiplex all requests on it.

---

### Q3. What is HPACK header compression in HTTP/2?

**Interview Answer**

HPACK compresses HTTP headers using a dynamic table shared between client and server. Common headers like Content-Type or Authorization are assigned short indices rather than being sent in full on every request. The dynamic table learns which headers are frequently used and encodes them as single-byte indices. This reduces header overhead from hundreds of bytes to a few bytes for repeated headers. HPACK also uses Huffman encoding for header values. This is critical because HTTP/1.1 sends headers uncompressed on every request, and with cookies, headers can grow to several KB per request.

---

### Q4. What are the key improvements HTTP/3 makes over HTTP/2?

**Interview Answer**

HTTP/3 uses QUIC instead of TCP, providing four key improvements: no transport-layer head-of-line blocking (packet loss only affects the affected stream), 0-RTT connection resumption (data can be sent immediately on reconnect), connection migration (connections survive network changes like switching from WiFi to cellular), and built-in TLS 1.3 (encryption is mandatory and integrated into the protocol). QUIC also handles congestion control in user space, allowing faster deployment of improvements like BBR. For Rust backends, these improvements mean better performance on mobile networks and unreliable connections.

---

### Q5. How do you enable HTTP/2 in a Rust Axum server?

**Interview Answer**

Axum uses hyper which supports HTTP/2 automatically. When using hyper with TLS (via rustls), HTTP/2 is negotiated via ALPN during the TLS handshake — the client advertises support for h2 and the server selects it. Without TLS, HTTP/2 can be negotiated using h2c (HTTP/2 cleartext) via the Upgrade mechanism, though this is less common. In practice, you configure TLS with rustls, and hyper handles the HTTP/2 protocol negotiation and framing automatically. The client also needs to support HTTP/2 — modern browsers do, and HTTP clients like reqwest support it.

---

### Q6. What is 0-RTT in HTTP/3 and what are its security implications?

**Interview Answer**

0-RTT (Zero Round Trip Time Resumption) allows a client to send encrypted application data during the TLS handshake on a repeated connection, without waiting for the handshake to complete. This eliminates the latency penalty of connection setup. However, 0-RTT data is vulnerable to replay attacks — an attacker can capture and replay the 0-RTT data. For non-idempotent requests (like POST requests that create resources), this is a security risk. Servers must either reject 0-RTT for non-idempotent requests or implement replay protection using tokens or server-side deduplication.

---

### Q7. When would you stick with HTTP/1.1 in a modern Rust backend?

**Interview Answer**

HTTP/1.1 is still appropriate for internal service-to-service communication where the overhead of HTTP/2 framing is unnecessary, for simple APIs with few concurrent requests per connection, and for compatibility with legacy clients that do not support HTTP/2. HTTP/1.1 is also simpler to debug and test with tools like curl. Additionally, for long-lived streaming connections (SSE), HTTP/1.1 is still commonly used. Most production Axum servers serve both HTTP/1.1 and HTTP/2 simultaneously, allowing the client to negotiate the best available version.

---

### Q8. How does HTTP/3 handle connection migration?

**Interview Answer**

QUIC connections are identified by a connection ID rather than the traditional 4-tuple (source IP, source port, destination IP, destination port). When a client switches networks (e.g., from WiFi to cellular), the QUIC connection continues using the same connection ID without re-establishing. The server recognizes the connection ID and routes packets to the correct connection state. This is a significant improvement over TCP, where a network change requires a completely new connection with full handshake overhead. For mobile clients communicating with a Rust backend, this means seamless network transitions without dropped connections or re-authentication.

---

### Q9. What is the current state of HTTP/3 support in the Rust ecosystem?

**Interview Answer**

HTTP/3 support in Rust is primarily through the quinn crate, which implements the QUIC protocol. quinn-proto provides the protocol implementation, and quinn provides the async Tokio integration. For serving HTTP/3, you can use hyper-h3 or similar crates that build on quinn. The ecosystem is maturing but not yet as production-ready as HTTP/2 support. For clients, reqwest has experimental HTTP/3 support. In production, many Rust backends still serve HTTP/1.1 and HTTP/2, with HTTP/3 adoption growing as the tooling stabilizes and as CDN providers like Cloudflare handle HTTP/3 termination at the edge.

---

### Q10. How does the frame structure differ between HTTP/1.1 and HTTP/2?

**Interview Answer**

HTTP/1.1 uses a text-based format where the request line (METHOD /path HTTP/1.1) and headers (Key: Value) are separated by \r\n, with a blank line ending headers, followed by the body. HTTP/2 uses binary frames with a 9-byte header containing length, type, flags, stream identifier, and payload. Frame types include DATA, HEADERS, PRIORITY, RST_STREAM, SETTINGS, and others. This binary format is more efficient to parse, eliminates ambiguities in text parsing, and supports multiplexing through stream IDs. The SETTINGS frame at connection start negotiates parameters like maximum concurrent streams, initial window size, and maximum frame size.
