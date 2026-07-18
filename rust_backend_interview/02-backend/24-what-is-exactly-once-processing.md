# What is exactly-once processing?

## Interview Question

What is exactly-once processing?

## Interview Answer

"It's achieved using idempotent producers, transactional writes, and idempotent consumers to avoid duplicate effects."

---

## Follow-up Questions & Answers

### Q1. Why is exactly-once processing difficult to achieve in distributed systems?

**Interview Answer**

Network failures, retries, and crashes make it impossible to guarantee single delivery without coordination. Messages can be duplicated during producer retries or consumer crashes after processing but before offset commit. Exactly-once semantics require end-to-end coordination between producer, broker, and consumer, which adds complexity.

---

### Q2. How does Kafka achieve exactly-once semantics?

**Interview Answer**

Kafka uses idempotent producers with sequence numbers to prevent duplicate writes, and transactional APIs to atomically write messages and commit offsets. Consumers must be configured with `isolation.level=read_committed` to only see committed messages. This requires `enable.idempotence=true` and transactional producer IDs.

---

### Q3. What is idempotency and how does it relate to exactly-once processing?

**Interview Answer**

Idempotency means processing the same request multiple times produces the same result as processing it once. In Kafka, idempotent producers deduplicate writes using sequence numbers, and idempotent consumers handle duplicate delivery gracefully. This is the foundation for exactly-once semantics in distributed systems.

---

### Q4. How do you implement idempotent consumers in Rust with Kafka?

**Interview Answer**

Track processed message IDs in a database or Redis using `SETNX` with TTL. Before processing, check if the message ID exists; if so, skip it. In an Axum backend, use a unique message key or timestamp as the idempotency identifier and store it atomically within the same transaction as the business logic.

---

### Q5. What is the difference between exactly-once delivery and exactly-once processing?

**Interview Answer**

Exactly-once delivery means each message is received once, which Kafka achieves with idempotent producers. Exactly-once processing means the message effect occurs once, requiring coordination with external systems like databases. The distinction matters because delivery guarantees don't automatically extend to downstream side effects.

---

### Q6. How do transactional outbox patterns help with exactly-once processing?

**Interview Answer**

The outbox pattern stores events in the same database transaction as business logic, then a separate process publishes them to Kafka. This ensures either both the business operation and event publish succeed, or neither does. In Rust, use sqlx transactions to write to both the business table and outbox table atomically.

---

### Q7. What are the performance costs of exactly-once semantics?

**Interview Answer**

Idempotent producers add overhead from sequence number tracking and deduplication. Transactions require broker-side coordination and reduce throughput. In practice, exactly-once processing adds 10-30% latency overhead compared to at-least-once. Use it only when duplicate processing has business-critical consequences like financial transactions.

---

### Q8. How do you handle exactly-once processing when the consumer crashes mid-processing?

**Interview Answer**

Kafka doesn't commit the offset until processing completes, so the message is redelivered after consumer restart. If the consumer has side effects, use the transactional outbox or idempotency keys to detect and skip duplicates. In Axum, store processed message IDs in the database and check before executing business logic.
