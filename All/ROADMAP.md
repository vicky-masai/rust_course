# Full Roadmap — 24 Modules, 222 Daily Topics

One topic per day. A checked box means the video is out and the folder is live in this repo.

Folder naming: each topic lives at `MODULE-FOLDER/NN-topic-name/` (for example, Day 1 is [01-ownership-memory-model/01-ownership-basics/](01-ownership-memory-model/01-ownership-basics/)).

## Module 01 — Ownership & Memory Model (`01-ownership-memory-model/`)

- [x] Day 1 — Ownership basics → `01-ownership-basics/`
- [x] Day 2 — Ownership internals (stack, heap, and who frees what) → `02-ownership-internals/`
- [ ] Day 3 — Borrow checker deep dive → `03-borrow-checker-deep-dive/`
- [ ] Day 4 — Move semantics → `04-move-semantics/`
- [ ] Day 5 — Copy vs Clone → `05-copy-vs-clone/`
- [ ] Day 6 — The Drop trait → `06-drop-trait/`
- [ ] Day 7 — Drop order → `07-drop-order/`
- [ ] Day 8 — Deref coercion → `08-deref-coercion/`
- [ ] Day 9 — Interior mutability → `09-interior-mutability/`
- [ ] Day 10 — Memory layout (size, alignment, repr) → `10-memory-layout/`
- [ ] Day 11 — Zero-cost abstractions → `11-zero-cost-abstractions/`
- [ ] Day 12 — Pinning (intro) → `12-pinning-intro/`
- [ ] Day 13 — Self-referential structs → `13-self-referential-structs/`
- [ ] Day 14 — PhantomData → `14-phantomdata/`



## Module 02 — Lifetimes (`02-lifetimes/`)

- [ ] Day 15 — Lifetime elision → `01-lifetime-elision/`
- [ ] Day 16 — Explicit lifetimes → `02-explicit-lifetimes/`
- [ ] Day 17 — Lifetime bounds → `03-lifetime-bounds/`
- [ ] Day 18 — The 'static lifetime → `04-static-lifetime/`
- [ ] Day 19 — Higher-Ranked Trait Bounds (HRTB) → `05-hrtb/`
- [ ] Day 20 — Generic lifetime parameters → `06-generic-lifetime-parameters/`
- [ ] Day 21 — Lifetime variance → `07-lifetime-variance/`
- [ ] Day 22 — Lifetime subtyping → `08-lifetime-subtyping/`
- [ ] Day 23 — Async lifetime challenges → `09-async-lifetime-challenges/`



## Module 03 — Traits & Generics (`03-traits-and-generics/`)

- [ ] Day 24 — Trait bounds → `01-trait-bounds/`
- [ ] Day 25 — Associated types → `02-associated-types/`
- [ ] Day 26 — Generic Associated Types (GATs) → `03-gats/`
- [ ] Day 27 — Trait objects → `04-trait-objects/`
- [ ] Day 28 — Dynamic dispatch → `05-dynamic-dispatch/`
- [ ] Day 29 — Static dispatch → `06-static-dispatch/`
- [ ] Day 30 — Marker traits → `07-marker-traits/`
- [ ] Day 31 — Auto traits → `08-auto-traits/`
- [ ] Day 32 — Send → `09-send/`
- [ ] Day 33 — Sync → `10-sync/`
- [ ] Day 34 — Sized → `11-sized/`
- [ ] Day 35 — Unsize → `12-unsize/`
- [ ] Day 36 — Blanket implementations → `13-blanket-implementations/`
- [ ] Day 37 — Trait upcasting → `14-trait-upcasting/`
- [ ] Day 38 — Specialization → `15-specialization/`



## Module 04 — Smart Pointers (`04-smart-pointers/`)

- [ ] Day 39 — BoxT → `01-box/`
- [ ] Day 40 — RcT → `02-rc/`
- [ ] Day 41 — ArcT → `03-arc/`
- [ ] Day 42 — CellT → `04-cell/`
- [ ] Day 43 — RefCellT → `05-refcell/`
- [ ] Day 44 — MutexT → `06-mutex/`
- [ ] Day 45 — RwLockT → `07-rwlock/`
- [ ] Day 46 — OnceCell → `08-oncecell/`
- [ ] Day 47 — LazyLock → `09-lazylock/`
- [ ] Day 48 — Atomic types → `10-atomic-types/`



## Module 05 — Concurrency (`05-concurrency/`)

