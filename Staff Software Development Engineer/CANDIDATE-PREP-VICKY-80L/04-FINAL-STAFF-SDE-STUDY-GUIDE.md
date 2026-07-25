# ZSCALER STAFF SDE — FINAL STUDY DOCUMENT (ONE FILE)
# For: Vicky Kumar | Role: Staff Software Development Engineer | Goal: ~₹80L

**How to use this file:** Read end to end once. Then re-read any weak section. This is the only document you need before the 60-minute Staff interview.

**Assumptions:** You already have ~6 years ownership experience. Do not waste interview time proving “I take ownership.” Prove **Staff depth**: systems, Rust, security, failure modes, and clear trade-offs.

**Interview shape (60 min):** Ownership story → Rust → System design → Security/networking → Leadership → Your questions.

---

# PART 0 — HOW TO ANSWER ANY QUESTION (USE EVERY TIME)

## The Staff answer formula (30–90 seconds per probe)

1. **Clarify** what they asked (1 line).  
2. **Answer directly** first (the conclusion).  
3. **Explain why** (trade-off / invariant).  
4. **Give a concrete example** from your work (WMS / Rust / tenancy).  
5. **Name failure mode** + what you’d measure.  
6. **Stop.** Let them ask the next probe.

Bad: long story with no point.  
Good: point → reason → example → failure → metric.

## Magic phrases (simple English, Staff signal)

- “The invariant we protect is…”
- “On the hot path we never…”
- “Blast radius if this fails is…”
- “Trade-off is X vs Y; I chose X because…”
- “Default deny when context is missing.”
- “Control plane vs data plane…”
- “I’d measure p99, error rate, and queue depth.”
- “Rollback first if customers are hurt, then root cause.”

## If you don’t know

Say:  
> “I don’t want to invent details. Here is the framework I’d use: A, B, C. In week one I’d verify with docs/metrics and pair with a domain expert.”

That is Staff honesty. Fake depth fails.

## Your resume numbers (only use real ones)

- Founding backend engineer, multi-tenant WMS  
- Rust/Axum/Tokio; Node → Rust on locking path  
- P99 **800ms → 120ms**  
- 5+ facilities, **10K+** daily inventory transactions  
- **5K+** daily Redis jobs  
- **40+** versioned REST APIs, idempotent writes, RFC 7807  
- RLS + RBAC + API authz  
- EKS, Terraform, CI/CD **45 min → 8 min**  
- OpenTelemetry; diagnosis hours → minutes  

Scale honesty line:  
> “Absolute QPS is lower than Zscaler edge. The hard problem I owned was correctness under concurrency + multi-tenant isolation + p99. Here’s how I’d scale 100×…”

---

# PART 1 — OPENING + OWNERSHIP (THEY START HERE)

## Q1. Tell me about yourself / what do you do?

**Answer (45–60 sec):**  
> “I’m a backend and distributed systems engineer with about six years building and owning production systems. For the last few years I’ve been founding backend engineer on a multi-tenant warehouse platform. I own the Rust/Axum services, PostgreSQL data model and tenant isolation, Redis async jobs, APIs, and EKS production path. I migrated our inventory-locking hot path from Node to Rust and cut P99 from about 800ms to 120ms. I’m interviewing for Staff SDE at Zscaler to apply that same ownership — invariants, isolation, performance, operability — at cloud security scale.”

---

## Q2. Walk me through the most complex system you owned.

**Answer (3 minutes — practice this until smooth):**  
> “I own the multi-tenant WMS backend.  
>  
> **Problem:** Multiple facilities need correct stock under concurrent warehouse work. Wrong allocation breaks shipping and money. So the invariant is: stock mutations are correct per tenant under concurrency.  
>  
> **What I own:** APIs, schema, RLS/RBAC/authz, Redis jobs, WebSockets for ops views, EKS releases, tracing.  
>  
> **Decisions:**  
> 1) PostgreSQL is source of truth for inventory; Redis is queue/cache, not authoritative stock.  
> 2) Tenant isolation with RLS + API authz + RBAC — defense in depth.  
> 3) Heavy work off the request path — 5K+ jobs/day on Redis queues.  
> 4) Rust on the locking hot path after Node showed races and high P99 under load.  
>  
> **Scale:** 5+ facilities, 10K+ daily inventory transactions, 40+ versioned APIs.  
>  
> **Failure:** Concurrent reserve could double-allocate; retries could duplicate side effects without idempotency.  
>  
> **Fix:** Transactional reserve in Rust, idempotency keys, OpenTelemetry, conflict/p99 dashboards. Result: P99 800→120; faster incident diagnosis.”

