# Redis Persistence

## Interview Question

Explain the different Redis persistence mechanisms and how to configure them for a production Rust backend.

## Interview Answer

Redis offers three persistence strategies: RDB (periodic point-in-time snapshots of the entire dataset), AOF (Append-Only File that logs every write operation), and a hybrid approach that combines both. RDB gives fast restarts and compact files but can lose data between snapshots; AOF provides near-zero data loss with `appendfsync everysec` but produces larger files and slower restarts. The hybrid approach uses AOF for durability and RDB for fast rewrites, and is the recommended production configuration. In production, you should set `appendonly yes`, `appendfsync everysec`, configure `auto-aof-rewrite-percentage 100`, and monitor `INFO persistence` to track dirty bytes and rewrite status.

---

## Follow-up Questions & Answers

### Q1. How does RDB persistence work internally?

**Interview Answer**

RDB saves a compact binary snapshot of the entire dataset to `dump.rdb` at configurable intervals (e.g., `save 900 1` means save if 900 keys changed in 15 minutes). When triggered, Redis forks a child process that writes the snapshot to a temporary file, then atomically replaces the old `dump.rdb`. During the fork, the parent process uses copy-on-write (CoW) memory, so if the dataset is large and writes are heavy, memory usage can temporarily double. RDB is ideal for backups and fast restarts, not for maximum durability.

---

### Q2. What is AOF and how does `appendfsync` work?

**Interview Answer**

AOF logs every write command to `appendonly.aof` in append-only format. The `appendfsync` setting controls when data is flushed to disk: `always` (every write, safest but slowest), `everysec` (once per second, recommended), or `no` (OS decides, fastest but least safe). With `everysec`, you lose at most ~1 second of writes on crash. AOF rewrites periodically to compact the file by replaying the current dataset into a new, smaller AOF — controlled by `auto-aof-rewrite-percentage` and `auto-aof-rewrite-min-size`.

---

### Q3. What is the hybrid persistence approach?

**Interview Answer**

Since Redis 4.0, you can enable `aof-use-rdb-preamble yes` (default in Redis 5.0+), which writes an RDB snapshot at the beginning of the AOF file during rewrites, followed by incremental AOF commands. On restart, Redis loads the RDB preamble (fast) and replays the remaining AOF tail. This combines RDB's fast startup with AOF's durability. In your `redis.conf`: `appendonly yes`, `aof-use-rdb-preamble yes`, `appendfsync everysec`.

---

### Q4. How do you configure persistence in production?

**Interview Answer**

In `redis.conf`:

```
appendonly yes
appendfsync everysec
aof-use-rdb-preamble yes
save 900 1
save 300 10
save 60 10000
auto-aof-rewrite-percentage 100
auto-aof-rewrite-min-size 64mb
```

This enables both RDB snapshots and AOF with per-second fsync. Monitor `INFO persistence` for `aof_rewrite_in_progress` and `rdb_last_bgsave_status`. Ensure the disk has low latency (SSD recommended). Use `redis-cli BGSAVE` and `redis-cli BGREWRITEAOF` for manual triggers.

---

### Q5. What happens on Redis restart with AOF enabled?

**Interview Answer**

Redis checks for `appendonly.aof` first (if `appendonly yes`). If the AOF file is valid, Redis replays all commands from the AOF to reconstruct the dataset. If AOF is corrupted, Redis refuses to start to prevent data loss. If AOF is disabled but `dump.rdb` exists, Redis loads the RDB snapshot. With hybrid mode, the AOF file starts with an RDB snapshot (fast load) followed by incremental commands. AOF replay can be slow for very large files — use `redis-check-aof --fix` if corruption is detected.

---

### Q6. How does Redis handle data integrity on crash?

**Interview Answer**

With RDB only, you lose all writes since the last snapshot — potentially up to `save` interval worth of data. With AOF `appendfsync everysec`, you lose at most ~1 second of writes. With AOF `appendfsync always`, you lose zero writes but take a severe performance hit (each write blocks for fsync). In practice, `everysec` is the sweet spot. Redis also uses a checksum on both RDB and AOF files to detect corruption. For critical data, treat Redis as a cache and rely on PostgreSQL for durability.

---

### Q7. How do you monitor persistence health?

**Interview Answer**

Use `INFO persistence` to check: `rdb_last_bgsave_status` (should be `ok`), `aof_last_bgrewrite_status` (should be `ok`), `aof_current_size` vs `aof_base_size` (to see rewrite progress), and `rdb_last_cow_size` (to monitor copy-on-write memory usage during saves). Alert on non-`ok` status, high `rdb_last_cow_size`, and growing `aof_current_size` without rewrites. In Rust, you can periodically run `redis::cmd("INFO").arg("Persistence")` and parse the response.

---

### Q8. Can you switch between persistence modes without data loss?

**Interview Answer**

Yes — to switch from RDB to AOF, enable `appendonly yes` in the config and restart Redis (or use `CONFIG SET appendonly yes` which triggers an AOF rewrite in the background). To switch from AOF to RDB, disable AOF and restart. You can also convert `dump.rdb` to AOF format with `redis-check-aof --convert-from`. During transitions, ensure no writes are lost by enabling AOF before disabling RDB. Always take a backup `BGSAVE` before any persistence config change.

---

### Q9. How does persistence interact with replication?

**Interview Answer**

Replicas automatically receive data from the master via replication stream, not from persistence files. However, if a replica restarts, it loads data from its own RDB/AOF file before resuming replication. The master can persist however it wants (RDB, AOF, or both) — replicas don't care about the master's persistence config, only about the replication stream. If the master crashes and a replica is promoted, the new master's data is whatever the replica had — which may lag slightly behind the master's latest writes depending on replication offset.

---

### Q10. What are the memory implications of each persistence mode?

**Interview Answer**

RDB uses fork() for snapshotting, which creates a copy-on-write process — if the dataset is 10 GB and you write heavily during the snapshot, memory usage can spike to ~20 GB temporarily. AOF does not use fork() for regular writes (only for rewrites), so it avoids this memory spike, but AOF rewrite does fork. Hybrid mode has the same fork overhead as RDB during rewrites. On systems with limited RAM, prefer AOF with `appendfsync everysec` and limit RDB snapshot frequency. Monitor `used_memory_peak` to catch spikes.