- [ ] Day 49 — Threads → `01-threads/`
- [ ] Day 50 — Thread pools → `02-thread-pools/`
- [ ] Day 51 — Scoped threads → `03-scoped-threads/`
- [ ] Day 52 — Shared state concurrency → `04-shared-state/`
- [ ] Day 53 — Arc + Mutex → `05-arc-mutex/`
- [ ] Day 54 — Arc + RwLock → `06-arc-rwlock/`
- [ ] Day 55 — Condvar → `07-condvar/`
- [ ] Day 56 — Message passing → `08-message-passing/`
- [ ] Day 57 — MPSC channels → `09-mpsc-channels/`
- [ ] Day 58 — Crossbeam → `10-crossbeam/`
- [ ] Day 59 — Lock-free programming → `11-lock-free/`
- [ ] Day 60 — Atomics → `12-atomics/`
- [ ] Day 61 — Memory ordering → `13-memory-ordering/`
- [ ] Day 62 — Compare-And-Swap (CAS) → `14-cas/`
- [ ] Day 63 — The ABA problem → `15-aba-problem/`



## Module 06 — Async Rust (`06-async-rust/`)

- [ ] Day 64 — The Future trait → `01-future-trait/`
- [ ] Day 65 — Poll → `02-poll/`
- [ ] Day 66 — Waker → `03-waker/`
- [ ] Day 67 — Context → `04-context/`
- [ ] Day 68 — Async/await internals → `05-async-await-internals/`
- [ ] Day 69 — Async state machines → `06-async-state-machines/`
- [ ] Day 70 — Async cancellation → `07-async-cancellation/`
- [ ] Day 71 — Async streams → `08-async-streams/`
- [ ] Day 72 — Backpressure → `09-backpressure/`



## Module 07 — Tokio Deep Dive (`07-tokio-deep-dive/`)

- [ ] Day 73 — The Tokio runtime → `01-tokio-runtime/`
- [ ] Day 74 — Scheduler internals → `02-scheduler-internals/`
- [ ] Day 75 — Work stealing → `03-work-stealing/`
- [ ] Day 76 — Cooperative scheduling → `04-cooperative-scheduling/`
- [ ] Day 77 — spawn → `05-spawn/`
- [ ] Day 78 — spawn_blocking → `06-spawn-blocking/`
- [ ] Day 79 — JoinHandle → `07-joinhandle/`
- [ ] Day 80 — JoinSet → `08-joinset/`
- [ ] Day 81 — select! → `09-select/`
- [ ] Day 82 — timeout → `10-timeout/`
- [ ] Day 83 — Semaphore → `11-semaphore/`
- [ ] Day 84 — Notify → `12-notify/`
- [ ] Day 85 — Barrier → `13-barrier/`
- [ ] Day 86 — Tokio MPSC channels → `14-mpsc-channels/`
- [ ] Day 87 — Broadcast channels → `15-broadcast-channels/`
- [ ] Day 88 — Watch channels → `16-watch-channels/`



## Module 08 — Unsafe Rust (`08-unsafe-rust/`)

- [ ] Day 89 — Raw pointers → `01-raw-pointers/`
- [ ] Day 90 — Unsafe functions → `02-unsafe-functions/`
- [ ] Day 91 — Unsafe traits → `03-unsafe-traits/`
- [ ] Day 92 — Unsafe blocks → `04-unsafe-blocks/`
- [ ] Day 93 — Dereferencing pointers → `05-dereferencing-pointers/`
- [ ] Day 94 — FFI basics → `06-ffi-basics/`
- [ ] Day 95 — Manual memory management → `07-manual-memory-management/`



## Module 09 — Pinning & Futures Internals (`09-pinning-and-futures-internals/`)

- [ ] Day 96 — PinT in depth → `01-pin/`
- [ ] Day 97 — Unpin → `02-unpin/`
- [ ] Day 98 — Self-referential types and why Pin exists → `03-self-referential-types/`
- [ ] Day 99 — Implementing a Future by hand → `04-implementing-a-future/`
- [ ] Day 100 — Build a mini async runtime → `05-mini-async-runtime/`



## Module 10 — Collections Internals (`10-collections-internals/`)

- [ ] Day 101 — Vec internals → `01-vec-internals/`
- [ ] Day 102 — HashMap internals → `02-hashmap-internals/`
- [ ] Day 103 — BTreeMap → `03-btreemap/`
- [ ] Day 104 — BinaryHeap → `04-binaryheap/`
- [ ] Day 105 — Hashing algorithms → `05-hashing-algorithms/`
- [ ] Day 106 — Memory allocation → `06-memory-allocation/`



