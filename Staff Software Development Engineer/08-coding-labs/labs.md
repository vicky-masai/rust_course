# Coding Labs — Rust (Staff Bar)

**Duration:** 35–50m coding + 10–15m review discussion  
**Environment:** Candidate’s laptop or shared editor; allow docs/`rustc`/clippy.  
**Rules:** No LeetCode trivia for its own sake. Prefer systems-flavored problems.

Evaluate: correctness, API taste, tests, edge cases, complexity, communication.

---

## Lab A — Concurrent rate limiter (recommended)

### Prompt

Implement an in-memory rate limiter:

```text
allow(tenant_id: &str, cost: u32) -> bool
```

- Token bucket or sliding window (candidate chooses; must explain)
- Safe under concurrent use (`Arc<…>`)
- Bounded memory (evict inactive tenants)
- Unit tests for burst, refill, multi-tenant isolation

### Staff expectations
- Clear invariants; no data races; thoughtful lock granularity
- Discusses distributed rate limiting as follow-up (Redis, sticky edge)
- Does not ignore eviction → DoS via tenant ID cardinality

### Follow-ups
- Make it async-friendly
- Fairness across tenants
- Observability hooks

---

## Lab B — Parse & validate a policy rule DSL (subset)

### Prompt

Parse lines like:

```text
ALLOW user:alice app:payroll
DENY group:contractors app:*
```

Build an evaluator: first match wins (or most-specific — candidate must specify). Include tests for conflicts and defaults.

### Staff expectations
- Explicit precedence; default deny; rejects malformed input safely
- Avoids ReDoS / pathological patterns if using regex
- Clean error types

---

## Lab C — Bounded async worker pool

### Prompt

Given an async job source, process with concurrency limit N, graceful shutdown, and no job loss on shutdown (or defined at-least-once behavior).

### Staff expectations
- `tokio` structured concurrency; cancellation story; backpressure
- Explains what happens mid-job on Ctrl-C

---

## Lab D — Fix-the-bug (hand them broken code)

Provide a short snippet with one of:

1. `Rc` used across `tokio::spawn`
2. Holding `Mutex` across `.await`
3. Unbounded channel growth
4. `unwrap` on network result in request path

Ask them to diagnose and fix.

**Excellent signal:** Finds issue quickly; explains failure mode in production.

---

## Lab E — Take-home (optional, 3–4 hours max)

**Only if loop cannot host live coding.** Keep scoped:

> Build a tiny HTTP service (Axum) with mTLS *or* API key auth, per-tenant rate limit, structured tracing, and a `/health` + `/metrics` endpoint. Include README with threat notes.

**Reject take-homes that are weekend-long projects.** Respect candidate time.

---

## What not to ask

- Obscure bit-twiddling with no systems link
- Memorizing API names
- Implementing TLS by hand
- Gotcha undefined-behavior quizzes unrelated to their fix

---

## Scoring rubric (lab)

| Score | Meaning |
|-------|---------|
| 4 | Correct, clean API, tests, discusses production concerns |
| 3 | Correct core; minor gaps in tests/concurrency |
| 2 | Works partially; weak concurrency or unclear thinking |
| 1 | Does not progress; cannot compile mental model |

**Staff hire:** typically **3+** on lab AND strong design/security rounds. A 4 on coding alone does not justify Staff.
