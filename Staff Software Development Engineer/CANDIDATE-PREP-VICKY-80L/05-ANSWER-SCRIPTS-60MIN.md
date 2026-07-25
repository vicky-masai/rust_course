# 03 — ANSWER SCRIPTS (60 MIN)
# Speak like Staff SDE — aligned to Vicky’s resume

Practice out loud. Aim: **~70% of these words**, not a robotic memorization.  
Numbers below are from your PDF only.

---

# SECTION A — Ownership (3–12 min)

## Q1 — Most complex system you owned

### They ask
> Walk me through the most complex production system you personally owned…

### You say (2.5–3.5 minutes)

> “The system I own is the multi-tenant warehouse management backend at CodeApto — I’m the founding backend engineer.  
>  
> **Problem:** Multiple facilities needed correct inventory under concurrent warehouse operations. Wrong stock allocation means shipping failures and financial mismatch. We needed strong tenant isolation and transactional integrity, not just CRUD.  
>  
> **What I owned:** API design, PostgreSQL data model, authz/RBAC, Redis async jobs, WebSocket ops views, EKS releases, and OpenTelemetry.  
>  
> **Architecture decisions I made:**  
> 1) PostgreSQL as source of truth with transactions for inventory mutations; Redis for queues/cache — not for authoritative stock.  
> 2) Row-level security + API-scoped RBAC so tenant isolation is defense-in-depth.  
> 3) Move heavy work off the request path — 5K+ daily jobs via Redis queues.  
> 4) Migrate the inventory-locking hot path from Node.js to Rust/Axum/Tokio after we proved races and latency under concurrent load.  
>  
> **Scale:** 5+ facilities, 10K+ daily inventory transactions, 40+ versioned REST APIs. The hard constraint was **correctness under concurrency**, with bursty peaks.  
>  
> **Result:** P99 on inventory-locking path from about 800ms to 120ms after the Rust migration; deploy cycle 45 minutes to 8 minutes with better CI/CD.  
>  
> **What broke:** Early on, concurrent reservations could double-allocate under Node concurrency plus weak locking. We also had an incident class where async jobs and API writes needed clearer idempotency — retries duplicated side effects.  
>  
> **What I changed:** Transactional locking in Rust path, idempotency keys on writes/jobs, structured tracing so diagnosis dropped from hours to minutes, and explicit API error contracts (RFC 7807) so clients fail predictably.”

### If they probe “your decision vs team”
> “As founding backend I made the tenancy model, Rust migration call, and ‘jobs off request path’ call. Frontend and ops constrained the API shapes — I wrote the contracts they consumed.”

### If they probe “incident you led”
Pick one concrete story and stick to it (customize with real detail you remember):

> “Symptom: intermittent oversell / conflicting stock under peak.  
> Detect: customer ops report + rising 409/conflict and latency.  
> Mitigate: pause noncritical jobs, tighten reservation API, feature-flag risky path.  
> Root cause: race between read-check-write on stock.  
> Fix: transactional reserve in Rust + tests under parallel load.  
> Prevent: load test in CI for locking path; dashboards on conflict rate and p99.”

**Target score:** 3–4

---

# SECTION B — Rust (12–27 min)

## Q2 — Ownership + Send/Sync

### You say

> “Ownership: every value has one owner; moves by default; borrows are either many immutable refs or one mutable ref. Drop runs deterministically — like RAII — so we reason about resource lifetimes without a GC.  
>  
> **Send** means a type can move to another thread. **Sync** means shared references are safe across threads.  
>  
> Real issue in async Rust: people reach for `Rc`/`RefCell` out of habit from single-threaded mental models. In Tokio, tasks can run across threads, so futures often need to be `Send`. `Rc` is `!Send`. On our services I use `Arc` for shared state across tasks.  
>  
> Related production bug class on inventory: sharing mutable state without a clear concurrency story — we fixed authoritative mutations with DB transactions, and in-process state with `Arc` + appropriate synchronization, never holding a mutex across `.await` on a request path if we can avoid it.”

**Tie to resume:** race elimination + Tokio services.

---

## Q3 — Async CPU + cancellation

### You say