**Probes — short answers:**

| Probe | Answer |
|-------|--------|
| Your decision vs team? | Tenancy model, Rust migration, jobs-off-request-path were my calls. Frontend/ops shaped API needs; I wrote contracts. |
| How measured P99? | Latency histograms around reserve/commit under concurrent load tests + production traces. |
| Why not only optimize Node? | Race + runtime model; Rust gave clearer concurrency + latency headroom after we measured. |

---

## Q3. Tell me a production incident you led.

**Template (fill with your real detail, keep structure):**  
> “**Detect:** rising conflicts/latency + ops reports.  
> **Mitigate:** protect customers — tighten/flag risky path, pause noncritical jobs if needed.  
> **Root cause:** read-check-write race on stock.  
> **Fix:** transactional locking path in Rust + parallel tests.  
> **Prevent:** load test in CI, dashboards on conflict rate and p99, clearer idempotency.  
> **Learning:** correctness invariants beat micro-optimizations.”

---

# PART 2 — RUST (DEEP, SIMPLE ENGLISH)

## Q4. Explain ownership to a C++ engineer.

**Answer:**  
> “In Rust, every value has one owner. When the owner goes out of scope, Drop runs — like RAII. Moves are default, not deep copies. You can borrow: many shared `&T` **or** one `&mut T`, not both at once. That’s the borrow checker. We avoid data races at compile time for safe Rust. `Clone` copies explicitly; `Arc` shares ownership across threads; `Rc` is single-threaded only.”

---

## Q5. What are Send and Sync? Bug example?

**Answer:**  
> “`Send` = can move this value to another thread.  
> `Sync` = can share `&T` across threads safely. If `T` is Sync, `&T` is Send.  
>  
> Classic bug: using `Rc`/`RefCell` in async code that must be `Send` for `tokio::spawn`. `Rc` is not Send. Fix: `Arc`, and for mutation `Mutex`/`RwLock`/atomics — and don’t hold locks across `.await` on hot paths.  
>  
> In our inventory work, the real production bug class was shared mutable intent without a clear concurrency story. We made DB transactions the authority and used Arc carefully in-process.”

---

## Q6. Why is CPU work on the async runtime bad?

**Answer:**  
> “Async runtimes assume tasks yield at await points. Heavy CPU (crypto, compression, huge parse) blocks the worker thread. Other requests on that thread get bad latency. Fix: `spawn_blocking` or a dedicated thread pool. Keep async for IO.”

---

## Q7. What is cancellation safety? Example?

**Answer:**  
> “Timeouts and `select!` cancel by dropping a future. If that future already did half a side effect, state can be wrong. Example: started a write, cancelled before commit/ack. Design: idempotent operations, cancel-safe APIs, clear state machines. Our jobs use idempotency keys so retry/cancel doesn’t corrupt stock.”

---

## Q8. When is unsafe OK? What about unwrap?

**Answer:**  
> “Unsafe: FFI or building a small safe wrapper with documented invariants — after review. Not for random speedups. Prefer trusted crates.  
>  
> Unwrap on request path: no for IO/DB/parse. Return typed errors (we use predictable API errors / RFC 7807). Panic = DoS risk in multi-tenant servers. `expect` only for true ‘bug if false’ cases, rare.”

---

## Q9. Pin / why Pin in async? (if asked)

**Answer:**  
> “Async futures can be self-referential. If you move them in memory after polling starts, internal references break. `Pin` means ‘this won’t move.’ Normal app code using async/await rarely writes unsafe Pin logic; library authors might.”

---

## Q10. Arc Mutex vs channels?

**Answer:**  
> “Shared state with `Arc<Mutex<T>>` is fine for simple coordination. If many tasks contend, locks hurt p99. Prefer message passing (channels), sharding, or move ownership of work. On hot paths, measure contention.”

---

## Q11. How did you debug / improve p99 in Rust?

**Answer:**  
> “Measure first: histograms, traces, DB time, CPU. Find if blocking, lock waits, clones, or SQL. For us, concurrent locking path was the hotspot. Migrated to Rust with transactional guarantees, load-tested, watched p99 800→120. If p99 spikes after deploy: check deploy marker, rollback if needed, then flamegraph/traces.”

