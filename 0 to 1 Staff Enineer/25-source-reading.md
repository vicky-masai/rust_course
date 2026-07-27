# LEVEL 25 — Source Code Reading (Mandatory)

Reading production code teaches what tutorials skip: real invariants, escape hatches, and performance tricks. For each project below: clone, pick one subsystem, trace one request/operation end-to-end, take notes on invariants.

---

### 0457. Rust std

The standard library — `Vec`, channels, `HashMap`, I/O, sync primitives. See how safe APIs wrap unsafe, how niches work, how OS differences are abstracted.

**How to read:** Start with `Vec` grow/reserve or `Arc` — small enough to finish, deep enough to learn.

**Talk track:** *"std is the reference for idiomatic safe wrappers over unsafe and OS primitives."*

---

### 0458. Tokio

Runtime, reactor, task scheduler, I/O drivers, time. Understand how `poll` meets epoll/kqueue and how tasks wake.

**How to read:** Trace `TcpStream::read` from async fn down to registration with the reactor.

**Talk track:** *"Tokio shows how an industrial async runtime is built — scheduler + I/O + time."*

---

### 0459. Axum

Routing, extractors, into_response, Tower integration. See how ergonomic HTTP APIs map to `Service` traits.

**How to read:** Follow a `Json<T>` extractor from handler signature to body bytes.

**Talk track:** *"Axum is a thin, typed layer over Tower/Hyper — extractors are the magic."*

---

### 0460. Tower

`Service` trait, layers, middleware composition, backpressure via `poll_ready`. The reusable middleware ecosystem for Rust networking.

**How to read:** Read `Service` + one Layer implementation (timeout or buffer).

**Talk track:** *"Tower is the middleware algebra — Services and Layers compose request pipelines."*

---

### 0461. Hyper

Low-level HTTP implementation — connections, bodies, versions. Performance-conscious parsing and IO.

**How to read:** Skim client or server connection lifecycle; note body streaming design.

**Talk track:** *"Hyper is the HTTP engine Axum sits on — connections and bodies without framework sugar."*

---

### 0462. Serde

Serialize/Deserialize traits, derive expansion concepts, data model. Format-agnostic design is the lesson.

**How to read:** Trace how a struct field becomes a serializer call.

**Talk track:** *"Serde separates data model from format — that's why one derive serves JSON and bincode."*

---

### 0463. SQLx

Async drivers, connection pooling, query macros, Postgres protocol bits. See how compile-time checking is wired.

**How to read:** Follow a `query_as!` path or pool acquire/release.

**Talk track:** *"SQLx shows async DB access with SQL retained as SQL — macros bridge types."*

---

### 0464. HashBrown

SwissTable/hashbrown — Rust's HashMap engine. SIMD-ish probing, control bytes, load factors. Peak systems Rust.

**How to read:** Read the table lookup path and control byte design summaries/comments.

**Talk track:** *"HashBrown is why Rust HashMap is fast — SwissTable layout and probing."*

---

### 0465. Redis

Single-threaded event loop, command processing, data structures (SDS, dict, ziplist/listpack historically), persistence, replication. C codebase, crystal-clear systems lessons.

**How to read:** Trace `SET`/`GET` from networking to dict; skim RDB/AOF conceptually.

**Talk track:** *"Redis proves a simple event loop + good structures can dominate an industry."*

---

### 0466. PostgreSQL

Process model, planner/executor, MVCC heap, WAL, vacuum. Reading even parts of executor/heap pays forever.

**How to read:** Use docs + selective source; follow what `EXPLAIN` plans map to.

**Talk track:** *"Postgres source is a masterclass in durable, concurrent storage engines."*

---

### 0467. Kafka

Log segments, replication protocol, controller, consumer group coordination. Distributed systems in production Java.

**How to read:** Partition leadership + produce path conceptually; skim ISR rules.

**Talk track:** *"Kafka is a replicated distributed log — replication and consumer coordination are the core."*

---

### 0468. etcd

Raft-based KV used as cluster brain (k8s). Watch API, leases, MVCC revisions. Read to see Raft applied practically.

**How to read:** Trace a `Put` through Raft log apply; understand watch/revision.

**Talk track:** *"etcd is Raft as a product — consistency for control planes."*
