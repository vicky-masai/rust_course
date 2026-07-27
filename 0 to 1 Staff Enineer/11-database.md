# LEVEL 11 — Database Engineering

### 0253. SQLx

Async Rust SQL toolkit: talk to Postgres/MySQL/SQLite with optional compile-time query checking. You write SQL (not a full ORM) — stays close to the database.

**Talk track:** *"SQLx keeps SQL first-class with Rust type checks — control without heavy ORM magic."*

---

### 0254. Connection Pooling

Reuse DB connections instead of connecting per request. Pool size is a critical knob — too small queues latency; too large overwhelms Postgres.

**Talk track:** *"Pools bound concurrent DB sessions — size them from DB capacity, not guesswork."*

---

### 0255. Transactions

Group statements into all-or-nothing units. `BEGIN`/`COMMIT`/`ROLLBACK`. Define the atomic business operation.

Keep transactions short — long transactions hold locks and hurt throughput.

**Talk track:** *"Transactions are atomic units of business truth — keep them tight."*

---

### 0256. Isolation Levels

Read Uncommitted → Read Committed → Repeatable Read → Serializable. Higher = fewer anomalies, more locking/aborts.

Postgres default is Read Committed. Know dirty/non-repeatable/phantom reads.

**Talk track:** *"Isolation is the consistency-vs-concurrency dial for concurrent transactions."*

---

### 0257. MVCC

Multi-Version Concurrency Control — readers see a snapshot; writers create new versions. Postgres's model enables non-blocking reads.

Old versions need cleanup (VACUUM).

**Talk track:** *"MVCC lets readers avoid blocking writers by reading old row versions."*

---

### 0258. Deadlocks

Two transactions wait on each other's locks — DB detects and aborts one. App must retry safely (idempotent).

Prevent with consistent lock ordering and shorter transactions.

**Talk track:** *"Deadlocks happen; design retries. Prevent with ordered locking and lean transactions."*

---

### 0259. Optimistic Locking

Assume little conflict: read version, write `WHERE version = old`, bump version. On 0 rows affected → conflict → retry.

Great for low-contention updates without holding long locks.

**Talk track:** *"Optimistic locking detects conflicts at commit time with versions — no long lock holds."*

---

### 0260. Pessimistic Locking

Lock rows upfront (`SELECT FOR UPDATE`) so others wait. Useful for high-contention critical sections (inventory).

Increases wait time; can deadlock; keep critical section tiny.

**Talk track:** *"Pessimistic locking reserves the row now — use when conflicts are likely and costly."*

---

### 0261. Indexes

Extra data structures (usually B-trees) to find rows without scanning tables. Speed reads; slow writes; use space.

Index columns used in WHERE/JOIN/ORDER BY carefully. Unused indexes waste write I/O.

**Talk track:** *"Indexes trade write cost and space for read speed — create them from real query patterns."*

---

### 0262. Query Planner

Database component that chooses how to execute SQL: which indexes, join order, scan type. Statistics feed it.

When queries are slow, read the plan before rewriting randomly.

**Talk track:** *"The planner picks the strategy — bad stats or missing indexes make it choose poorly."*

---

### 0263. Query Optimizer

Overlaps with planner: cost-based decisions to minimize estimated cost. Understand sequential scan vs index scan vs bitmap, nested loop vs hash join.

**Talk track:** *"Optimizers estimate costs — your job is good schema, stats, and SQL shape."*

---

### 0264. WAL

Write-Ahead Log: before data files change, log the change durably. Crash recovery replays WAL. Basis for replication.

`fsync` policy affects durability vs speed (`synchronous_commit`).

**Talk track:** *"WAL is how databases survive crashes and feed replicas — durability starts here."*

---

### 0265. VACUUM

Postgres reclaims/mark dead MVCC row versions, updates visibility maps, optional analyze. Autovacuum usually handles it; starvation causes bloat and performance death.

**Talk track:** *"VACUUM is MVCC garbage collection — neglect it and tables bloat."*

---

### 0266. HOT Updates

Heap-Only Tuple updates in Postgres: update without touching indexes when possible (same page, no indexed column change). Reduces index churn.

Schema design (fillfactor, what you index) affects HOT eligibility.

**Talk track:** *"HOT updates avoid index maintenance on friendly updates — a quiet Postgres win."*

---

### 0267. Buffer Cache

DB's in-memory cache of disk pages (Postgres `shared_buffers`, plus OS page cache). Hot working sets should fit for good latency.

**Talk track:** *"If the working set fits in cache, the DB feels in-memory; if not, you're disk-bound."*

---

### 0268. PostgreSQL Internals

Processes (postmaster, backends), MVCC, WAL, catalogs, vacuum, TOAST for large values. Knowing internals turns tuning from folklore into engineering.

**Talk track:** *"Postgres is a process-per-connection MVCC engine with WAL — internals explain most production mysteries."*

---

### 0269. Replication

Copy changes to replicas. Streaming physical replication, logical decoding. Read replicas scale reads; failover needs fencing to avoid split brain.

Replication lag is a consistency tradeoff.

**Talk track:** *"Replication buys read scale and HA — lag and failover fencing are the hard parts."*

---

### 0270. Partitioning

Split a table into partitions (by time/key). Improves maintenance and query pruning when filters match the partition key. Misuse creates worse plans.

**Talk track:** *"Partition when data lifecycle or query patterns align with a key — not as cargo cult scale."*
