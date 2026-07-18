# WebSockets vs Server-Sent Events (SSE)

## Interview Question

Compare WebSockets and Server-Sent Events for real-time communication in a Rust backend.

## Interview Answer

WebSockets provide full-duplex communication over a single long-lived TCP connection, allowing both client and server to send messages independently at any time. Server-Sent Events (SSE) provide unidirectional server-to-client streaming over regular HTTP, where the server pushes events and the client reads them. WebSockets are ideal for chat applications, collaborative editing, and gaming where both sides need to send data frequently. SSE is ideal for notifications, live feeds, and server-pushed updates where client-to-server communication is infrequent (handled by regular HTTP requests). In Rust, Axum supports both through tokio-tungstenite for WebSockets and axum::response::sse for SSE, with SSE being simpler to implement and WebSockets providing more flexibility.

---

## Follow-up Questions & Answers

### Q1. How does the WebSocket handshake work?

**Interview Answer**

The WebSocket connection starts as an HTTP request with an Upgrade header. The client sends a GET request with headers: Upgrade: websocket, Connection: Upgrade, and a Sec-WebSocket-Key. The server responds with 101 Switching Protocols and a Sec-WebSocket-Accept header computed from the key. After this handshake, the TCP connection is upgraded to the WebSocket protocol, and both sides can send frames (text, binary, ping, pong, close) without HTTP overhead. In Axum, you handle this with axum::extract::ws::WebSocketUpgrade, which upgrades the connection and provides a WebSocket handler callback.

---

### Q2. What are the advantages of SSE over WebSockets?

**Interview Answer**

SSE has several advantages: it works over standard HTTP (no protocol upgrade needed), supports automatic reconnection with the Last-Event-ID header, is simpler to implement and debug, works through HTTP proxies and load balancers without special configuration, and can leverage existing HTTP infrastructure (cookies, authentication, CORS). SSE also provides built-in event IDs for ordering and deduplication. For server-pushed notifications, live feeds, or AI streaming responses (like ChatGPT-style APIs), SSE is the simpler and more reliable choice. The TextEventStream content type is standardized and well-supported by browsers.

---

### Q3. When should you choose WebSockets over SSE?

**Interview Answer**

Choose WebSockets when you need full-duplex communication (both client and server send data frequently), binary data transfer, lower latency for bidirectional messaging, or when you need to send messages from the client without the overhead of HTTP requests. Common use cases include chat applications, multiplayer games, collaborative editing (like Google Docs), and financial trading platforms where both sides push data constantly. WebSockets also support binary frames, making them suitable for sending binary protocol data or compressed messages.

---

### Q4. How do you implement SSE in Axum?

**Interview Answer**

In Axum, you use axum::response::sse::Sse with a stream of events. Create a tokio::sync::broadcast channel or use tokio::sync::mpsc to feed events, then wrap the receiver in a stream. Each event is an axum::response::sse::Event with optional id, event type, and data. The handler returns Sse<impl Stream<Item = Result<Event, Infallible>>>. The client uses the browser's EventSource API or reqwest to consume the stream. Axum automatically sets the correct Content-Type (text/event-stream), Cache-Control (no-cache), and Connection (keep-alive) headers for SSE.

---

### Q5. How do you implement WebSockets in Axum?

**Interview Answer**

Use the WebSocketUpgrade extractor in an Axum handler. The handler receives a WebSocketUpgrade, calls .on_upgrade with a callback that processes the WebSocket connection. Inside the callback, you get a WebSocket object that implements Stream and Sink, allowing you to receive and send messages. For multiple connected clients, share state through Arc<Mutex<HashMap>> or use tokio::sync::broadcast for pub/sub. Each WebSocket connection runs as a Tokio task, allowing thousands of concurrent connections. Axum handles the HTTP upgrade handshake and provides a clean async interface.

---

### Q6. What are the scaling challenges with WebSockets?

**Interview Answer**

WebSockets are stateful connections that cannot be easily load-balanced across servers without sticky sessions or a shared message bus. When a client connects to server 1, all its messages must go to server 1. This complicates horizontal scaling. Solutions include: using Redis Pub/Sub as a message bus between servers, implementing sticky sessions at the load balancer level, or using a dedicated WebSocket service like Socket.IO. For Rust backends, a broadcast channel per server with Redis as the cross-server bus is a common pattern. The connection state must be managed carefully during deployments and server failures.

---

### Q7. How do you handle WebSocket reconnection and message recovery?

**Interview Answer**

WebSockets do not have built-in reconnection — when the connection drops, the client must reconnect manually. Implementing resilience requires: the client to detect disconnection (missing pong responses), exponential backoff reconnection, and message recovery using sequence numbers or timestamps. The server assigns monotonically increasing sequence numbers to messages, and on reconnection the client sends the last received sequence number. The server then replays any missed messages from its buffer. In Rust, you can use a bounded channel to buffer recent messages and replay them on reconnection. SSE handles this automatically with the Last-Event-ID header.

---

### Q8. What is the difference between WebSocket and long polling?

**Interview Answer**

Long polling is a technique where the client sends an HTTP request and the server holds it open until new data is available or a timeout occurs. The client then immediately sends a new request. Long polling works over standard HTTP but introduces latency from the round-trip between polls. WebSockets maintain a persistent connection with no HTTP overhead per message, providing lower latency and more efficient communication. For real-time features in a Rust backend, WebSockets are preferred over long polling for performance and simplicity. Long polling is a fallback for environments where WebSockets are not supported (corporate proxies that block WebSocket upgrades).

---

### Q9. How does authentication work with WebSockets and SSE?

**Interview Answer**

For WebSockets, authentication typically happens during the HTTP upgrade handshake — you validate tokens (JWT, session cookies) in the WebSocketUpgrade handler before calling on_upgrade. After upgrade, the connection is authenticated. For SSE, authentication uses the same mechanisms as regular HTTP — cookies, Authorization headers, or query parameters. However, browsers do not allow custom headers on EventSource connections, so authentication via cookies is preferred for SSE. Both approaches should validate tokens and handle disconnection when tokens expire. In Axum, use middleware or extractors to validate authentication before establishing the WebSocket or SSE connection.

---

### Q10. How do WebSockets and SSE perform under high concurrency in Rust?

**Interview Answer**

Both WebSockets and SSE scale well in Rust due to Tokio's efficient async runtime. A single Tokio worker can handle tens of thousands of WebSocket connections because each connection only consumes memory for its state (no threads). SSE connections are even lighter since they use standard HTTP with chunked transfer encoding. In benchmarks, Tokio-based WebSocket servers can handle 50,000-100,000 concurrent connections on a single machine. The bottleneck is typically memory (each connection uses 2-10KB) and network bandwidth, not CPU. For broadcast scenarios (all clients receive the same message), the cost is O(n) where n is the number of connected clients, which can be optimized with message batching.
