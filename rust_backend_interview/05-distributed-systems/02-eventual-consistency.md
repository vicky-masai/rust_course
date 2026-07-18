# Eventual Consistency

## Interview Question

What is eventual consistency and when is it acceptable in a distributed system?

## Interview Answer

Eventual consistency is a consistency model where, after all updates stop, all replicas will eventually converge to the same state, but there is no guarantee about how long this will take. It is the foundation of many high-availability distributed systems because it allows writes to succeed even when some replicas are unavailable. This trade-off buys higher availability and lower latency at the cost of temporarily serving stale reads. Eventual consistency is acceptable in use cases like social media feeds, DNS propagation, and cached product catalogs where brief inconsistency does not cause financial or safety harm.

---

## Follow-up Questions & Answers

### Q1. How does eventual consistency differ from strong consistency?

**Interview Answer**

Strong consistency (linearizability) guarantees that every read reflects the most recent write across all nodes, typically achieved through synchronous replication or consensus protocols like Raft. Eventual consistency relaxes this guarantee — after a write, replicas may lag behind temporarily but will catch up eventually. The difference manifests in practice: with strong consistency, you might see a 50-200ms latency penalty on every write due to cross-node coordination, while eventual consistency allows sub-millisecond writes at the cost of reading stale data. Systems like Cassandra or DynamoDB default to eventual consistency for performance.

---

### Q2. What real-world systems use eventual consistency, and is it acceptable in those contexts?

**Interview Answer**

DNS is a classic example — when you update a DNS record, it can take minutes to hours to propagate globally, which is acceptable because brief inconsistency is tolerable. Social media feeds are another: seeing a post a few seconds late does not impact correctness. Amazon's product catalog uses eventual consistency for product details and prices, accepting that a cached page might show a slightly outdated price for a brief window. In contrast, financial systems like bank transfers cannot use pure eventual consistency — they need strong consistency or at minimum causal consistency to prevent double-spending.

---

### Q3. What is the anti-entropy mechanism and how does it help eventual consistency converge?

**Interview Answer**

Anti-entropy is a protocol where replicas periodically compare their state with other replicas to detect and repair inconsistencies. The most common implementation is Merkle trees (used by Cassandra and DynamoDB) — each replica builds a hash tree of its data, and when two replicas exchange trees, they can quickly identify which partitions of data differ and exchange only the divergent data. This is a background process that runs continuously, gradually bringing replicas back into alignment. It is efficient because it only transfers the minimum data needed to reconcile differences rather than the entire dataset.

---

### Q4. What is the read-your-writes consistency guarantee, and how does it relate to eventual consistency?

**Interview Answer**

Read-your-writes consistency guarantees that a client always sees its own writes, even in an eventually consistent system. This can be achieved by directing both the write and subsequent reads to the same replica, using session tokens, or implementing version vectors. In a Rust backend with Redis as a cache, you might ensure read-your-writes by reading from the primary for a short window after a write before falling back to replicas. This is important for user experience — a user who creates a post should immediately see it in their feed, even if other users see a stale view temporarily.

---

### Q5. What is causal consistency and how does it improve upon eventual consistency?

**Interview Answer**

Causal consistency preserves the causal ordering of operations — if operation A causally precedes operation B, all nodes will observe A before B. This is stronger than eventual consistency (which guarantees nothing about ordering) but weaker than linearizability (which requires real-time ordering). For example, if a user posts a comment and then edits it, causal consistency guarantees no one sees the edit before the original post. Implementation requires tracking causality through vector clocks or logical timestamps. Systems like MongoDB's causal sessions and COPS provide causal consistency guarantees.

---

### Q6. How do you handle conflict resolution in eventually consistent systems?

**Interview Answer**

When concurrent updates reach different replicas before replication completes, conflicts arise. Common resolution strategies include Last-Writer-Wins (LWW), which uses timestamps to pick a winner — simple but can lose data. Multi-Version Concurrency Control (MVCC) keeps both versions and lets the application resolve. CRDTs (Conflict-free Replicated Data Types) are data structures that merge automatically without conflicts, like G-Counters or OR-Sets used in Riak and Redis. In a Rust backend, you might implement LWW with nanosecond timestamps or use CRDTs for collaborative features like real-time editing.

---

### Q7. How does eventual consistency affect API design in a Rust/Axum backend?

**Interview Answer**

When using eventual consistency, API endpoints should document and handle potential stale reads. For example, an Axum endpoint returning a user profile should indicate the read consistency level, or the API should provide a `sync` flag that forces a read from the primary when the user needs guaranteed fresh data. Additionally, you should design idempotent endpoints because clients may retry reads or writes due to uncertainty about whether their operation succeeded. Implementing eventual consistency also means handling merge conflicts gracefully — returning meaningful HTTP status codes and allowing clients to handle version mismatches.

---

### Q8. What is the write-write conflict problem and how do eventually consistent systems deal with it?

**Interview Answer**

Write-write conflicts occur when two concurrent writes to the same key reach different replicas. In eventual consistency, both writes succeed on their respective replicas, creating divergent states that must be resolved. Systems use strategies like vector clocks to detect concurrent writes, then apply conflict resolution — either automatic (LWW, CRDTs) or application-level (prompting the user to choose). Cassandra uses LWW by default, while Riak prompts the application with siblings. In a Rust backend, you might store multiple versions and merge them using a custom resolver function in your domain logic.

---

### Q9. What are the performance implications of eventual consistency versus strong consistency?

**Interview Answer**

Eventual consistency typically offers 10-100x lower write latency because writes do not need to wait for cross-node acknowledgment. A Redis or Cassandra write in eventual consistency mode completes in sub-millisecond on a single node, while strong consistency with synchronous replication might take 10-50ms per write depending on network latency. Throughput also improves dramatically — eventual consistency can handle millions of writes per second across a cluster, while strong consistency is bounded by the slowest replica. The trade-off is that read latency for strongly consistent reads may be higher due to quorum requirements, and stale reads may cause application-level retries in eventual consistency.

---

### Q10. How do you test an application that relies on eventual consistency?

**Interview Answer**

Testing eventual consistency requires simulating network partitions and replication delays. Tools like Chaos Monkey or Toxiproxy can inject latency between your service and database replicas. You should write integration tests that write data to a primary, immediately read from a replica, and assert that the data converges within an expected time window (e.g., 500ms). Property-based testing with tools like proptest can verify that concurrent writes always converge to a valid state. In Rust, you can use testcontainers to spin up a PostgreSQL primary and replica, then inject delays to verify your application handles stale reads gracefully.