> “If I run heavy CPU — big JSON transforms, compression, crypto — directly on the async worker, I starve the runtime. Other requests on that thread spike latency. Fix: `spawn_blocking` or a dedicated pool; keep async tasks for IO.  
>  
> Cancellation: `select!` or timeouts drop the other future. Hazard: half-finished work — e.g. wrote partial side effect then cancelled. For jobs I design **idempotent** handlers with job IDs so retry/cancel doesn’t corrupt inventory. On HTTP, short timeouts with clear client contracts beat silent partial updates.”

**Tie to resume:** Redis jobs decoupled; idempotent writes.

---

## Q4 — Unsafe + unwrap

### You say

> “I almost never need `unsafe` in our Axum services. I’d use it only for FFI or a tightly reviewed safe abstraction with documented invariants — not for ‘speed.’ Prefer vetted crates.  
>  
> On the request path: no `unwrap` on IO, parse, or DB results. Map to typed errors and RFC 7807 responses. `expect` only for invariants that mean ‘bug in our code if false,’ and even then sparingly. Panics are a DoS risk in a multi-tenant API.”

---

## Q5 — p99 became 3× worse

### You say

> “First: confirm blast radius — which facility/tenant, which route, after which deploy. Check error rate vs pure latency.  
>  
> Metrics: p50/p99 histograms, DB time, queue depth, CPU, allocation, lock waits. Traces — we invested in OpenTelemetry so I look for slow spans on reserve/commit.  
>  
> Rust/async suspects I’ve lived: accidental blocking on runtime, contention, extra clones, DB lock waits, Redis timeouts causing retry amplification, logging too much on hot path.  
>  
> Mitigate: rollback or feature flag if customers hurt; then root cause. That’s how we approached the Node→Rust locking work — measure, fix invariant, re-measure 800→120ms.”

**Target section B:** 3+

---

# SECTION C — System design (27–42 min)

## Q6 — Multi-tenant forward proxy (they will push you outside WMS)

### Opening (clarify 60–90 sec)

> “Assumptions I’ll state: enterprise users, multi-tenant, need allow/deny/inspect style policy, audit logs, global edge. I’ll separate **data plane** (session path) from **control plane** (policy/identity/config). Please correct constraints anytime.”

### Design (draw while talking)

> “**Data plane / edge POP:** terminate or tunnel TLS, attach identity context, evaluate **cached compiled policy** locally, connect upstream, emit audit events async.  
>  
> **Control plane:** policy authoring, identity integration, signing/versioning of policy bundles, admin APIs, rollout rings.  
>  
> Hot path must not synchronously depend on control plane every request — same lesson as our WMS: don’t put Redis-job style work or remote config fetch on the critical request path without a cache.  
>  
> Policy: versioned bundles pushed/pulled to edge; monotonic versions; rollback to last-known-good.  
>  
> Logs: async pipeline, bounded buffer, tenant_id on every event; query path authz-enforced.  
>  
> Scale: many edge nodes; connection limits; pool upstreams; protect CPU on handshakes with session resumption.”

### Probe — policy down 5 minutes

> “For a security gateway I’d bias **fail closed** for sensitive categories, with a **short-TTL last-known-good policy cache** so we don’t melt availability on every control-plane blip — and we page hard when serving stale. Silent ‘allow all’ is unacceptable without an explicit product decision and audit. This mirrors our tenancy stance: default deny when authz context is missing.”

### Probe — Tenant A vs B

> “Tenant ID in auth context, policy objects, and every log line. Control-plane APIs authorize per tenant — no IDOR on policy IDs. Separate quotas so noisy tenants can’t starve others. Metrics labels carefully — don’t explode cardinality with raw user IDs. Same pattern we use with RLS + API authz + scoped RBAC, applied to edge + logs.”

### If stuck on TLS mechanics
Be honest + structured:  
> “I’m stronger on app-layer multi-tenant systems than on building a TLS proxy day-to-day. Here’s how I’d decompose handshake cost, cert rotation, and resumption — and what I’d validate with a networking partner in week one.”

Honesty + structure scores better than fake expertise.

**Target:** 3

---

# SECTION D — Security (42–50 min)

