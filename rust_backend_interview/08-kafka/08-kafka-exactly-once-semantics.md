# Kafka Exactly-Once Semantics

## Interview Question

How does Kafka achieve exactly-once semantics and what are the requirements for implementing it?

## Interview Answer

Kafka achieves exactly-once semantics (EOS) by combining three mechanisms: idempotent producers that eliminate duplicate writes from retries, the transactional API that enables atomic writes across multiple partitions, and `read_committed` consumer isolation that ensures consumers only see committed messages. Together, these provide end-to-end exactly-once delivery from producer to consumer. The producer assigns a unique `transactional.id` to prevent duplicates across sessions, while the transaction coordinator ensures atomic commit/abort semantics. In Rust, `rdkafka`'s `TransactionalProducer` API implements these mechanisms with async support.

---

## Follow-up Questions & Answers

### Q1. What is the difference between at-least-once, at-most-once, and exactly-once delivery?

**Interview Answer**

**At-most-once** sends a message without waiting for acknowledgment; if the broker fails, the message is lost. **At-least-once** retries until acknowledgment, potentially causing duplicates on retries. **Exactly-once** ensures each message is processed exactly one time with no duplicates and no loss. Kafka's default is at-least-once; enabling idempotent producers upgrades to effectively-once per producer session. Full end-to-end exactly-once requires the transactional API and consumer `read_committed`. The trade-off is that exactly-once adds latency and complexity but is essential for financial and inventory systems.

---

### Q2. How do idempotent producers work internally?

**Interview Answer**

When `enable.idempotence=true`, the producer assigns a producer ID (PID) and a monotonically increasing sequence number to each message. The broker tracks the last sequence number per PID per partition and rejects messages with sequence numbers ≤ the last committed one, effectively deduplicating retries. This works within a single producer session; if the producer restarts, a new PID is assigned (hence not truly exactly-once across restarts). The `transactional.id` extends this across restarts by mapping to the same PID. In `rdkafka`, enable with `enable.idempotence=true` and set `acks=all`.

---

### Q3. What is the transactional API and how does it work?

**Interview Answer**

The transactional API allows a producer to send messages to multiple partitions atomically. The producer initializes a transaction with `initTransactions()`, begins a transaction, produces records, and commits or aborts. The transaction coordinator (a broker) writes a commit or abort marker to all participating partitions, ensuring consumers with `read_committed` see a consistent state. Transactions also enable consuming from one topic and producing to another atomically (consume-transform-produce pattern). In `rdkafka`, use `TransactionalProducer` with `begin_transaction()`, `commit_transaction()`, and `abort_transaction()`.

---

### Q4. What is the consume-transform-produce pattern and why does it need transactions?

**Interview Answer**

The pattern consumes messages from a source topic, transforms them, and produces to a destination topic. Without transactions, if the producer crashes after consuming but before committing offsets, messages may be reprocessed (duplicates). With transactions, the consumer offset commit and producer writes are bundled atomically: either both succeed or both are rolled back. This ensures exactly-once processing of the source topic. In `rdkafka`, the transaction context coordinates the consumer offset commit within the producer transaction. This is the most common use case for Kafka EOS.

---

### Q5. What are the limitations of Kafka exactly-once semantics?

**Interview Answer**

EOS has several limitations: (1) it only works within Kafka - external systems (databases, APIs) require their own idempotency; (2) transaction timeout (`transaction.timeout.ms`) limits how long a transaction can stay open; (3) the transaction coordinator has a fixed number of partitions, limiting throughput; (4) consumer groups must use `read_committed` isolation; (5) there's overhead from sequence number tracking and transaction coordination. For Rust services, the external system integration (e.g., database writes) must use idempotent operations or two-phase commit to maintain exactly-once guarantees beyond Kafka.

---

### Q6. How do you implement exactly-once semantics in a Rust Kafka service?

**Interview Answer**

In Rust with `rdkafka`: (1) create a `TransactionalProducer` with a unique, stable `transactional.id`; (2) call `init_transactions()` before first use; (3) begin a transaction, consume and produce, then commit atomically; (4) consumer uses `isolation.level=read_committed`. Handle transaction aborts by logging and retrying. For database writes within the transaction, use idempotent upserts with unique constraints. Example pattern: consume message, extract key, begin transaction, upsert to database with message key, produce output, commit. The `rdkafka` `TransactionalProducer` wraps this in an async-compatible API for Tokio integration.

---

### Q7. How does Kafka handle exactly-once across multiple consumer groups?

**Interview Answer**

Kafka's EOS guarantees are per-producer-transaction and per-consumer-group. Each consumer group independently commits offsets and processes messages. If two consumer groups process the same topic, they independently achieve exactly-once processing within their group. Cross-group exactly-once (e.g., ensuring two groups see the same view) requires application-level coordination, as each group maintains its own offset tracking. For most systems, per-group exactly-once is sufficient. In Rust, each consumer group runs in a separate Tokio task with its own offset management.

---

### Q8. What is the role of the transaction coordinator in Kafka?

**Interview Answer**

The transaction coordinator is a broker responsible for managing transaction state. It tracks which partitions participate in a transaction and writes commit/abort markers to those partitions when the transaction completes. The coordinator stores transaction metadata in an internal topic (`__transaction_state`). When a producer sends a message to a partition, the coordinator ensures the partition is included in the current transaction. On commit, it writes markers to all partitions; on abort, it writes abort markers. This ensures atomicity across partitions.

---

### Q9. How does exactly-once semantics interact with Kafka Streams?

**Interview Answer**

Kafka Streams enables exactly-once processing by default when configured with `processing.guarantee=exactly_once_v2`. It combines idempotent producers, transactional writes, and consumer offset commits within a single transaction. Each Stream task processes records, produces to output topics, and commits source offsets atomically. This eliminates the need for manual transaction management. For Rust, Kafka Streams is JVM-only; achieve similar guarantees by manually implementing the consume-transform-produce pattern with `rdkafka`'s transactional API. The result is equivalent EOS for Rust-native stream processing.

---

### Q10. When should you NOT use exactly-once semantics in Kafka?

**Interview Answer**

Avoid EOS when: (1) throughput is the priority and duplicates can be handled idempotently by consumers; (2) the external system doesn't support idempotent writes (making Kafka's EOS insufficient); (3) simplicity is valued and the system can tolerate at-least-once with deduplication; (4) transaction timeout constraints conflict with long processing times. Many systems achieve effectively-once processing through idempotent consumers with at-least-once delivery, which is simpler and nearly as reliable. In Rust, measure the latency impact of transactions before committing to EOS for performance-sensitive paths.
