# Event-Driven Architecture

## Interview Question

What is Event-Driven Architecture (EDA) and what are its core patterns?

## Interview Answer

Event-Driven Architecture is a software design pattern where services communicate by producing and consuming events rather than making synchronous API calls. An event represents a fact that occurred (e.g., `OrderCreated`, `PaymentProcessed`) and is published to a broker like Kafka. Consumers react to events independently, enabling loose coupling and horizontal scalability. EDA supports patterns like event sourcing, CQRS, and the saga pattern. In Rust microservices, EDA is implemented using `rdkafka` with Axum services acting as both producers and consumers.

---

## Follow-up Questions & Answers

### Q1. What are the three core components of Event-Driven Architecture?

**Interview Answer**

The three core components are: (1) **Event Producers** - services that detect state changes and publish events to a broker; (2) **Event Broker** - infrastructure like Kafka that stores and routes events; (3) **Event Consumers** - services that subscribe to events and react accordingly. Additional components include event channels (topics) and event stores (for event sourcing). The decoupling between producer and consumer means neither needs to know about the other's existence at compile time. This enables independent deployment and scaling of each service.

---

### Q2. What is the difference between event notification, event-carried state transfer, and event sourcing?

**Interview Answer**

**Event notification** carries minimal data (an ID and event type), requiring consumers to query the producer's database for full state. **Event-carried state transfer** embeds the full state in the event, eliminating the need for callback queries but increasing payload size. **Event sourcing** persists every state change as an immutable event, reconstructing state by replaying the event stream. Event sourcing provides a complete audit trail and supports temporal queries. Most production systems use a hybrid of these patterns.

---

### Q3. What are common pitfalls of Event-Driven Architecture?

**Interview Answer**

The main pitfalls are: (1) **Eventual consistency complexity** - developers must reason about systems that are not instantly consistent; (2) **Circular event dependencies** - events triggering other events in loops causing cascading failures; (3) **Debugging difficulty** - tracing a request across multiple async event handlers requires distributed tracing (OpenTelemetry); (4) **Ordering guarantees** - events may arrive out of order across partitions. Using idempotent consumers and correlation IDs in events mitigates many of these issues. The overhead is justified only when the scalability and decoupling benefits outweigh the complexity.

---

### Q4. How does Event Sourcing work with Kafka and what are the trade-offs?

**Interview Answer**

In event sourcing with Kafka, each aggregate's state changes are appended as events to a dedicated topic. A consumer applies events in order to reconstruct current state, typically stored in a materialized view or snapshot database. Kafka's log retention and replay capability make it a natural fit for event sourcing. Trade-offs include increased storage requirements, complexity in handling schema evolution of historical events, and eventual consistency of read models. Snapshots at periodic intervals reduce replay time for aggregates with long event histories.

---

### Q5. How do you implement CQRS with Kafka?

**Interview Answer**

Command Query Responsibility Segregation (CQRS) separates write and read models. Commands (writes) are published as events to Kafka topics. A consumer processes these events to build optimized read models (e.g., denormalized views in Elasticsearch or Redis). The write side uses the command model, while the read side uses a query-optimized model. Kafka acts as the bridge ensuring the read model stays eventually consistent with the write model. In Rust, the Axum write service produces events while a separate read service consumes and builds projections.

---

### Q6. What role does idempotency play in Event-Driven systems?

**Interview Answer**

Idempotency ensures that processing the same event multiple times produces the same result, which is critical because Kafka guarantees at-least-once delivery. Without idempotency, duplicate events cause duplicate side effects like double charges or duplicate order confirmations. Techniques include storing processed event IDs in a deduplication table, using natural unique keys in database upserts, and leveraging Kafka's idempotent producer. In Rust, database upserts with unique constraints on event IDs are a common idempotency strategy. Idempotency is non-negotiable for production EDA systems.

---

### Q7. How does Event-Driven Architecture improve scalability compared to synchronous REST APIs?

**Interview Answer**

Synchronous REST calls create tight coupling where the caller blocks until the callee responds, limiting throughput to the slowest service. EDA allows producers to publish events and continue immediately, while consumers process at their own pace. Kafka absorbs traffic spikes by buffering events in partitions. Multiple consumer instances can scale horizontally without coordination. This eliminates the "waterfall" bottleneck where Service A waits for B waits for C. The trade-off is eventual consistency, which is acceptable for most business domains.

---

### Q8. How do you handle event versioning in EDA?

**Interview Answer**

Event versioning uses a version field in the event envelope (e.g., `v1`, `v2`) alongside an event type identifier. Consumers must handle multiple versions simultaneously during rolling deployments. The strategy includes: (1) backward-compatible changes (adding optional fields) require no version bump; (2) breaking changes create a new version topic or use schema registry compatibility modes; (3) a consumer version router dispatches to the correct handler. Confluent Schema Registry with BACKWARD compatibility ensures old consumers can read new events. Documenting the event contract is as important as API contracts in REST systems.

---

### Q9. What is the role of correlation IDs and distributed tracing in EDA?

**Interview Answer**

A correlation ID is a UUID attached to an event that propagates through all downstream events, enabling end-to-end traceability across services. Without it, debugging a single user request that touches five services becomes nearly impossible. Distributed tracing tools like OpenTelemetry use the correlation ID to build a request timeline. In Kafka, the correlation ID is stored in event headers. Rust services propagate headers through `rdkafka` producer/consumer message metadata. This is essential for production debugging and SLA monitoring in EDA systems.

---

### Q10. When should you NOT use Event-Driven Architecture?

**Interview Answer**

Avoid EDA when: (1) strong consistency is required (e.g., financial balance calculations) where synchronous calls are safer; (2) the system is simple with only 2-3 services where REST is sufficient; (3) low-latency request-response is needed (EDA adds async overhead); (4) the team lacks experience with distributed systems concepts like eventual consistency. EDA adds operational complexity (Kafka cluster management, consumer lag monitoring) that is not justified for small or early-stage systems. Start with synchronous communication and adopt EDA when scale demands it.