---

## Q12. Error handling philosophy?

**Answer:**  
> “Domain errors typed at boundaries. Don’t swallow. Add context. Map to correct HTTP status without leaking internals. Separate retryable vs fatal. Logs with request/tenant ids — never secrets.”

---

# PART 3 — SYSTEM DESIGN (ZSCALER-STYLE)

## How to do any design question (method)

1. Requirements (functional + non-functional: latency, tenancy, audit, HA).  
2. Numbers (rough).  
3. High-level diagram.  
4. Deep dive 1–2 parts.  
5. Failure modes.  
6. Security / multi-tenant.  
7. Observability.  

Always separate **control plane** (config/policy/admin) and **data plane** (request/session path).

---

## Q13. Design a multi-tenant cloud forward proxy (MAIN DESIGN)

**Clarify first:**  
> “Enterprise users, TLS, allow/deny/inspect policy, audit logs, millions of sessions, multi-tenant. I’ll assume edge POPs globally. Correct me if wrong.”

**Architecture (say while drawing):**

```
Client → Edge / Enforcer (DATA PLANE)
           - TLS
           - identity context
           - local policy evaluate (cached bundle)
           - upstream connect
           - emit audit event (async)

Control plane:
  - policy authoring
  - identity / config
  - sign + version policy bundles
  - rollout / rollback
  - admin APIs (strict authz)
```

**Key design points (must say):**

1. **Hot path:** local compiled policy cache — do not call control plane every request.  
2. **Policy bundles:** versioned, signed, monotonic version, staged rollout, rollback.  
3. **Logs:** async off hot path; bounded queue; tenant_id on every event.  
4. **Tenancy:** auth context + authz on admin APIs + log access control + per-tenant limits.  
5. **Scale:** many edges; connection limits; pool upstreams; TLS resumption for CPU.  

**Bridge from your work:**  
> “Same lesson as WMS: don’t put slow control work on the request path. We moved 5K+ jobs off API. Here, policy compile/distribution is control plane; enforcement is data plane.”

---

## Q14. Policy service down 5 minutes — fail open or fail closed?

**Answer:**  
> “For a security product, I bias **fail closed** for sensitive traffic — don’t silently allow everything. For availability, keep **last-known-good cached policy** with TTL and alert loudly while stale. Silent allow-all needs explicit product decision and audit. Missing authz context in our APIs is default deny — same philosophy.”

---

## Q15. How guarantee Tenant A never sees Tenant B?

**Answer:**  
> “Tenant ID in every request context, policy object, and log. Authorize every control-plane read/write by tenant (no IDOR). Separate quotas. Careful metrics labels. Encrypt/access-control log queries. Defense in depth like our API authz + RLS + RBAC.”

---

## Q16. Where do logs go so p99 doesn’t die?

**Answer:**  
> “Enqueue asynchronously with a **bounded** buffer. Batch export. If buffer full: explicit policy — for security logs usually alert and shed lower-priority detail, not silent loss without signal. Never sync write to cold storage on request path.”

---

## Q17. Design policy evaluation with p99 &lt; 10–20ms

**Answer:**  
> “Compile policies to a fast structure at edge. Cache identity group attributes with TTL. Pin policy version in the decision log for audit. Canary new policies in shadow mode. Multi-tenant compiled state isolated. Hot path = memory lookup, not remote DB.”

---

## Q18. Global config distribution

**Answer:**  
> “Content-addressed signed artifacts. Edges pull/CDN. Ring rollout. Auto rollback on error budget burn. Idempotent apply. Distinguish ‘downloaded’ vs ‘active’.”

---

## Q19. Rate limiting design

**Answer:**  
> “Token bucket or sliding window per tenant/key. In-memory at edge for speed; Redis/global store if need cluster consistency — accept approximate limits for performance. Bound memory; evict inactive keys or attackers flood tenant-id space. Fairness across tenants.”

---

## Q20. Zero Trust / ZTNA vs VPN (conceptual)

**Answer:**  
> “VPN often gives broad network access → lateral movement risk. ZTNA: identity + device posture, per-app access, often outbound connector from customer network, continuous evaluation, strong audit. Least privilege to apps, not flat network.”

---

# PART 4 — DISTRIBUTED SYSTEMS

## Q21. CAP in simple words — policy worldwide?

