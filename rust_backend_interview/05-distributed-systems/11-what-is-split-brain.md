# What is Split-Brain?

## Interview Question

What is split-brain and how do distributed systems prevent it?

## Interview Answer

Split-brain occurs when a network partition causes a cluster to分裂 into two or more independent groups, each believing it is the authoritative majority. Both partitions may elect leaders, accept writes, and diverge in state, leading to data inconsistency when the partition heals. This is one of the most dangerous failure modes in distributed systems because it can cause silent data corruption. Prevention strategies include quorum-based systems (requiring majority agreement for writes), fencing tokens (monotonically increasing tokens that prevent stale leaders from acting), and consensus algorithms like Raft that guarantee at most one leader per term.

---

## Follow-up Questions & Answers

### Q1. How does split-brain occur in practice? Give a concrete example.

**Interview Answer**

Consider a 5-node cluster with nodes A, B, C, D, E. A network partition splits them into {A, B} and {C, D, E}. Both partitions have active nodes, but only {C, D, E} has the majority. If the system does not enforce quorum, {A, B} might elect a leader and accept writes while {C, D, E} also has a leader and accepts different writes. When the partition heals, the two partitions have divergent data. This happened in the 2012 GitHub outage where a network partition caused two MySQL servers to accept conflicting writes, requiring manual reconciliation. Proper quorum enforcement would have prevented {A, B} from accepting writes.

---

### Q2. What is a fencing token and how does it prevent split-brain?

**Interview Answer**

A fencing token is a monotonically increasing number issued by a lock manager or consensus layer. Every write must include the token, and the storage layer rejects writes with a stale token. If a partition isolates a leader, the surviving majority elects a new leader with a higher fencing token. When the partition heals, the old leader's writes are rejected because its token is lower. In practice, this is implemented with ZooKeeper or etcd providing sequential znodes or revision numbers. For example, Redis Sentinel uses fencing through its configuration epoch — a new sentinel's epoch is always higher, preventing stale sentinels from making configuration changes.

---

### Q3. How do consensus algorithms like Raft prevent split-brain?

**Interview Answer**

Raft prevents split-brain through its majority quorum requirement. A leader must be elected by a majority of nodes, and every committed entry must be acknowledged by a majority. If a partition isolates a minority of nodes, they cannot elect a leader (no majority) and cannot commit entries. The majority partition continues operating normally. When the partition heals, the minority nodes discover the higher term from the majority and revert to follower state, discarding any uncommitted entries. This guarantees that at most one leader is active at any time, preventing divergent state. The term mechanism ensures stale leaders from old terms cannot make decisions.

---

### Q4. What are the consequences of split-brain in a database cluster?

**Interview Answer**

Split-brain in a database cluster can cause duplicate primary keys, conflicting updates to the same row, divergent data across partitions, and data loss when the partition heals and one partition's data overwrites the other's. In a PostgreSQL cluster with synchronous replication, split-brain could result in both partitions accepting writes, creating data that cannot be automatically reconciled. Recovery requires manual intervention: comparing the two partitions, identifying conflicting changes, and merging them. In the worst case, some data must be lost. This is why production PostgreSQL clusters use tools like Patroni (which relies on etcd/ZooKeeper for leader election) to prevent split-brain through quorum enforcement.

---

### Q5. How does split-brain affect Redis Sentinel and Cluster?

**Interview Answer**

Redis Sentinel prevents split-brain through quorum-based failover — a new master is only promoted when a quorum of Sentinels agrees the old master is down. If a network partition isolates the old master, the remaining Sentinels elect a new master if they form a quorum. The old master becomes a slave when it rejoins. Redis Cluster uses gossip protocol and cluster state to detect partitions — if a master is in the minority partition, its slots are reassigned to replicas in the majority partition. However, if both partitions can accept writes (e.g., due to misconfigured `min-replicas-to-write`), split-brain can occur, causing divergent data. Proper configuration is critical.

---

### Q6. What is the difference between split-brain and a network partition?

**Interview Answer**

A network partition is the underlying cause — two groups of nodes cannot communicate with each other. Split-brain is the consequence — both groups independently believe they are the authoritative majority and take conflicting actions. Not all network partitions lead to split-brain: if the system correctly enforces quorum, the minority partition becomes unavailable (refuses writes) rather than operating independently. Split-brain only occurs when the system fails to properly detect and respond to the partition. In distributed systems terminology, a partition is the network event, and split-brain is the failure mode that results from incorrect handling of that event.

---

### Q7. How do you test for split-brain scenarios in a distributed system?

**Interview Answer**

Use fault injection tools like Chaos Monkey, Toxiproxy, or Pumba to simulate network partitions between nodes. Test scenarios include: partition that isolates the current leader, partition that splits the cluster into equal halves (neither has majority), partition during leader election, and partition during log replication. Verify that only the majority partition accepts writes, that the minority partition becomes unavailable or read-only, and that data converges correctly when the partition heals. In a Rust-based distributed system, use testcontainers to set up a 3-node Raft cluster, inject network partitions with `iptables` rules, and verify correctness properties using Jepsen-style testing.

---

### Q8. What is the "STONITH" approach and how does it prevent split-brain?

**Interview Answer**

STONITH stands for "Shoot The Other Node In The Head" — it is a fencing mechanism where a node is forcibly powered off or rebooted when it is suspected of being in a split-brain state. In HA (High Availability) clusters using Pacemaker, STONITH devices (IPMI, power switches) are used to ensure that a node that has been partitioned out cannot access shared resources. This is a hard guarantee — the node is physically prevented from acting. In cloud environments, STONITH is implemented through API calls to terminate or reboot instances. While effective, STONITH is a blunt instrument and can cause availability issues if the fencing mechanism is overly aggressive.

---

### Q9. How does the split-brain problem relate to CAP theorem?

**Interview Answer**

The split-brain problem is essentially what happens when a system tries to provide both availability and consistency during a partition — the A and C choices in CAP. An AP system (like Cassandra) allows the minority partition to accept writes, which is a form of controlled split-brain with eventual reconciliation. A CP system (like etcd) rejects writes from the minority partition, preventing split-brain but sacrificing availability. The CAP theorem formalizes the trade-off: you cannot prevent split-brain AND maintain availability during a partition AND guarantee consistency. You must choose at least one property to sacrifice.

---

### Q10. How do you recover from split-brain in a production system?

**Interview Answer**

Recovery requires identifying which partition has the most recent and authoritative data. Steps include: stop all writes to both partitions, compare the data (using timestamps, vector clocks, or log positions), determine which partition has the most complete data, promote that partition as the primary, and reconcile or discard data from the minority partition. In a PostgreSQL cluster, this might mean running `pg_rewind` to resync the old primary from the new primary. Automated recovery is risky — most teams perform split-brain recovery manually with careful data comparison. Prevention through proper quorum configuration and fencing is far preferable to post-incident recovery.
