# Kafka Producers

## Interview Question

How do Kafka producers work, and what are the key configuration decisions for reliable message delivery?

## Interview Answer

A Kafka producer serializes messages, optionally determines the target partition, buffers records, and sends them to the broker. Partitioning can be automatic (round-robin or key hash) or manual (explicit partition specification). The producer batches records to improve throughput and compresses them to reduce network overhead. Acknowledgment settings (`acks`) control the durability guarantee: 0 for fire-and-forget, 1 for leader acknowledgment, and all for full ISR replication. In Rust, `rdkafka::FutureProducer` provides an async API that integrates with Tokio for non-blocking production.

---

## Follow-up Questions & Answers

### Q1. What partitioning strategies are available in Kafka and when do you use each?

**Interview Answer**

Kafka provides: (1) **DefaultPartitioner** - uses key hash if a key is set, otherwise round-robin; (2) **Round Robin** - distributes messages evenly across partitions without a key; (3) **Key Hash** - `murmur2` hash of the serialized key determines partition, ensuring per-key ordering; (4) **Custom Partitioner** - implements `Partitioner` trait for application-specific logic. Use key hash when ordering per entity (e.g., per user, per order) is required. Use round-robin for maximum distribution when ordering doesn't matter. In `rdkafka`, implement the `Partitioner` trait for custom logic.

---

### Q2. How does producer batching work and how do you tune it?

**Interview Answer**

Producer batching accumulates records in a buffer before sending them as a batch to improve throughput. `linger.ms` controls how long the producer waits before flushing (0 = immediate, 100 = wait 100ms). `batch.size` sets the maximum batch size in bytes; reaching this limit triggers an immediate send. `buffer.memory` sets the total buffer capacity; producers block if the buffer is full. Increasing `linger.ms` significantly improves throughput at the cost of slight latency. In Rust async producers, the `send` call returns a future that resolves when the batch is sent or an error occurs.

---

### Q3. What happens when a producer sends a message and the leader broker is down?

**Interview Answer**

If the leader is down, the producer receives a `LeaderNotAvailable` or `NotLeaderForPartition` error. The producer retries according to `retries` and `retry.backoff.ms` settings. With `metadata.max.age.ms`, the producer refreshes its metadata to discover the new leader. If retries are exhausted, the message is sent to the dead letter queue (if configured) or an error callback is invoked. In `rdkafka`, the delivery report callback (`DeliveryFuture`) signals success or failure for each message. Reliable producers in Rust must handle these errors with retry logic and dead letter queues.

---

### Q4. What is the idempotent producer and why is it important?

**Interview Answer**

Setting `enable.idempotence=true` gives each producer a unique ID and sequence number per partition. Brokers detect and discard duplicate writes from the same producer session, preventing duplicates during retries. This eliminates the need for application-level deduplication for most cases. The producer's `transactional.id` extends this across multiple partitions for atomic writes. Idempotent producers have slight overhead but are essential for financial or inventory systems. In `rdkafka`, enable with `enable.idempotence=true` and set `acks=all` for maximum reliability.

---

### Q5. How do you handle large messages in Kafka producers?

**Interview Answer**

Kafka has a `max.request.size` limit (default 1MB) and a broker-side `message.max.bytes` limit. For large messages: (1) compress the message using Snappy, LZ4, or Zstd; (2) increase `max.request.size` on both producer and broker; (3) consider chunking large payloads into smaller messages and reconstructing on the consumer side. Compression ratios for large messages are significant (10x for text-heavy payloads). In Rust, `rdkafka` supports compression configuration via producer properties. For messages over 10MB, chunking is recommended to avoid memory pressure.

---

### Q6. What is the difference between synchronous and asynchronous message production?

**Interview Answer**

Synchronous production blocks until the broker acknowledges, providing immediate delivery confirmation but limiting throughput. Asynchronous production returns a future immediately and sends delivery reports via callbacks, enabling higher throughput through pipelining. In `rdkafka`, `FutureProducer::send()` returns a `DeliveryFuture` that resolves to the delivery report. For high-throughput Rust services, async production with batching is preferred; use synchronous only for critical one-off messages where confirmation is needed before proceeding. Combine with `tokio::spawn` for parallel production of independent messages.

---

### Q7. How do you implement a dead letter queue (DLQ) pattern in Kafka producers?

**Interview Answer**

A DLQ is a separate topic where messages are sent after producer retries are exhausted or consumer processing fails repeatedly. On the producer side, implement a retry loop with exponential backoff; after max retries, produce to the DLQ topic with the original error metadata. On the consumer side, after N processing failures, produce to the DLQ topic instead of the main topic. In `rdkafka`, this is a separate producer instance producing to `topic.DLQ`. Monitor the DLQ topic for failed messages that need manual investigation or reprocessing.

---

### Q8. What are the key performance tuning parameters for Kafka producers in Rust?

**Interview Answer**

Key tuning parameters: `linger.ms` (batching delay - increase for throughput), `batch.size` (maximum batch in bytes - increase for larger batches), `compression.type` (Snappy/LZ4/Zstd for network and disk savings), `buffer.memory` (total producer buffer - increase for bursty workloads), `acks` (1 for speed, all for durability), `max.in.flight.requests.per.connection` (default 5, increase for throughput, set to 1 for strict ordering without idempotency). In Rust, profile producer throughput using `tracing::info_span!` around send calls and monitor delivery latency histograms with the `metrics` crate.

---

### Q9. How does message compression work and which algorithm should you choose?

**Interview Answer**

Compression happens at the batch level on the producer side. The producer compresses the entire batch, and the broker stores it compressed, decompressing only when serving consumers. Algorithms: **Gzip** - high compression ratio, high CPU; **Snappy** - fast compression, moderate ratio; **LZ4** - very fast, moderate ratio (best for high-throughput); **Zstd** - best ratio with configurable speed. For Rust services, LZ4 offers the best throughput-to-ratio balance. Configure via `compression.type=lz4` on the producer. The consumer decompresses automatically.

---

### Q10. How do you implement exactly-once production in a Rust Kafka service?

**Interview Answer**

Exactly-once production requires: (1) enable idempotent producers (`enable.idempotence=true`); (2) set a `transactional.id` for transactional writes; (3) begin a transaction, produce to one or more topics, and commit atomically; (4) consumers use `isolation.level=read_committed`. In `rdkafka`, the `TransactionContext` and `TransactionalProducer` API handle this. In Rust, wrap the transactional producer in `Arc<Mutex<TransactionalProducer>>` or use a channel-based approach. This ensures that a message is written exactly once even if the producer retries. The transactional ID must be unique per producer instance and stable across restarts.
