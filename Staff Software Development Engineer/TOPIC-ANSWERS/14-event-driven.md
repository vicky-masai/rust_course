# LEVEL 14 — Event Driven Architecture

---

## Messaging Systems

### 0293. Kafka

Distributed commit log: producers append to topics; consumers read by offset. Huge scale, durable retention, replay. Backbone of event-driven platforms.

**Talk track:** *"Kafka is a durable, replayable log — not just a queue you empty."*

---

### 0294. RabbitMQ

Smart broker message queue (AMQP): routing exchanges, work queues, flexible patterns. Better for complex routing/task queues; different durability/throughput profile than Kafka.

**Talk track:** *"RabbitMQ is a broker-centric queue with rich routing — great for work distribution."*

---

### 0295. NATS

Lightweight messaging — simple, fast, cloud-native. JetStream adds persistence. Excellent for fanout and internal signaling with low ops weight.

**Talk track:** *"NATS prioritizes simplicity and speed; JetStream adds persistence when needed."*

---

## Kafka Internals

### 0296. Broker

A Kafka server node storing partitions and serving produce/fetch. Clusters of brokers share load.

**Talk track:** *"Brokers are the Kafka nodes that hold the log and serve clients."*

---

### 0297. Topic

Named stream category — like a logical feed (`orders`, `payments`). Split into partitions for scale.

**Talk track:** *"Topics are named event streams you publish to and subscribe from."*

---

### 0298. Partition

Ordered append-only segment of a topic. Unit of parallelism — one consumer in a group reads a partition at a time (typically). Key chooses partition.

**Talk track:** *"Partitions give order within a key and parallel throughput across keys."*

---

### 0299. Leader

Each partition has a leader broker handling writes/reads (classic model). Followers replicate. Leader failure → new leader election among ISR.

**Talk track:** *"The partition leader is the source of truth for that slice of the log."*

---

### 0300. ISR

In-Sync Replicas — followers caught up enough to be eligible for leadership / counted for ack=`all`. Shrinks under lag; availability vs durability tension.

**Talk track:** *"ISR is the set of replicas trusted for durability and failover."*

---

### 0301. Consumer Groups

Cooperative consumers sharing a topic — each partition assigned to one group member. Scale consumption horizontally; different groups independently replay.

**Talk track:** *"A consumer group load-balances partitions; separate groups each get the full stream."*

---

### 0302. Offsets

Position in a partition log. Committing offsets marks progress. At-least-once vs exactly-once depends on when you commit relative to side effects.

**Talk track:** *"Offsets are bookmarks — commit timing defines your delivery semantics."*

---

### 0303. Rebalancing

When members join/leave, partitions reassign. Can pause consumption (stop-the-world historically; incremental cooperative improves). Sticky assignment reduces movement.

**Talk track:** *"Rebalances reshuffle partition ownership — design for brief pauses and avoid unnecessary joins."*

---

### 0304. Retention

How long (time/size) Kafka keeps data even after consume. Enables replay and late joiners. Disk is the cost.

**Talk track:** *"Retention turns Kafka into a time machine — pay in storage."*

---

### 0305. Compaction

Keep latest value per key; delete older. Changelog topics for state (KTables). Not a general TTL delete for all cases.

**Talk track:** *"Compaction retains the newest event per key — perfect for state snapshots as logs."*

---

### 0306. Ordering

Kafka guarantees order *per partition*, not global across topic. Choose keys so related events share a partition.

**Talk track:** *"Order is per partition — key design is order design."*

---

## Patterns

### 0307. CQRS

Command Query Responsibility Segregation — separate write model from read model. Writes optimize transactions; reads optimize queries (denormalized views).

Complexity rises; use when read/write shapes truly diverge.

**Talk track:** *"CQRS splits write and read models when one schema can't serve both well."*

---

### 0308. Event Sourcing

Store state as a sequence of events; rebuild current state by folding. Perfect audit; harder queries; need snapshots.

**Talk track:** *"Event sourcing makes the log the source of truth — state is a projection."*

---

### 0309. Outbox Pattern

Write business row + outbox event in the same DB transaction; a publisher relays outbox to Kafka. Avoids dual-write inconsistency.

**Talk track:** *"Outbox atomically records 'emit this event' with the business write — then publish reliably."*

---

### 0310. CDC

Change Data Capture — stream DB changes (Debezium/logical decoding) into events. Great integration; beware schema change and delete semantics.

**Talk track:** *"CDC turns the database WAL into an event stream."*

---

### 0311. Saga Pattern

Distributed long transaction as a sequence of local steps with compensations on failure. Choreography (events) vs orchestration (coordinator).

Not ACID across services — design compensations carefully.

**Talk track:** *"Sagas chain local transactions with compensations — eventual consistency across services."*

---

### 0312. Idempotency

(See 0239 in event context.) Consumers must handle duplicate deliveries. Idempotent handlers or dedupe stores are mandatory with at-least-once buses.

**Talk track:** *"At-least-once messaging demands idempotent consumers."*

---

### 0313. Dead Letter Queue

Park poison messages that repeatedly fail. Preserve data; alert; allow replay after fix. Without DLQ, one bad message can block a partition consumer.

**Talk track:** *"DLQs isolate poison pills so the rest of the stream can move."*
