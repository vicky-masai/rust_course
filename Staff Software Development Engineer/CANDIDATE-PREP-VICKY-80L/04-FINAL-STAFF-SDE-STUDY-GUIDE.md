# 04 — FINAL STAFF SDE STUDY GUIDE (UPDATED FOR REAL VICKY)
# Staff Software Development Engineer @ Zscaler | Interview Tuesday

**This file matches who you really are.**  
Builder-owner since 2020: first freelance fullstack SaaS/e-commerce, then joined a startup and built its multi-tenant SaaS WMS from zero to one as the end-to-end project owner.  
You implement JWT, RLS, RBAC/ABAC, nginx/Cloudflare/DNS, CI/CD.  
Security depth is still growing — answers below teach **honest Staff language** that still satisfies a Senior Staff panel.

---

# PART 0 — HOW TO ANSWER ANY QUESTION

1. **Direct answer first** (1 sentence).  
2. **Why / trade-off.**  
3. **Your real example** (freelance / startup / Rust / nginx).  
4. **Failure mode + what you’d measure.**  
5. **Stop.**

If deep security unknown:  
> “I haven’t specialized there yet. Framework I’d use is A/B/C. I’ve shipped X pattern in production. I’d deepen with the security team’s standards.”

**Never:** invent crypto expertise or fake millions of RPS.

---

# PART 1 — WHO YOU ARE + OWNERSHIP

## Q1. Tell me about yourself (45–60 sec)

> “I’m a product-building backend engineer with about six years of ownership. From 2020 I freelanced fullstack — clients gave requirements for e-commerce and SaaS products; I designed, built, secured at the app layer, and deployed with nginx, Cloudflare DNS/proxy, and CI/CD to VPS, later AWS.  
>  
> After that I joined a startup specifically to build its multi-tenant SaaS warehouse-management product from zero to one. The owner provided the product idea and requirements; I translated them into the architecture and working product. I own the project end to end: Rust/Axum/Tokio backend, PostgreSQL and RLS tenancy, JWT, RBAC/ABAC, Redis jobs, APIs, WebSockets, AWS/EKS deployment, CI/CD, observability, and production support. The Rust inventory-locking path cut P99 from about 800ms to 120ms.  
>  
> My core strength is zero-to-one ownership: give me a PRD or product idea and I turn it into a production SaaS product, then continue owning its reliability and evolution. I’m here for Staff SDE at Zscaler to apply that ownership at cloud-security scale and deepen security engineering judgment.”

---

## Q2. Most complex system you owned (3 min)

> “My strongest example is the startup SaaS WMS that I built from zero to one. My earlier freelancing prepared me to own complete products, but this WMS is the long-lived system where I own architecture, implementation, deployment, and production outcomes.  
>  
> **Starting point:** The startup owner gave me the product idea and PRD. I converted warehouse workflows into domains, data models, APIs, tenant boundaries, roles, background processes, deployment, and observability.  
>  
> **Invariant:** Inventory must stay correct for each facility and tenant under concurrent warehouse operations. I own API design, Rust services, PostgreSQL, JWT + RBAC/ABAC + RLS, Redis jobs (5K+/day), WebSockets, AWS/EKS deployment, CI/CD, tracing, and production support.  
>  
> **Hard decisions:** Postgres as stock source of truth; Redis for jobs/cache only; defense-in-depth tenancy; Rust on locking path after Node races/latency.  
>  
> **Results:** 5+ facilities, 10K+ daily inventory tx, 40+ versioned APIs, P99 800→120, deploy 45→8 min.  
>  
> **Failures:** double-allocation races; retry duplication without idempotency. Fixed with transactional Rust path + idempotency keys + better tracing.”

### If they ask about the earlier freelance period

> “From 2020, I built client e-commerce and SaaS products end to end: frontend, backend, authentication, admin, database, nginx, Cloudflare, DNS, CI/CD, and VPS/cloud deployment. That taught me zero-to-one delivery. The startup WMS is the stronger Staff example because I built it from scratch and continue owning its architecture and production lifecycle.”

---

## Q3. “Can you build from a PRD?” (they may ask this of you)

> “Yes — that’s been my job. I clarify actors, tenancy, authz, data model, APIs, non-functionals (latency, audit), then slice MVP, deploy behind nginx/Cloudflare, add observability, iterate. I don’t wait for perfect specs; I surface risks early — especially tenant isolation and idempotency.”

---

## Q4. Incident story (structure — fill real detail)

> Detect → mitigate (protect customers) → root cause → fix → prevent (test/dashboard).  
> Best story: inventory race / oversell under concurrency.

---

# PART 2 — AUTH + MULTI-TENANT (YOUR REAL STRENGTH — GO DEEP)

## Q5. How do you do authentication?

> “Typically JWT access tokens (short-lived) after login; refresh carefully; HTTPS only. Server validates signature, expiry, issuer. Identity claims include user id + tenant id + roles/attributes. Never trust tenant_id from body alone — must match token.”

