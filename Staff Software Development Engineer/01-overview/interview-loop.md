# Interview Loop — Staff SWE (Rust)

## End-to-end schedule (recommended)

| # | Round | Time | Goal | Materials |
|---|-------|------|------|-----------|
| 0 | Recruiter / Tech HR screen | 30m | Role fit, motivation, logistics, salary expectation | This file + behavioral light |
| 1 | Hiring manager | 45m | Scope, ownership stories, Staff signals, team fit | `06`, `07` |
| 2 | Rust deep dive + coding | 60–75m | Language mastery + problem solving under time | `02`, `08` |
| 3 | System design (security/cloud) | 60–75m | Architecture, trade-offs, Zscaler-like constraints | `03` |
| 4 | Distributed systems / reliability | 45–60m | Failure, consistency, scale | `04` |
| 5 | Security & networking | 45m | Threat model, TLS, networking judgment | `05` |
| 6 | Leadership + behavioral | 45m | Influence, conflict, mentorship, culture | `06`, `07` |
| 7 | Bar raiser (optional but recommended) | 30m | Independent calibration | `09`, `10` |
| — | Debrief | 30m | Hire / no-hire / level / ₹80L | `10` |

Compress to 4 technical rounds for senior candidates with strong referrals; never skip system design + Rust for this profile.

---

## Round 0 — Tech HR screen (script)

**Ask**
1. Walk me through the last 2 years of your role — what did *you* own?
2. Why Zscaler / why cloud security now?
3. Primary language is Rust — where have you shipped Rust to production?
4. Largest system by QPS / data / users you touched?
5. On-call experience? Last incident you led?
6. Compensation expectation and notice period?
7. Any competing offers / timeline?

**Pass to next round if**
- Clear ownership narrative (not “we”)
- Real Rust production story (or honest “strong Rust, production mostly X”)
- Expectation aligns with Staff + ~₹80L band (or open to discuss)
- No hard red flags (ethics, inability to work with customers/security constraints)

---

## Interviewer operating principles

1. **Spend 70% on depth, 30% on breadth.** One deep thread beats ten shallow questions.
2. **Force concrete examples:** metrics, timelines, decisions, failures.
3. **Change constraints mid-design:** “Now multi-region,” “tenant A must not see tenant B,” “p99 < 20ms.”
4. **Score independently** before debrief. Do not share scores in Slack before written feedback.
5. **Write feedback same day** using scorecard template.

---

## Candidate experience (non-negotiable)

- Share agenda and interviewers’ roles in advance
- Allow questions at end of every round
- No trick puzzles disconnected from the job
- Accommodate timezone; keep coding environment fair (candidate’s IDE OK if monitored)

---

## Debrief order

1. Each interviewer: Hire / Lean Hire / Lean No / No — with evidence
2. Level: Senior vs Staff
3. Comp: ₹80L justified? What risks remain?
4. HM decides with bar raiser veto for quality
