# LEVEL 12 — Caching

### 0271. Redis

In-memory data structure server: strings, hashes, lists, sets, sorted sets, streams. Used as cache, session store, rate limiter, lock helper, queue.

Single-threaded command execution (with I/O threads in newer versions) keeps operations simple and fast. Persistence optional (RDB/AOF).

**Talk track:** *"Redis is shared memory with a network API — blazing fast if you accept memory cost and eviction/consistency tradeoffs."*

---

### 0272. Cache Aside

App reads cache; on miss, loads DB, writes cache. App owns consistency. Most common pattern. On writes, update DB then invalidate (or update) cache.

**Talk track:** *"Cache-aside: app manages cache. Miss → DB → fill. Write → DB then invalidate."*

---

### 0273. Read Through

Cache sits in front; on miss the *cache layer* loads from DB. App always talks to cache. Needs a smart cache component.

**Talk track:** *"Read-through hides DB loads inside the cache layer."*

---

### 0274. Write Through

Writes go to cache and DB synchronously. Stronger freshness; higher write latency.

**Talk track:** *"Write-through keeps cache warm on writes at the cost of write latency."*

---

### 0275. Write Back

Writes hit cache; flush to DB later asynchronously. Fast writes; risk data loss on crash; complex.

**Talk track:** *"Write-back is speed for durability risk — rare in critical money paths."*

---

### 0276. Write Around

Writes skip cache, go to DB; cache fills on read. Avoids cache pollution from write-once data.

**Talk track:** *"Write-around keeps write-heavy cold data from thrashing the cache."*

---

### 0277. Cache Invalidation

The hard problem: when source data changes, cache must update or die. TTL, explicit delete, versioned keys. Stale reads vs thundering herds on expiry.

**Talk track:** *"Invalidation is the real cache problem — TTL is a blunt tool; event-driven deletes are sharper."*

---

### 0278. Distributed Cache

Cache shared across app nodes (Redis cluster, Memcached). Avoids per-node duplication and inconsistency of local-only caches — adds network hop.

**Talk track:** *"Distributed cache is one coherent hot layer for many app instances."*

---

### 0279. Redis Cluster

Sharded Redis with hash slots. Scale memory/throughput horizontally. Multi-key ops limited when keys span slots; resharding operational work.

**Talk track:** *"Redis Cluster shards by hash slot — design keys for locality of multi-key ops."*