**Answer:**  
> “During partition you can’t have perfect always-on strong consistency everywhere and perfect availability. For policy, we usually want versioned bundles and **bounded staleness**, never apply older version over newer, signed configs. Prefer correctness of enforcement mode over split-brain free-for-all.”

---

## Q22. Exactly-once?

**Answer:**  
> “End-to-end exactly-once is rare. Practically: **at-least-once delivery + idempotent consumers** (idempotency keys, dedup store). We use idempotent writes/jobs so retries are safe.”

---

## Q23. Retries done right?

**Answer:**  
> “Exponential backoff + jitter. Cap attempts. Honor Retry-After. Idempotency keys for non-GET. Circuit breaker. Avoid retry storms that amplify outages.”

---

## Q24. Queue lag / consumers behind?

**Answer:**  
> “Measure lag. Scale consumers. Check hot keys/partitions. Poison → DLQ. Backpressure producers. Security logs: don’t silent-drop forever without alert — add capacity.”

---

## Q25. Leader election pitfalls?

**Answer:**  
> “Leases can lie under GC/network blips. Use fencing tokens. Prefer designs that don’t need a single global leader when sharding works.”

---

## Q26. Clocks?

**Answer:**  
> “Wall clocks skew. Don’t order critical events only by local time. Use versions/epochs. Be careful with token expiry and leases.”

---

## Q27. Hot tenant / noisy neighbor?

**Answer:**  
> “Per-tenant quotas, fair queuing, isolation, dedicated capacity if needed, rate limits, detect and shed fairly.”

---

## Q28. Outbox / dual write problem?

**Answer:**  
> “Don’t write DB and message queue as two independent commits without a story. Outbox pattern: store event in DB transaction, publisher drains outbox. Keeps consistency.”

---

# PART 5 — SECURITY + NETWORKING (MUST PASS FOR ZSCALER)

## Q29. TLS 1.3 high level + CPU cost + resumption

**Answer:**  
> “Client and server exchange hellos and key shares, derive keys, verify certificate, Finished, then encrypted data (AEAD like AES-GCM).  
>  
> **CPU:** handshake and cert verify cost more than encrypting bulk bytes.  
>  
> **Resumption:** session tickets avoid full handshakes → less CPU and latency at proxy scale. Rotate ticket keys. 0-RTT has replay risks — use carefully. Monitor cert expiry.”

---

## Q30. TCP timeouts / idle / keepalive

**Answer:**  
> “Idle timeouts free resources. Keepalives detect dead peers. NATs may drop silent connections. Proxy must set coordinated timeouts or you leak connections / cause surprise drops.”

---

## Q31. HTTP/2 and HTTP/3 (QUIC) issues for proxies

**Answer:**  
> “HTTP/2 multiplexes streams on one TCP connection — one packet loss blocks many streams (HOL). Many streams can be a DoS vector — limit. HTTP/3/QUIC on UDP avoids that HOL but has middlebox/ops challenges.”

---

## Q32. Why DNS matters for security products

**Answer:**  
> “DNS decides where you go. Attacks: tunneling, spoofing, malicious domains. Policy may need DNS visibility. Caching and resolver choice affect security and privacy (DoH/DoT).”

---

## Q33. Threat model a multi-tenant control-plane API

**Answer (list confidently):**  
1. Cross-tenant access / IDOR → authz every object + tests  
2. Stolen session/token → short TTL, rotate, revoke  
3. Admin abuse → least privilege, MFA, audit  
4. SSRF from webhooks → allowlist, block link-local/metadata  
5. Injection / huge payloads → validate, size limits  
6. Dependency compromise → pin, audit, least privilege CI  
7. PII in logs → redact, access control  
8. DoS → rate limit, auth expensive APIs  

> “In our system we already practice API authz + RLS. At Zscaler I’d add stronger admin audit and edge abuse controls.”

---

## Q34. Rust memory safety ≠ application security

**Answer:**  
> “Rust reduces memory corruption. You can still ship broken authz, fail-open config, secret leaks, panic DoS, SSRF, logic bugs, bad crypto use, CVE in deps. Memory safety is necessary, not enough — especially in a security company.”

---

## Q35. Secrets in K8s / services

**Answer:**  
> “Prefer secret manager / short-lived creds over long-lived env secrets. Rotate. Don’t log secrets. mTLS between services where needed. Least privilege IAM.”

