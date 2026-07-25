# 02 — What They Will Ask From YOUR Resume

Senior Staff interviewer at Zscaler will skim your PDF in 60 seconds.  
Here is what they will **highlight** and the **predicted questions** — prepare these before generic trivia.

---

## Resume lines that trigger probes

### 1. “Founding Engineer” + “owned … end to end”
**They think:** Is this Staff or just sole developer on a small app?  
**They ask:**
- What decisions were yours alone?
- Who reviewed your designs?
- What would break if you left tomorrow?

**Your prep:** Name 3 irrevocable decisions (RLS tenancy model, Rust migration of locking path, jobs off request path) + how you documented contracts for others.

---

### 2. “Migrated … Node.js to Rust … P99 800ms → 120ms”
**They think:** Prove it — methodology, not slogan.  
**They ask:**
- How did you measure P99?
- What was the bottleneck — GC, locks, DB, event loop?
- Why not optimize Node first?
- Any regressions after migrate?

**Your prep:** Load test setup → flame/trace → race on concurrent reserve → Rust + transactions → before/after histogram → canary/rollback thought.

---

### 3. “Eliminated race conditions” / Inventory locking
**They think:** Perfect bridge to Rust `Send`/`Sync` and DB transactions.  
**They ask:**
- What was the race (lost update? double allocate?)?
- SELECT FOR UPDATE vs optimistic version vs Redis lock?
- What happens under retry?

**Your prep:** One crisp race story + why DB transaction was source of truth.

---

### 4. “PostgreSQL RLS … RBAC … API authorization”
**They think:** Multi-tenant — Zscaler’s favorite.  
**They ask:**
- RLS alone enough?
- Cross-tenant IDOR example you prevented?
- Fail open or fail closed when authz service/config wrong?

**Your prep:** Defense in depth: JWT/session → API authz → RLS → audit. Default deny.

---

### 5. “Redis queues … 5K+ daily jobs … decoupled from API”
**They think:** Backpressure / reliability.  
**They ask:**
- At-least-once? Idempotency?
- Poison message?
- What if Redis is down — API still up?

**Your prep:** Job idemptency keys, DLQ idea, degrade mode for non-critical jobs.

---

### 6. “40+ REST APIs … idempotent writes … RFC 7807”
**They think:** API taste / integration safety.  
**They ask:**
- Idempotency key design?
- Versioning strategy?
- Breaking change process?

---

### 7. “AWS EKS … OpenTelemetry … deploy 45→8 min”
**They think:** Operability for Staff.  
**They ask:**
- What traces did you add on the hot path?
- How do you catch p99 regressions before full rollout?

---

### 8. “Google Cloud Agentic AI Hackathon Finalist”
**They think:** Interesting, maybe off-topic.  
**They ask (maybe):** How does this relate to Zscaler?  
**Your prep:** One sentence — agents need tool authz and audit; then pivot back to systems/security. Do **not** deepen unless they insist.

---

### 9. Education: B.A. Psychology & Economics
**They may ask:** Non-traditional path?  
**Your answer (30 sec):**  
> “I built production systems for years as a consultant, then as founding backend. My evaluation should be the systems I shipped — tenancy, Rust concurrency, production ops — not the degree title.”

---

## Predicted 60-minute flow mapped to YOU

| Min | Their question (from FINAL book) | They will connect to your resume |
|-----|----------------------------------|----------------------------------|
| 3–12 | Ownership story | CodeApto WMS / Rust migration |
| 12–27 | Rust Send/Sync, async, unsafe, p99 | Inventory locking + Tokio |
| 27–42 | Proxy design | They test if you can **generalize** beyond WMS |
| 42–50 | TLS, threat model, appsec | Your RLS/RBAC must expand to full threat model |
| 50–55 | Cross-team + blocked ship | Founding influence + “I delayed a bad release” story |

---

## Interviewer private hypothesis (what you must disprove)

> “Strong founding Senior. Maybe not Staff. Scale small. Security product experience missing.”

**Disprove by:**
1. Structured design on Q6 (control/data plane, fail-closed, tenancy).  
2. Security answers that go beyond RLS.  
3. Leadership story with **adoption + metric** (API contracts, migration playbook, deploy time).  
4. Honest scale + clear 10×/100× scaling plan.

---

## Cheat: phrases that raise your level mid-answer

Use naturally (not as buzzword salad):

- “The invariant we protect is…”
- “Blast radius if this fails…”
- “Default deny / fail closed because…”
- “On the hot path we never…”
- “I’d separate control plane from data plane by…”
- “Idempotency key so at-least-once is safe…”
- “Tenant_id is on every log and authz check…”
- “Rollback plan before ship…”
