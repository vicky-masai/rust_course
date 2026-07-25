# 02 — FULL QUESTIONS & ANSWERS
# Role: Staff Software Development Engineer at Zscaler

**Interviewer only.** Do not share this file with the candidate.  
**Role under evaluation:** Staff Software Development Engineer (Staff SDE) — Zscaler  
**Language focus:** Rust · **Comp bar:** ~₹80 Lakh CTC · **Time:** 60 minutes

For each question:
- **Ask** = exact wording
- **Expected answer (Staff SDE / ₹80L)** = hireable for this Zscaler role
- **Acceptable (Senior SDE)** = solid developer, not Staff title
- **Weak / Reject signals** = fail for this role
- **Score tip** = how to mark 1–4

---

# SECTION A — OWNERSHIP (3–12 min) — 1 question

## Q1. Most complex production system you owned

### Ask
> As a **Staff Software Development Engineer**, walk me through the most complex production system **you** personally owned in the last 2–3 years. Cover: problem, your design decisions, scale numbers, what broke in production, and what you changed after.

### Expected answer (Staff SDE @ Zscaler / ₹80L) — Score 4 or strong 3
Candidate gives a **clear “I” narrative** with:

1. **Problem** in business/reliability/security terms (not only “we needed a service”).
2. **Architecture** they chose and **why** (trade-offs: latency vs consistency, cost, complexity).
3. **Numbers**: e.g. QPS, p50/p99, data volume, tenants, regions, error budget.
4. **Failure**: a real incident or near-miss they owned — detection, mitigation, root cause, prevention.
5. **Aftermath**: tests, dashboards, runbooks, design change, or process change.
6. **Scope**: impact beyond their squad (other teams adopted, platform reuse, reduced incidents).

Example shape (any domain OK — Rust proxy, billing, policy, messaging, etc.):
> “I owned the request-path rate limiter / policy cache service. We served ~X RPS with p99 &lt; Y ms. I chose versioned policy bundles + edge cache because sync calls to control plane blew our latency budget. During an incident, stale cache + bad rollback caused denials; I led fail-safe to last-known-good with TTL, added bundle signatures, and cut related sevs by Z%.”

### Acceptable (Senior SDE) — Score 2
- Strong feature ownership inside one team.
- Some metrics, weak on cross-team leverage or weak incident story.
- Good executor, not domain owner — **not enough for Staff SDE title at Zscaler**.

### Weak / Reject — Score 1
- Only “we” — cannot separate personal contribution.
- No numbers, no failures, no trade-offs.
- Resume claims Staff; story is ticket-closing.

### Score tip
- **4** = owned domain + metrics + incident + lasting fix + influence  
- **3** = owned system + metrics + solid incident  
- **2** = owned feature end-to-end  
- **1** = vague / not credible

---

# SECTION B — RUST (12–27 min) — 4 questions

## Q2. Ownership, borrowing, and Send/Sync

### Ask
> Explain Rust ownership to a strong C++ engineer in 2 minutes. Then: what are `Send` and `Sync`? Give a real bug you’ve seen (or would expect) when someone uses the wrong type across threads or `.await` points.

### Expected answer (Staff SDE) — Score 3–4
**Ownership**
- Each value has one owner; move by default; borrow checker: either many shared `&T` **or** one `&mut T`, not both.
- Drop/RAII = deterministic cleanup (contrast GC / manual free).
- `Clone` / `Arc` / `Rc` used intentionally, not as default escape hatch.

**Send / Sync**
- **`Send`**: value can be transferred to another thread.
- **`Sync`**: `&T` can be shared across threads safely (`T: Sync` ⇒ `&T: Send`).
- Classic bug: `Rc` / `RefCell` across `tokio::spawn` or holding non-`Send` data across `.await` → future is `!Send` → won’t compile, or if forced wrongly, UB/risk.
- Fix: `Arc` + `Mutex`/`RwLock`/atomics as appropriate; don’t hold locks across `.await` without care.

### Acceptable (Senior SDE) — Score 2
Correct ownership story; Send/Sync roughly right; weak real bug example.

### Weak — Score 1
“Rust has no pointers”; cannot explain Send/Sync; only framework CRUD Rust.

