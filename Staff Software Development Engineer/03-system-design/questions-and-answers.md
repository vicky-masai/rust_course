# Round: System Design — Questions & Expected Answers

**Duration:** 60–75m  
**Style:** Collaborative whiteboard. Change constraints every ~15 minutes.  
**Zscaler lens:** multi-tenant cloud security, low latency, high reliability, auditability.

**Scoring:** `09-scorecards/system-design-scorecard.md`

---

## How to run

1. Clarify requirements (functional + non-functional).
2. High-level diagram (control plane vs data plane).
3. Deep dive 1–2 components.
4. Failure modes, security, multi-tenancy, observability.
5. Scale numbers (back-of-envelope).

**Interviewer tip:** Prefer one design done deeply over three shallow designs.

---

## Design 1 — Cloud HTTP/S forward proxy for enterprise users (core)

### Prompt

Design a multi-tenant forward proxy that:

- Terminates TLS from clients (or handles CONNECT)
- Applies URL/category/DLP-like policy
- Logs transactions for audit
- Targets millions of concurrent sessions globally

### What good looks like

**Clarifying questions**
- Inline vs PAC vs tunnel; identity source; policy latency budget; logging retention; HA model; compliance regions.

**Architecture**
- Edge POP / enforcer nodes (data plane)
- Policy / identity / config control plane (cached at edge)
- Logging pipeline (async, durable, sampled if needed)
- Health, cert management, key management

**Staff topics they should hit**
- Hot path vs slow path (policy compile/cache vs per-request)
- Connection multiplexing, timeouts, retries (careful with non-idempotent)
- Tenant isolation (config, crypto keys, logs, rate limits)
- Certificate handling & trust stores
- Failure: policy service down → fail open vs fail closed (product decision; articulate risk)
- Observability: accept/reject rates, latency, upstream errors, CPU

**Numbers**
- Concurrent connections, throughput, log GB/day, cache hit ratio for policy

**Weak**
- Single region monolith; no cache; synchronous log write on request path; no tenant isolation.

---

## Design 2 — Zero Trust private app access (ZTNA-style)

### Prompt

Employees access internal apps without VPN. Design connector + broker + policy.

### Strong answer themes

- Client identity (IdP, device posture signals)
- Broker authenticates user/device; issues short-lived access
- Connector in customer network initiates **outbound** connections (firewall-friendly)
- Policy: user/group/app/device; continuous evaluation
- Session relay / tunnel; least privilege per app not network
- Audit trail; compromised device revoke path

**Probes**
- Connector HA and sticky sessions
- Split tunneling / DNS resolution
- Lateral movement reduction vs classic VPN

---

## Design 3 — Real-time policy evaluation service

### Prompt

Policy engine evaluating allow/deny/inspect for every session start (and optionally continuously). p99 < 10–20ms at edge.

### Strong answer themes

- Compile policies to efficient form (tries, decision trees, Wasm, Rete-like — any justified)
- Edge cache + versioned policy bundles; incremental updates
- Identity context enrichment (groups) with TTLs
- Determinism + audit: decision ID + policy version in logs
- Canary policy rollout; shadow mode
- Multi-tenant: O(tenants) isolation of compiled state

**Staff plus:** Conflict resolution order; default deny; testing policy correctness at scale.

---

## Design 4 — Global config distribution

### Prompt

Push security config to thousands of edge nodes in minutes, consistently, with rollback.

### Strong answer themes

- Versioned artifacts (content-addressed); signed bundles
- Staged rollout rings; automatic rollback on error budget burn
- Gossip vs central pull vs CDN — trade-offs
- Exactly-once not required; **monotonic versions** + idempotent apply
- Split “config arrived” vs “config active”

---

## Design 5 — Transaction logging & SIEM export at huge volume

### Prompt

Billions of log lines/day; customers query & export; PII controls.

### Strong answer themes

- Ingest → buffer (Kafka-like) → transform → cold/hot storage
- Schema evolution; sampling; aggregation for metrics vs raw for forensics
- Tenant partitioning; encryption at rest; access control on query path
- Backpressure when exporters slow
- Cost awareness (storage tiers)

---

## Cross-cutting probes (use in any design)

| Probe | Expect |
|-------|--------|
| Multi-region active-active | Data residency, sticky routing, conflict rules |
| Tenant noisy neighbor | Fair queues, quotas, isolation |
| Key compromise | Rotation, short-lived certs, blast radius |
| Dependency outage | Bulkheads, cached defaults, degrade modes |
| Abuse | Rate limits, authn on control APIs, replay protection |

---

## Mini prompts (if time left)

1. Design rate limiting for authenticated API + anonymous edge.
2. Design health checking for upstreams in a proxy.
3. Design feature flags safe for security-critical paths.

---

## Pass / fail snapshot

| Signal | Meaning |
|--------|---------|
| Separates control vs data plane; caches policy; discusses fail open/closed | Staff |
| Mentions TLS and boxes only | Senior at best |
| Ignores tenancy / audit / failure | No-hire for this role |
