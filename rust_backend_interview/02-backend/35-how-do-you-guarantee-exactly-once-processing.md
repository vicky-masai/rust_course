# How do you guarantee exactly-once processing?

## Interview Question

How do you guarantee exactly-once processing?

## Interview Answer

- Idempotent producer
- Transactional writes
- Idempotent consumer
- Offset management

---

## Follow-up Questions & Answers

### Q1. How do you implement idempotent producers in Rust with Kafka?

**Interview Answer**

Enable `enable.idempotence=true` in the rdkafka producer config to activate sequence-based deduplication. Use `transactional` API to atomically produce messages and commit offsets. The broker tracks producer IDs and sequence numbers to detect and reject duplicate messages automatically.

---

### Q2. What is the transactional outbox pattern and how does it ensure exactly-once?

**Interview Answer**

Write business data and event to the same database transaction, then a separate process publishes events to Kafka. This ensures atomicity between business logic and event publishing. In Rust, use sqlx transactions to write to both tables and a polling service to publish uncommitted events.

---

### Q3. How do you implement idempotent consumers for Kafka in Axum?

**Interview Answer**

Check a processed message IDs table before executing business logic. Use database unique constraints to prevent duplicate processing atomically. In Axum, implement this as a middleware or within the consumer task that forwards messages to handlers via channels.

---

### Q4. What is the difference between at-least-once and exactly-once delivery?

**Interview Answer**

At-least-once guarantees every message is delivered but may be duplicated; consumers must handle duplicates. Exactly-once ensures each message effect occurs once, requiring end-to-end coordination. Kafka achieves exactly-once within itself but extending it to external systems requires additional patterns like outbox or idempotency.

---

### Q5. How do you handle exactly-once when consuming from multiple Kafka topics?

**Interview Answer**

Use transactional consumption with `isolation.level=read_committed` to ensure only committed messages are processed. Commit offsets for all topics atomically using Kafka transactions. In rdkafka, use `commit_transaction()` to ensure all topic offsets are committed together.

---

### Q6. What happens if the consumer crashes after processing but before offset commit?

**Interview Answer**

Kafka redelivers the message since the offset wasn't committed, potentially causing duplicate processing. Use idempotent consumers with deduplication keys to handle this gracefully. Configure `max.poll.interval.ms` to balance between long processing times and timely rebalancing.

---

### Q7. How do you test exactly-once processing guarantees?

**Interview Answer**

Simulate producer retries, consumer crashes, and network partitions in integration tests. Verify that duplicate messages don't cause duplicate side effects by checking idempotency keys. Use Chaos Mesh or Toxiproxy to inject failures during processing and validate exactly-once behavior.

---

### Q8. What are the scalability limits of exactly-once processing?

**Interview Answer**

Exactly-once requires coordination overhead that limits throughput compared to at-least-once. Kafka transactions add latency from broker-side coordination. For most backends, at-least-once with idempotent consumers provides sufficient guarantees with better scalability; use exactly-once only for critical financial operations.