---

## Q36. mTLS pitfalls

**Answer:**  
> “Cert proves identity; you still need authorization. Rotation and clock skew hurt. Short TTL helps when revocation is hard.”

---

## Q37. Should you write your own crypto?

**Answer:**  
> “Almost never. Use vetted libraries. High misuse risk.”

---

## Q38. Device posture in access decisions — failures?

**Answer:**  
> “Stale posture, false positives blocking work, agent compromise, privacy, cache TTL, clear fallback policy.”

---

# PART 6 — APIs, DATA, OPS (YOUR STRENGTH — GO DEEPER)

## Q39. Idempotent APIs — how?

**Answer:**  
> “Client sends Idempotency-Key. Server stores key → result. Retries return same result. Critical for payments-like and inventory mutations. We use idempotent writes so network retries don’t double-apply.”

---

## Q40. RFC 7807 errors — why care?

**Answer:**  
> “Standard problem JSON: type, title, status, detail. Clients handle errors predictably. We documented 40+ APIs with clear error contracts so frontend/partners don’t guess.”

---

## Q41. Cursor pagination vs offset?

**Answer:**  
> “Offset drifts under inserts and gets slow on large offsets. Cursor/keyset is stable and cheaper for big lists.”

---

## Q42. RLS — is it enough alone?

**Answer:**  
> “No. Defense in depth: authenticate → authorize in API → RLS as safety net → audit. Mis-set DB role can bypass if you rely on RLS only. We use both API authz and RLS.”

---

## Q43. Transactions isolation (simple)

**Answer:**  
> “For inventory reserve, we need to avoid lost updates. Use transactions with proper locking (`SELECT FOR UPDATE`) or optimistic version checks. Keep transactions short.”

---

## Q44. Redis: cache vs queue vs source of truth?

**Answer:**  
> “Cache: OK to lose with fallback to DB. Queue: at-least-once, need idempotent consumers. Source of truth for stock: **Postgres**, not Redis — we learned that invariant early.”

---

## Q45. Observability you actually use

**Answer:**  
> “Structured logs + OpenTelemetry traces + metrics (latency, errors, queue depth, conflict rate). Goal: cut MTTR. We cut diagnosis from hours to minutes.”

---

## Q46. CI/CD 45→8 min — why Staff cares

**Answer:**  
> “Faster safe feedback and rollback. Smaller batches. But speed without tests is danger. Pair fast pipeline with canaries and critical path tests.”

---

## Q47. Multi-tenant SaaS data modeling tips

**Answer:**  
> “tenant_id everywhere. Indexes include tenant. Unique constraints scoped per tenant. Never trust client-supplied tenant without authz.”

---

# PART 7 — STAFF LEADERSHIP (EVEN IF SMALL COMPANY)

You have 6 years ownership. For Staff they want **leverage**, not only hero coding.

## Q48. Cross-team initiative (your true story, Staff framing)

**Answer:**  
> “As founding backend I didn’t have five backend teams, but I drove cross-role change: frontend, ops, partners.  
>  
> I created one versioned REST contract — 40+ endpoints, idempotent writes, RFC 7807 — and made it the paved road. I rejected breaking changes without versioning.  
>  
> Second: measured Node→Rust locking migration with rollback thinking so others could extend safely.  
>  
> Metrics: P99 800→120; fewer race incidents; deploy 45→8 min so the whole product shipped faster.  
>  
> At Zscaler I’d do the same at larger scale: written designs, adoption path, metrics.”

---

## Q49. Time you blocked a dangerous ship

**Answer:**  
> “There was pressure to weaken transactional checks / isolation for speed. I blocked it. Risk: wrong stock or cross-tenant bleed. I showed a failing concurrent test, proposed flag + safe path, and delivered speed via Rust locking instead. I won’t bypass authz/tenancy for a date — that matters at Zscaler.”

---

## Q50. Disagree with someone on design

**Answer:**  
> “I bring data: latency, failure mode, blast radius. Propose alternatives. If still disagree, escalate on risk, then disagree-and-commit if decision is made and risk accepted explicitly.”

---

## Q51. 90-day plan if you inherit a messy critical Rust service

**Answer:**  
> “Days 1–30: SLOs, runbooks, on-call shadow, map unsafe/authz/data-loss risks.  
> 30–60: characterization tests on golden paths; fix top reliability risks.  
> 60–90: incremental refactors; no big-bang rewrite without proof; share ownership docs.”

