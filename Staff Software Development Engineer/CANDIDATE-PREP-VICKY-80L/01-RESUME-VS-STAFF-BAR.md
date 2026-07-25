# 01 — Resume vs Staff SDE Bar (Honest Assessment)

**Resume source:** Vicky Kumar — Backend & Distributed Systems Engineer  
**Target:** Staff Software Development Engineer @ Zscaler · ~₹80L

---

## What already matches Zscaler Staff signals

| Resume evidence | Why interviewer likes it |
|-----------------|--------------------------|
| Founding engineer, owned API + data model + observability + releases | Clear “I owned the domain” |
| Multi-tenant SaaS + RLS + RBAC + API authz | Directly maps to **tenant isolation** (Zscaler must-pass) |
| Rust/Axum/Tokio migration; P99 **800ms → 120ms** | Production Rust + performance story |
| Eliminated race conditions on inventory locking | Concurrency judgment (Send/Sync / transactions) |
| Redis queues 5K+ jobs/day off request path | Backpressure / async / control vs data path thinking |
| 40+ versioned REST APIs, idempotent writes, RFC 7807 | API craft + reliability |
| AWS EKS, Terraform, CI/CD 45→8 min, OpenTelemetry | Operability / production maturity |
| Calm under pressure; tracing cut diagnosis hours → minutes | Incident mindset |

---

## Gaps vs ₹80L Staff SDE @ Zscaler (fix these in prep)

| Gap | Risk in interview | How you close it with words (not fake experience) |
|-----|-------------------|-----------------------------------------------------|
| No cloud **proxy / TLS data-plane** on resume | Design round feels foreign | Prepare Q6 design from first principles + map to your policy/authz cache ideas |
| Scale is **facility/WMS** (10K tx/day), not millions of sessions | “Is this Staff scale?” | Be honest on absolute scale; emphasize **concurrency correctness, tenancy, p99 under load**, and how you’d scale next |
| Small-company founding ≠ multi-team FAANG Staff | Q10 leadership weak | Reframe: frontend, ops, partner integrators, warehouse users as “multiple stakeholders”; show RFCs/API contracts as influence artifacts |
| Non-CS degree (Psych/Econ) | Occasional bias probe | Lead with shipped systems; if asked: self-taught systems path + measurable outcomes |
| Agentic AI on resume | Distraction from systems/security | Mention only if asked; **do not** lead with AI in a Zscaler Staff SDE loop |
| Limited explicit “fail open vs fail closed” language | Security section | Practice product-risk framing using RLS/authz defaults (default deny) |

---

## Level you should claim in the room

**Say:**  
> “I’ve operated as the founding backend owner — Staff-*scope* ownership of the platform backend. I’m targeting Staff SDE because I want to apply that ownership at cloud security scale.”

**Do not say:**  
> “I’ve already been Staff at FAANG” (false)  
> “I’m only Mid” (undersells)

**Internal target:** Interviewer scores you **3s** (solid Staff) by **framing + depth**, even if absolute QPS is lower than Zscaler’s.

---

## Numbers to memorize (from your PDF — do not invent new ones)

Use **only** resume-backed numbers unless you truly measured more:

- 5+ facilities (tenants/sites)
- 10K+ daily inventory transactions
- P99 inventory-locking: **800ms → 120ms**
- 5K+ daily async jobs (Redis)
- 40+ versioned REST endpoints
- Deploy cycle: **45 min → 8 min**
- Incident diagnosis: hours → minutes (tracing/logs)

If asked for QPS peak: convert honestly  
> “Order of tens of transactions per second sustained with sharp concurrent bursts during warehouse peaks — the hard problem was correctness under concurrency, not raw QPS. Here’s how I’d redesign for 100×…”

That last sentence is how you sound Staff.

---

## Red lines (never do these)

- Invent “millions of RPS” or fake Zscaler experience  
- Blame Node.js as “bad”; explain **why** Rust fit the locking path  
- Spend 10 minutes on LangGraph/RAG in this interview  
- Say “security is handled by RLS so we’re done” — expand to appsec + API authz + ops  

---

## Verdict

| Dimension | Today | After this prep book |
|-----------|-------|----------------------|
| Ownership | Strong | Strong + Staff language |
| Rust | Strong | Strong + Send/Sync/async scripts |
| Design (proxy) | Weak if cold | Passable→strong with practiced Q6 |
| Security | Good tenancy, thin TLS | Must drill Q7–Q9 |
| Staff leadership | At risk | Must use stakeholder/influence scripts |
| ₹80L odds if cold | Low–medium | Medium–high if scripts internalized |
