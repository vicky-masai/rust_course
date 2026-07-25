# Round: Rust Deep Dive — Questions & Expected Answers

**Duration:** 60–75m (often combined with a short coding lab)  
**Goal:** Verify production Rust mastery suitable for Staff-level systems work at Zscaler.

**Scoring:** See `09-scorecards/rust-scorecard.md`

---

## Section A — Ownership, borrowing, lifetimes (15–20m)

### Q1. Explain ownership as if onboarding a strong C++ engineer.

**Strong answer**
- Every value has one owner; move semantics by default; borrow checker enforces exclusive mutable XOR shared immutable (with nuanced exceptions via interior mutability).
- RAII drop = deterministic cleanup; no GC pauses, but you design for lifetimes and graphs carefully.
- Mentions when `Clone` / `Arc` / `Rc` are appropriate vs overused.

**Probe:** When is `Rc` wrong in async servers?  
**Expect:** `Rc` is `!Send`; async tasks may move across threads → need `Arc`.

**Weak:** “Rust doesn’t have pointers” / cannot contrast with shared_ptr.

---

### Q2. When do you need explicit lifetime annotations? Give a real API example.

**Strong answer**
- Elision covers many cases; explicit when returning references tied to inputs, or storing references in structs.
- Example: `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str` or a struct holding `&'a Config`.
- Prefer owned types at API boundaries for simplicity when performance allows.

**Staff plus:** Discusses lifetime as *contracts* between caller and callee; avoids lifetime soup with arenas / owning wrappers.

---

### Q3. Interior mutability: `Cell`, `RefCell`, `Mutex`, `RwLock`, atomics — when which?

**Strong answer**

| Tool | Use |
|------|-----|
| `Cell` | `Copy` types, single-threaded |
| `RefCell` | Runtime borrow checks, single-threaded |
| `Mutex`/`RwLock` | Cross-thread shared mutability |
| Atomics | Lock-free counters/flags with correct ordering |

**Probe:** Contended `RwLock` on request path?  
**Expect:** Prefer sharding, message passing, or redesign; read-heavy with rare writes still needs measurement.

---

## Section B — Traits, generics, API design (10–15m)

### Q4. Design a trait for a pluggable “policy evaluator.” What bounds matter?

**Strong answer**
- Trait object vs generics: static dispatch for hot path; `dyn` for plugin boundaries.
- Bounds: `Send + Sync + 'static` for multi-threaded servers.
- Error type strategy: associated `Error` or `anyhow`/`thiserror` at boundaries.
- Async trait considerations (`async-trait` / RPITIT / `dyn` limitations).

**Staff plus:** Versioning policies, feature flags, deterministic evaluation for audit.

---

### Q5. `Send` / `Sync` — explain and give a bug you’ve seen.

**Strong answer**
- `Send`: transfer ownership across threads; `Sync`: shared reference (`&T`) safe across threads (`T: Sync` ⇒ `&T: Send`).
- Classic bug: holding `RefCell`/`Rc` across `.await`; or non-`Send` future captured in spawn.
- Mentions `tokio::spawn` requiring `Send` futures.

---

## Section C — Async Rust / Tokio (15–20m)

### Q6. What goes wrong if you run CPU-heavy work on the async runtime?

**Strong answer**
- Starves the executor; latency spikes for all tasks on that worker.
- Fix: `spawn_blocking`, dedicated thread pool, or separate process for heavy crypto/compression.
- Mentions cooperative scheduling; long CPU loops need `yield_now` or chunking (still prefer offload).

---

### Q7. Explain pinning and why `Pin<&mut Fut>` exists. Do you need `unsafe` to write normal async?

**Strong answer**
- Self-referential futures; moving would invalidate internal references.
- Pin guarantees the future won’t move after polling starts.
- Everyday async/`async fn` users rarely write unsafe; library authors sometimes do.
- Staff: can sketch why `poll` takes `Pin<&mut Self>`.

