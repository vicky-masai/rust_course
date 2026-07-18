# Kafka

## Interview Question

Explain what Apache Kafka is and why it is used in production systems.

## Interview Answer

Apache Kafka is a distributed event-streaming platform designed for high-throughput, fault-tolerant, and scalable data streaming. It decouples producers and consumers by persisting events in topics that can be replayed at any time. Kafka guarantees ordering within a partition and supports horizontal scaling by adding brokers and partitions. It is used for log aggregation, real-time analytics, event sourcing, and microservice communication. In Rust backends, the `rdkafka` crate (a librdkafka binding) provides a performant, async-compatible client.

---

## Follow-up Questions & Answers

### Q1. What is the difference between Kafka as a message queue and Kafka as an event log?

**Interview Answer**

Kafka is fundamentally an append-only commit log, not a traditional message queue. Messages are retained based on time or size policies (e.g., 7 days), allowing consumers to replay or catch up from any offset. In a queue model, one consumer processes a message and it is deleted; in Kafka, multiple consumer groups independently read the same events. This retention model enables new services to be added later and process historical data without re-sending.

---

### Q2. What are Kafka topics and partitions?

**Interview Answer**

A topic is a named stream of records, and partitions are the unit of parallelism within a topic. Each partition is an ordered, immutable sequence of records appended sequentially. Partitions are distributed across brokers for load balancing. Messages with the same key are guaranteed to land in the same partition, preserving per-key ordering. The number of partitions determines the maximum consumer parallelism for a topic.

---

### Q3. What role does ZooKeeper play in Kafka, and what is KRaft?

**Interview Answer**

ZooKeeper has traditionally been used by Kafka for metadata management: broker registration, topic configuration, partition leadership election, and consumer group coordination. KRaft (Kafka Raft) is the replacement introduced in Kafka 3.x that eliminates the ZooKeeper dependency by using an internal Raft consensus protocol. KRaft simplifies deployment, reduces operational overhead, and improves scalability. Starting with Kafka 4.0, ZooKeeper support has been removed entirely. For production deployments, migrating to KRaft mode is strongly recommended.

---

### Q4. What guarantees does Kafka provide regarding message delivery?

**Interview Answer**

Kafka provides at-least-once delivery by default, meaning a message may be delivered more than once during failures. With idempotent producers (`enable.idempotence=true`), duplicate writes from the same producer session are eliminated, achieving effectively-once semantics per partition. Combined with the transactional API and consumer `isolation.level=read_committed`, Kafka supports exactly-once semantics across reads and writes. Order is guaranteed only within a partition, not across partitions of a topic. Delivery time depends on producer acknowledgment settings and broker replication.

---

### Q5. How does Kafka achieve fault tolerance?

**Interview Answer**

Kafka replicates each partition across multiple brokers using a leader-follower model. One broker is elected leader for each partition, handling all reads and writes, while followers replicate data. The replication factor (typically 3) controls how many copies exist. If a broker fails, an ISR (in-sync replica) is promoted to leader automatically. With `acks=all`, the producer only gets acknowledgment once all in-sync replicas have written the record, preventing data loss.

---

### Q6. How would you use Kafka in a Rust backend with Axum?

**Interview Answer**

Use the `rdkafka` crate (wrapping librdkafka) to produce and consume messages from an Axum service. Spawn a dedicated Kafka consumer task in the application startup using `tokio::spawn`, feeding messages into channels or processing them directly with Axum handlers as background tasks. For producing, wrap the `FutureProducer` in application state via `axum::extract::State`. Use `rdkafka::consumer::StreamConsumer` with its async `recv()` method for non-blocking consumption. Ensure graceful shutdown by cancelling consumer tasks on server shutdown.

---

### Q7. What are Kafka consumer groups and why do they matter?

**Interview Answer**

A consumer group is a set of consumers that collectively consume all messages from a topic. Each partition is assigned to exactly one consumer within a group, enabling horizontal scaling of message processing. Different consumer groups independently read the full topic, supporting multiple downstream services processing the same events. If a consumer in a group fails, its partitions are reassigned to other consumers (rebalancing). This model is central to Kafka's scalability and decoupling design.

---

### Q8. What is the difference between `acks=0`, `acks=1`, and `acks=all`?

**Interview Answer**

`acks=0` means the producer does not wait for any acknowledgment, offering the lowest latency but risking data loss. `acks=1` means the leader broker acknowledges after writing locally, providing a balance but risking loss if the leader fails before replication. `acks=all` (or `-1`) means the producer waits for all in-sync replicas to acknowledge, providing the strongest durability guarantee. The trade-off is higher latency with `acks=all`. In Rust with `rdkafka`, this is configured via `queue.buffering.max.ms` and the `message.timeout.ms` properties.

---

### Q9. How does Kafka handle schema evolution of messages?

**Interview Answer**

Kafka itself stores raw bytes and is schema-agnostic, but integrating with a Schema Registry (Confluent or Apicurio) enforces schema compatibility at the producer and consumer level. Schemas are registered with compatibility modes (BACKWARD, FORWARD, FULL) to prevent breaking changes. Avro and Protobuf are common wire formats that carry schema IDs, allowing consumers to deserialize messages with the correct schema. In Rust, the `apache-avro` or `prost` crates handle serialization. This prevents the "broken contract" problem in distributed systems.

---

### Q10. What monitoring and observability should you set up for Kafka in production?

**Interview Answer**

Monitor broker metrics including under-replicated partitions, ISR shrink rate, request latency, and disk utilization using JMX or Prometheus exporters. Track consumer group lag to detect backlogs before they impact SLAs. Set alerts on partition leader elections, broker disconnections, and message throughput anomalies. Tools like Confluent Control Center, Burrow, or Kafka Exporter provide out-of-the-box dashboards. In Rust services, instrument `rdkafka` consumer lag and produce latency using `metrics` or `tracing` crates.
