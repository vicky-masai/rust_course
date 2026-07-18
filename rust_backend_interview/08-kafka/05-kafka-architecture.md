# Kafka Architecture

## Interview Question

Explain the internal architecture of Apache Kafka including brokers, topics, partitions, consumer groups, and offsets.

## Interview Answer

Kafka's architecture consists of brokers (servers that store and serve data), topics (logical categories of messages), partitions (ordered, immutable sequences within topics), and consumers organized into consumer groups. Brokers form a cluster where each partition has one leader and zero or more followers for replication. Partitions are the unit of parallelism; more partitions enable more concurrent consumers. Offsets track each consumer's position in a partition, enabling resumption after restarts. The system is designed for horizontal scalability by adding brokers and partitions as load increases.

---

## Follow-up Questions & Answers

### Q1. What is the role of a Kafka broker and how do brokers in a cluster communicate?

**Interview Answer**

A broker is a single Kafka server that receives messages from producers, stores them on disk, and serves them to consumers. Brokers in a cluster communicate via the controller broker (elected via ZooKeeper or KRaft) for partition leadership and metadata coordination. Inter-broker replication uses the follower fetch protocol where followers pull data from leaders. Each broker is identified by a unique `broker.id` and listens on configured ports. A typical production cluster runs at least 3 brokers for fault tolerance.

---

### Q2. How does partition assignment work within a consumer group?

**Interview Answer**

When a consumer joins a group, the group coordinator (a broker) triggers a rebalance. The group leader (the first consumer to join) receives partition assignments from the coordinator and distributes them among consumers using a partition assignment strategy. Common strategies include RangeAssignor (assigns contiguous partitions), RoundRobinAssignor (distributes evenly), and StickyAssignor (minimizes partition movement during rebalances). Each partition is assigned to exactly one consumer within a group, ensuring no duplicate processing. Custom assignors can be implemented for specialized needs.

---

### Q3. What happens during a consumer group rebalance and how do you minimize its impact?

**Interview Answer**

A rebalance is triggered when a consumer joins or leaves the group, or when a topic's partitions change. During rebalance, all consumers in the group stop processing and the coordinator reassigns partitions. This causes a temporary processing pause known as "stop-the-world." To minimize impact: use static group membership (`group.instance.id`) to avoid rebalances on transient failures; use CooperativeStickyAssignor for incremental rebalances; keep session timeouts reasonable. In Rust `rdkafka`, rebalance events are handled via the `Rebalance` callback where you can commit offsets before partitions are revoked.

---

### Q4. What are offsets and how is offset management handled in production?

**Interview Answer**

An offset is a sequential ID uniquely identifying each record within a partition. Consumers track their position by committing offsets to Kafka's internal `__consumer_offsets` topic. Auto-commit periodically saves offsets at configured intervals; manual commit gives precise control over when offsets are saved. In production, manual commits after successful processing are preferred to avoid data loss. If a consumer crashes before committing, it resumes from the last committed offset, potentially reprocessing messages. Idempotent consumers handle this gracefully. In `rdkafka`, `consumer.commit(...)` provides explicit offset management.

---

### Q5. How does Kafka handle broker failures and partition leader election?

**Interview Answer**

When a broker hosting a partition leader fails, the controller detects the failure (via ZooKeeper session expiry or KRaft health check) and elects a new leader from the in-sync replica (ISR) list. If `acks=all` was used, no data loss occurs because all ISR members have the data. If unclean leader election is enabled (`unclean.leader.election.enable=true`), a non-ISR replica can become leader, risking data loss but maintaining availability. For production, `unclean.leader.election.enable=false` is recommended. The ISR model balances durability and availability.

---

### Q6. What is the relationship between partitions and parallelism?

**Interview Answer**

Partitions define the maximum degree of consumer parallelism within a consumer group. If a topic has 10 partitions, a consumer group can have at most 10 consumers processing in parallel; additional consumers remain idle. More partitions mean higher throughput through parallelism but also increase metadata overhead and recovery time during broker failures. The general recommendation is to size partitions based on throughput requirements, not future growth. In Rust, each partition can be assigned to a separate Tokio task for concurrent processing.

---

### Q7. How does Kafka store data on disk and why is it so efficient?

**Interview Answer**

Kafka appends records sequentially to partition log segments on disk, leveraging the OS page cache for reads. Sequential I/O is orders of magnitude faster than random I/O, making disk performance comparable to network throughput. The `log.segment.bytes` setting controls segment size; older segments are deleted or compacted based on retention policies. Zero-copy transfer (`sendfile` syscall) moves data from disk to network without user-space copies. This architecture enables Kafka to achieve near-memory performance while storing data on disk for durability.

---

### Q8. What is the ISR (In-Sync Replica) and how does it affect data durability?

**Interview Answer**

The ISR is the set of replicas that are fully caught up with the leader. Only ISR members are eligible for leader election when the current leader fails. The `replica.lag.time.max.ms` setting determines how long a replica can lag before being removed from the ISR. If all ISR members fail and `min.insync.replicas` is not met, producers with `acks=all` receive an `NotEnoughReplicas` error. Maintaining a healthy ISR is critical for durability. Monitoring ISR shrink rate is a key operational metric.

---

### Q9. How do you choose the right number of partitions for a topic?

**Interview Answer**

Partition count is determined by: (1) throughput requirements - each partition handles ~10MB/s write; (2) consumer parallelism needs - more partitions allow more consumers; (3) end-to-end latency - more partitions increase recovery time. A common formula is: `partitions = max(target_throughput / per_partition_throughput, max_consumer_count)`. Start conservatively (e.g., 6-12 partitions) and scale up if needed. Increasing partitions is easy; decreasing is impossible without recreating the topic. In Rust, partition count directly affects how many Tokio tasks can consume in parallel.

---

### Q10. How does KRaft mode change the architecture compared to ZooKeeper mode?

**Interview Answer**

In ZooKeeper mode, an external ZooKeeper cluster manages metadata (broker registration, topic config, leadership). KRaft mode embeds this functionality within Kafka itself using a Raft-based quorum of controller nodes. KRaft eliminates the ZooKeeper dependency, simplifying deployment and reducing failure modes. It supports millions of partitions (vs. ZooKeeper's ~200K limit) and faster controller failover. The metadata log is replicated across controllers like a Kafka topic internally. KRaft is the standard starting with Kafka 4.0, and new deployments should use it exclusively.
