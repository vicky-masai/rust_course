# Kafka Consumers

## Interview Question

How do Kafka consumer groups work, and what are the challenges of rebalancing and offset management?

## Interview Answer

A Kafka consumer group is a set of consumers that collaboratively consume all partitions of a topic. Each partition is assigned to exactly one consumer within the group, enabling parallel processing. When a consumer joins or leaves, a rebalance redistributes partitions among active consumers. Offsets track each consumer's position in a partition and are committed to Kafka's internal `__consumer_offsets` topic. Proper offset management is critical to avoid data loss or duplicate processing. In Rust, `rdkafka::StreamConsumer` with its async `recv()` method integrates cleanly with Tokio.

---

## Follow-up Questions & Answers

### Q1. What triggers a consumer group rebalance and how can you minimize rebalance frequency?

**Interview Answer**

Rebalances are triggered by: (1) a new consumer joining the group; (2) a consumer crashing or heartbeat timeout; (3) a consumer voluntarily leaving; (4) topic partition count changes. Minimize rebalances by: using static group membership (`group.instance.id`) so transient failures don't trigger rebalances; using CooperativeStickyAssignor for incremental rebalances that only move affected partitions; tuning `session.timeout.ms` and `heartbeat.interval.ms` appropriately. In `rdkafka`, set `group.instance.id` to a stable identifier per consumer instance for graceful restarts.

---

### Q2. What is the difference between cooperative and eager rebalancing?

**Interview Answer**

Eager rebalancing (default) revokes all partitions from all consumers before reassigning, causing a "stop-the-world" pause where no messages are processed. Cooperative rebalancing (CooperativeStickyAssignor) only revokes the partitions that need to move, allowing other consumers to continue processing uninterrupted. Cooperative rebalancing significantly reduces the impact of rebalances in large consumer groups. In `rdkafka`, set `partition.assignment.strategy=org.apache.kafka.clients.consumer.CooperativeStickyAssignor`. This requires two rebalance rounds but is worth the reduced downtime.

---

### Q3. How do you ensure exactly-once consumption in Kafka?

**Interview Answer**

Exactly-once consumption requires: (1) commit offsets only after successful processing; (2) use `isolation.level=read_committed` to read only committed transactional messages; (3) ensure consumers are idempotent. Without these, at-least-once delivery causes duplicates on consumer restart. In Rust, process the message, then commit the offset in the same logical unit. If processing fails, do not commit; the consumer will retry. For critical operations, use a deduplication store (Redis, database) with the message key to detect duplicates. The combination of idempotent processing and offset management after processing achieves effectively-once semantics.

---

### Q4. What is offset reset policy and when does it matter?

**Interview Answer**

`auto.offset.reset` determines where a consumer starts when no committed offset exists for the partition (e.g., first consumption or expired offset). `earliest` starts from the beginning of the log; `latest` starts from the newest message; `none` throws an error. `earliest` is safest for ensuring no data loss on first consumption; `latest` avoids replaying old data during development. In production, `earliest` is preferred to avoid missing messages during consumer group creation. In `rdkafka`, configure via `auto.offset.reset=earliest`.

---

### Q5. How does consumer lag develop and why is it dangerous?

**Interview Answer**

Consumer lag is the difference between the latest offset in a partition and the consumer's committed offset. Lag develops when consumers cannot keep up with the produce rate, often due to slow processing, insufficient consumer instances, or network bottlenecks. Growing lag means the consumer is falling behind, potentially causing: (1) missed SLAs; (2) message expiry before consumption (if retention is shorter than lag); (3) cascading failures as downstream systems wait. Monitor lag using `kafka-consumer-groups.sh`, Burrow, or Kafka Exporter. In Rust, instrument lag metrics per partition using `rdkafka`'s `committed()` and `position()` APIs.

---

### Q6. What happens when a consumer crashes and how do you handle it gracefully?

**Interview Answer**

When a consumer crashes, its heartbeat (`heartbeat.interval.ms`) stops, and after `session.timeout.ms` (default 45s), the coordinator triggers a rebalance. Graceful shutdown should: (1) stop consuming; (2) commit current offsets for all assigned partitions; (3) leave the group. In `rdkafka`, handle the `Rebalance::Revoke` callback to commit offsets before partition reassignment. Use `consumer.commit(...)` with `CommitMode::Sync` for reliable offset saves. Static group membership allows the consumer to rejoin without triggering a rebalance if it recovers within the session timeout.

---

### Q7. How do you implement a dead letter queue in Kafka consumers?

**Interview Answer**

A DLQ pattern in consumers routes messages that fail processing to a separate topic. After N consecutive failures (tracked per message key), produce the message to `topic.DLQ` with error metadata (error message, failure count, original timestamp). This prevents poison messages from blocking the consumer indefinitely. In Rust, implement a retry counter using a Redis-backed or in-memory map, incrementing on each failure. When the threshold is reached, produce to the DLQ topic and commit the offset to advance past the poison message. Monitor the DLQ topic for operational visibility.

---

### Q8. What is the difference between `read_uncommitted` and `read_committed` isolation levels?

**Interview Answer**

`read_uncommitted` (default) returns all messages including those from uncommitted transactions, potentially including messages that are later rolled back. `read_committed` only returns messages from committed transactions, ensuring the consumer sees a consistent view. For most use cases, `read_committed` is preferred to avoid processing messages that may be rolled back. The trade-off is slightly higher latency as the consumer waits for transaction commits. In `rdkafka`, set `isolation.level=read_committed` on the consumer. In Rust, this is configured at consumer creation time.

---

### Q9. How do you handle partition rebalancing in Rust async consumers?

**Interview Answer**

In `rdkafka`, the `StreamConsumer`'s `recv()` method returns a `Result<BorrowedMessage>`. On rebalance, the consumer receives a `Rebalance` enum via the rebalance callback. In the `Revoke` variant, commit offsets for all assigned partitions. In the `Assign` variant, begin consuming the newly assigned partitions. Use a shared state (e.g., `Arc<HashMap<Partition, Offset>>`) to track per-partition offsets. In Tokio, spawn a separate task per partition if parallel processing is needed, using channels to coordinate rebalance events. The key is to never process messages from revoked partitions after rebalance.

---

### Q10. How do you implement consumer group monitoring in a Rust Kafka service?

**Interview Answer**

Monitor consumer group health by tracking: (1) per-partition lag using `consumer.committed()` and `consumer.position()`; (2) rebalance frequency via the rebalance callback; (3) processing latency per message using `tracing::instrument`; (4) consumer lag growth rate. Expose these metrics via Prometheus using the `metrics` crate. Alert on: growing lag over time, high rebalance count, and message processing errors. In `rdkafka`, the `consumer.group_metadata()` provides group information. Use `kafka-consumer-groups.sh` or Confluent Control Center for cluster-level consumer group monitoring.