## Module 11 — Error Handling (`11-error-handling/`)

- [ ] Day 107 — ResultT, E → `01-result/`
- [ ] Day 108 — OptionT → `02-option/`
- [ ] Day 109 — Custom errors → `03-custom-errors/`
- [ ] Day 110 — thiserror → `04-thiserror/`
- [ ] Day 111 — anyhow → `05-anyhow/`
- [ ] Day 112 — eyre → `06-eyre/`
- [ ] Day 113 — Error propagation → `07-error-propagation/`



## Module 12 — Macros (`12-macros/`)

- [ ] Day 114 — macro_rules! → `01-macro-rules/`
- [ ] Day 115 — Procedural macros → `02-proc-macros/`
- [ ] Day 116 — Derive macros → `03-derive-macros/`
- [ ] Day 117 — Attribute macros → `04-attribute-macros/`
- [ ] Day 118 — Function-like macros → `05-function-like-macros/`
- [ ] Day 119 — syn → `06-syn/`
- [ ] Day 120 — quote → `07-quote/`



## Module 13 — Serialization (`13-serialization/`)

- [ ] Day 121 — Serde → `01-serde/`
- [ ] Day 122 — JSON → `02-json/`
- [ ] Day 123 — Bincode → `03-bincode/`
- [ ] Day 124 — Protobuf → `04-protobuf/`
- [ ] Day 125 — FlatBuffers → `05-flatbuffers/`



## Module 14 — FFI (`14-ffi/`)

- [ ] Day 126 — Rust ↔ C → `01-rust-c/`
- [ ] Day 127 — Rust ↔ C++ → `02-rust-cpp/`
- [ ] Day 128 — Rust ↔ Python → `03-rust-python/`
- [ ] Day 129 — extern "C" → `04-extern-c/`
- [ ] Day 130 — no_mangle → `05-no-mangle/`



## Module 15 — Network Programming (`15-network-programming/`)

- [ ] Day 131 — TCP → `01-tcp/`
- [ ] Day 132 — UDP → `02-udp/`
- [ ] Day 133 — HTTP → `03-http/`
- [ ] Day 134 — HTTP/2 → `04-http2/`
- [ ] Day 135 — HTTP/3 → `05-http3/`
- [ ] Day 136 — WebSocket → `06-websocket/`
- [ ] Day 137 — Server-Sent Events (SSE) → `07-sse/`
- [ ] Day 138 — gRPC → `08-grpc/`
- [ ] Day 139 — Hyper → `09-hyper/`
- [ ] Day 140 — Tonic → `10-tonic/`



## Module 16 — Database Engineering (`16-database-engineering/`)

- [ ] Day 141 — SQLx → `01-sqlx/`
- [ ] Day 142 — Connection pooling → `02-connection-pooling/`
- [ ] Day 143 — Transactions → `03-transactions/`
- [ ] Day 144 — Isolation levels → `04-isolation-levels/`
- [ ] Day 145 — Deadlocks → `05-deadlocks/`
- [ ] Day 146 — Optimistic locking → `06-optimistic-locking/`
- [ ] Day 147 — Pessimistic locking → `07-pessimistic-locking/`
- [ ] Day 148 — Query optimization → `08-query-optimization/`
- [ ] Day 149 — PostgreSQL internals → `09-postgresql-internals/`



## Module 17 — Distributed Systems (`17-distributed-systems/`)

- [ ] Day 150 — CAP theorem → `01-cap-theorem/`
- [ ] Day 151 — Consistency models → `02-consistency-models/`
- [ ] Day 152 — Consensus algorithms → `03-consensus-algorithms/`
- [ ] Day 153 — Raft → `04-raft/`
- [ ] Day 154 — Paxos → `05-paxos/`
- [ ] Day 155 — Leader election → `06-leader-election/`
- [ ] Day 156 — Distributed locking → `07-distributed-locking/`
- [ ] Day 157 — Sharding → `08-sharding/`
- [ ] Day 158 — Replication → `09-replication/`
- [ ] Day 159 — Partitioning → `10-partitioning/`



## Module 18 — Event-Driven Architecture (`18-event-driven-architecture/`)

- [ ] Day 160 — Kafka → `01-kafka/`
- [ ] Day 161 — RabbitMQ → `02-rabbitmq/`
- [ ] Day 162 — NATS → `03-nats/`
- [ ] Day 163 — Event sourcing → `04-event-sourcing/`
- [ ] Day 164 — CQRS → `05-cqrs/`
- [ ] Day 165 — Outbox pattern → `06-outbox-pattern/`
- [ ] Day 166 — Change Data Capture (CDC) → `07-cdc/`
- [ ] Day 167 — Saga pattern → `08-saga-pattern/`
- [ ] Day 168 — Idempotency → `09-idempotency/`



