# Round: Distributed Systems — Questions & Expected Answers

**Duration:** 45–60m  
**Goal:** Staff-level reasoning about failure, consistency, and scale — not textbook recitation.

---

## Q1. CAP in practice: customer policy update must be visible worldwide “soon.” What do you choose?

**Strong answer**
- Clarifies: strong consistency globally is expensive/latency-heavy; for policy, usually **versioned eventual consistency** with bounded staleness + monotonic reads at a POP.
- Uses: epoch/version numbers, don’t apply older bundles, signed configs.
- Discusses split-brain and rollback.

**Weak:** “We need CA” without explaining partitions.

---

## Q2. Exactly-once delivery — myth or reality?

**Strong answer**
- End-to-end exactly-once is rare; usually **at-least-once + idempotent consumers** or transactional outbox.
- Dedup keys, idempotency store with TTL, careful with side effects (emails, charges, firewall rules).

---

## Q3. Design retries for a client calling your API.

**Strong answer**
- Exponential backoff + jitter; honor `Retry-After`; budget; idempotency keys for POSTs; circuit breaker; distinguish 429/503 vs 400.
- Warns about retry storms amplifying outages.

---

## Q4. You have a queue (Kafka-like) and consumers fall behind. What do you do?

**Strong answer**
- Metrics: lag by partition; scale consumers; check hot keys/partitions; poison pill handling; backpressure to producers; selective drop/sample only if product allows (security logs often cannot drop silently — alert and expand capacity).
- Ordering guarantees vs parallelism trade-off.

---

## Q5. Leader election for an active component in a region.

**Strong answer**
- Lease-based (etcd/ZooKeeper/Consul/K8s leases); fencing tokens to prevent dual writers; what happens on GC pause / network blip.
- Prefer avoiding single leader when possible (sharding).

---

## Q6. Split brain after network partition between two regions — your active-active writer?

**Strong answer**
- Avoid multi-primary without CRDTs/conflict rules; prefer single-writer per keyspace; quorum; or region affinity.
- Security config: never silently diverge tenant policy; detect and reconcile with audit.

---

## Q7. Distributed lock to ensure only one job runs. Is it enough?

**Strong answer**
- Lock + fencing; lock loss under GC; “lock held” ≠ work safe; prefer idempotent jobs and transactional claims.
- For correctness-critical paths, design without relying solely on locks.

---

## Q8. How do you achieve multi-tenant fairness on a shared platform?

**Strong answer**
- Quotas, weighted fair queuing, per-tenant concurrency limits, noisy-neighbor detection, separate noisy tenants, storage/IO isolation where needed.

---

## Q9. Clock skew bites you. Where?

**Strong answer**
- Token expiry, TLS cert validity, leadership leases, event ordering by wall clock.
- Prefer logical clocks / version vectors / server-issued time; NTP awareness; skew-tolerant designs.

---

## Q10. Hot key / hot tenant problem.

**Strong answer**
- Cache, shard by secondary dimension, replicate read-only, rate limit, dedicated capacity, request coalescing.

---

## Q11. Saga vs 2PC for a workflow (provision connector → register DNS → enable policy).

**Strong answer**
- 2PC fragile across heterogeneous systems; sagas with compensating actions more common in cloud.
- Need clear state machine, timeouts, human escalation, audit log of steps.

---

## Q12. Load shedding strategy when overload hits the data plane.

**Strong answer**
- Protect core: shed low-priority work first; never shed in a way that fails open insecurely without explicit product policy.
- Admission control at edge; degrade logging detail before dropping enforcement (product-dependent — force them to articulate).

---

## Mini scenarios (5 minutes each)

1. **Duplicate webhook deliveries** from IdP — how do you make user provisioning safe?
2. **Regional outage** — traffic shift; sticky sessions; cold caches.
3. **Poison message** crashes consumer — quarantine, alert, continue.

---

## Pass / fail snapshot

| Strong Staff | Applies patterns to security/SaaS constraints; talks about blast radius |
| Fail | Buzzwords only (CAP/Paxos) with no operational story |
