# Outbox Pattern

## Interview Question

Explain the Outbox Pattern and why it is used in event-driven architectures.

## Interview Answer

The Outbox Pattern addresses the dual-write problem by writing business data and event records to the same database in a single atomic transaction, then publishing events asynchronously via a background worker. This ensures that if the business write succeeds, the event will eventually be published — no inconsistency between what is stored and what is communicated to other services. The pattern uses either a polling publisher that periodically queries the outbox table or CDC (Change Data Capture) that tails the database WAL for new entries. It is a foundational pattern in microservice architectures where reliable event publication is critical.

---

## Follow-up Questions & Answers

### Q1. What is the dual-write problem and why can't you just write to the database and publish to Kafka?

**Interview Answer**

Writing to a database and publishing to Kafka as two separate operations creates a race condition. If the database write succeeds but the Kafka publish fails (network issue, Kafka down, service crash), the database has new data but the event is lost. Other services that depend on that event never learn about the change, leading to inconsistent state across the system. The reverse is also problematic — Kafka receives the event but the database write fails. The Outbox Pattern solves this by making both writes atomic within a single transaction. The event is reliably stored in the database, and a separate worker handles the publish asynchronously, ensuring eventual delivery.

---

### Q2. How do you implement polling publisher versus CDC in a Rust backend?

**Interview Answer**

A polling publisher is a Tokio task that runs a loop with `tokio::time::sleep(Duration::from_millis(500))`. It queries PostgreSQL: `SELECT id, aggregate_type, aggregate_id, event_type, payload FROM outbox WHERE published = false ORDER BY id LIMIT 100 FOR UPDATE SKIP LOCKED`. For each batch, publish to Kafka using `rdkafka` and mark as published with `UPDATE outbox SET published = true, published_at = NOW() WHERE id = ANY($1)`. CDC uses Debezium to tail PostgreSQL's WAL, which captures INSERT events on the outbox table in real-time and publishes them to Kafka topics. CDC has near-zero latency but requires running Debezium and Kafka Connect as additional infrastructure.

---

### Q3. How do you handle idempotent event publication with the Outbox Pattern?

**Interview Answer**

Even with the outbox, events might be published more than once — if the polling publisher crashes after publishing but before marking as published, the event will be re-published. Consumers must be idempotent. Use the event ID (a UUID stored in the outbox) as a natural deduplication key. On the consumer side, track processed event IDs in a table or Redis set and skip duplicates. Alternatively, use Kafka's exactly-once semantics (idempotent producer + transactional consumer) to prevent duplicates at the broker level. In a Rust consumer, check `Redis SISMEMBER processed_events $event_id` before processing, and add it after processing completes.

---

### Q4. What schema design do you use for the outbox table?

**Interview Answer**

The outbox table should include: `id` (UUID primary key), `aggregate_type` (string, e.g., "order", "user"), `aggregate_id` (UUID, the entity the event relates to), `event_type` (string, e.g., "OrderCreated"), `payload` (JSONB with the event data), `metadata` (JSONB for trace context, correlation IDs), `created_at` (timestamp), `published` (boolean, default false), and `published_at` (timestamp, nullable). Index on `(published, id)` for efficient polling and `(aggregate_type, aggregate_id)` for querying event history. Use `FOR UPDATE SKIP LOCKED` in the polling query to allow multiple publisher instances without contention. Partition the table by `created_at` for efficient cleanup of old events.

---

### Q5. What are the performance implications of the Outbox Pattern at scale?

**Interview Answer**

At high throughput, the outbox table becomes a hot write path — every business operation adds an INSERT. With proper indexing and `SKIP LOCKED`, multiple polling publishers can process events concurrently. At 100K+ events/second, consider: partitioning the outbox table, using CDC instead of polling to avoid database load, archiving old events to cold storage, and batching inserts. The INSERT adds ~1-3ms to the original transaction, which is acceptable for most use cases. For extreme throughput, use an in-memory outbox (buffered in the application and flushed asynchronously) with the understanding that a crash might lose a small window of events.

---

### Q6. How does the Outbox Pattern interact with database transactions in sqlx?

**Interview Answer**

In Rust with sqlx, wrap both the business INSERT and the outbox INSERT in a single database transaction: `let mut tx = pool.begin().await?; sqlx::query("INSERT INTO orders ...").execute(&mut *tx).await?; sqlx::query("INSERT INTO outbox ...").execute(&mut *tx).await?; tx.commit().await?;`. The transaction ensures atomicity — either both writes succeed or neither does. Use `sqlx::Postgres` with connection pooling (`PgPool`) for efficient connection management. The transaction isolation level should be at least READ COMMITTED to ensure the outbox INSERT is visible to the polling publisher after commit. Use `SERIALIZABLE` isolation if strict ordering is required.

---

### Q7. How do you handle event schema evolution with the Outbox Pattern?

**Interview Answer**

Store events with a version field in the payload: `{"version": 2, "order_id": "abc", "amount": 100}`. When you change the event schema, increment the version. Consumers check the version and handle both old and new formats. Use a schema registry (Confluent Schema Registry) with Avro or Protobuf to enforce compatibility rules (backward, forward, or full compatibility). In Rust, use `serde` with `#[serde(tag = "version")]` on an enum to deserialize different versions: `enum OrderEvent { V1 { order_id: String }, V2 { order_id: String, amount: f64 } }`. The outbox payload should always contain enough information for the current consumer version to process the event.

---

### Q8. What is the difference between the Outbox Pattern and Change Data Capture (CDC)?

**Interview Answer**

The Outbox Pattern is an application-level pattern — the application writes events to an outbox table within a transaction, and a separate worker reads and publishes them. CDC is an infrastructure-level approach — a tool like Debezium tails the database WAL (Write-Ahead Log) to capture all changes and publish them as events. You can combine both: use the Outbox Pattern for explicit event publication (the application controls what events are published) and CDC for capturing all database changes (useful for data replication and audit). CDC captures all changes including schema migrations and manual updates, while the outbox only captures intentional events.

---

### Q9. How do you clean up old events from the outbox table?

**Interview Answer**

Implement a retention policy — delete or archive events that have been published for longer than a configured period (e.g., 7 days). Use a scheduled job (cron or a Tokio task) that runs: `DELETE FROM outbox WHERE published = true AND published_at < NOW() - INTERVAL '7 days'`. For large tables, use batch deletes: `DELETE FROM outbox WHERE id IN (SELECT id FROM outbox WHERE published = true AND published_at < NOW() - INTERVAL '7 days' LIMIT 10000)`. If you need event history for auditing, archive to a separate table or object storage before deleting. PostgreSQL table partitioning by `created_at` makes cleanup efficient — you can drop entire partitions instead of deleting individual rows.

---

### Q10. Can you use the Outbox Pattern without a separate outbox table?

**Interview Answer**

Yes, there are alternatives. A transactional outbox is the standard approach, but you can also use: a `NOTIFY/LISTEN` mechanism in PostgreSQL where the business INSERT trigger fires a notification, avoiding the need for a separate table (though notifications are lost if no one is listening). You can use database triggers to write to the outbox (reducing application logic complexity). Or you can use an in-memory queue within the application, flushing to Kafka asynchronously (simpler but loses events on crash). For most production systems, the explicit outbox table is preferred because it provides durability, auditability, and the ability to replay events — properties that the alternatives lack.
