# Raft

## Interview Question

Explain the Raft consensus algorithm and its key components.

## Interview Answer

Raft is a consensus algorithm designed for understandability, achieving the same fault tolerance as Paxos but with a clearer structure. It operates through three sub-problems: leader election (nodes elect a single leader to coordinate all decisions), log replication (the leader receives client commands and replicates them to followers in a replicated log), and safety (ensuring that committed logs are never lost). A leader heartbeats followers periodically; if followers stop receiving heartbeats, they become candidates and request votes. The leader appends entries to its log and replicates them to a majority of nodes before considering them committed. Raft guarantees that a committed log entry is applied to all nodes' state machines in the same order.

---

## Follow-up Questions & Answers

### Q1. How does leader election work in Raft?

**Interview Answer**

When a follower does not receive a heartbeat within the election timeout (randomized between 150-300ms), it becomes a candidate, increments its term, and votes for itself. It sends `RequestVote` RPCs to all other nodes. A node votes for the first candidate it receives a valid vote request from (one vote per term). If a candidate receives votes from a majority (including itself), it becomes the leader. The leader then begins sending `AppendEntries` heartbeats to maintain authority. Randomized election timeouts prevent split votes — the node with the shortest timeout wins. In practice, election typically completes within one or two RTTs of the cluster.

---

### Q2. What is the role of terms in Raft?

**Interview Answer**

Terms are logical clocks in Raft — monotonically increasing counters that identify different leadership periods. Each RPC includes the sender's term, and nodes update their term if they receive a higher term. If a node discovers a higher term (e.g., from a candidate), it reverts to follower state. This prevents stale leaders from making decisions — a leader from term 5 cannot commit entries if the cluster has moved to term 6. Terms ensure that at most one leader exists per term and provide a mechanism to detect and disqualify outdated leaders. In etcd, terms are stored in the raft state and visible in cluster metadata.

---

### Q3. How does log replication work in Raft?

**Interview Answer**

When a client sends a command to the leader, the leader appends it to its log as a new entry with its current term. The leader then sends `AppendEntries` RPCs to all followers containing the new entry. Each follower appends the entry to its log and responds with success. Once the leader receives acknowledgment from a majority of nodes, the entry is considered committed. The leader then applies the entry to its state machine and responds to the client. Followers apply committed entries to their state machines on the next `AppendEntries`. If a follower's log is out of sync, the leader overwrites the divergent entries with the correct ones, ensuring all logs eventually converge.

---

### Q4. How does Raft handle network partitions?

**Interview Answer**

During a network partition, the majority partition continues to function normally — the leader can still receive acknowledgments from a majority and commit entries. The minority partition cannot elect a leader or commit entries because it cannot achieve a majority. When the partition heals, the minority partition's nodes discover the higher term and updated log from the majority, and they discard any uncommitted entries and synchronize with the majority leader. This safety guarantee is fundamental: no committed entry is ever lost, and uncommitted entries from the minority partition are safely discarded. Clients connected to the minority partition will experience unavailability until the partition heals.

---

### Q5. What is the difference between committed and applied entries in Raft?

**Interview Answer**

An entry is committed when the leader has replicated it to a majority of nodes — it is guaranteed to be durable and will eventually be applied to all state machines. An entry is applied when it has been executed against the state machine (e.g., written to the database or applied to the key-value store). There can be a delay between commitment and application. The leader applies committed entries to its state machine and includes the last applied index in subsequent heartbeats, telling followers what they should apply. This distinction matters because a client might see a committed entry before it is applied if the leader responds after commitment but before application.

---

### Q6. How does Raft implement snapshots for log compaction?

**Interview Answer**

Over time, the Raft log grows unbounded. Snapshots allow nodes to discard old log entries by capturing the state machine's current state at a particular index. A node takes a snapshot of its state machine (e.g., a full database backup), records the last included index and term, and discards all log entries up to that point. When a slow follower catches up, the leader sends an `InstallSnapshot` RPC with the snapshot data instead of replaying the entire log. In etcd, snapshots are triggered when the log exceeds a configurable size threshold, and the snapshot is stored locally and can be downloaded by new nodes joining the cluster.

---

### Q7. What are the performance characteristics of Raft?

**Interview Answer**

Raft requires a majority quorum for every commit, so the minimum cluster size is 3 nodes (tolerating 1 failure). Write throughput is bounded by the leader's ability to replicate entries and receive acknowledgments — typically limited by network latency between the leader and the slowest node in the quorum. Read performance can be optimized by serving reads from followers (with linearizable reads requiring an additional round of heartbeats). In a 3-node cluster with 1ms RTT, write latency is approximately 2ms (leader to follower and back). Throughput can be improved with batching — the leader batches multiple entries into a single `AppendEntries` RPC, amortizing the network round-trip cost.

---

### Q8. How is Raft implemented in production systems like etcd?

**Interview Answer**

etcd implements Raft as described in the paper with practical optimizations: it uses a separate goroutine for the Raft state machine and another for the apply pipeline, allowing concurrent log replication and state machine application. It supports learner nodes (non-voting members) for scaling reads without affecting write quorum. etcd uses snapshots aggressively to keep the log manageable, with a default threshold of 100,000 entries. The `raft` crate in Rust implements the Raft protocol as a library (used by databases like TiKV and Databend), providing a state-machine-agnostic implementation that applications can integrate with their own storage backends.

---

### Q9. What is the pre-vote optimization in Raft and why is it needed?

**Interview Answer**

Pre-vote is an extension to Raft that prevents disruptive elections when a node recovers from a network partition. Without pre-vote, a recovering node with a stale term sends `RequestVote` RPCs, causing the current leader to step down and triggering a disruptive election. With pre-vote, a node first sends `PreVote` RPCs (without incrementing its term) to check if it would win an election. Only if it receives a majority of pre-votes does it increment its term and start the actual election. This prevents recovered nodes from disrupting an otherwise healthy cluster. Pre-vote is implemented in etcd and the Rust `raft` crate, and is recommended for production deployments.

---

### Q10. How does Raft handle dynamic membership changes (adding/removing nodes)?

**Interview Answer**

Raft handles membership changes through joint consensus — the cluster transitions through a configuration change where both the old and new configurations must agree on decisions. The leader receives a configuration change request (add/remove node), applies it to its log, and replicates it. During the joint consensus phase, entries are committed when both old and new majorities agree. After the joint configuration is committed, the cluster transitions to the new configuration. This ensures safety — no two majorities can exist simultaneously that could elect different leaders. In etcd, membership changes are done through the `etcdctl member add/remove` commands, which orchestrate the Raft configuration change.
