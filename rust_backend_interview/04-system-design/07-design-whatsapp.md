# Design WhatsApp

## Interview Question

How would you design a WhatsApp-like messaging system using Rust?

## Interview Answer

I would design a WhatsApp clone with an Axum-based API gateway handling authentication and message ingestion, WebSocket servers for real-time bidirectional communication, and a Kafka event backbone for message routing and persistence. The architecture includes a chat service that manages conversations and message storage in Cassandra (optimized for write-heavy workloads with time-ordered data), Redis for online presence and active session tracking, and a push notification service for offline users. Message media (images, videos) is stored in S3 object storage with metadata in PostgreSQL. The system uses a multi-region deployment with Cassandra's multi-datacenter replication for global availability, and WebSocket connections are load-balanced by consistent hashing on user ID to ensure all messages for one user route to the same server.

---

## Follow-up Questions & Answers

### Q1. How do you handle WebSocket connections at scale?

**Interview Answer**

A single Axum server can handle approximately 50,000-100,000 concurrent WebSocket connections with Tokio, limited mainly by file descriptor limits and memory (each connection uses ~5-10KB). For millions of concurrent users, I deploy 20-30 WebSocket servers behind a load balancer. The load balancer uses consistent hashing on the user ID to route reconnection attempts to the same server, maintaining locality for in-memory state. Each WebSocket server maintains an in-memory map of `user_id -> WebSocket sender` using `DashMap`. When a message arrives for a user, the routing layer looks up which server holds their connection (via Redis) and forwards the message via an internal gRPC or Kafka call. I use `tokio-tungstenite` for the WebSocket protocol layer on top of Axum.

---

### Q2. How do you design the message storage schema?

**Interview Answer**

I use Cassandra as the primary message store because it excels at write-heavy workloads and time-series data. The schema uses a `messages` table partitioned by `conversation_id` with `message_id` (UUIDv7 for time-ordering) as the clustering key in DESC order. This ensures messages for a conversation are stored in chronological order and retrieved efficiently with a simple range query. For a conversation with 100,000 messages, Cassandra retrieves the last 50 messages in under 1ms. I also maintain a `conversations` table in PostgreSQL for metadata (participants, creation time, last message preview) because relational queries are needed for the conversation list screen. The dual-database approach leverages each database's strengths.

---

### Q3. How do you handle message delivery and read receipts?

**Interview Answer**

I implement a three-state delivery model: sent (message stored in database), delivered (message reached recipient's device), and read (recipient opened the chat). When a user sends a message, the server stores it and forwards it via the recipient's WebSocket connection. When the recipient's device receives the message, it sends an acknowledgment back through the WebSocket. The server updates the message status in Cassandra and publishes a delivery receipt event via Kafka. The sender's client receives the receipt and updates the UI. Read receipts follow the same flow but are triggered when the recipient opens the conversation. I use lightweight Kafka events for receipt propagation, keeping the WebSocket connection uncluttered with receipt traffic.

---

### Q4. How do you implement group messaging?

**Interview Answer**

Group messaging uses a fan-out approach: when a user sends a message to a group, the server stores the message once in Cassandra with the group's conversation_id, then fans it out to all group members by publishing to Kafka with each member's user ID as the routing key. For small groups (< 256 members), fan-out on write is acceptable. For large broadcast channels (> 1000 members), I use fan-out on read — the message is stored once, and subscribers pull it when they connect. Group metadata (name, members, admin permissions) is stored in PostgreSQL with a `groups` and `group_members` table. I use Redis to cache the member list of active groups, reducing database lookups for frequently accessed groups.

---

### Q5. How do you handle offline message delivery?

**Interview Answer**

When a recipient is offline, the message is stored in Cassandra and marked as "pending delivery." When the user reconnects via WebSocket, the client sends a `last_message_id` in the connection handshake. The server queries Cassandra for all messages after that ID across all the user's conversations and delivers them in a batch. I also maintain an "unread count" per conversation in Redis, which is incremented when a message is sent and decremented when the recipient opens the conversation. For push notifications, I only send them for the first unread message in a conversation to avoid notification spam — subsequent messages while offline don't trigger additional push notifications. The push notification service uses FCM for Android and APNs for iOS.

---

### Q6. How do you implement end-to-end encryption?

**Interview Answer**

I implement the Signal Protocol for end-to-end encryption. When a user registers, they generate a key pair and upload their public key to the server. The server stores the public key but never sees the private key. When sending a message, the sender encrypts the message with the recipient's public key locally on their device. The server stores and forwards the encrypted ciphertext — it cannot read the message content. For group chats, I use the Sender Keys protocol where each member has an independent encryption key. The Rust server only handles ciphertext relay and storage. The trade-off is that server-side search becomes impossible (you can't search encrypted content), so client-side indexing is required.

---

### Q7. How do you handle media messages (images, videos, voice notes)?

**Interview Answer**

When a user sends media, the client uploads the file directly to a pre-signed S3 URL obtained from the Axum API. The upload endpoint returns a media_id, which the client includes in the message. The server stores the media metadata (media_id, sender_id, content_type, size, thumbnail_url) in PostgreSQL but never sees the actual file content. When the recipient requests the media, the server provides a pre-signed S3 URL with a 1-hour expiry. For videos, I use a background worker (consuming from a Kafka topic) to generate thumbnails and transcode to web-compatible formats using FFmpeg. Media is encrypted client-side before upload, so S3 stores only encrypted blobs. I implement CDN caching for frequently accessed media.

---

### Q8. How do you scale the WebSocket server fleet?

**Interview Answer**

I scale WebSocket servers based on three metrics: concurrent connections, message throughput, and CPU usage. A custom Prometheus metric `ws_connections_active` is exported from each Axum server and used by Kubernetes HPA. When connections approach 80% of capacity (e.g., 80,000 on a 100,000-capable server), a new pod is added. For session affinity during scaling, I use Redis to track which server holds each user's connection: `SET ws:user:{id} server:{pod_ip} EX 300`. When a new server accepts a connection, it updates Redis. When a user sends a message, the routing layer checks Redis to find the correct server. During scale-down, the server sends a close frame, the client reconnects, and is routed to the new server.

---

### Q9. How do you handle typing indicators and presence?

**Interview Answer**

Typing indicators are ephemeral and don't need persistence. When a user starts typing, the client sends a `typing_start` event via WebSocket. The server publishes it to a Redis Pub/Sub channel for the conversation. All other participants subscribed to that channel receive the indicator in real time. If the user stops typing after 5 seconds, a `typing_stop` event is published. Redis Pub/Sub is ideal here because it's fire-and-forget — no storage, no persistence, just real-time fan-out. For presence (online/offline status), I store a Redis key `presence:{user_id}` with a 60-second TTL. The client sends a heartbeat every 30 seconds to refresh the key. Other users check presence by reading this key, and I implement a Redis Pub/Sub channel for real-time presence updates.

---

### Q10. How do you ensure message ordering in a distributed system?

**Interview Answer**

Message ordering is guaranteed at the conversation level by using monotonic message IDs. I generate message IDs using UUIDv7, which embeds a timestamp in the first 48 bits. Within a conversation, messages are ordered by this ID. Since a single user sends messages sequentially, the server assigns IDs in order. For Cassandra, the clustering key sorts messages by ID in DESC order, so the most recent message is always first. In Kafka, I partition by conversation_id, ensuring all messages for one conversation go to the same partition and are processed in order. The trade-off is that global ordering across all conversations is impossible at scale, but users only care about ordering within a single conversation, which is preserved.
