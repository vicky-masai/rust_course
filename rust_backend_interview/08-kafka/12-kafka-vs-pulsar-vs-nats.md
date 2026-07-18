# Kafka vs Pulsar vs NATS

## Interview Question

Compare Apache Kafka, Apache Pulsar, and NATS as message brokers. When would you choose each?

## Interview Answer

Kafka is a distributed commit log optimized for high-throughput event streaming with strong durability guarantees and mature ecosystem. Pulsar is a multi-tenant, geo-replicated messaging system with built-in tiered storage and unified queue/stream semantics. NATS is a lightweight, low-latency messaging system designed for simplicity and edge computing with at-most-once by default. Kafka excels for durable event streaming and event sourcing. Pulsar suits multi-tenant, geo-distributed deployments. NATS is ideal for low-latency request-reply and edge/IoT scenarios. In Rust, `rdkafka` (Kafka), `pulsar-rs` (Pulsar), and `async-nats` (NATS) are the respective client libraries.

---

## Follow-up Questions & Answers

### Q1. What are the fundamental architectural differences between Kafka, Pulsar, and NATS?

**Interview Answer**

Kafka uses a partitioned log architecture where brokers handle both serving and storage. Pulsar separates serving (brokers) from storage (Apache BookKeeper), enabling independent scaling. NATS uses a subject-based pub/sub model with optional JetStream for persistence. Kafka is pull-based (consumers pull messages); Pulsar supports both pull and push; NATS is push-based by default with JetStream offering pull. Kafka stores data on local broker disks; Pulsar offloads to BookKeeper; NATS stores in memory or file-based JetStream. These architectural differences drive their respective strengths in durability, latency, and scalability.

---

### Q2. How do their message delivery guarantees compare?

**Interview Answer**

Kafka provides at-least-once by default, exactly-once with idempotent producers and transactions. Pulsar provides at-least-once by default, effectively-once with transaction support. NATS provides at-most-once by default, at-least-once with JetStream acknowledgment, and effectively-once with deduplication. Kafka's guarantees are the most mature and battle-tested. Pulsar's transaction support is newer but functional. NATS JetStream's exactly-once is message-ID based and simpler. For financial systems requiring strong guarantees, Kafka's EOS is the most proven. For simpler at-least-once needs, NATS JetStream is sufficient.

---

### Q3. How do their throughput and latency characteristics compare?

**Interview Answer**

Kafka achieves highest throughput (millions of msgs/sec) but higher latency (ms to tens of ms) due to batching and disk writes. Pulsar achieves similar throughput to Kafka with slightly lower latency due to its segmented storage model. NATS achieves lowest latency (sub-millisecond) but lower throughput than Kafka/Pulsar for large-scale streaming. NATS Core (without JetStream) is optimized for real-time messaging. For Rust services, NATS's low latency suits request-reply patterns; Kafka's throughput suits event streaming; Pulsar balances both with geo-replication.

---

### Q4. What are the differences in multi-tenancy and isolation?

**Interview Answer**

Kafka has limited multi-tenancy - topics are globally shared with quotas for rate limiting. Pulsar has native multi-tenancy with tenant/namespace hierarchy, per-tenant quotas, and isolation. NATS has accounts for multi-tenancy with per-account isolation and security. Pulsar's multi-tenancy is the most comprehensive, supporting separate resource allocation per tenant. Kafka requires external tools (Confluent Control Center) for tenant management. For SaaS platforms serving multiple customers, Pulsar's native tenancy reduces operational complexity compared to Kafka's topic-level management.

---

### Q5. How do their operational complexities compare in production?

**Interview Answer**

Kafka requires ZooKeeper (or KRaft) and careful partition management; operational complexity is moderate-high. Pulsar requires BookKeeper and ZooKeeper, adding more components; operational complexity is high. NATS is a single binary with optional JetStream; operational complexity is very low. Kafka's ecosystem (Confluent Platform) provides tools but adds complexity. Pulsar's tiered storage simplifies storage management but adds architectural complexity. NATS's simplicity makes it attractive for small teams. In production, Kafka has the largest community and most operational tooling; NATS is easiest to operate; Pulsar requires the most infrastructure expertise.

---

### Q6. When would you choose Pulsar over Kafka?

**Interview Answer**

Choose Pulsar when: (1) **Multi-tenancy is required** - native tenant/namespace isolation; (2) **Geo-replication** - built-in active-active replication across data centers; (3) **Tiered storage** - automatic offloading of old data to S3/GCS; (4) **Unified queue and stream** - same system for both patterns; (5) **Schema evolution** - built-in schema registry. Pulsar's separation of compute and storage enables independent scaling. For Rust backends, `pulsar-rs` provides async Tokio integration, though it has fewer production deployments than `rdkafka`. Pulsar is a strong choice for globally distributed, multi-tenant systems.

---

### Q7. When would you choose NATS over Kafka?

**Interview Answer**

Choose NATS when: (1) **Low latency is critical** - sub-millisecond delivery; (2) **Edge computing** - NATS's small binary and low resource footprint suit edge deployments; (3) **Request-reply patterns** - NATS Core excels at synchronous-like messaging; (4) **Simplicity is valued** - single binary, minimal configuration; (5) **IoT/device communication** - NATS's lightweight protocol suits constrained devices. NATS is not ideal for event sourcing or replay-heavy workloads. For Rust services, `async-nats` provides clean async integration. NATS with JetStream provides durability for most use cases without Kafka's operational overhead.

---

### Q8. How do their consumer group models compare?

**Interview Answer**

Kafka's consumer groups are first-class with automatic partition assignment and rebalancing. Pulsar's consumer groups use subscriptions (exclusive, shared, failover, key-shared) with different delivery semantics. NATS uses pull-based consumers with durable consumers in JetStream, with queue groups for load balancing. Kafka's partition-based assignment ensures per-key ordering. Pulsar's subscription types provide more flexibility (e.g., shared for work queues). NATS's queue groups are simpler but less feature-rich. For Rust, all three provide async consumer APIs; Kafka's `rdkafka` has the most mature rebalancing support.

---

### Q9. How does the ecosystem maturity compare between the three?

**Interview Answer**

Kafka has the most mature ecosystem: Kafka Connect, Kafka Streams, ksqlDB, Schema Registry, Confluent Platform. Pulsar has Pulsar Functions, Pulsar IO connectors, and built-in schema registry. NATS has minimal ecosystem but covers core messaging well. Kafka's ecosystem is the largest with extensive community tooling and integrations. Pulsar's ecosystem is growing but smaller. NATS relies on community tools for advanced features. For Rust, `rdkafka` (Kafka) is the most production-proven, `pulsar-rs` (Pulsar) is functional but less mature, and `async-nats` (NATS) is well-maintained. Ecosystem maturity matters for production systems requiring monitoring, connectors, and operational tooling.

---

### Q10. What is the cost comparison for running these in cloud environments?

**Interview Answer**

Kafka: moderate cost due to disk storage for retention; Confluent Cloud charges per partition and throughput. Pulsar: higher cost due to BookKeeper + broker + ZooKeeper components; StreamNative Cloud charges per throughput. NATS: lowest cost due to minimal infrastructure; Synadia Cloud charges per connection and message volume. Self-hosted Kafka requires 3+ brokers with significant disk; Pulsar requires brokers + BookKeeper nodes; NATS can run on minimal infrastructure. For small to medium workloads, NATS is cheapest; for large-scale streaming, Kafka's cost-per-message is lowest; Pulsar's cost is justified by multi-tenancy and geo-replication features.