## Module 19 — Performance Engineering (`19-performance-engineering/`)

- [ ] Day 169 — Benchmarking → `01-benchmarking/`
- [ ] Day 170 — Profiling → `02-profiling/`
- [ ] Day 171 — Flamegraphs → `03-flamegraphs/`
- [ ] Day 172 — Memory profiling → `04-memory-profiling/`
- [ ] Day 173 — CPU profiling → `05-cpu-profiling/`
- [ ] Day 174 — Cache optimization → `06-cache-optimization/`
- [ ] Day 175 — Allocation reduction → `07-allocation-reduction/`



## Module 20 — Observability (`20-observability/`)

- [ ] Day 176 — tracing → `01-tracing/`
- [ ] Day 177 — OpenTelemetry → `02-opentelemetry/`
- [ ] Day 178 — Structured logging → `03-structured-logging/`
- [ ] Day 179 — Metrics → `04-metrics/`
- [ ] Day 180 — Prometheus → `05-prometheus/`
- [ ] Day 181 — Grafana → `06-grafana/`
- [ ] Day 182 — Distributed tracing → `07-distributed-tracing/`



## Module 21 — Advanced Axum (`21-advanced-axum/`)

- [ ] Day 183 — Middleware → `01-middleware/`
- [ ] Day 184 — Tower layers → `02-tower-layers/`
- [ ] Day 185 — Custom extractors → `03-custom-extractors/`
- [ ] Day 186 — Request extensions → `04-request-extensions/`
- [ ] Day 187 — Authentication → `05-authentication/`
- [ ] Day 188 — Authorization → `06-authorization/`
- [ ] Day 189 — RBAC → `07-rbac/`
- [ ] Day 190 — ABAC → `08-abac/`
- [ ] Day 191 — Rate limiting → `09-rate-limiting/`
- [ ] Day 192 — WebSockets → `10-websockets/`
- [ ] Day 193 — SSE → `11-sse/`
- [ ] Day 194 — Background workers → `12-background-workers/`



## Module 22 — Enterprise Backend Patterns (`22-enterprise-backend-patterns/`)

- [ ] Day 195 — Repository pattern → `01-repository-pattern/`
- [ ] Day 196 — Unit of Work → `02-unit-of-work/`
- [ ] Day 197 — Domain Driven Design (DDD) → `03-ddd/`
- [ ] Day 198 — CQRS in practice → `04-cqrs/`
- [ ] Day 199 — Event Sourcing in practice → `05-event-sourcing/`
- [ ] Day 200 — Saga pattern in practice → `06-saga-pattern/`
- [ ] Day 201 — Outbox pattern in practice → `07-outbox-pattern/`
- [ ] Day 202 — Workflow engines → `08-workflow-engines/`



## Module 23 — Enterprise Architecture (`23-enterprise-architecture/`)

- [ ] Day 203 — Multi-tenancy → `01-multi-tenancy/`
- [ ] Day 204 — Audit logging → `02-audit-logging/`
- [ ] Day 205 — Event framework → `03-event-framework/`
- [ ] Day 206 — Inventory reservation → `04-inventory-reservation/`
- [ ] Day 207 — Distributed transactions → `05-distributed-transactions/`
- [ ] Day 208 — Permission systems → `06-permission-systems/`
- [ ] Day 209 — Feature flags → `07-feature-flags/`
- [ ] Day 210 — Service discovery → `08-service-discovery/`
- [ ] Day 211 — API gateways → `09-api-gateways/`
- [ ] Day 212 — Resilience patterns → `10-resilience-patterns/`



## Module 24 — System Design (`24-system-design/`)

- [ ] Day 213 — Scalability → `01-scalability/`
- [ ] Day 214 — Reliability → `02-reliability/`
- [ ] Day 215 — Availability → `03-availability/`
- [ ] Day 216 — Fault tolerance → `04-fault-tolerance/`
- [ ] Day 217 — Capacity planning → `05-capacity-planning/`
- [ ] Day 218 — Caching → `06-caching/`
- [ ] Day 219 — Load balancing → `07-load-balancing/`
- [ ] Day 220 — Rate limiting → `08-rate-limiting/`
- [ ] Day 221 — Disaster recovery → `09-disaster-recovery/`
- [ ] Day 222 — High availability → `10-high-availability/`