---

## Q52. Build vs buy vs open source

**Answer:**  
> “Is it core competency? Latency/security/support/license/ops cost? CVE response? Exit plan? On request path be extra conservative.”

---

## Q53. How do you mentor / raise bar?

**Answer:**  
> “Concrete review standards: tenancy checks, idempotency, tests for races, no unwrap on IO. Design docs for risky changes. Pair on production debugging.”

---

## Q54. Why Zscaler? Why Staff SDE?

**Answer:**  
> “I want to apply multi-tenant systems and Rust production discipline to cloud security at global scale. Staff SDE means owning a domain’s invariants — policy, isolation, reliability — and multiplying other engineers, not only closing tickets.”

---

## Q55. Why should we hire you at Staff / ₹80L band?

**Answer:**  
> “Because I already own end-to-end production systems with tenancy, concurrency correctness, measurable performance wins, and operability. I communicate trade-offs and protect safety invariants under pressure. I’m ready to grow that ownership to Zscaler’s data/control plane scale. Level me on evidence from this loop; if Staff scope is confirmed, the Staff band including around ₹80L CTC is the package I’m targeting.”

---

# PART 8 — BEHAVIORAL SHORT ANSWERS

## Q56. Strength / weakness

**Strength:** ownership of backend invariants; measure then change.  
**Weakness (honest + fix):**  
> “I can go deep on implementation before aligning stakeholders. I fixed that by writing short design notes early and confirming constraints first — which I’ll do here on cross-team work.”

## Q57. Conflict with peer

> “Data + customer impact + experiment. Keep respect. Escalate only on risk.”

## Q58. Missed deadline

> “Cut scope, protect correctness, communicate early, ship safe slice.”

## Q59. Feedback that changed you

> Pick real feedback (e.g. document APIs more). Show lasting change (RFC 7807 contracts).

## Q60. Ethical / security shortcut pressure

> “I refuse silent security bypass. Use exception process with audit if business accepts risk.”

---

# PART 9 — QUESTIONS YOU ASK THEM (END)

Pick 2–3:

1. What domain would this Staff SDE own in first two quarters?  
2. What does great look like at month six?  
3. Biggest pain: reliability, policy correctness, performance, or leverage?  
4. How do you decide fail-open vs fail-closed with product?  
5. How much Rust vs other languages on the critical path?

---

# PART 10 — 60-MINUTE CHEAT TIMELINE (WHAT TO SAY WHEN)

| Min | Focus | Your job |
|-----|-------|----------|
| 0–3 | Intro | 45-sec pitch from Q1 |
| 3–12 | Ownership | Q2 story + incident |
| 12–27 | Rust | Q4–Q12 ideas; stay concrete |
| 27–42 | Design | Q13–Q16; draw; fail-closed; tenancy |
| 42–50 | Security | Q29, Q33, Q34 must nail |
| 50–55 | Leadership | Q48, Q49 |
| 55–60 | You ask | Part 9 |

---

# PART 11 — FINAL PASS CHECKLIST (NIGHT BEFORE)

- [ ] Q2 ownership story smooth in under 3:30  
- [ ] Can explain Send/Sync + async CPU + no unwrap  
- [ ] Can draw proxy control vs data plane from memory  
- [ ] Fail open/closed answer in 4 sentences  
- [ ] Threat model: 8 risks without notes  
- [ ] Rust ≠ appsec in 5 sentences  
- [ ] Leadership: API contracts + Rust migration + blocked unsafe ship  
- [ ] Honest scale line ready  
- [ ] Sleep  

---

# PART 12 — ONE-PAGE MEMORY CARD

**Invariant:** correct + isolated + observable.  
**Hot path:** no slow control-plane dependency without cache.  
**Tenancy:** authz + RLS mindset + tenant on logs.  
**Async:** don’t block runtime; idempotent jobs.  
**Security product:** fail closed + last-known-good + audit.  
**Staff:** written contracts, measured change, multiply others.  
**Numbers:** 800→120 p99 · 10K+ tx/day · 5K+ jobs · 40+ APIs · 45→8 deploy.  

---

**End of document.**  
Read this file fully. Then practice Part 1 Q2, Part 3 Q13, Part 5 Q29/Q33/Q34, Part 7 Q48/Q49 out loud. That combination clears most of the Staff hour.
