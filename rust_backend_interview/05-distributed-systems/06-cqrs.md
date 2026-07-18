# CQRS

## Interview Question

Explain CQRS and when you would use it.

## Interview Answer

CQRS (Command Query Responsibility Segregation) is an architectural pattern that separates the read model (query) from the write model (command) of a system. Instead of a single model handling both reads and writes, CQRS uses separate optimized models — the write side focuses on business rules and validation, while the read side is denormalized for fast queries. This is valuable in distributed systems where read and write workloads have different scalability, performance, and consistency requirements. CQRS is often paired with event sourcing and is particularly useful in domains with complex query patterns, high read-to-write ratios, or bounded contexts with different data access needs.

---

## Follow-up Questions & Answers

### Q1. What is the difference between CQRS at the database level versus the application level?

**Interview Answer**

At the application level, CQRS means having separate service classes or modules for commands and queries, but both may use the same database. This is a lightweight approach that provides code organization benefits without infrastructure complexity. At the database level, CQRS uses separate databases — writes go to a normalized OLTP database (PostgreSQL), and reads come from a denormalized read-optimized store (Elasticsearch, Redis, or a separate PostgreSQL replica with materialized views). The database-level approach provides true independent scalability and optimization but adds complexity for data synchronization between the write and read stores.

---

### Q2. How do you synchronize the write model and read model in CQRS?

**Interview Answer**

Synchronization typically uses events — the write model publishes domain events (via the outbox pattern or event sourcing), and a consumer updates the read model accordingly. For example, when an order is created, the write side publishes an `OrderCreated` event, and a projection handler updates the read-optimized denormalized view. The read model can lag behind the write model, providing eventual consistency. For cases needing strong consistency, you can read from the write model for specific queries or use a transactional outbox to ensure events are published synchronously. The read model is rebuilt from events when the schema changes, providing a natural migration path.

---

### Q3. How would you implement CQRS in a Rust/Axum backend?

**Interview Answer**

In a Rust Axum application, separate your handlers into command handlers and query handlers. Command handlers accept POST/PUT/DELETE requests, validate business rules using domain models, persist to PostgreSQL, and publish events via the outbox pattern. Query handlers accept GET requests and read from a denormalized read store — this could be Redis for hot data, Elasticsearch for search, or materialized views in PostgreSQL. Use a shared event bus (Kafka or an in-process channel) to propagate events from write to read side. The Axum router separates these cleanly: `/api/commands/*` for mutations and `/api/queries/*` for reads, with different middleware and rate limits for each.

---

### Q4. What are the trade-offs of using CQRS?

**Interview Answer**

CQRS adds significant complexity: you must maintain two models, handle synchronization between them, deal with eventual consistency, and manage multiple data stores. The benefits are independent scaling (read replicas can scale independently), optimized read/write models, and better separation of concerns. CQRS is overkill for simple CRUD applications — it shines in complex domains with different read/write patterns, high-throughput systems, and event-driven architectures. The eventual consistency between models can confuse users if not handled properly (e.g., a user creates a record and does not see it immediately). Use CQRS when the complexity is justified by the domain requirements.

---

### Q5. How does CQRS relate to Domain-Driven Design (DDD)?

**Interview Answer**

CQRS maps naturally onto DDD bounded contexts — each bounded context can have its own command and query models. The write model aligns with the domain model (aggregates, entities, value objects), enforcing business invariants. The read model is shaped by the query requirements of consumers, potentially denormalized across multiple aggregate boundaries. Events published by the write model represent domain events that other bounded contexts can subscribe to. In a Rust backend, each Axum service might represent a bounded context with its own CQRS implementation, communicating through an event bus rather than shared databases.

---

### Q6. What is the difference between CQRS and event sourcing?

**Interview Answer**

CQRS and event sourcing are complementary but independent patterns. CQRS separates read and write models; event sourcing stores all state changes as an immutable sequence of events rather than the current state. You can use CQRS without event sourcing (write to a relational DB, read from Elasticsearch), or event sourcing without CQRS (single model that reads from and writes to the event store). When combined, the event store becomes the write model, and read models are projections derived from the event stream. Event sourcing naturally complements CQRS because events provide the synchronization mechanism between write and read models.

---

### Q7. How do you handle query optimization in the CQRS read model?

**Interview Answer**

The read model is designed specifically for query patterns, so you denormalize data to avoid joins. For example, instead of joining orders, users, and products, you store a single document with all the data a query needs. Use Elasticsearch for full-text search, Redis for low-latency key-value lookups, and PostgreSQL materialized views for complex analytical queries. Pre-compute expensive aggregations (counts, sums, averages) and store them in the read model. In a Rust service, use `sqlx` to query materialized views, or the `elasticsearch` crate for search queries. The read model can have multiple projections optimized for different query patterns.

---

### Q8. What are the consistency challenges in CQRS?

**Interview Answer**

The primary consistency challenge is eventual consistency between write and read models — a write might succeed but the read model might not reflect it immediately. This can cause user confusion ("I just created this, why can't I see it?"). Mitigations include: reading from the write model for the requesting user's own data, implementing read-your-writes guarantees using session-based routing, or accepting a small delay and showing a "processing" state. Another challenge is event ordering — if events arrive out of order, the read model may become inconsistent. Use sequence numbers and per-aggregate ordering to prevent this. In Rust, implement a consistency check middleware that waits for the read model to catch up before returning.

---

### Q9. When is CQRS the wrong choice?

**Interview Answer**

CQRS is wrong for simple CRUD applications where read and write patterns are similar and the domain is straightforward — it adds unnecessary complexity. It is also inappropriate when you cannot tolerate eventual consistency, such as systems requiring strict real-time consistency across all views (financial trading). Small teams may struggle with the operational overhead of maintaining multiple data stores and synchronization mechanisms. If your application's bottleneck is primarily in a single area (either reads or writes), targeted optimization (caching, read replicas) may be more cost-effective than full CQRS. Start simple and adopt CQRS when the pain of the monolithic model becomes clear.

---

### Q10. How do you handle schema evolution in CQRS read models?

**Interview Answer**

Read model schema evolution is simpler than write model evolution because the event store contains the full history. When the read model schema changes, you rebuild projections by replaying all events from the beginning (or from a snapshot). Implement versioned projections — maintain multiple versions of the read model simultaneously during migration, then switch consumers to the new version. In a Rust service, define projection handlers as versioned functions: `project_v1(event) -> ReadModelV1` and `project_v2(event) -> ReadModelV2`. Use feature flags to route queries to the old or new read model during the transition period, enabling zero-downtime schema migrations.