## Q6. Authorization: RBAC vs ABAC?

> “**RBAC:** roles like admin/operator/viewer → permissions.  
> **ABAC:** attributes — facility, resource owner, time, action.  
> I use both: roles for coarse access, attributes for facility/tenant scoping. Missing context → **deny**.”

## Q7. What is RLS and is it enough?

> “Row Level Security in Postgres enforces tenant filters at DB layer. It’s a **safety net**, not enough alone. I also authorize in the API. If DB role is misconfigured, RLS-only is dangerous. Defense in depth: API authz + RLS + audit.”

## Q8. Cross-tenant attack you prevent?

> “IDOR: user passes another tenant’s resource id. Mitigate: authz check every read/write; RLS; automated tests that user A cannot read B’s rows.”

## Q9. Honest answer if they ask “how deep is your security?”

> “I’ve implemented production authn/authz and tenant isolation patterns end-to-end. I am not a cryptographer or full-time security engineer. I work from threat checklists, default deny, least privilege, and tests. At Zscaler I expect to deepen under stronger security review culture — that’s part of why I want this role.”

**This answer builds trust.** Fake depth destroys it.

---

# PART 3 — RUST (MUST BE CLEAN)

## Q10. Ownership (simple)

> One owner per value; moves by default; many `&T` or one `&mut T`; Drop cleans up. `Arc` shares across threads; `Rc` is single-thread only.

## Q11. Send / Sync + bug

> Send = move across threads. Sync = share `&T` across threads.  
> Bug: `Rc` in `tokio::spawn` — not Send. Use `Arc`. Don’t hold Mutex across `.await` on hot path.  
> My inventory work: clear concurrency story via DB transactions + careful in-process sharing.

## Q12. CPU on async runtime

> Blocks worker → latency spikes. Use `spawn_blocking` / separate pool.

## Q13. Cancellation / idempotency

> Timeouts drop futures. Half-finished side effects hurt. Design idempotent jobs/APIs (I use idempotency keys).

## Q14. Unsafe / unwrap

> Almost no unsafe in app services. No unwrap on IO/DB/parse on request path. Typed errors; panics are DoS risk.

## Q15. Why Rust for you?

> “I needed safer concurrency and lower latency on locking. Measured Node pain, migrated hot path, P99 800→120, races reduced.”

## Q16. p99 regression debug

> Blast radius → metrics/traces → blocking/locks/DB/retries → rollback if needed → fix. I use OpenTelemetry-style thinking.

---

# PART 4 — NGINX / CLOUDFLARE / DNS / DEPLOY (BRIDGE TO ZSCALER)

## Q17. How do you deploy a client/SaaS app today?

> “GitHub Actions CI/CD builds and deploys to VPS or AWS. nginx reverse-proxies to the app, terminates or passes TLS. Cloudflare manages DNS and often sits as edge proxy/CDN — TLS, caching, basic filtering, DDoS posture. I manage DNS records, origins, and health.”

## Q18. What is a reverse proxy? (nginx)

> “Client hits nginx; nginx forwards to app servers. Central place for TLS, routing, buffering, rate limits, headers. I’ve used this on every serious deploy.”

## Q19. Cloudflare role?

> “DNS + edge proxy in front of origin. Hides origin IP when proxied, TLS at edge, caching static, some WAF/rate features depending on plan. Origin still needs real app security — edge is not enough.”

## Q20. Bridge line to Zscaler design

> “I haven’t built a global security cloud POP, but I already think in layers: DNS → edge proxy → origin proxy → app → data. Zscaler’s data plane is that idea at massive multi-tenant security scale. I’ll separate control plane (policy) from data plane (session path).”

---

# PART 5 — SYSTEM DESIGN (PROXY) — PRACTICE ON PAPER

## Q21. Design multi-tenant forward proxy (MAIN)

**Clarify:** users, TLS, allow/deny/inspect, audit, millions of sessions, multi-tenant.

**Say while drawing:**

```
User → Edge POP (DATA PLANE)
         TLS, identity, LOCAL policy cache, upstream, async logs

Control plane: policy, identity, signed versioned bundles, admin APIs, rollout
```

**Must say:**
- Hot path uses **cached policy**, not live control-plane every request (like not doing heavy work on API request path — same lesson as Redis jobs off request path).
- Versioned policy bundles + rollback.
- Async logs, bounded queue, tenant_id on events.
- Per-tenant quotas.

**Your bridge:** nginx/Cloudflare experience = local proxy instincts; Zscaler = global policy-aware proxy.

## Q22. Fail open vs fail closed

> “Security gateway: bias **fail closed** for sensitive destinations. Use **last-known-good policy cache** with TTL + alert for availability. Silent allow-all only with explicit product decision. Matches my app stance: missing authz context → deny.”

## Q23. Tenant isolation in proxy/logs

