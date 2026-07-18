# Redis Data Structures

## Interview Question

Explain the core Redis data structures, their use cases, and how you would use them in a Rust backend.

## Interview Answer

Redis provides six primary data structures beyond simple strings: Lists (ordered sequences for queues and stacks), Sets (unordered unique collections for tagging and membership), Sorted Sets (ranked collections for leaderboards), Hashes (field-value maps for object storage), HyperLogLogs (probabilistic cardinality estimation), and Streams (append-only logs for event sourcing). Each structure has optimized internal encodings — for example, small lists use `listpack`, small hashes use `ziplist`, and they automatically convert to more efficient structures as they grow. In Rust, `redis-rs` provides typed commands like `redis::cmd("HSET")`, `redis::cmd("ZADD")`, and `redis::cmd("LPUSH")` for each structure, and the `serde` integration lets you serialize Rust structs directly into hashes.

---

## Follow-up Questions & Answers

### Q1. What is the difference between Strings and Hashes?

**Interview Answer**

Strings store a single value (text, number, or binary) under a key, e.g., `SET user:123_name "Alice"`. Hashes store multiple field-value pairs under a single key, e.g., `HSET user:123 name "Alice" age 30 email "a@b.com"`. Hashes are more memory-efficient for objects because Redis uses compact encoding (ziplist/listpack) for small hashes instead of one string per field. Use hashes when storing structured objects; use strings for simple caching or counters.

---

### Q2. When would you use a Sorted Set over a Set?

**Interview Answer**

Sets are unordered and support operations like membership testing (`SISMEMBER`), intersection (`SINTER`), and difference (`SDIFF`). Sorted Sets add a score to each member, enabling ordered operations like range queries (`ZRANGEBYSCORE`), rank lookups (`ZRANK`), and top-N queries (`ZREVRANGE`). Use Sorted Sets for leaderboards, time-sorted event queues, and priority queues. Use plain Sets for tagging, unique item tracking, and set operations.

---

### Q3. What is HyperLogLog and when would you use it?

**Interview Answer**

HyperLogLog is a probabilistic data structure that estimates the cardinality (number of unique elements) of a set using only ~12 KB of memory, regardless of the set size. You add elements with `PFADD` and get the count with `PFCOUNT`. It has a standard error of ~0.81%. Use it for counting unique website visitors, unique search queries, or unique IP addresses in analytics where approximate counts are acceptable and memory efficiency is critical. It cannot retrieve individual elements — only the count.

---

### Q4. What are Redis Streams and how do they differ from Lists?

**Interview Answer**

Streams are append-only logs with consumer group support, designed for event streaming and message queuing. Lists are simple ordered sequences with `LPUSH`/`RPOP` semantics — a message is consumed (removed) when popped. Streams retain all messages, support multiple independent consumers via consumer groups, track acknowledgment with `XACK`, and handle pending message recovery with `XPENDING`. Use Streams for durable event logs and message queues; use Lists for simple task queues where once consumed, the item is gone.

---

### Q5. How do Redis data structure encodings work?

**Interview Answer**

Redis uses compact encodings for small data structures to save memory: small lists use `listpack` (a single contiguous buffer), small hashes use `listpack` (instead of a full hash table), and small sorted sets also use `listpack`. When collections exceed configurable thresholds (`list-max-ziplist-size`, `hash-max-ziplist-entries`), Redis converts them to the general-purpose encoding (linked list, hash table, or skiplist). This is transparent to the application — you use the same commands regardless of encoding.

---

### Q6. How do you use Sorted Sets for a real-time leaderboard in Rust?

**Interview Answer**

Use `ZADD leaderboard <score> <user_id>` to update scores, `ZREVRANK leaderboard <user_id>` to get a user's rank, and `ZREVRANGE leaderboard 0 9 WITHSCORES` for the top 10. In Rust with `redis-rs`:

```rust
use redis::AsyncCommands;
con.zadd("leaderboard", "user:42", 1500).await?;
let rank: Option<i64> = con.zrevrank("leaderboard", "user:42").await?;
let top: Vec<(String, f64)> = con.zrevrange_withscores("leaderboard", 0, 9).await?;
```

This gives O(log N) for all operations, efficient for millions of players.

---

### Q7. How do you store a Rust struct in a Redis Hash?

**Interview Answer**

Serialize the struct to JSON with `serde_json` and store it as a single hash field, or map struct fields to hash fields. The simplest approach:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User { name: String, age: u32, email: String }

let user = User { name: "Alice".into(), age: 30, email: "a@b.com".into() };
let json = serde_json::to_string(&user)?;
redis::cmd("HSET").arg("user:123").arg("data").arg(&json).execute(&mut con).await?;
```

Alternatively, use `HSET user:123 name "Alice" age 30` to map each field directly, which allows partial reads with `HGET`.

---

### Q8. What are the time complexities of common Redis operations?

**Interview Answer**

`SET`/`GET`: O(1). `LPUSH`/`RPUSH`/`LPOP`: O(1). `LRANGE`: O(S+N) where S is the start offset and N is the requested range. `SADD`/`SISMEMBER`: O(1). `ZADD`/`ZRANK`: O(log N). `HSET`/`HGET`: O(1). `PFADD`: O(1). `XADD`: O(1). Understanding these matters because `LRANGE 0 -1` (fetching an entire list) is O(N) and can block Redis on large lists — use `LTRIM` to cap list sizes.

---

### Q9. How does `redis-rs` handle different data structures in Rust?

**Interview Answer**

`redis-rs` provides the `RedisResult<T>` type where `T` determines how the response is deserialized: `String` for simple strings, `Vec<String>` for multi-bulk responses, `HashMap<String, String>` for hash results, and `(String, f64)` tuples for sorted set results. Use the `redis::cmd()` builder for raw commands, or the `AsyncCommands` trait for typed method calls like `con.get::<_, String>(key)`. The `serde` feature enables automatic serialization/deserialization of Rust structs to/from Redis values.

---

### Q10. What is the memory overhead of each data structure?

**Interview Answer**

Every Redis key has a fixed overhead (~50-70 bytes for the key metadata). Strings have minimal overhead per value. Hashes under `listpack` encoding have very low overhead (~5-10 bytes per field), but switch to hash table encoding (~100+ bytes overhead) at larger sizes. Lists use `listpack` until they exceed `list-max-ziplist-size`, then switch to linked lists with higher overhead. Sorted Sets use `listpack` for small sets and skiplists (~32 bytes per node) for larger ones. Monitor memory with `INFO memory` and `MEMORY USAGE <key>` to understand actual footprint.

