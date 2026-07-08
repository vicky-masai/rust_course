# Rust Advanced Course — One Concept a Day

A self-study Rust course, written for interview preparation. Every day covers **one concept** in **simple English**, in question-and-answer form — the questions an interviewer would ask, answered the way a senior engineer would answer their CTO: direct, precise, no filler.

## How this repo works

Every day = one topic = one folder. Each completed topic contains two files:

- **`notes.md`** — the concept as a series of Q&As. Read it top to bottom; each answer is written the way you should say it in an interview.
- **`assignment.md`** — questions only, no answers. Answer them from memory, out loud. If you can't, reread the notes. Don't move to the next day until you can.

Empty folders are topics coming soon — the full schedule is in [ROADMAP.md](ROADMAP.md).

## How to study

1. Read the day's `notes.md` once, slowly.
2. Close it. Open `assignment.md` and answer every question out loud, as if in an interview.
3. Stuck on a question? Reread only that section of the notes, then answer again from memory.
4. Next day, before starting the new topic, re-answer yesterday's warm-up questions. Spaced repetition is what makes it stick.

## Prerequisites

- Basic Rust syntax (variables, functions, structs) — this course starts at ownership and goes deep
- Rust installed via [rustup](https://rustup.rs) if you want to try concepts in code as you read

## The roadmap

The full plan — 24 modules, ~230 topics — lives in [ROADMAP.md](ROADMAP.md). Checked boxes are published; unchecked are coming soon.

| # | Module | What you'll master |
|---|--------|--------------------|
| 01 | [Ownership & Memory Model](01-ownership-memory-model/) | How Rust manages memory without a garbage collector |
| 02 | [Lifetimes](02-lifetimes/) | How Rust proves references are always valid |
| 03 | [Traits & Generics](03-traits-and-generics/) | Rust's powerful abstraction system |
| 04 | [Smart Pointers](04-smart-pointers/) | Box, Rc, Arc, RefCell, Mutex and friends |
| 05 | [Concurrency](05-concurrency/) | Threads, channels, atomics, lock-free programming |
| 06 | [Async Rust](06-async-rust/) | Futures, polling, wakers, async/await internals |
| 07 | [Tokio Deep Dive](07-tokio-deep-dive/) | The async runtime powering most of Rust's ecosystem |
| 08 | [Unsafe Rust](08-unsafe-rust/) | Raw pointers, FFI, and manual memory management |
| 09 | [Pinning & Futures Internals](09-pinning-and-futures-internals/) | Pin, Unpin, and how async really works |
| 10 | [Collections Internals](10-collections-internals/) | How Vec, HashMap, BTreeMap work under the hood |
| 11 | [Error Handling](11-error-handling/) | Result, Option, thiserror, anyhow done right |
| 12 | [Macros](12-macros/) | macro_rules! and procedural macros |
| 13 | [Serialization](13-serialization/) | Serde, JSON, Bincode, Protobuf |
| 14 | [FFI](14-ffi/) | Calling C, C++, and Python from Rust (and back) |
| 15 | [Network Programming](15-network-programming/) | TCP, HTTP/1-2-3, WebSockets, gRPC |
| 16 | [Database Engineering](16-database-engineering/) | SQLx, transactions, locking, query optimization |
| 17 | [Distributed Systems](17-distributed-systems/) | CAP, Raft, consensus, sharding, replication |
| 18 | [Event-Driven Architecture](18-event-driven-architecture/) | Kafka, event sourcing, CQRS, sagas |
| 19 | [Performance Engineering](19-performance-engineering/) | Benchmarking, profiling, flamegraphs |
| 20 | [Observability](20-observability/) | tracing, OpenTelemetry, Prometheus, Grafana |
| 21 | [Advanced Axum](21-advanced-axum/) | Middleware, auth, rate limiting, WebSockets |
| 22 | [Enterprise Backend Patterns](22-enterprise-backend-patterns/) | Repository, DDD, CQRS, outbox |
| 23 | [Enterprise Architecture](23-enterprise-architecture/) | Multi-tenancy, feature flags, API gateways |
| 24 | [System Design](24-system-design/) | Scalability, caching, load balancing, HA |

## Daily workflow (for the author)

1. Open the day's topic folder (they're all pre-created — see [ROADMAP.md](ROADMAP.md)).
2. Write `notes.md` as Q&As with CTO-level answers, and `assignment.md` with questions only.
3. Tick the topic's checkbox in `ROADMAP.md`, commit, push.
