# LEVEL 24 — Staff Engineering

### 0441. Architecture Decision Records (ADR)

Short docs that capture an architectural choice, context, and consequences. Lightweight history so future you knows *why*. Keep them immutable; supersede with new ADRs.

**Talk track:** *"ADRs record the why of decisions — cheaper than archaeology in Slack six months later."*

---

### 0442. Technical RFC Writing

Proposals for substantial change: problem, options, recommendation, risks, rollout. Invite critique before coding. Quality RFCs reduce thrash.

**Talk track:** *"RFCs socialize design before sunk cost — write to decide, not to impress."*

---

### 0443. Design Reviews

Structured critique of a design with peers. Look for failure modes, scalability, security, operability, and simplicity. Staff run reviews that teach, not dunk.

**Talk track:** *"Design reviews catch expensive mistakes early — optimize for learning and risk reduction."*

---

### 0444. Tradeoff Analysis

Explicit comparison: what you gain, what you give up, what you postpone. Numbers when possible (latency, cost, eng weeks). Staff language is tradeoffs.

**Talk track:** *"Every design is a bet — name the stakes and the reject alternatives."*

---

### 0445. Incident Response

Coordinate under pressure: declare, communicate, mitigate first, then investigate. Roles (incident commander), clear status updates, known playbooks.

**Talk track:** *"Incidents prioritize mitigation and communication — root cause comes after the bleeding stops."*

---

### 0446. Root Cause Analysis (RCA)

Dig past symptoms to systemic causes (often multiple). Five whys + evidence. Blame the process/system, not the human (just culture).

**Talk track:** *"RCA seeks systemic fixes — 'human error' is usually where investigation stopped too early."*

---

### 0447. Postmortems

Write-up after incidents: timeline, impact, causes, action items with owners. Blameless. Track actions to closure or they never happen.

**Talk track:** *"Postmortems turn pain into prevention — action items are the product."*

---

### 0448. SLO

Service Level Objective — target reliability/latency you commit to internally (e.g., 99.9% success). Guides how much risk you can take shipping.

**Talk track:** *"SLOs are the reliability targets that drive engineering priorities."*

---

### 0449. SLI

Service Level Indicator — the measured signal (success ratio, p99 latency). SLOs are thresholds on SLIs.

**Talk track:** *"SLIs are the metrics; SLOs are the goals on those metrics."*

---

### 0450. SLA

Service Level Agreement — external contractual promise, often with credits. Stricter legal/business weight than SLO. Don't promise SLAs you can't measure.

**Talk track:** *"SLAs are customer contracts; SLOs are engineering policy — don't confuse them."*

---

### 0451. Error Budgets

Allowed unreliability derived from SLO (e.g., 0.1% budget). When budget burned, freeze risky launches and focus on reliability. Aligns product speed with stability.

**Talk track:** *"Error budgets turn reliability into a spendable resource — empty budget means slow down."*

---

### 0452. Platform Engineering

Build internal products (CI, paved roads, golden paths) that make every team faster and safer. Treat internal developers as customers.

**Talk track:** *"Platform engineering multiplies other engineers — product thinking for internal DX."*

---

### 0453. Developer Experience (DX)

Friction of building/shipping: local setup, docs, CI times, clear errors. Bad DX taxes every feature. Measure time-to-first-PR and CI pain.

**Talk track:** *"DX is productivity infrastructure — slow CI is an org-wide pay cut."*

---

### 0454. Technical Mentoring

Grow others via pairing, reviews, and sequenced challenges. Staff impact scales through people, not only personal PRs.

**Talk track:** *"Mentoring compounds — your standards become the team's standards."*

---

### 0455. Cross Team Architecture

Align boundaries, contracts, and shared platforms across teams. Political and technical. Prefer clear ownership and explicit APIs over hidden coupling.

**Talk track:** *"Cross-team architecture is interfaces and ownership — ambiguity creates outages."*

---

### 0456. Engineering Leadership

Set technical direction, make decisions under uncertainty, create clarity, and raise the quality bar. Influence without needing hierarchy for every move.

**Talk track:** *"Leadership is clarity + judgment + accountability — especially when tradeoffs hurt."*
