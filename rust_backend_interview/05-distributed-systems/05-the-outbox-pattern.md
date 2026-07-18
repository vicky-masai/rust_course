# The Outbox Pattern

## Interview Question

Explain the Outbox Pattern and why it is needed in distributed systems.

## Interview Answer

The Outbox Pattern solves the dual-write problem by ensuring that business state changes and event publication happen atomically within a single database transaction. Instead of writing to a database and then publishing to a message broker separately (which can fail in between, causing inconsistency), you write both the business data and an event record to the same database in one transaction. A separate background process (polling publisher or CDC tailing) reads the outbox table and publishes events to Kafka or another broker. This guarantees that if the business data is committed, the event will eventually be published, and if the transaction rolls back, no event is published.

---

## Follow-up Questions & Answers

### Q1. What is the dual-write problem and how does the Outbox Pattern solve it?

**Interview Answer**

The dual-write problem occurs when a service writes to a database and then publishes a message to a broker as two separate operations. If the database write succeeds but the broker publish fails (or the service crashes), the database has new data but no event is published — other services never learn about the change. The reverse is also problematic: the message publishes but the database write fails. The Outbox Pattern eliminates this by combining both writes in a single database transaction — either both succeed or neither does. A polling publisher or CDC connector then reliably delivers the event from the outbox table to the broker.

---

### Q2. What is the difference between polling publisher and CDC-based outbox?

**Interview Answer**

A polling publisher runs a background worker that periodically queries the outbox table for unpublished events (e.g., `SELECT * FROM outbox WHERE published = false LIMIT 100`), publishes them to the broker, and marks them as published. It is simple to implement but adds latency proportional to the polling interval and creates load on the database. CDC (Change Data Capture) uses database triggers or log tailing (e.g., PostgreSQL WAL via Debezium) to capture changes in real-time with near-zero latency. CDC is more efficient and lower-latency but requires infrastructure like Debezium and Kafka Connect, adding operational complexity.

---

### Q3. How do you implement the Outbox Pattern in a Rust/Axum backend with PostgreSQL?

**Interview Answer**

In a Rust Axum handler, within the same database transaction (using sqlx's transaction support), insert the business record and an outbox event: `INSERT INTO orders (...) VALUES (...)` followed by `INSERT INTO outbox (aggregate_type, aggregate_id, event_type, payload) VALUES ('order', $1, 'OrderCreated', $2)`. Commit the transaction atomically. A separate Tokio task or background worker polls the outbox table every 500ms, fetches unpublished events, publishes them to Kafka using `rdkafka`, and marks them as published. The worker uses `UPDATE outbox SET published = true WHERE id = ANY($1)` for batch updates.

---

### Q4. What are the failure scenarios with the Outbox Pattern?

**Interview Answer**

If the application crashes after committing the transaction but before the polling publisher reads the events, the events remain in the outbox table and will be picked up on restart — this is the pattern working correctly. If the polling publisher crashes mid-publish, some events may be published but not marked as published, leading to duplicate deliveries — consumers must be idempotent. If the outbox table grows too large without cleanup, performance degrades — implement a retention policy to archive or delete old published events. If the database itself is unavailable, the entire operation (business write + outbox write) fails atomically, which is the correct behavior.

---

### Q5. How do you ensure ordering of events with the Outbox Pattern?

**Interview Answer**

Ordering is guaranteed per aggregate (e.g., per order, per user) by using a monotonically increasing sequence number per aggregate in the outbox table. When the polling publisher reads events, it orders them by sequence number within each aggregate. For global ordering across all aggregates, you can use a single global sequence, but this becomes a bottleneck in high-throughput systems. In practice, global ordering is rarely needed — most systems only require per-aggregate ordering. Kafka also provides ordering per partition, so you can partition events by aggregate ID to maintain ordering guarantees at the broker level.

---

### Q6. What is transactional outbox versus the inbox pattern?

**Interview Answer**

The transactional outbox ensures that outgoing events are reliably published by writing them in the same transaction as business data — it is for outbound messages. The inbox pattern is the complementary inbound counterpart: when receiving messages from a broker, you store the message ID in an inbox table within the same transaction as processing, and check the inbox before processing to deduplicate. Together, they provide exactly-once semantics: outbox for publishing and inbox for consuming. In a Rust microservice, you might use Redis SET NX for the inbox check and a PostgreSQL outbox table for publishing, creating a robust exactly-once pipeline.

---

### Q7. How does the Outbox Pattern interact with event schemas and evolution?

**Interview Answer**

Events stored in the outbox must have a versioned schema to support backward compatibility. Use a schema registry (like Confluent Schema Registry) with Avro or Protobuf to define and evolve event schemas. Store the schema version in the outbox event metadata. When consumers upgrade, they can handle both old and new schema versions. The outbox payload should be stored as serialized JSON or binary with the schema version included, enabling consumers to deserialize correctly even as schemas evolve. In Rust, use `serde` with tagged enums to version event payloads, e.g., `enum OrderEvent { V1 { ... }, V2 { ... } }`.

---

### Q8. What is the performance impact of the Outbox Pattern?

**Interview Answer**

The outbox table write adds minimal overhead to the original transaction — a single INSERT with a small payload typically adds 1-2ms. The main performance concern is the polling query, which scans the outbox table frequently. Mitigate this by adding an index on `(published, created_at)`, batching reads, and using `SKIP LOCKED` in PostgreSQL to allow multiple polling workers without contention. For high-throughput systems, CDC-based outbox eliminates polling overhead entirely. The outbox table also requires periodic cleanup to prevent unbounded growth — archive or delete published events older than a configured retention period (e.g., 7 days).

---

### Q9. Can you use the Outbox Pattern with NoSQL databases?

**Interview Answer**

Yes, though it is most commonly associated with relational databases. With MongoDB, you can use a multi-document transaction to write both the business document and an outbox document atomically. With DynamoDB, you can use DynamoDB Streams as a built-in outbox — every write automatically generates a stream event. Cassandra does not support multi-table transactions, so you implement a "log-structured" approach where the event log is the primary data store and views are materialized from it. The key principle remains the same: ensure the business write and event record are atomically committed.

---

### Q10. How do you handle large event payloads in the outbox?

**Interview Answer**

Large payloads (e.g., event data with embedded images or large documents) slow down the outbox polling query and consume excessive storage. Solutions include: storing only a reference (URL or key) in the outbox payload and putting the full data in object storage (S3), compressing the payload before storage, or splitting large events into chunks. In a Rust service, you can use `bincode` or `zstd` compression for efficient serialization. The outbox table should store the compressed payload as bytes, and the polling publisher should decompress before publishing to Kafka. Alternatively, store the event payload in a separate table and reference it by ID from the outbox entry.
