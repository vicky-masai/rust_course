# LEVEL 26 — Build Production Projects

Building is how staff knowledge becomes muscle. For each project: define API, data model, failure modes, observability, and deploy path. Prefer boring tech that works; add complexity only when forced.

---

### 0469. Authentication Service

Issues and validates credentials/sessions/tokens. Password hashing, MFA hooks, refresh rotation, lockouts, audit. Highest security bar — threat model first.

**Talk track:** *"Auth services mint trust — design for theft, replay, and revocation."*

---

### 0470. User Service

Profiles, identity records, preferences. CRUD with strong validation, PII handling, tenant isolation if needed. Often the system of record for "who."

**Talk track:** *"User service owns identity data — privacy and consistency matter more than fancy features."*

---

### 0471. Notification Service

Email/SMS/push/webhooks with templates, preferences, retries, DLQ. Fan-out at scale; never block checkout on SMTP.

**Talk track:** *"Notifications are async delivery with preferences and retries — throughput with quiet failure handling."*

---

### 0472. File Storage Service

Upload/download, virus scan hooks, signed URLs, metadata DB, S3 backing. Large objects, multipart, access control.

**Talk track:** *"File services are metadata + object storage + authz — don't serve bytes through app memory naively."*

---

### 0473. API Gateway

Edge routing, authn, rate limits, request IDs, maybe aggregation. Keep it policy-heavy and logic-light.

**Talk track:** *"Your gateway is the company's front door policy engine."*

---

### 0474. Inventory Service

Stock levels, reservations, adjustments. Contention hotspot — transactions, idempotency, maybe event sourcing for movements.

**Talk track:** *"Inventory is correctness under contention — oversell is a product failure."*

---

### 0475. Order Service

Order lifecycle: create → pay → fulfill → cancel. State machine clarity; sagas with payment/inventory; immutable order history.

**Talk track:** *"Orders are long-lived state machines integrated with money and stock."*

---

### 0476. Payment Service

Capture/authorize/refund via providers. Idempotency keys mandatory; ledger mindset; PCI scope minimization (don't store cards if you can avoid).

**Talk track:** *"Payments demand idempotency and auditability — never double-charge."*

---

### 0477. Search Service

Index pipeline + query API. Eventual consistency with source of truth; relevance tuning; reindex story.

**Talk track:** *"Search is a derived view — indexing pipelines and relevance are the product."*

---

### 0478. Workflow Engine

Durable execution of multi-step jobs with retries/timers. Either integrate Temporal or build a simpler orchestrator for learning.

**Talk track:** *"Workflow engines persist process state so crashes don't forget business progress."*

---

### 0479. Distributed Scheduler

Cron-at-scale: exactly-once-ish job firing across nodes, leader election or partitioned schedules, catch-up after downtime.

**Talk track:** *"Distributed schedulers need fencing so two nodes don't fire the same job."*

---

### 0480. Distributed Cache

Layered cache service or library: TTL, stampede control, metrics, optional replication. Clear consistency promises.

**Talk track:** *"A cache product is APIs + invalidation + observability — not just Redis wrappers."*

---

### 0481. Feature Flag Service

Flag CRUD, targeting rules, SDKs, streaming updates, audit. Extremely high availability — flags must not take down apps (local defaults).

**Talk track:** *"Flag services must fail open/closed deliberately — they sit on every request path."*

---

### 0482. Identity Provider

Full IdP: OIDC/OAuth2 authorization server, clients, consent, tokens. Standards compliance heavy.

**Talk track:** *"An IdP is OAuth/OIDC done correctly — security and interoperability first."*

---

### 0483. Warehouse Management System (WMS)

Locations, inbound/outbound, picks, pack, inventory accuracy. Heavy domain rules; barcode/scan workflows; integration with orders.

**Talk track:** *"WMS is physical-world consistency — locations and movements must match reality."*

---

### 0484. ERP Backend

Finance, procurement, inventory, HR integrations — modular monolith often wins. Strong transactional integrity and reporting.

**Talk track:** *"ERP backends are broad business systems — modularize by domain or drown."*

---

### 0485. Multi-Tenant SaaS Platform

Tenancy model, billing hooks, isolation, onboarding, admin. Platform concerns dominate feature code.

**Talk track:** *"SaaS platforms are tenancy + billing + isolation wrapped around product features."*

---

### 0486. AI Agent Platform

Tool registry, sandboxes, budgets, tracing of agent steps, eval harness. Safety and cost controls are the product.

**Talk track:** *"Agent platforms productize tools, policy, and observability around LLM loops."*

---

### 0487. RAG Platform

Ingestion, chunking, embeddings, vector store, query API, citation, evals. Ops for reindex and model upgrades.

**Talk track:** *"RAG platforms are data pipelines plus retrieval quality — the LLM is the last mile."*

---

### 0488. Build Your Own Redis

Implement RESP parsing, dict/strings, single-threaded event loop, basic persistence. Teaching project for systems thinking.

**Talk track:** *"A toy Redis teaches event loops, protocols, and in-memory data structure design."*

---

### 0489. Build Your Own Kafka (Simplified)

Append-only log segments, producers/consumers, basic replication or single-node partitions. Learn why Kafka's model works.

**Talk track:** *"A toy Kafka teaches durable logs, offsets, and partition parallelism."*
