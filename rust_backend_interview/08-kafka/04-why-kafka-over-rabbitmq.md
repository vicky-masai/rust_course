# Why Kafka Over RabbitMQ?

## Interview Question

What concrete scenarios make Kafka a better choice than RabbitMQ in production?

## Interview Answer

Kafka excels in scenarios requiring high-throughput event streaming, durable message retention, event sourcing, and real-time data pipelines. When multiple consumer groups need to independently process the same event stream, Kafka's log-based model is far more efficient than RabbitMQ's exchange-queue architecture. Kafka's horizontal scalability through partitions handles growing data volumes predictably. For systems requiring event replay, audit trails, or CQRS, Kafka is the standard choice. In Rust, `rdkafka`'s async-native API integrates seamlessly with Tokio-based services for production workloads.

---

## Follow-up Questions & Answers

### Q1. What specific metrics demonstrate Kafka's throughput advantage?

**Interview Answer**

Benchmarking shows Kafka handling 2+ million messages per second per broker with batching enabled and compression (Snappy/LZ4). RabbitMQ typically maxes out at 20K-50K messages per second per node depending on message size and acknowledgment settings. Kafka achieves this through sequential disk I/O, zero-copy transfers, and producer-side batching. The gap widens further with larger message sizes where Kafka's compression shines. For Rust services producing high-frequency events (telemetry, logging), Kafka prevents the message broker from becoming the bottleneck.

---

### Q2. How does Kafka's log compaction feature work and when is it useful?

**Interview Answer**

Log compaction retains the latest value for each message key, deleting older versions while preserving at least one value per key. This creates a compacted topic that acts like a key-value store of the latest state for each entity. It is useful for change data capture (CDC), configuration snapshots, and materialized view rebuilding. Unlike time-based retention, compaction ensures no key is ever completely lost. In Rust microservices, compacted topics enable services to bootstrap their state by consuming the entire compacted log without external snapshots.

---

### Q3. Why is Kafka preferred for event sourcing systems?

**Interview Answer**

Kafka's immutable, append-only log with configurable retention naturally stores the complete history of state changes. Events can be replayed from any offset to rebuild state at any point in time. Log compaction provides efficient state snapshots. Kafka's partition model maps cleanly to aggregate roots in event sourcing. RabbitMQ lacks built-in retention and replay capabilities, making it unsuitable for event sourcing. In Rust, the `rdkafka` consumer can seek to any offset, enabling snapshot rebuilds and temporal queries on event streams.

---

### Q4. How does Kafka handle backpressure better than RabbitMQ?

**Interview Answer**

Kafka absorbs backpressure at the broker level through disk-backed partitions. Producers can write at full speed regardless of consumer lag, as messages persist on disk. Consumers process at their own pace by managing their offset commit. RabbitMQ applies backpressure to producers when queues are full (blocking or dropping messages). Kafka's approach decouples producer and consumer throughput completely. In Rust async services, this means the Kafka producer task never blocks the HTTP handler, while RabbitMQ connection flow control can introduce latency spikes.

---

### Q5. What is Kafka Connect and how does it reduce integration code?

**Interview Answer**

Kafka Connect is a framework for building data integration pipelines between Kafka and external systems (databases, S3, Elasticsearch) without custom code. Source connectors ingest data into Kafka topics; sink connectors export data from topics to external stores. Connectors handle offset tracking, schema management, and fault tolerance automatically. This eliminates the need to write custom producer/consumer code for common integrations. For Rust services, Kafka Connect handles data pipeline concerns while your Rust code focuses on business logic through the `rdkafka` crate.

---

### Q6. How does Kafka Streams compare to RabbitMQ for stream processing?

**Interview Answer**

Kafka Streams is a client library for building stream processing applications directly within your service, handling windowed aggregations, joins, and filtering. RabbitMQ has no equivalent; you must build processing logic in consumer code. Kafka Streams provides exactly-once semantics, fault tolerance, and horizontal scaling with state stores backed by RocksDB. For Rust, `rdkafka` combined with Tokio enables similar stream processing patterns with manual state management. Kafka Streams is the more mature choice for complex processing; Rust gives you more control and performance when building custom processors.

---

### Q7. How does Kafka perform for IoT and time-series data ingestion compared to RabbitMQ?

**Interview Answer**

Kafka is purpose-built for high-volume, ordered time-series ingestion with partition-based ordering and time-based retention. IoT sensors producing thousands of events per second benefit from Kafka's batching, compression, and horizontal scaling. RabbitMQ struggles under the connection count and message volume typical of IoT deployments. Kafka's log compaction and time-based retention handle the "latest sensor value" and "historical analysis" use cases respectively. The Confluent ecosystem includes ksqlDB for real-time time-series analytics directly on Kafka topics. For Rust IoT gateways, `rdkafka`'s performance characteristics handle high-throughput sensor data effectively.

---

### Q8. What are the migration strategies from RabbitMQ to Kafka?

**Interview Answer**

Migration strategies include: (1) **Dual-write** - producers write to both RabbitMQ and Kafka during transition; (2) **Bridge pattern** - a consumer reads from RabbitMQ and produces to Kafka, with gradual consumer migration; (3) **Strangler fig** - new services use Kafka while existing services remain on RabbitMQ until decommissioned. The bridge pattern is safest for production systems. Schema differences must be mapped (RabbitMQ headers vs Kafka headers, message formats). Gradual migration reduces risk and allows validation at each step. The migration timeline depends on the number of consumers and their tolerance for inconsistency.

---

### Q9. How does Kafka's Exactly-Once Semantics (EOS) give it an advantage?

**Interview Answer**

Kafka's exactly-once semantics combines idempotent producers, transactional writes, and `read_committed` consumer isolation to prevent duplicate processing. RabbitMQ achieves exactly-once through publisher confirms and manual consumer acknowledgment, but this is per-queue, not across the full pipeline. Kafka's EOS spans multiple topics and partitions atomically. This is critical for financial transactions, inventory updates, and audit-critical events. In Rust, enabling `transactional.id` and using `rdkafka`'s transaction API ensures no duplicate side effects even during producer retries.

---

### Q10. In a Rust backend, what is the developer experience difference between `rdkafka` and `lapin`?

**Interview Answer**

`rdkafka` wraps librdkafka (C library) and provides a mature, battle-tested API with async support via `tokio`. It has excellent documentation, strong community adoption, and production-grade features like transactions and exactly-once semantics. `lapin` is a pure-Rust AMQP client with idiomatic async/await and strong type safety, but has a smaller community and fewer production deployments. `rdkafka`'s C dependency may concern some Rust purists, but its reliability in production is unmatched. For Kafka-specific features (compacted topics, consumer groups, partitioning), `rdkafka` is the clear choice; for RabbitMQ, `lapin` is the standard.