> “Tenant on context, policy, logs; authz on admin APIs; no IDOR; careful metrics labels; quotas.”

## Q24. ZTNA vs VPN (short)

> “VPN = broad network; ZTNA = identity + per-app access, less lateral movement, continuous checks, audit.”

---

# PART 6 — SECURITY FOR PANEL (HONEST + STRUCTURED)

You will not out-expert a Zscaler security Staff on crypto.  
You **will** satisfy them if you show checklist thinking + production authz + humility.

## Q25. TLS 1.3 (learn this script)

> “ClientHello/ServerHello with key share → keys → cert verify → Finished → encrypted data (AES-GCM).  
> Handshake/cert verify costs more CPU than bulk encrypt.  
> At proxy scale, **session resumption** reduces full handshakes. Rotate ticket keys. Watch cert expiry.  
> I’ve operated TLS via Cloudflare/nginx certs in deploys; I’m deepening protocol internals for this role.”

## Q26. Threat model control-plane API (memorize 8)

1. Cross-tenant IDOR → authz + tests + RLS  
2. Stolen JWT → short TTL, rotate, revoke  
3. Admin abuse → least privilege, audit  
4. SSRF webhooks → allowlist  
5. Fat payloads / injection → validate, limits  
6. Dependency CVE → pin/audit  
7. Secrets/PII in logs → redact  
8. DoS → rate limit per tenant  

## Q27. Rust safe ≠ appsec

> “Rust helps memory safety. Authz bugs, fail-open config, secret leaks, panic DoS still happen. I shipped tenancy in API+RLS because memory safety doesn’t equal application security.”

## Q28. Secrets

> “Env/secrets carefully; prefer managers in cloud; never log tokens; rotate; least privilege.”

## Q29. If they push “AI wrote your security?”

> “Tools speed boilerplate. I still decide isolation boundaries, write cross-tenant tests, and own production incidents. Same as using StackOverflow or docs — responsibility stays with me.”

---

# PART 7 — DISTRIBUTED / RELIABILITY (SHORT DEEP)

## Q30. Exactly-once → at-least-once + idempotency  
## Q31. Retries → backoff + jitter + idempotency keys  
## Q32. Queue lag → measure, scale, DLQ, don’t silent-drop security logs  
## Q33. Redis not source of truth for stock  
## Q34. Outbox if DB + queue dual-write matters  

---

# PART 8 — STAFF LEADERSHIP (FOUNDING / FREELANCE FRAMED RIGHT)

## Q35. Cross-team initiative

> “I often was the backend owner across clients/frontend/ops. I drove shared API contracts — versioned endpoints, idempotent writes, clear errors — so others could integrate without breaking. At the startup I drove the measured Rust migration with rollback thinking. Metrics: p99, fewer races, faster deploys. Staff pattern: written contracts + measured change + adoption.”

## Q36. Blocked unsafe ship

> “Pressure to skip strict tenant/transaction checks for speed. I blocked it, showed a failing concurrent/cross-tenant case, shipped safe path (Rust locking) instead. I won’t bypass isolation for a deadline — critical at Zscaler.”

## Q37. Why Zscaler / Staff?

> “I want my PRD-to-production ownership on a platform where isolation and policy are the product. Staff means owning domain invariants and raising engineering bar — not only features.”

## Q38. Comp ~₹80L

> “For confirmed Staff SDE scope I’m targeting Staff band around ₹80L CTC depending on equity mix. If leveling says Senior, align title and comp honestly.”

---

# PART 9 — QUESTIONS YOU ASK THEM

1. What domain does this Staff SDE own in Q1–Q2?  
2. What does great look like at month six?  
3. How do product and eng decide fail-open vs fail-closed?  
4. How is Rust used on your critical path?  

---

# PART 10 — 60 MIN GAME PLAN

| Min | Win with |
|-----|----------|
| 0–12 | PRD builder + WMS ownership + numbers |
| 12–27 | Rust clean + your race/p99 story |
| 27–42 | Proxy design + nginx/Cloudflare bridge + fail closed |
| 42–50 | Honest security + 8 threats + Rust≠appsec |
| 50–55 | Contracts + blocked unsafe ship |
| 55–60 | Your 2–3 questions |

---

# PART 11 — NIGHT-BEFORE MEMORY CARD

- Identity: PRD→product owner · freelance SaaS · Rust tenancy · nginx/CF/DNS/CI  
- Auth: JWT + RBAC/ABAC + API authz + RLS · default deny  
- Rust: Send/Sync · no block runtime · no unwrap IO · p99 800→120  
- Design: control vs data plane · policy cache · fail closed · tenant on logs  
- Security: 8 threats · honest depth · edge ≠ appsec  
- Staff: contracts · measured migration · no safety shortcuts  

---

**End.** Next → practice spoken file [`05-ANSWER-SCRIPTS-60MIN.md`](05-ANSWER-SCRIPTS-60MIN.md)