### Score tip
Must get Send/Sync roughly right for ≥3. Wrong Send/Sync → max 2 for whole Rust section unless later Qs save them.

---

## Q3. Async runtime pitfall

### Ask
> In a Tokio (or async) Rust service, what goes wrong if you run heavy CPU work (crypto, compression, huge JSON parse) directly inside an async task? How do you fix it? Also name one **cancellation** hazard with `select!` or timeouts.

### Expected answer (Staff SDE) — Score 3–4
**CPU on async runtime**
- Async executors assume tasks **yield**; CPU-bound work **starves** the worker thread → latency spikes for unrelated requests.
- Fix: `spawn_blocking`, dedicated thread pool, or separate process for heavy work; measure before/after.

**Cancellation**
- `tokio::select!` / timeout **drops** the losing future mid-operation.
- Hazard example: read partial data into a buffer, then cancel before commit → inconsistent state; or non-cancel-safe APIs that leave half-applied side effects.
- Mitigations: cancel-safe primitives, make operations idempotent, use structured concurrency, careful state machines.

### Acceptable (Senior SDE) — Score 2
Knows spawn_blocking; vague on cancellation.

### Weak — Score 1
“Async is just threads”; no idea about runtime starvation.

---

## Q4. Unsafe and production discipline

### Ask
> When is `unsafe` justified in production Rust? How do you contain it? What is your rule for `unwrap`/`expect` on a request-serving path?

### Expected answer (Staff SDE) — Score 3–4
**Unsafe justified when**
- FFI to C libraries.
- Building a **safe abstraction** with documented invariants (e.g. custom container) after measurement proves need.
- Rare performance cases with **proven** invariants.

**Containment**
- Minimize surface area; isolate in module; document **safety invariants**; expert review; fuzzing / Miri where applicable; prefer vetted crates over home-grown unsafe.

**unwrap**
- **No unwrap** on request path for network/IO/parse errors.
- `expect` only for truly invariant “impossible if bug-free” cases with a message — prefer typed errors (`thiserror`/`anyhow` at boundaries).
- Panics on request path = DoS / process crash risk.

### Weak — Score 1
“Unsafe for speed” with no invariants; unwrap everywhere “because it can’t fail.”

---

## Q5. Production debugging (p99 regression)

### Ask
> After a Rust deploy, p99 latency becomes 3× worse. Walk me through your first 30–60 minutes. What tools and signals do you look at?

### Expected answer (Staff SDE) — Score 3–4
Structured incident mindset:

1. **Confirm blast radius**: region, tenant, endpoint, deploy marker, error rate.
2. **Metrics**: latency histogram, CPU, memory, allocation rate, queue depth, GC N/A but allocator; lock wait; downstream latency; timeout/retry rates.
3. **Profiles**: flamegraph / `perf` / tokio-console — find hot locks, blocking, unexpected clones, TLS handshake spikes.
4. **Common Rust/async causes**: lock held across await, blocking on runtime, unbounded queue, DNS, connection pool exhaustion, accidental large `Clone`, logging on hot path.
5. **Mitigation**: rollback / feature flag first if customer impact; then root cause.
6. **Prevent**: canary, better SLOs, regression tests.

### Acceptable (Senior SDE) — Score 2
Mentions metrics + rollback; thin on Rust-specific causes.

### Weak — Score 1
Only “add more logs” / “restart pods” with no structure.

### Section B overall score
Average Q2–Q5 (or judgment call). One fatal miss on Send/Sync + async → cap section at 2.

---

# SECTION C — SYSTEM DESIGN (27–42 min) — 1 design

## Q6. Multi-tenant cloud forward proxy (Zscaler-style Staff SDE design)

### Ask
> You are interviewing for **Staff Software Development Engineer at Zscaler**. Design a multi-tenant cloud **forward proxy** for enterprise customers — a data-plane system a Staff SDE might own:
> - Handles TLS (terminate or CONNECT — you choose and justify)
> - Applies URL / category style **allow / deny / inspect** policy
> - Writes **audit logs**
> - Targets **millions of concurrent sessions** globally  
> Draw **control plane vs data plane**. I will add constraints.

### Then ask (required probes)
1. **Policy service down 5 minutes — fail open or fail closed?**
2. **How do you guarantee Tenant A never sees Tenant B data/logs/policy?**
3. (If time) **Where does logging sit so it doesn’t destroy p99?**