## Q7 — TLS 1.3 + resumption

### You say

> “TLS 1.3: ClientHello/ServerHello with key share, derive secrets, certificate verify, Finished, then encrypted application data. Handshake crypto and cert verify dominate CPU versus bulk AES-GCM.  
>  
> At proxy scale, short-lived connections mean handshakes burn CPU. **Session resumption/tickets** cut full handshakes — lower latency and CPU. Ticket keys must rotate; treat 0-RTT carefully because of replay. Ops: monitor cert expiry and rotation.”

If weaker on details, still hit: handshake vs bulk cost + why resumption matters.

---

## Q8 — Threat model control-plane API

### You say (list 6+, map to your experience)

> “1) Cross-tenant IDOR — mitigate with authz on every object, tests, tenant scoping (we use API authz + RLS).  
> 2) Stolen tokens — short TTL, rotation, revoke.  
> 3) Admin abuse — least privilege, MFA, audit logs.  
> 4) Injection / oversized payloads — validation, limits.  
> 5) SSRF via webhooks/connectors — allowlists, block metadata IPs.  
> 6) Supply chain — pin deps, audit, least privilege CI.  
> 7) PII in logs — redaction, controlled access.  
> 8) DoS — rate limits per tenant, auth on expensive admin APIs.”

---

## Q9 — Rust safe ≠ appsec

### You say

> “Rust helps memory safety. We can still ship broken authz, fail-open config, secret leakage in logs, panic-based DoS on bad input, SSRF, dependency CVEs, and logic bugs in policy. In our system, RLS doesn’t replace checking authorization in the API — defense in depth. Memory safety is necessary, not sufficient — especially at Zscaler.”

**Target section D:** ≥3 (mandatory for offer)

---

# SECTION E — Staff leadership (50–55 min)

## Q10 — Cross-team initiative

### Problem: you may not have 3 engineering teams. Reframe truthfully.

### You say

> “In a founding backend role I don’t have five backend teams — influence was across **frontend, operations users, and partner integrations**.  
>  
> **Initiative:** I drove a single versioned REST contract set — 40+ endpoints with idempotent writes and RFC 7807 errors — so frontend and partners stopped breaking on ad-hoc responses. I wrote the API docs/contracts, reviewed client usage, and rejected incompatible changes.  
>  
> **Second initiative:** Node→Rust migration of inventory locking. I sequenced it behind measurements, kept a rollback path, and made the concurrency model explicit so others could extend it safely.  
>  
> **Metric:** P99 800→120ms; fewer conflict/race incidents; deploy 45→8 min so the whole product org shipped faster.  
>  
> That’s the Staff pattern I practice: written contracts, measured migrations, reduced organizational friction. At Zscaler I’d apply the same to cross-team platform contracts at larger scale.”

---

## Q11 — Blocked a dangerous ship

### You say (use a real “I said no” moment — customize)

> “We had pressure to ship a faster inventory path that skipped strict transactional checks / weakened tenant scoping for speed. I blocked it.  
>  
> **Risk:** cross-facility data bleed or double-allocation — integrity and trust loss.  
>  
> **What I did:** showed failure mode with a concurrent test case; proposed shipping behind a flag with the safe transaction path; offered a performance plan (Rust locking) instead of safety shortcuts.  
>  
> **After:** we shipped the safe Rust path and won the latency goal without violating isolation. I won’t bypass authz or tenancy to hit a date — especially relevant for a security company.”

**Target section E:** ≥3

---

# 55–60 — Close

Ask 2–3 from `06-QUESTIONS-AND-80L-POSITIONING.md`.  
Do not ask about ₹80L in the technical hour unless they open compensation.

---

# 60-second recovery lines (if you blank)

| Situation | Line |
|-----------|------|
| Don’t know TLS detail | “I don’t want to invent. Framework is X; I’d verify with RFC/runbook and pair with net specialist.” |
| Scale pushback | “Absolute QPS is lower than Zscaler edge; invariants I owned are tenancy, concurrency, p99. Here’s 100× plan…” |
| “Are you Staff?” | “I’ve owned founding-backend scope; I’m interviewing to operate that ownership at Staff SDE scope here.” |
