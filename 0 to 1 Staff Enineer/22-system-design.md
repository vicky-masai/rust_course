# LEVEL 22 — System Design

### 0418. Scalability

Ability to handle growth — vertical (bigger box) vs horizontal (more boxes). Identify bottlenecks; design stateless app tiers; shard data when one DB isn't enough. Scalability is a cost curve, not a slogan.

**Talk track:** *"Scale by removing bottlenecks and adding capacity where the workload actually hurts."*

---

### 0419. Reliability

Correct operation over time — fewer failures, safer recoveries. Redundancy, tested failover, backups, disciplined changes. Reliability is earned in incidents and postmortems.

**Talk track:** *"Reliability is delivering on promises under stress — redundancy plus practice."*

---

### 0420. Availability

Fraction of time the system is usable (`99.9%` etc.). Depends on failure domains and recovery speed. Partial availability (degraded mode) often beats binary up/down.

**Talk track:** *"Availability is uptime as experienced by users — design degradation paths."*

---

### 0421. Fault Tolerance

Continue operating when parts fail. Replication, retries, isolation. Distinguish transient vs permanent faults.

**Talk track:** *"Fault tolerance assumes components fail — the system still meets a minimum bar."*

---

### 0422. Capacity Planning

Estimate load, headroom, and growth. Use peak + margin; include failure scenarios (N+1). Revisit when traffic shape changes.

**Talk track:** *"Capacity planning is math on peaks, growth, and failure headroom — not average Tuesday."*

---

### 0423. Load Balancing

(See 0054.) Distribute traffic; health checks; avoid hot spots. Client-side vs server-side LB. Consistent hashing for sticky caches.

**Talk track:** *"Load balancing spreads work and removes bad nodes — health is the real feature."*

---

### 0424. Caching Strategy

What to cache, where (client/CDN/app/Redis/DB), TTL, invalidation, stampede control (singleflight/lock). Cache is a correctness and cost decision.

**Talk track:** *"Caching strategy picks layers and invalidation — speed without lying too long."*

---

### 0425. High Availability

Architecture for minimal downtime: multi-AZ, failover, no single points of failure. HA is topology + tested ops.

**Talk track:** *"HA removes single points of failure and rehearses failover."*

---

### 0426. Disaster Recovery

Survive region loss / major corruption: RPO (data loss tolerance) and RTO (time to restore). Backups are useless without restore drills.

**Talk track:** *"DR is RPO/RTO with practiced restores — backups without drills are theater."*

---

### 0427. Zero Downtime Deployment

Ship without user outages: rolling/blue-green/canary, backward-compatible migrations (expand/contract), connection draining.

**Talk track:** *"Zero downtime needs compatible schemas and draining traffic — deploy and migrate as separate careful steps."*

---

### 0428. Multi Region Architecture

Serve globally, survive region failure. Active-active vs active-passive; data replication conflicts; latency vs consistency. Expensive operationally.

**Talk track:** *"Multi-region is latency and survival — data plane consistency is the hard design."*

---

### 0429. Cost Optimization

Performance and scale always have a bill. Right-size instances, storage tiers, cache to cut DB, kill idle resources, measure cost per request. Staff engineers own cost as a reliability/product constraint.

**Talk track:** *"Cost is an architecture requirement — optimize the expensive path with numbers."*
