# LEVEL 13 — Distributed Systems

### 0280. CAP Theorem

In a partition (P), you choose Consistency or Availability for that period — not both perfectly. It's a lens, not a math excuse to ignore tradeoffs. Most systems pick CP or AP for specific operations.

**Talk track:** *"CAP says under partition you prioritize consistency or availability — design which side fails how."*

---

### 0281. Consistency Models

Strong, eventual, causal, read-your-writes, monotonic reads, etc. Define what users are allowed to observe. Stronger models cost latency/availability.

**Talk track:** *"Consistency models are the product promise about what readers can see after writes."*

---

### 0282. Consensus

Multiple nodes agree on a value/order despite failures. Hard problem. Algorithms: Raft, Paxos. Needed for leader election, replicated logs, config.

**Talk track:** *"Consensus is agreeing on one truth across unreliable machines."*

---

### 0283. Raft

Understandable consensus: leader election, log replication, safety rules. etcd, Consul, many systems use Raft. Leader handles client writes; followers replicate.

**Talk track:** *"Raft is practical consensus — a leader replicates a log under majority ack."*

---

### 0284. Leader Election

Pick one primary to coordinate. Via consensus, age/priority, or cloud locks. Leaders must fence old leaders (fencing tokens) to avoid dual writes.

**Talk track:** *"Election picks a leader; fencing stops the old leader from corrupting state."*

---

### 0285. Distributed Locking

Mutex across machines (Redis Redlock debates, etcd locks, ZooKeeper, DB leases). Easy to get subtly wrong — clocks, GC pauses, expiry. Prefer design that doesn't need distributed locks when possible.

**Talk track:** *"Distributed locks are leases with failure modes — use fencing tokens or avoid them."*

---

### 0286. Replication

Copy data to multiple nodes for HA and read scale. Sync vs async: durability vs latency. Conflict resolution if multi-primary.

**Talk track:** *"Replication is copies for survival and scale — sync for safety, async for speed."*

---

### 0287. Sharding

Split data by key across nodes. Enables scale beyond one machine. Cross-shard transactions hurt. Hot keys melt single shards.

**Talk track:** *"Sharding scales writes by splitting keys — choose a key that avoids hotspots and joins across shards."*

---

### 0288. Partitioning

Related to sharding (data split). Also network partitions (nodes can't talk). Be clear which you mean. Data partitioning = placement; network partition = failure mode.

**Talk track:** *"Data partitioning places data; network partitions break communication — both shape design."*

---

### 0289. Vector Clock

Version vectors tracking causal history per replica. Detect concurrent conflicting updates vs ordered ones. Used in Dynamo-style systems.

**Talk track:** *"Vector clocks detect concurrent writes that need conflict resolution."*

---

### 0290. Lamport Clock

Logical counters that order events by happened-before via message piggybacking. Not enough for full causality across all replicas like vectors, but simple for total-ish ordering aids.

**Talk track:** *"Lamport clocks give a logical time consistent with causality of messages."*

---

### 0291. Failure Detection

Guess whether a node is dead: heartbeats, phi accrual detectors. False positives under GC/network blips cause unnecessary failovers — tune carefully.

**Talk track:** *"Failure detectors are suspicions with timeouts — false positives cause flip-flops."*

---

### 0292. Clock Drift

Physical clocks disagree and wander. Don't rely on synchronized clocks for correctness (use logical time or NTP-aware tolerances). `last-write-wins` by wall clock can lose data.

**Talk track:** *"Wall clocks lie enough to break correctness — don't use them as the source of truth."*
