# 03 — What They Will Ask (From Real Vicky Story)

Senior Staff will hear “freelance fullstack → Rust → multi-tenant SaaS.”  
Predicted probes and how you win.

---

## Probe set A — Builder / PRD ownership (YOUR HOME GROUND)

**They ask:** “Someone gives you a PRD — what do you do?”  
**You win by:** requirements → tenancy/auth → data model → APIs → deploy (nginx/Cloudflare/CI) → observability → iterate.

**They ask:** “Freelance vs startup — what’s different?”  
**You win by:** freelance = full ownership + client scope; startup = longer-lived invariants, multi-facility tenancy, Rust performance.

---

## Probe set B — Multi-tenant auth (THEY WILL GO DEEP)

**They ask:** JWT vs session? RLS enough alone? RBAC vs ABAC?  
**You win by:** defense in depth — API authz + RLS; RBAC roles; ABAC attributes (facility, permission); default deny; never trust client tenant_id.

**They ask:** “Did you design security yourself or copy from AI/docs?”  
**Honest Staff answer:**  
> “I use references and tools like any modern engineer, but I own the decisions: tenant isolation model, where checks run, and tests for cross-tenant access. I don’t claim novel crypto research.”

---

## Probe set C — nginx / Cloudflare / DNS (BRIDGE TO ZSCALER)

**They ask:** How do you terminate TLS / proxy traffic today?  
**You win by:** Cloudflare DNS/proxy/WAF-ish edge → origin VPS/AWS → nginx reverse proxy → app. Certificates, caching, rate limits at edge.  
**Bridge:** “I haven’t built Zscaler’s global data plane, but I reason in proxy layers: edge, TLS, routing, upstream, logs.”

---

## Probe set D — Rust (MUST BE CLEAN)

**They ask:** Why Rust? Send/Sync? Async pitfalls?  
**You win by:** concurrency/races/latency on hot path; Arc vs Rc; no blocking on runtime; no unwrap on IO.

---

## Probe set E — Security product design (HARDEST)

**They ask:** Fail open vs fail closed? Threat model?  
**You win by:** memorized frameworks from file **04** — structured, honest, default deny.  
**You lose by:** hand-waving “HTTPS + JWT is enough.”

---

## Probe set F — Staff leadership

**They ask:** Cross-team impact?  
**You win by:** client + frontend + ops adoption of your API contracts; migration playbooks; saying no to unsafe shortcuts.

---

## Next

→ [`04-FINAL-STAFF-SDE-STUDY-GUIDE.md`](04-FINAL-STAFF-SDE-STUDY-GUIDE.md) — main Q&A (updated for you)
