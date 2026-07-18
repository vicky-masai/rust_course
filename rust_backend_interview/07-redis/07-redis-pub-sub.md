# Redis Pub/Sub and Streams

## Interview Question

Explain Redis Pub/Sub and Streams, their differences, and when to use each in a Rust backend.

## Interview Answer

Redis Pub/Sub is a fire-and-forget messaging pattern where publishers send messages to channels and subscribers receive them in real-time — messages are not persisted and are lost if no subscriber is connected. Redis Streams are append-only persistent logs with consumer group support, providing message durability, acknowledgment, and at-least-once delivery. Pub/Sub is ideal for real-time notifications, live feeds, and ephemeral events where message loss is acceptable. Streams are ideal for job queues, event sourcing, and workflows where message durability and ordered processing are critical. In Rust, `redis-rs` supports both via `pub_sub` for Pub/Sub and `XADD`/`XREADGROUP` commands for Streams.

---

## Follow-up Questions & Answers

### Q1. How does Redis Pub/Sub work internally?

**Interview Answer**

When a client subscribes to a channel, Redis registers the subscription in an in-memory dictionary. When a publisher sends `PUBLISH channel message`, Redis iterates over all subscribers of that channel and pushes the message to each client's output buffer. There is no persistence — if a subscriber is disconnected, messages published during the disconnect are lost. Redis Pub/Sub supports pattern subscriptions (`PSUBSCRIBE`) with glob patterns. The `redis-rs` crate provides `pub_sub` with async stream support for clean subscriber loops.

---

### Q2. What are the limitations of Pub/Sub?

**Interview Answer**

Pub/Sub is fire-and-forget: no message persistence, no delivery guarantees, no replay capability. If a subscriber is down, messages are lost. There is no backpressure — slow subscribers get disconnected if their output buffer overflows (`client-output-buffer-limit pubsub`). Pub/Sub is single-delivery only — each message goes to each subscriber once. For any workflow requiring durability, ordering guarantees, or at-least-once delivery, use Redis Streams instead.

---

### Q3. How do Redis Streams solve Pub/Sub's limitations?

**Interview Answer**

Streams are append-only logs stored persistently. Each message gets a unique ID (timestamp-sequence format like `1234567890-0`). Consumer groups allow multiple consumers to process a stream cooperatively, with automatic load balancing across consumers in the group. `XACK` acknowledges messages, and `XPENDING` shows unacknowledged messages for at-least-once delivery. Streams support `XTRIM` to cap length and `XCLAIM` to reassign idle messages from slow consumers. Unlike Pub/Sub, Streams persist messages regardless of subscriber availability.

---

### Q4. How do you implement a job queue with Redis Streams in Rust?

**Interview Answer**

Producer: `XADD jobs * task "process_order" order_id 123`. Consumer group: `XGROUP CREATE jobs workers 0`. Consumers: `XREADGROUP GROUP workers consumer1 COUNT 10 BLOCK 5000 STREAMS jobs >`. Process each message, then `XACK jobs workers <message_id>`. In Rust with `redis-rs`:

```rust
// Producer
redis::cmd("XADD").arg("jobs").arg("*").arg("task").arg("process_order").arg("order_id").arg("123").execute_async(&mut con).await?;

// Consumer
let result: redis::RedisResult<Vec<(String, Vec<(String, HashMap<String, String>)>)>> = redis::cmd("XREADGROUP")
    .arg("GROUP").arg("workers").arg("consumer1")
    .arg("COUNT").arg(10).arg("BLOCK").arg(5000)
    .arg("STREAMS").arg("jobs").arg(">")
    .query_async(&mut con).await;
```

---

### Q5. What are Consumer Groups and how do they work?

**Interview Answer**

A consumer group is a named group of consumers that cooperatively process a stream. Messages are distributed across consumers in the group — each message is delivered to exactly one consumer. The group tracks the last delivered ID per consumer and maintains a Pending Entries List (PEL) of unacknowledged messages. If a consumer crashes, its unacknowledged messages can be reclaimed by other consumers via `XCLAIM`. Each consumer in the group needs a unique name. Consumer groups enable horizontal scaling of message processing without duplicate processing.

---

### Q6. When would you use Pub/Sub over Streams?

**Interview Answer**

Use Pub/Sub for real-time ephemeral notifications (live chat, real-time dashboards, WebSocket broadcasts) where messages are transient and losing a few during a disconnect is acceptable. Pub/Sub has lower latency than Streams because it does not persist messages or track acknowledgments. Use Streams for anything requiring guaranteed delivery, ordered processing, retry logic, or audit trails — event sourcing, task queues, order processing pipelines, and inter-service communication where message loss is unacceptable.

---

### Q7. How do you handle message acknowledgment and retry in Streams?

**Interview Answer**

After processing a message, acknowledge it with `XACK stream group message_id`. If processing fails, do not acknowledge — the message stays in the PEL. Use `XPENDING` to inspect unacknowledged messages and their idle time. If a consumer is slow or crashes, use `XCLAIM` to reassign idle messages to another consumer:

```rust
redis::cmd("XCLAIM").arg("jobs").arg("workers").arg("backup_consumer")
    .arg("300000").arg(min_id).arg("COUNT").arg(10).execute_async(&mut con).await?;
```

Set a reasonable idle threshold (e.g., 5 minutes) before reclaiming. For poison messages that always fail, move them to a dead-letter stream after N retries.

---

### Q8. How does `redis-rs` support Pub/Sub?

**Interview Answer**

`redis-rs` provides `PubSub` via `as_pubsub()` on a connection, with methods like `subscribe("channel")` and `get_message()`. For async, use the `tokio-comp` feature:

```rust
use redis::AsyncCommands;
let mut pubsub = con.as_pubsub();
pubsub.subscribe("notifications").await?;
loop {
    let msg = pubsub.get_message().await?;
    let payload: String = msg.get_payload()?;
    println!("Received: {}", payload);
}
```

The `tokio` runtime handles the async blocking internally. For publishing, use `PUBLISH channel message` via `redis::cmd`.

---

### Q9. What is Stream trimming and when do you use it?

**Interview Answer**

Stream trimming removes old entries to prevent unbounded memory growth. Two strategies: `MAXLEN` (hard limit — `XADD stream MAXLEN ~ 10000 * field value` keeps approximately 10,000 entries) and `MINID` (time-based — `XTRIM stream MINID ~ <timestamp>` removes entries older than a threshold). Use `~` (approximate) for better performance, as Redis only trims at radix tree node boundaries. For job queues, trim aggressively after processing. For event logs, use longer retention. Monitor stream length with `XLEN` to ensure trimming is working.

---

### Q10. How do you implement pub/sub with message filtering?

**Interview Answer**

Redis Pub/Sub does not support server-side message filtering — all subscribers to a channel receive all messages. Filtering must happen client-side. Use pattern subscriptions (`PSUBSCRIBE notifications:*`) for channel-level filtering. For application-level filtering, subscribe to a single channel and filter in your Rust code based on message content (e.g., JSON field matching). Alternatively, use multiple channels for different message types (e.g., `events:user:created`, `events:order:placed`) and subscribe only to relevant channels. For complex filtering, Streams + consumer groups give more control over what each consumer processes.