---

### Expected answer (Staff SDE @ Zscaler / ₹80L) — Score 3–4

#### Clarifying questions (they should ask some)
- Inline proxy vs PAC vs tunnel; identity source (IdP); latency budget; retention; compliance regions; HA; client types.

#### High-level architecture
```
Clients → Edge POP / Enforcer (DATA PLANE)
              ↓ policy cache (compiled bundle)
         Allow / Deny / Inspect → Upstream Internet / Apps
              ↓ async
         Log pipeline → storage / SIEM export

Control plane: identity, policy authoring, config distribution,
               cert/key management, admin APIs, rollout rings
```

#### Data plane (hot path)
- Connection accept, TLS, identity context, **local policy evaluation** (not sync call every request if avoidable).
- Timeouts, connection limits, upstream pools.
- Minimal work on request path; avoid heavy alloc/log sync I/O.

#### Control plane
- Policy CRUD, versioning, signing, staged rollout, rollback.
- Distributes **versioned policy bundles** to edges (pull/CDN/push).
- Admin authz strongly isolated per tenant.

#### Scale
- Horizontal edge POPs; anycast/geo routing; shard by connection.
- Policy compiled to fast structure (map/trie/rules engine); high cache hit rate.
- Back-of-envelope: concurrent sessions × memory per conn; log GB/day.

#### Probe 1 — Fail open vs fail closed (must score well)
Staff answer:
- **Product/risk decision**, not only engineering preference.
- Enterprise security gateways often **fail closed** for sensitive destinations; may use **last-known-good cached policy** with TTL + loud alerts for availability.
- Never silently “allow all” without explicit product policy and audit.
- State: cached bundle version, expiry, degrade mode metrics.

#### Probe 2 — Multi-tenant isolation
- Tenant ID on every request context, policy object, log event.
- Authz checks on every control-plane API (no IDOR).
- Separate rate limits / quotas per tenant (noisy neighbor).
- Log/query access control by tenant; careful metrics cardinality (no raw user IDs as Prometheus labels).
- Optional: separate encryption keys for high-assurance tenants.

#### Probe 3 — Logging without killing p99
- Async off hot path; bounded queue; drop/sample only with explicit policy (security logs often cannot silent-drop — alert + capacity).
- Batching; backpressure to shedding low-priority work first.

### Acceptable (Senior SDE) — Score 2
Boxes and arrows OK; weak on fail-open/closed or tenancy; little scale math.

### Weak / Reject — Score 1
Monolith, sync policy every request, sync disk log on path, no tenant isolation, no failure modes.

### Score tip
- **4** = control/data split + cache/versioning + fail mode judgment + tenancy + operability  
- **3** = solid architecture + good answers to both probes  
- **2** = basic proxy design, weak probes  
- **1** = unsafe / incomplete for cloud security product  

---

# SECTION D — SECURITY + NETWORKING (42–50 min) — 3 questions

## Q7. TLS at proxy scale

### Ask
> Walk through a TLS 1.3 handshake at a high level. Where is the CPU cost? Why does **session resumption** matter for a proxy handling millions of connections?

### Expected answer (Staff SDE) — Score 3–4
- ClientHello / ServerHello with key share → shared secrets; certificate + verify; Finished; app data with AEAD (e.g. AES-GCM).
- **Handshake crypto + cert verify** cost more than bulk encryption per byte.
- At proxy scale: handshakes dominate CPU if short-lived connections.
- **Resumption / tickets**: fewer full handshakes → lower CPU and latency; must rotate ticket keys; be careful with 0-RTT replay risks if used.
- Operational: cert expiry monitoring, rotation, trust stores.

### Weak — Score 1
Cannot separate handshake vs bulk; no resumption concept.

---

## Q8. Threat model — multi-tenant control plane API

### Ask
> Threat-model our multi-tenant **admin/control-plane API**. Name the top risks and how you mitigate them.

### Expected answer (Staff SDE) — Score 3–4
At least 5 of these, with mitigations:

| Risk | Mitigation |
|------|------------|
| Cross-tenant access / IDOR | Strong authn + authz every object; tests; tenant scoping |
| Stolen session/token | Short TTL, rotation, binding, anomaly detection |
| Admin privilege abuse | Break-glass, MFA, audit logs, least privilege |
| SSRF via connectors/webhooks | Allowlists, block link-local/metadata IPs |
| Injection / malformed input | Strict validation, size limits |
| Dependency / supply chain | `cargo audit`, pinning, SBOM, least privilege CI |
| PII leakage in logs | Redaction, access-controlled log query |
| DoS | Rate limits, auth on expensive APIs, payload limits |

### Weak — Score 1
“Use HTTPS and JWT” only; no tenant or authz thinking.

---

## Q9. Rust memory safety ≠ application security

### Ask
> Rust prevents many memory bugs. What security problems can you **still** ship in a Rust service? Give examples relevant to a cloud security product.

### Expected answer (Staff SDE) — Score 3–4
Examples:
- Broken **authorization** / IDOR (logic bug)
- **Fail open** misconfiguration
- Secrets in logs; PII leakage
- **Panic** on untrusted input → DoS
- Timing side channels in custom crypto (don’t roll crypto)
- SSRF, path traversal, XSS at boundaries
- TOCTOU / race in policy apply
- Unsafe FFI bugs if `unsafe` used poorly
- Dependency CVEs

**Key line:** Memory safety helps; it does **not** replace threat modeling, authz tests, or secure design.

### Weak — Score 1
“Rust is safe so security is done.”

### Section D score
Average Q7–Q9. **If Q8 or Q9 is 1 → section max 2** (security is non-negotiable for Zscaler).

---

# SECTION E — STAFF LEADERSHIP (50–55 min) — 2 questions

## Q10. Cross-team technical initiative (Staff SDE leverage)

### Ask
> For **Staff Software Development Engineer at Zscaler**, we expect impact beyond one squad. Tell me about a technical initiative you led that required **more than one team** to change. What did you write (RFC/ADR)? Who disagreed? How did you get adoption? What metric proved it worked?

### Expected answer (Staff SDE) — Score 3–4
- Problem framed for org (reliability, security, cost, velocity).
- Written design; stakeholder map; handled dissent with data.
- Adoption path (migration, tooling, paved road) — not “I told them.”
- **Metric**: e.g. incident ↓, latency ↓, duplicate code removed, onboard time ↓.
- Personal role crystal clear.

### Acceptable (Senior SDE) — Score 2
Led a project inside one team; limited external adoption story.

### Weak — Score 1
“I finished the hardest tickets”; no influence evidence.

---

## Q11. Blocked a dangerous change

### Ask
> Tell me about a time you **stopped or delayed** a release because of security, correctness, or reliability risk. What was the risk? How did you convince people? What happened after?

### Expected answer (Staff SDE) — Score 3–4
- Concrete risk (authz hole, data leak, irreversible migration, fail-open).
- Escalated professionally with evidence; offered alternatives (flag, canary, fix).
- Did not proudly bypass process; protected customers.
- Outcome: fix shipped safely; relationship/process improved.

### Weak / Hard no — Score 1
Proud of shipping past review; or never challenged anything; or purely political sabotage story with no technical substance.

### Section E score
Average Q10–Q11. **Staff SDE ₹80L at Zscaler needs ≥3 here.**

---

# QUICK REFERENCE — ALL 11 QUESTIONS (Staff Software Development Engineer @ Zscaler)

| # | Ask (short) |
|---|-------------|
| Q1 | System you owned as Staff SDE — design, scale, incident, fix |
| Q2 | Ownership + Send/Sync + bug |
| Q3 | CPU on async runtime + cancellation |
| Q4 | When unsafe; unwrap on request path? |
| Q5 | p99 ×3 — debug first hour |
| Q6 | Design multi-tenant proxy (Zscaler-style) + fail mode + tenancy |
| Q7 | TLS 1.3 + resumption at scale |
| Q8 | Threat model multi-tenant control-plane API |
| Q9 | Rust safe ≠ appsec — examples |
| Q10 | Cross-team Staff SDE initiative — adoption + metric |
| Q11 | Blocked a dangerous ship |

---

# AFTER INTERVIEW

Fill `03-LIVE-SCORECARD.md` → decide with `04-OFFER-DECISION.md` for **Staff Software Development Engineer at Zscaler**.
