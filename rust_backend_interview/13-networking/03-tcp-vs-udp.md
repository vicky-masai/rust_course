# TCP vs UDP

## Interview Question

What are the differences between TCP and UDP, and when would you use each in a Rust backend?

## Interview Answer

TCP (Transmission Control Protocol) is a connection-oriented protocol that provides reliable, ordered delivery of data through handshakes, acknowledgments, retransmission, and flow control. UDP (User Datagram Protocol) is connectionless and sends datagrams without guarantees of delivery, ordering, or duplicate protection. In a Rust backend, TCP is used for HTTP servers, database connections, and any scenario where data integrity matters — which is nearly everything. UDP is used for DNS resolution, real-time applications like gaming or video streaming, service discovery, and health checks where low latency matters more than guaranteed delivery. The choice comes down to whether you need reliability (TCP) or speed with tolerance for loss (UDP).

---

## Follow-up Questions & Answers

### Q1. How does the TCP three-way handshake work?

**Interview Answer**

The client sends a SYN packet to the server, the server responds with a SYN-ACK, and the client completes with an ACK. This establishes a full-duplex connection where both sides have agreed on initial sequence numbers and window sizes. In Rust, when you call TcpStream::connect(), the OS kernel handles this handshake transparently before the connection is handed to your application. The handshake prevents old duplicate connection requests from being accepted and ensures both sides are ready for data transfer. The overhead is one round-trip before the first byte of application data can be sent.

---

### Q2. What guarantees does TCP provide that UDP does not?

**Interview Answer**

TCP provides four key guarantees: reliable delivery (lost packets are retransmitted), ordered delivery (packets arrive in the order they were sent), error detection (checksums detect corrupted data), and flow control (the receiver tells the sender how much data it can handle). UDP provides none of these — it sends datagrams best-effort with no acknowledgment mechanism. If a UDP packet is lost, the application must handle recovery itself. This makes TCP suitable for HTTP, database connections, and file transfers, while UDP is suitable for DNS queries, video streaming, and real-time gaming where occasional packet loss is acceptable.

---

### Q3. How does TCP flow control and congestion control work?

**Interview Answer**

Flow control uses a sliding window mechanism where the receiver advertises how much buffer space it has available (the receive window), preventing the sender from overwhelming the receiver. Congestion control uses algorithms like TCP Cubic (Linux default) or BBR to prevent the sender from overwhelming the network. The sender tracks round-trip times and packet loss to estimate available bandwidth. In Rust, these mechanisms are handled by the OS kernel — when you write to a TcpStream, the kernel manages buffering and backpressure automatically. Tokio's TcpStream respects these mechanisms, and you can observe backpressure when writes block because the kernel buffer is full.

---

### Q4. When would you use UDP in a Rust backend?

**Interview Answer**

UDP is used for DNS resolution (the standard protocol for DNS queries), service discovery (like mDNS or Consul agent health checks), real-time communication (game state updates, video/audio streaming), and metrics collection (StatsD uses UDP). In Rust, the trust-dns-resolver crate sends UDP queries to DNS servers, and tokio::net::UdpSocket provides async UDP for custom protocols. UDP is preferred when you need minimal latency, can tolerate some packet loss, or when the request-response pattern is simple enough that TCP's overhead is unnecessary (like a single DNS query that fits in one datagram).

---

### Q5. What is the maximum size of a TCP segment vs a UDP datagram?

**Interview Answer**

TCP segments have no inherent maximum at the protocol level — data is stream-oriented and broken into segments based on the Maximum Segment Size (MSS), typically around 1460 bytes for Ethernet (1500 byte MTU minus IP and TCP headers). Large writes are fragmented into multiple segments and reassembled by the receiver. UDP datagrams have a maximum size of 65,535 bytes (including headers), but practically, datagrams larger than the MTU (1500 bytes) will be fragmented at the IP layer, and any fragment loss means the entire datagram is lost. This is why UDP is best suited for small messages.

---

### Q6. How do you handle TCP connection timeouts and retries in Rust?

**Interview Answer**

In Rust with Tokio, TcpStream::connect() uses the OS default timeout (typically 30-120 seconds). You can wrap it with tokio::time::timeout to set a custom timeout: timeout(Duration::from_secs(5), TcpStream::connect(addr)).await. For retries, you can implement exponential backoff by retrying the connect in a loop with increasing delays. For HTTP clients like reqwest, timeouts are configurable at the client level (connect timeout, read timeout, write timeout). TCP retransmission itself is handled by the kernel — if an ACK is not received, the kernel retransmits with exponential backoff (typically starting at 200ms and increasing to several seconds).

---

### Q7. What are TCP keep-alive probes and when should you use them?

**Interview Answer**

TCP keep-alive sends periodic probes on idle connections to detect if the remote end is still alive. Without keep-alive, a connection that goes silent (due to a crashed peer or network failure) would remain open indefinitely, consuming resources. The OS sends keep-alive probes after an idle period (default 2 hours on Linux, configurable via TCP_KEEPIDLE). In Rust, you can enable keep-alive on TcpStream using socket2 crate to set TCP_KEEPIDLE, TCP_KEEPINTVL, and TCP_KEEPCNT. This is important for long-lived connections like database pools or WebSocket connections to detect dead peers quickly.

---

### Q8. What is TCP_NODELAY and why does it matter for web servers?

**Interview Answer**

TCP_NODELAY disables Nagle's algorithm, which buffers small writes to combine them into larger segments. While Nagle's algorithm improves network efficiency for bulk transfers, it adds latency for small messages because it waits for more data or an ACK before sending. For web servers handling many small HTTP responses, Nagle's algorithm can add up to 40ms of latency per request. In Rust, you can set TCP_NODELAY on a TcpStream using set_nodelay(true), and Axum/Tokio typically enables this by default. This ensures small HTTP responses are sent immediately without buffering delays.

---

### Q9. How does QUIC combine TCP and UDP characteristics?

**Interview Answer**

QUIC runs over UDP but implements reliability, ordering, and congestion control at the application layer, providing TCP-like guarantees with significant improvements. QUIC eliminates head-of-line blocking (a TCP problem where one lost packet blocks all subsequent data), supports connection migration (connections survive IP address changes), and performs 0-RTT connection establishment for repeated connections. In Rust, the quinn crate provides QUIC support. QUIC is the foundation of HTTP/3 and represents the future of web transport, combining UDP's speed with TCP's reliability while eliminating TCP's architectural limitations.

---

### Q10. How do you implement a custom protocol over UDP in Rust?

**Interview Answer**

Using tokio::net::UdpSocket, you can bind a socket and use send_to/recv_from for datagram-based communication. For a custom protocol, you typically define a binary format with a fixed header (magic bytes, version, message type, length) followed by a payload. You serialize messages using bincode or manual byte packing. Since UDP does not handle fragmentation well, keep messages under 1200 bytes. For reliability over UDP, you implement your own acknowledgment and retransmission logic. Libraries like quinn (QUIC) or custom帧协议 can be built on top of Tokio's UDP socket for production-grade custom protocols.
