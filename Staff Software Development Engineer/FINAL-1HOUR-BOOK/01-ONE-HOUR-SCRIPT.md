# 01 — ONE HOUR SCRIPT
# Role: Staff Software Development Engineer at Zscaler

**Total: 60 minutes**

| Min | Section | What you do |
|-----|---------|-------------|
| 0–3 | Open | Intro role title + agenda |
| 3–12 | A. Ownership | 1 deep production ownership story |
| 12–27 | B. Rust development | 4 Staff SDE Rust questions |
| 27–42 | C. System design | Zscaler-style cloud security design |
| 42–50 | D. Security + networking | Must-pass for Zscaler SDE |
| 50–55 | E. Staff SDE leadership | Cross-team + bar raising |
| 55–60 | Close + score | Candidate Qs → your decision |

---

## 0–3 min — OPENING (you say)

> “Thanks for joining. This interview is for **Staff Software Development Engineer at Zscaler**. We’ll spend about 55 minutes on: a system you owned, Rust development depth, one cloud security system design, security judgment, and Staff-level technical leadership. Think aloud. Ask clarifying questions anytime. You’ll have a few minutes for questions at the end.”

Confirm quickly: notice period, competing timeline (30 seconds).

---

## 3–12 min — SECTION A: OWNERSHIP (score /4)

**Ask Q1:**
> “As a Staff Software Development Engineer, walk me through the most complex production system **you** personally owned in the last 2–3 years. Cover the problem, your design decisions, scale numbers, what broke, and what you changed after.”

**Probes if vague:**
- “What was *your* decision vs the team’s?”
- “Scale numbers — QPS, latency, data size, tenants?”
- “One production incident on this system that you led end-to-end?”

---

## 12–27 min — SECTION B: RUST DEVELOPMENT (score /4)

Ask in order (full text in `02-FULL-QA.md`):

- **Q2** Ownership + `Send`/`Sync`
- **Q3** Async pitfall (blocking / cancellation)
- **Q4** `unsafe` + `unwrap` discipline on request path
- **Q5** p99 regression debug (Staff SDE production craft)

Stop early only if Q2 and Q3 both fail badly — note as below bar for Rust-primary Staff SDE.

---

## 27–42 min — SECTION C: SYSTEM DESIGN (score /4)

**Ask Q6:**
> “Design a multi-tenant cloud forward proxy for enterprise customers — the kind of data-plane system a Staff SDE at Zscaler might own: TLS, URL/policy allow-deny-inspect, audit logs, millions of concurrent sessions. Separate control plane vs data plane. I will change constraints.”

**~min 35:**
> “Policy service is down for 5 minutes. Fail open or fail closed? What do customers experience?”

**~min 38:**
> “Tenant A must never see Tenant B’s logs or policy. How do you guarantee isolation?”

---

## 42–50 min — SECTION D: SECURITY + NETWORKING (score /4)

Required for **Zscaler Staff SDE**:

- **Q7** TLS 1.3 + resumption at proxy scale  
- **Q8** Threat model multi-tenant control-plane API  
- **Q9** Rust memory safety ≠ application security  

---

## 50–55 min — SECTION E: STAFF SDE LEADERSHIP (score /4)

- **Q10** Cross-team technical initiative (RFC, adoption, metric)  
- **Q11** Blocked a dangerous ship (security/reliability)  

---

## 55–60 min — CLOSE

> “What questions do you have about the Staff Software Development Engineer role or the team at Zscaler?”

Silently complete `03-LIVE-SCORECARD.md` → `04-OFFER-DECISION.md`.

**Do not communicate the hire decision on the call.**
