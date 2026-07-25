# 04 — Zscaler Bridge Stories
# Translate your WMS resume into cloud security Staff language

Interviewers hire for **transferable systems thinking**. Use this translation table out loud.

---

## Concept bridge

| You built (WMS) | Say it in Zscaler Staff language |
|-----------------|----------------------------------|
| Multi-facility tenancy | Multi-tenant isolation; blast radius per tenant |
| PostgreSQL RLS + API RBAC | Defense-in-depth authz; default deny |
| Inventory locking correctness | Safety-critical invariant on concurrent path |
| Node → Rust hot path | Data-plane performance + memory/concurrency safety |
| Redis jobs off API path | Don’t block request/data path on slow work |
| Idempotent writes | At-least-once safe mutations |
| OpenTelemetry + structured logs | Operability; reduce MTTR |
| RFC 7807 + versioned APIs | Stable contracts; safe evolution |
| EKS + Terraform + faster deploys | Safe rollout, rollback, paved road |

---

## Three “hero stories” (memorize titles)

### Story A — Tenant isolation
**Title:** Defense in depth for multi-tenant reads/writes  
**Punchline:** API authz + RLS + scoped RBAC; missing tenant context → deny  
**Zscaler use:** Same pattern for policy/admin APIs and logs

### Story B — Concurrency + Rust
**Title:** Inventory locking race → Rust transactional path  
**Punchline:** P99 800→120; races eliminated under load  
**Zscaler use:** Hot-path correctness under concurrency; measure then migrate

### Story C — Request path purity
**Title:** 5K+ jobs decoupled via Redis  
**Punchline:** API stays fast; async is reliable with idempotency  
**Zscaler use:** Policy compile / log export / heavy work off data plane

---

## Design vocabulary cheat sheet (Q6)

Practice saying these without notes:

1. Control plane vs data plane  
2. Versioned signed policy bundle + edge cache  
3. Fail closed vs last-known-good TTL  
4. Async audit log pipeline + bounded queue  
5. Per-tenant quota / noisy neighbor  
6. Session resumption for TLS CPU  
7. Monotonic config version + rollback rings  

---

## Phrases that connect your past to their product

> “In WMS the invariant was stock correctness per tenant. At Zscaler the invariant is policy enforcement and tenant isolation on the session path — same discipline, higher blast radius.”

> “We learned never to put control-plane dependency on the hot path without cache — I’d apply that to policy evaluation at the edge.”

> “Default deny when context is missing — we did that for authz; I’d argue the same for security gateways unless product explicitly documents a degrade mode.”
