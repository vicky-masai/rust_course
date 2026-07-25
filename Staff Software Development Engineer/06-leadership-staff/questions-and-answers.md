# Round: Leadership & Staff Behaviors — Q&A

**Duration:** 45m (often merged with behavioral)  
**Goal:** Evidence of Staff *scope and influence*, not only code quality.

---

## Q1. Describe a technical initiative you led across multiple teams.

**Listen for**
- Problem framed in business/reliability/security terms
- Stakeholders mapped; dissent handled
- Written artifact (RFC, ADR, design doc)
- Adoption metrics; what didn’t work
- Their personal contribution vs “I was in the room”

**Staff bar:** Cross-team outcome with measurable impact.  
**Fail:** Feature delivery inside one squad sold as “platform leadership.”

---

## Q2. Tell me about a time you disagreed with a tech lead / manager on architecture.

**Strong**
- Disagreed on principles/data; proposed alternatives; escalated constructively; committed after decision (disagree-and-commit) *or* correctly escalated risk.
- No politics-only story; no “I was always right” without learning.

---

## Q3. How do you create leverage beyond your own pull requests?

**Strong examples**
- Shared libraries, paved roads, templates, CI gates, observability standards
- Mentoring / hiring loops / review culture
- Killing bad complexity; simplifying

---

## Q4. You inherit a critical Rust service with no tests and tribal knowledge. 90-day plan?

**Strong**
- Stabilize: SLOs, on-call runbooks, error budgets
- Risk map: unsafe, authz, data loss paths
- Characterization tests around golden paths
- Incremental refactors; no big-bang rewrite unless justified
- Knowledge sharing; ownership model

---

## Q5. How do you decide build vs buy vs open source for a component on the request path?

**Strong**
- Latency/security/support/license/operational cost; core competency test; exit strategy; CVE response process.

---

## Q6. Example of saying no to a product request.

**Strong**
- Quantified risk (security, reliability, cost); offered alternatives; aligned on outcomes not output.

---

## Q7. Mentorship: someone struggling. What did you do?

**Strong**
- Specific coaching plan; feedback; outcomes; didn’t secretly do their work forever.

---

## Q8. How do you run a design review for a security-sensitive change?

**Strong**
- Pre-read; threat model section required; rollback plan; load/failure testing; multi-approver for risky paths; record decision.

---

## Q9. Ambiguous problem: product wants “AI policy assistant.” What do you do first?

**Strong**
- Problem framing; data sensitivity; false positive/negative costs; eval harness; human-in-loop; incremental MVP; compliance.

---

## Q10. Metrics of success for a Staff engineer on your team?

**Strong**
- Team throughput/quality, incident reduction, platform adoption, hiring bar, strategic clarity — not LOC.

---

## Anti-patterns (mark Lean No / No)

- Credit stealing or pure “we”
- No written design history
- Avoids conflict entirely or only fights
- Wants Staff title for IC heroics alone
- Blames QA/SRE for quality

---

## Calibration quotes (map to score)

| Quote pattern | Score hint |
|---------------|------------|
| “I wrote an RFC, got 3 teams onto one auth middleware, cut authz bugs 40%” | Strong Staff |
| “I finished the hardest tickets on the board” | Senior |
| “I told them my design and they didn’t listen” | Weak influence |
