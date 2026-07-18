# Paxos vs Raft

## Interview Question

What are the differences between Paxos and Raft as consensus algorithms?

## Interview Answer

Both Paxos and Raft solve the distributed consensus problem — getting a cluster of nodes to agree on a single value or sequence of values despite failures. Raft was explicitly designed to be more understandable than Paxos, with a clearer decomposition into leader election, log replication, and safety. Paxos is more general and mathematically rigorous but notoriously difficult to implement correctly, leading to variants like Multi-Paxos and Cheap Paxos. Raft mandates a strong leader model where all client requests go through the leader, while Paxos can operate in a more decentralized fashion. In practice, Raft has become the dominant choice for new systems (etcd, TiKV, CockroachDB), while Paxos powers Google's infrastructure (Chubby, Spanner).

---

## Follow-up Questions & Answers

### Q1. Why is Paxos considered harder to understand and implement than Raft?

**Interview Answer**

Paxos describes consensus in terms of proposers, acceptors, and learners without an explicit leader, making the protocol harder to reason about. The original paper uses a single-decree example that is incomplete for production use, requiring extensions for multi-decree (log replication). Variants like Multi-Paxos, Fast Paxos, and Cheap Paxos each add complexity. Raft, by contrast, has a strong leader, explicit term structure, and a clear separation of concerns. The Raft paper was accompanied by an open-source reference implementation, which Paxos lacked. In practice, Google's Chubby team reported spending years getting Multi-Paxos right, while etcd's Raft implementation was functional in months.

---

### Q2. Where are Paxos and Raft used in production systems?

**Interview Answer**

Paxos is used in Google's Chubby (distributed lock service), Spanner (globally distributed database), and Megastore. It is also the basis for Apache ZooKeeper's atomic broadcast protocol (though ZooKeeper uses ZAB, which is Paxos-inspired). Raft is used in etcd (Kubernetes' backing store), TiKV (distributed key-value store), CockroachDB, Consul, and Databend. The Rust `raft` crate is used by TiKV and other databases. Raft has become the default choice for new systems because of its understandability and the availability of well-tested library implementations. Paxos remains in legacy systems and in Google's internal infrastructure.

---

### Q3. What are the fault tolerance guarantees of Paxos versus Raft?

**Interview Answer**

Both algorithms tolerate `f` failures in a cluster of `2f+1` nodes — the same mathematical guarantee. A 3-node cluster tolerates 1 failure; a 5-node cluster tolerates 2 failures. The difference is not in fault tolerance but in how they achieve it. Paxos uses quorum-based voting without a stable leader, so it can make progress even if the leader fails (in Multi-Paxos withPrepare phase). Raft requires a stable leader, so a leader failure triggers a new election, causing a brief unavailability window (typically 150-500ms). However, Raft's leader model simplifies reasoning about safety and makes implementation less error-prone, which in practice leads to fewer bugs — a form of practical fault tolerance.

---

### Q4. Can you implement Paxos and Raft in the same system?

**Interview Answer**

In theory, yes — different subsystems could use different consensus algorithms. In practice, this is extremely rare and inadvisable because it doubles the complexity of testing, debugging, and operational reasoning. Google's Spanner uses Paxos for replication across data centers and 2PC for cross-shard transactions, combining consensus with atomic commit. A more practical approach is to use Raft for your consensus needs (via etcd or TiKV's `raft` crate in Rust) and rely on the consensus layer for all coordination, rather than mixing algorithms. The operational overhead of maintaining two consensus implementations rarely justifies the theoretical benefits.

---

### Q5. What is the difference between leader-based Paxos (Multi-Paxos) and Raft?

**Interview Answer**

Multi-Paxos introduces a distinguished proposer (leader) to avoid the prepare phase for consecutive values, similar to Raft's leader model. The key difference is that Multi-Paxos allows the prepare phase to be skipped for multiple rounds once a leader is established, while Raft requires the leader to replicate every entry through `AppendEntries`. Multi-Paxos can achieve higher throughput for stable leaders because it avoids the overhead of preparing each value, but the protocol for leader election and failure detection is more complex. Raft's approach is simpler: the leader is always authoritative, and all entries go through the same path, making the protocol easier to reason about and verify.

---

### Q6. What is the role of log matching in both algorithms?

**Interview Answer**

Both Paxos and Raft use log matching to ensure that different replicas apply the same operations in the same order. In Raft, the log matching property states that if two logs contain an entry with the same index and term, all preceding entries are identical. This is maintained by the leader's `AppendEntries` RPC, which includes the previous entry's index and term for consistency checking. In Paxos, consensus is achieved per-value (or per-log-position in Multi-Paxos), with each position independently achieving agreement. Raft's approach is simpler because the leader controls the log ordering, while Paxos requires more coordination to ensure log consistency across acceptors.

---

### Q7. How do read operations differ between Paxos and Raft?

**Interview Answer**

In basic Raft, reads go through the leader, which can become a bottleneck. Linearizable reads require the leader to confirm it is still the leader by sending heartbeats before responding to reads, adding latency. Read-only optimizations include serving reads from followers with lease-based reads (accepting that clock skew may violate linearizability) or using `ReadIndex` (leader confirms its leadership before serving). In Paxos, reads can also be optimized by serving from any replica that has the latest committed value, or through a separate read protocol. Google's Spanner uses Paxos with TrueTime for globally consistent reads across data centers, which neither basic Raft nor Paxos provides natively.

---

### Q8. What are the throughput and latency characteristics of each?

**Interview Answer**

Both algorithms have similar theoretical performance: write throughput is bounded by the quorum size and network latency, with write latency approximately 2x the leader-to-follower RTT. In practice, Raft implementations like etcd can handle 10,000-50,000 writes per second with sub-10ms latency in a 3-node cluster. Multi-Paxos can theoretically achieve higher throughput by pipelining values without preparing each one, but practical implementations show similar performance. The main performance difference comes from implementation quality rather than algorithm choice — a well-optimized Raft (with batching, pipelining, and async I/O) performs comparably to a well-optimized Multi-Paxos. The Rust `raft` crate and etcd's implementation are both highly optimized for throughput.

---

### Q9. What is the relationship between ZAB (ZooKeeper) and Paxos/Raft?

**Interview Answer**

ZAB (ZooKeeper Atomic Broadcast) is ZooKeeper's consensus protocol, which is Paxos-inspired but not identical to Paxos. ZAB uses a leader-based model similar to Raft, with leader election and log replication. The key differences from Raft are: ZAB uses epoch numbers (similar to terms) but with different election rules, and ZAB guarantees prefix ordering (if a value is committed, all previously proposed values are also committed). ZAB is closer to Raft than to classic Paxos in terms of the leader model. Understanding the similarities helps: ZooKeeper, etcd, and Consul all provide similar distributed coordination services, but ZAB is specific to ZooKeeper while Raft is the generic algorithm used by etcd and Consul.

---

### Q10. If you were designing a new distributed system today, which would you choose?

**Interview Answer**

I would choose Raft for a new system. The primary reason is implementation availability and community support — the Rust `raft` crate, etcd's implementation, and CockroachDB's implementation provide battle-tested references. Raft's understandability reduces the risk of subtle correctness bugs, which are catastrophic in consensus protocols. The performance difference between Raft and Paxos is negligible in practice. The main exception would be if I needed to integrate with existing Google infrastructure that uses Paxos, or if I needed a specific Paxos optimization (like Fast Paxos for lower-latency consensus) that Raft does not provide. For most systems, Raft's simplicity and the availability of mature implementations make it the pragmatic choice.
