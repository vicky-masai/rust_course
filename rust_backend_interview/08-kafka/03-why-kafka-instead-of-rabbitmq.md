# Why Kafka Instead of RabbitMQ?

## Interview Question

When would you choose Kafka over RabbitMQ, and what are the fundamental differences?

## Interview Answer

Kafka is chosen over RabbitMQ when you need high-throughput event streaming, message retention/replay, or event sourcing. Kafka acts as a durable, append-only log where messages persist for configurable retention periods, enabling multiple consumers to independently read the same events. RabbitMQ is a traditional message broker optimized for low-latency task distribution with message deletion after acknowledgment. Kafka scales horizontally with partitions; RabbitMQ scales with clustering and sharding. For Rust backends, `rdkafka` provides mature, production-grade Kafka integration while `lapin` serves as the RabbitMQ equivalent.

---

## Follow-up Questions & Answers

### Q1. What are the architectural differences between Kafka and RabbitMQ?

**Interview Answer**

Kafka uses a partitioned log architecture where messages are stored on disk and replicated across brokers. RabbitMQ uses a queue-based model with exchanges routing messages to queues consumed by workers. Kafka's log-based approach retains messages; RabbitMQ deletes them after consumption. Kafka brokers are stateless relative to consumers; RabbitMQ maintains per-consumer delivery state. Kafka's architecture is optimized for throughput and durability; RabbitMQ's for routing flexibility and low-latency delivery.

---

### Q2. How do their message retention policies differ?

**Interview Answer**

Kafka retains messages for a configurable time period (e.g., 7 days) or until a size limit, regardless of consumer acknowledgment. This enables replay and new consumers to catch up on historical data. RabbitMQ deletes messages once they are acknowledged by a consumer, with no built-in retention mechanism. Kafka's retention model is fundamentally different from traditional queue semantics. This is why Kafka is preferred for event sourcing, audit logging, and data pipeline scenarios where historical access matters.

---

### Q3. What about throughput comparison between the two?

**Interview Answer**

Kafka achieves throughput of hundreds of thousands to millions of messages per second on modest hardware due to sequential disk I/O and zero-copy transfers. RabbitMQ typically handles tens of thousands of messages per second, limited by per-message routing overhead and queue management. Kafka's batching and compression (Snappy, LZ4, Zstd) further amplify throughput. For Rust services producing high-volume events (e.g., clickstream, IoT telemetry), Kafka is the clear choice. RabbitMQ is sufficient for low-volume task queues like email sending or background jobs.

---

### Q4. How does consumer model differ between Kafka and RabbitMQ?

**Interview Answer**

In RabbitMQ, each message is delivered to exactly one consumer (competing consumers pattern), making it ideal for work queues. In Kafka, each consumer group independently reads all messages from a topic, enabling fan-out patterns. Within a consumer group, partitions are assigned to consumers for parallel processing. This means Kafka supports both work-queue and pub-sub patterns natively, while RabbitMQ requires exchanges and multiple queues for fan-out. Kafka consumers control their own offset, enabling replay; RabbitMQ consumers have no such control.

---

### Q5. What are the operational complexity differences?

**Interview Answer**

RabbitMQ is simpler to set up initially with a single node and management UI. Kafka requires a ZooKeeper cluster (or KRaft mode) and more careful partition and replication planning. Kafka's operational complexity increases with partition rebalancing, ISR management, and monitoring under-replicated partitions. RabbitMQ's complexity surfaces in cluster networking, queue mirroring, and memory management under load. In production, both require monitoring, but Kafka's JMX metrics ecosystem is more mature. For Rust teams, the `rdkafka` library has more production track record than `lapin` for RabbitMQ.

---

### Q6. When would RabbitMQ be the better choice over Kafka?

**Interview Answer**

RabbitMQ is better when: (1) you need complex routing logic (topic exchanges, header-based routing); (2) message priority and dead-letter queues are required natively; (3) low-latency per-message delivery matters more than throughput; (4) your team is small and operational simplicity is valued; (5) traditional task queue semantics (competing consumers, message acknowledgment) fit the use case. RabbitMQ also has superior built-in management UIs for queue monitoring. For Rust microservices doing background job processing, RabbitMQ with `lapin` is often simpler and sufficient.

---

### Q7. Can Kafka and RabbitMQ be used together in the same system?

**Interview Answer**

Yes, this is a common pattern in mature architectures. Kafka handles the high-throughput event backbone (domain events, audit logs, data pipelines) while RabbitMQ handles low-latency task distribution (email sending, PDF generation, retries). A bridge service consumes Kafka events and produces RabbitMQ tasks for short-lived processing. This combines Kafka's durability and replay with RabbitMQ's routing flexibility and priority queues. The downside is increased operational overhead of running two messaging systems. The decision should be driven by concrete requirements, not "use everything" philosophy.

---

### Q8. How does Kafka's partitioning model compare to RabbitMQ's sharding?

**Interview Answer**

Kafka partitions are a first-class concept enabling horizontal scaling of both storage and processing. Partitions are distributed across brokers with clear leadership semantics. RabbitMQ does not have native partitioning; it uses queue sharding or consistent hash exchanges as workarounds. Kafka's partition-based scaling is more predictable and transparent. RabbitMQ sharding requires application-level logic and is less mature. For high-scale Rust services, Kafka's partition model provides clearer scaling boundaries and better observability of per-partition metrics.

---

### Q9. What about message ordering guarantees?

**Interview Answer**

Kafka guarantees ordering within a single partition, not across partitions. Messages with the same key land in the same partition, ensuring per-key ordering. RabbitMQ guarantees FIFO ordering within a single queue, but message ordering is lost across multiple queues. For Kafka, the producer controls partition assignment (by key hash or explicit partition). For RabbitMQ, the exchange routing logic determines queue assignment. If global ordering is required in Kafka, a single partition must be used, limiting throughput. Per-key ordering is usually sufficient and much more scalable.

---

### Q10. How does the cost of running Kafka vs RabbitMQ compare in production?

**Interview Answer**

Kafka requires more hardware due to replication (minimum 3 brokers for fault tolerance) and disk storage for retained messages. RabbitMQ can run on fewer nodes but may require more memory under high queue depth. Kafka's cost per message is lower at scale due to efficient batching and disk I/O. RabbitMQ's cost is lower at small scale with simpler infrastructure. In cloud environments, Kafka offerings (Confluent Cloud, MSK) abstract operational costs but add per-partition pricing. For Rust backends, the total cost depends on message volume, retention requirements, and team expertise with each system.