**Weak:** “Pin is for memory mapping” with no futures link.

---

### Q8. Cancellation safety: give an example of unsafe-with-respect-to-cancel async code.

**Strong answer**
- Example: read into buffer, then process; cancel between leaves inconsistent state.
- `tokio::select!` dropping futures mid-operation; use cancel-safe APIs (`read_buf` patterns), structured concurrency, or make operations idempotent.
- Mentions timeouts as cancellation.

---

### Q9. Backpressure in an async Rust service ingesting events.

**Strong answer**
- Bounded channels (`mpsc` with capacity), load shedding, admission control, queue depth metrics.
- Never unbounded queues in front of slow consumers for internet-facing paths.
- Tie to client timeouts and retry amplification.

---

## Section D — Unsafe, FFI, correctness (10m)

### Q10. When is `unsafe` justified in production? How do you contain it?

**Strong answer**
- FFI, performance-critical validated invariants, implementing safe abstractions.
- Minimize surface; module boundary; document safety invariants; fuzz + Miri where applicable; code review by experts.
- Prefer proven crates over home-grown unsafe.

**Red flag:** Casual unsafe for “speed” without measurement or invariants.

---

### Q11. How do you prevent memory unsafety when calling C libraries?

**Strong answer**
- `bindgen`/manual FFI; ownership of pointers clear; null checks; lifetimes modeled in safe wrappers; `Drop` frees once; no double-free; thread-safety documented.
- Prefer `rustls` over OpenSSL when possible for TLS stack control — discuss trade-offs honestly.

---

## Section E — Performance & production (10–15m)

### Q12. p99 latency regressed 3× after a Rust deploy. How do you debug?

**Strong answer**
- Metrics first: latency histograms, queue depths, CPU, allocation rate, error rates.
- Continuous profiling (e.g., `perf`, flamegraphs, tokio console).
- Check: lock contention, accidental clone of large buffers, DNS, TLS handshake spikes, blocking on runtime, GC in dependencies (unlikely), noisy neighbor, config flags.
- Canary / bisect / feature flag rollback.

---

### Q13. Zero-copy / buffer strategy for a proxy-like path.

**Strong answer**
- `Bytes`/`BytesMut`, buffer pooling, read into reusable slabs, avoid unnecessary `Vec` copies.
- Be careful with lifetime of borrowed buffers across await points → often need owned or reference-counted buffers.
- Security: clear sensitive buffers where required; don’t log payloads.

---

### Q14. Error handling philosophy in a Staff-owned crate.

**Strong answer**
- Typed errors at domain boundaries (`thiserror`); `anyhow` for apps; never swallow errors; attach context; map to correct HTTP/gRPC status without leaking internals.
- Distinguish retryable vs fatal; preserve causality for incidents.

---

## Section F — Ecosystem judgment (5–10m)

### Q15. Pick web stack for an internal control-plane API vs data-plane proxy. Why?

**Strong answer**
- Control plane: Axum/Actix/Tonic — developer velocity, middleware, OpenAPI.
- Data plane: lean custom or specialized proxy frameworks; minimize allocations; careful syscall batching; possibly C/Rust hybrid historically — justify.
- Mentions observability hooks (tracing, metrics) as first-class.

---

## Close probes (pick 1–2)

- “Show me how you’d structure a workspace for a multi-crate product.”
- “How do you manage dependency audit / supply chain (`cargo audit`, vetting)?”
- “What’s your view on `unwrap` in production servers?”

**Staff unwrap answer:** Forbidden on request path except documented unreachable; prefer expect with reason in binaries’ startup only.

---

## Pass / fail snapshot

| Level | Signal |
|-------|--------|
| Strong Staff | Fluent async + ownership + production debugging; unsafe discipline; API taste |
| Borderline | Good Rust, thin on async cancellation / production war stories |
| Fail | Cannot explain Send/Sync, or only toy Rust |
