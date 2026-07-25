# 05 — ANSWER SCRIPTS (60 MIN) — REAL VICKY VOICE

Speak ~70% of this. Sound like a builder, not a textbook.

---

## Opening identity (if they start soft)

> “I started freelancing in 2020, building complete fullstack e-commerce and SaaS products for clients. After that I joined a startup to build its multi-tenant SaaS WMS from zero to one. I own the project end to end: translating the PRD, architecture, Rust backend, PostgreSQL, JWT/RBAC/ABAC/RLS, Redis jobs, APIs, deployment, observability, and production support. On the inventory-locking path, the Rust work reduced P99 from about 800ms to 120ms.”

---

## Q1 Ownership (3 min)

> “My deepest ownership is the startup’s multi-tenant SaaS WMS, which I built from zero to one. The owner gave me the product idea and requirements; I translated them into the architecture and production system.  
>   
> The main invariant is correct inventory for every facility and tenant under concurrent warehouse operations. I own the complete technical path: Rust/Axum/Tokio backend, API and PostgreSQL design, JWT authentication, RBAC/ABAC and RLS authorization, Redis jobs, WebSockets, CI/CD, AWS/EKS deployment, observability, and production troubleshooting.  
>   
> Important decisions were keeping PostgreSQL as the inventory source of truth, moving slow workflows off the request path, enforcing tenant isolation at both API and database layers, and moving the locking path to Rust after measuring races and latency.  
>   
> The system supports 5+ facilities, 10K+ daily transactions, 5K+ background jobs, and 40+ APIs. The locking-path P99 improved from about 800ms to 120ms. I also handled failures such as concurrent allocation and duplicate retries using transactions, idempotency, tracing, and load tests.”

**Freelance add-on if asked:**  
> “Before joining the startup, I freelanced from 2020. Clients gave me requirements for e-commerce or SaaS products, and I built the full solution: frontend, backend, authentication, admin workflows, database, nginx, Cloudflare DNS/TLS proxy, GitHub Actions, and VPS/cloud deployment. That developed my PRD-to-production ownership; the startup WMS is where I applied it to a long-lived, multi-tenant product from scratch.”

---

## Rust block (keep crisp)

**Send/Sync:**  
> “Send moves across threads; Sync shares references. Rc fails with tokio spawn — use Arc. Don’t hold locks across await.”

**Async CPU:**  
> “CPU on async worker starves others → spawn_blocking.”

**Unwrap:**  
> “No unwrap on request IO. Typed errors. Panic is DoS.”

**p99:**  
> “Confirm deploy blast radius, traces/metrics, rollback if needed. That’s how we approached the Rust locking win.”

---

## Design proxy (talk + draw)

> “Data plane at edge: TLS, identity, **local policy cache**, upstream, async audit.  
> Control plane: policy authorship, signed versioned bundles, admin authz, rollout.  
> I’ve run nginx reverse proxy + Cloudflare edge for apps — same layering instinct, Zscaler adds global policy and tenancy at huge scale.  
> Policy down: fail closed for sensitive + last-known-good TTL + alert.  
> Tenant isolation: tenant on context/policy/logs; authz on admin APIs; quotas.”

---

## Security (honest Staff)

**TLS:** handshake cost > bulk; resumption matters; I’ve operated certs via CF/nginx; deepening protocol detail.

**Threats (count on fingers):** IDOR, stolen JWT, admin abuse, SSRF, injection/size, deps, log PII, DoS.

**Depth honesty:**  
> “I implement and test isolation patterns. Not a crypto specialist. Default deny. Ready to deepen here.”

**Rust ≠ appsec:** memory safe still can ship authz bugs.

---

## Leadership

> “Cross-role: API contracts for frontend/partners/clients; measured Rust migration. Metrics: latency, races, deploy time.  
> Blocked shipping without tenant/transaction safety — offered Rust path instead.”

---

## Close questions (pick 3)

1. Staff SDE domain first two quarters?  
2. Great at month six?  
3. Fail open/closed with product?  
4. Rust on critical path?

---

## If stuck

> “I won’t invent. Framework is… In production I’ve done… I’d verify with metrics/docs in week one.”
