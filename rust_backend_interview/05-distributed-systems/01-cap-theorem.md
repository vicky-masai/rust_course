# CAP Theorem

## Interview Question

Explain the CAP theorem and how it applies to distributed systems.

## Interview Answer

The CAP theorem states that a distributed data store can only simultaneously provide two out of three guarantees: Consistency (every read receives the most recent write), Availability (every request receives a non-error response), and Partition Tolerance (the system continues to operate despite network partitions). In practice, since network partitions are unavoidable in distributed systems, the real choice is between Consistency and Availability during a partition. Systems like PostgreSQL with synchronous replication favor consistency (CP), while systems like Cassandra favor availability (AP). The theorem, proposed by Eric Brewer in 2000, fundamentally shapes how we design and choose databases and distributed architectures.

---

## Follow-up Questions & Answers

### Q1. Why is Partition Tolerance non-negotiable in distributed systems?

**Interview Answer**

Network partitions are a physical reality — machines crash, cables get cut, switches fail. Since we cannot prevent partitions, we must tolerate them. The CAP theorem forces us to choose what happens during a partition: do we reject requests to maintain consistency (CP), or do we serve potentially stale data to remain available (AP). This is why CAP is often described as a choice between C and A during a partition event.

---

### Q2. What is the difference between a CP and an AP system? Give examples.

**Interview Answer**

A CP system like PostgreSQL with synchronous replication or etcd will refuse to serve requests from a minority partition to prevent inconsistent reads. An AP system like Cassandra or DynamoDB will continue serving requests from any partition, even if the data may be stale. For example, if a three-node Cassandra cluster splits into a 2+1 partition, the majority partition continues writing, and the minority partition continues serving reads with potentially outdated data. When the partition heals, Cassandra uses anti-entropy protocols to reconcile.

---

### Q3. Is the CAP theorem still relevant given PACELC was proposed later?

**Interview Answer**

CAP is still relevant as a foundational concept, but PACELC extends it by addressing normal operation trade-offs. PACELC states: if there is a Partition, choose between Availability and Consistency; Else, when running normally, choose between Latency and Consistency. This captures the trade-off that even without partitions, synchronous replication adds latency. Systems like Redis or MongoDB allow you to tune this dial — synchronous writes give strong consistency but higher latency, while asynchronous writes are faster but risk data loss on failure.

---

### Q4. How does the CAP theorem relate to database replication in Rust backends?

**Interview Answer**

When building a Rust backend with PostgreSQL using sqlx, you choose a replication strategy that reflects your CAP trade-off. Synchronous replication to replicas means writes block until confirmed (CP behavior), ensuring every read replica is consistent. Asynchronous replication means replicas may lag, but writes are faster (AP behavior). For a typical Axum API serving a web application, you might use synchronous replication for critical financial data but asynchronous for analytics queries, effectively mixing CP and AP within the same system.

---

### Q5. What does "consistency" mean in CAP versus "consistency" in ACID?

**Interview Answer**

In CAP, consistency means linearizability — every client sees the most recent write across all nodes at the same time. In ACID, consistency means the database moves from one valid state to another, enforcing constraints like foreign keys and unique indexes. These are fundamentally different guarantees. A database can be ACID-compliant on a single node while violating CAP consistency during a network partition. For example, PostgreSQL enforces ACID locally but under asynchronous replication may lose consistency guarantees during a partition.

---

### Q6. How do modern distributed databases handle the CAP trade-off?

**Interview Answer**

Modern databases like CockroachDB, YugabyteDB, and Google Spanner implement consensus-based replication (Raft) to provide external consistency while remaining available during most scenarios. They use techniques like Google's TrueTime (with bounded clock uncertainty) to order events globally. CockroachDB, for example, uses hybrid logical clocks and Raft consensus to guarantee serializable isolation across nodes. These systems lean heavily CP but achieve high availability by replicating across multiple regions and tolerating individual node failures without full partitions.

---

### Q7. Can a system partially satisfy all three CAP properties simultaneously?

**Interview Answer**

Yes, in practice systems can provide all three properties outside of a partition event. During normal operation (no partition), a system can be both consistent and available. The theorem only forces a choice during a partition. This is why many systems offer tunable consistency — you can configure different consistency levels per query. For example, Cassandra allows you to choose consistency levels like ONE, QUORUM, or ALL per read/write, letting you dial up consistency when needed and dial it down for availability.

---

### Q8. How does the CAP theorem influence microservice design decisions?

**Interview Answer**

In microservice architectures, each service may choose its own CAP trade-off based on its domain requirements. An order processing service might use PostgreSQL (CP) to ensure financial accuracy, while a product catalog service might use Redis or Cassandra (AP) for fast reads with eventual consistency. The key insight is that consistency boundaries align with bounded contexts in Domain-Driven Design. You design each service's data store around its specific consistency and availability needs rather than applying a one-size-fits-all approach across the entire system.

---

### Q9. What is the difference between linearizability and sequential consistency in the CAP context?

**Interview Answer**

Linearizability is the strongest consistency model — it appears as if there is a single copy of data with operations taking effect atomically at some point between invocation and completion. Sequential consistency only requires that all processes see operations in the same order, but not necessarily in real-time order. Under CAP, linearizability requires more coordination and reduces availability during partitions, while sequential consistency allows more concurrent operations. Spanner achieves external consistency (stronger than linearizability), while DynamoDB provides eventual consistency by default.

---

### Q10. How would you explain CAP to a non-technical stakeholder?

**Interview Answer**

I would explain it as a three-way trade-off triangle: you can pick any two sides — fast and always available (but data might be slightly out of date), fast and always accurate (but some requests might fail during network issues), or always available and always accurate (but the system becomes very slow during network problems). In business terms, this translates to: do you want the app to always respond even if it shows old data, or do you want the app to show the most current data even if that means occasional downtime?
