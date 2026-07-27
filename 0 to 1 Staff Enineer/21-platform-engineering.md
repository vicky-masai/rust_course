# LEVEL 21 — Enterprise Platform Engineering

---

## Platform Topics

### 0402. Multi-Tenancy

One system serves many customers (tenants) with isolation. Models: siloed DBs, schema-per-tenant, shared tables with `tenant_id`. Isolation vs cost tradeoff. Never leak data across tenants — row filters, tests, and authz must be paranoid.

**Talk track:** *"Multi-tenancy is shared software with hard isolation guarantees — the blast radius of a bug is every customer."*

---

### 0403. Audit Logging

Immutable record of who did what when. Compliance and forensics. Append-only, tamper-evident if required, careful with PII. Separate from debug logs.

**Talk track:** *"Audit logs answer accountability questions — treat them as evidence, not printf."*

---

### 0404. Event Framework

Shared library/platform for producing/consuming domain events consistently: schemas, tracing headers, idempotency keys, outbox helpers. Stops every team reinventing messaging differently.

**Talk track:** *"An event framework standardizes how the company speaks asynchronously."*

---

### 0405. Inventory Reservation

Hold stock temporarily during checkout so two buyers don't purchase the last item. Needs TTLs, release on cancel, and strong consistency or carefully designed reservations tables/locks.

**Talk track:** *"Reservation is a time-bounded lock on scarce inventory — expiry and idempotency are mandatory."*

---

### 0406. Permission Systems

Central or library-based authorization: roles, grants, resource ownership, policy engines (OPA). Must be fast, auditable, and correct under hierarchy changes.

**Talk track:** *"Permission systems are product-critical infrastructure — wrong allow is a security incident."*

---

### 0407. Feature Flags

Decouple deploy from release. Ramp traffic, kill switches, experiments. Needs clean flag hygiene or you drown in permanent conditionals.

**Talk track:** *"Flags separate shipping code from releasing risk — retire old flags ruthlessly."*

---

### 0408. API Gateway

Edge entry: auth, rate limit, routing, request shaping, sometimes aggregation. Centralizes cross-cutting concerns; don't put all business logic there.

**Talk track:** *"Gateways handle edge policy — keep domain logic in services."*

---

### 0409. Service Discovery

How services find each other: DNS, Consul, k8s Services, Eureka historically. Health-aware discovery prevents routing to dead instances.

**Talk track:** *"Discovery is dynamic addressing — DNS and control planes replace hardcoded hosts."*

---

### 0410. Distributed Transactions

Transactions across systems without a single ACID DB. 2PC is fragile at scale; prefer sagas/outbox/idempotency. Staff bias: avoid cross-service ACID if possible.

**Talk track:** *"Distributed transactions are a last resort — design for local ACID + eventual consistency."*

---

## Resilience Patterns

### 0411. Retry

Try again on transient failure. Must be bounded and prefer idempotent operations. Blind retries amplify outages (retry storms).

**Talk track:** *"Retries help flaky networks — without backoff and idempotency they cause stampedes."*

---

### 0412. Exponential Backoff

Wait 1s, 2s, 4s... (+jitter) between retries. Gives the failing system room to recover. Cap max delay.

**Talk track:** *"Backoff + jitter spreads retry load so recoveries aren't crushed."*

---

### 0413. Circuit Breaker

After enough failures, stop calling the dependency briefly ("open"), fail fast, then probe ("half-open"). Protects your service from waiting on a dead peer.

**Talk track:** *"Circuit breakers fail fast when a dependency is sick — save your threads and user latency."*

---

### 0414. Bulkhead

Isolate resources (thread pools, connections) so one failing feature can't exhaust the whole process. Ship compartments.

**Talk track:** *"Bulkheads contain blast radius — one bad dependency can't take every worker."*

---

### 0415. Timeout

Bound how long you'll wait. Without timeouts, hung dependencies hang you. Pair with cancellation and uncertainty about whether the write committed.

**Talk track:** *"Timeouts are latency SLOs enforced in code — always set them deliberately."*

---

### 0416. Fallback

Degraded response when primary path fails: cached data, default, partial page. Be honest in UX when stale.

**Talk track:** *"Fallbacks preserve core UX under failure — better stale than dead when product allows."*

---

### 0417. Chaos Engineering

Deliberately inject failure (kill pods, add latency) to verify resilience hypotheses. Start in staging; require observability and abort criteria.

**Talk track:** *"Chaos proves your resilience story with experiments, not slideware."*
