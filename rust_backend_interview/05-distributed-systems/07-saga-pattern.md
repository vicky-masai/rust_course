# Saga Pattern

## Interview Question

Explain the Saga Pattern and how it manages distributed transactions.

## Interview Answer

The Saga Pattern is a way to manage distributed transactions by breaking them into a sequence of local transactions, each with a corresponding compensating action. If any step fails, the previously completed steps are rolled back by executing their compensating actions in reverse order. This avoids the need for a distributed two-phase commit (2PC), which is blocking and has poor performance characteristics. Sagas can be orchestrated (a central coordinator directs the steps) or choreographed (each service publishes events that trigger the next step). The pattern is essential in microservice architectures where each service owns its data and cannot participate in cross-service database transactions.

---

## Follow-up Questions & Answers

### Q1. What is the difference between orchestration and choreography in Sagas?

**Interview Answer**

In orchestration, a central coordinator (saga orchestrator) tells each participant what to do and handles failure by calling compensating actions. It is easier to understand, debug, and modify because the flow is centralized. In choreography, each service reacts to events from other services and decides what to do next — there is no central coordinator. Choreography is more loosely coupled and avoids a single point of failure, but the flow is distributed and harder to trace. In a Rust microservice, orchestration might use a dedicated Axum service that calls other services via HTTP, while choreography uses Kafka events where each consumer publishes events that trigger downstream consumers.

---

### Q2. How do you implement a Saga orchestrator in Rust?

**Interview Answer**

A Saga orchestrator in Rust can be implemented as a state machine using an enum for saga states and a match statement for transitions. Each step is an async function that calls the participant service and returns success or failure. On failure, the orchestrator iterates through completed steps in reverse order, calling each step's compensating action. Use `sqlx` to persist saga state in a PostgreSQL `sagas` table with columns for saga ID, current step, status, and compensation data. This ensures the orchestrator can recover from crashes by resuming from the last persisted state. A Tokio task can run the orchestrator loop, with each step's HTTP call using `reqwest`.

---

### Q3. What are the failure scenarios in a Saga and how do you handle them?

**Interview Answer**

Failures include: participant service is down (use retries with exponential backoff and circuit breakers), participant returns an error (trigger compensation for all completed steps), orchestrator crashes mid-saga (persist state in the database and resume on restart), compensation itself fails (retry compensation indefinitely with dead-letter queue for manual intervention), and network partitions cause partial execution. The most dangerous scenario is "zombie" participants that execute after a timeout — use idempotent operations so re-execution is safe. Always implement a timeout for the entire saga and a monitoring system for sagas stuck in a particular state.

---

### Q4. How does the Saga pattern differ from two-phase commit (2PC)?

**Interview Answer**

2PC ensures atomicity through a prepare phase (all participants vote yes/no) and a commit phase (all commit or all rollback). It provides strong consistency but is blocking — if the coordinator fails after the prepare phase, participants hold locks and cannot proceed. Sagas provide eventual consistency, not atomicity — each step commits independently, and compensation is used for rollback. Sagas are non-blocking and perform better but allow temporary inconsistency. 2PC is suitable for tightly coupled systems with low latency between participants; Sagas are preferred in distributed microservice architectures where availability and scalability matter more than immediate consistency.

---

### Q5. What is a compensating transaction and how do you design one?

**Interview Answer**

A compensating transaction is the logical undo of a previously completed step. It does not physically reverse the operation (you cannot "un-send" an email) but brings the system to a state as if the operation never happened from a business perspective. For example, if a payment was charged, the compensation is a refund. If an inventory item was reserved, the compensation releases the reservation. Design compensations to be idempotent and safe to retry. Store compensation data (like payment reference IDs) in the saga state so the compensating action has the information it needs. Not all actions are compensatable — for non-compensatable actions, use the "Pivot" pattern where the saga commits the critical step last.

---

### Q6. How do you handle the semantic gap between a failed operation and its compensation?

**Interview Answer**

The semantic gap arises because compensation is not a true undo — it is a new operation that approximates the reverse effect. For example, canceling an order and refunding a payment does not restore the customer's original intent or the seller's inventory position perfectly. To handle this, design sagas with explicit business semantics: store the reason for compensation, track the compensation status separately, and provide a reconciliation process for edge cases. Use a saga log that records both forward and compensation actions for audit purposes. In critical domains like finance, include a manual review step for failed compensations rather than automating everything.

---

### Q7. How do you implement the Saga pattern with Kafka and Axum microservices?

**Interview Answer**

Using choreography with Kafka: Service A publishes `OrderCreated` to Kafka. Service B (inventory) consumes it, reserves stock, and publishes `StockReserved`. Service C (payment) consumes `StockReserved`, charges the customer, and publishes `PaymentProcessed`. If Service C fails, it publishes `PaymentFailed`, which triggers Service B to consume and release the reservation. In Rust, each Axum service uses `rdkafka` to produce and consume events. The saga state is tracked implicitly through events. For orchestration, a dedicated orchestrator service calls each participant sequentially via HTTP (using `reqwest`) and manages compensation on failure, with saga state persisted in PostgreSQL.

---

### Q8. What is the problem of "dirty reads" in Sagas and how do you solve them?

**Interview Answer**

Dirty reads occur when an intermediate state of a saga is visible to other transactions. For example, after the inventory step reserves items but before payment completes, another request might see insufficient stock. Solutions include: using isolation levels in the database (serializable or repeatable read), implementing a reservation system with explicit states (reserved, confirmed, released), and using semantic locks where intermediate states are flagged as "in-progress" and excluded from normal queries. In a Rust backend, you can implement this with an `order_status` column that is 'pending' during saga execution and only changes to 'confirmed' after the saga completes.

---

### Q9. How do you monitor and debug sagas in production?

**Interview Answer**

Implement a saga log or saga state table in PostgreSQL that records every step execution, compensation, and status transition. Use distributed tracing (OpenTelemetry) with a unique saga ID propagated across all service calls — this lets you see the full saga flow in Jaeger or Grafana Tempo. Set up alerts for sagas stuck in a particular state for longer than the expected duration. Use structured logging in each Rust service with the saga ID as a field for easy log aggregation in Loki or ELK. Create a dashboard showing saga completion rates, failure rates, and average duration per step to identify bottlenecks and failures.

---

### Q10. When should you avoid the Saga pattern?

**Interview Answer**

Avoid Sagas when you need strong atomicity guarantees — if the business requires that all steps either all succeed or all fail atomically, use 2PC or a single database instead. Sagas add significant complexity: you must design compensating actions, handle partial failures, manage eventual consistency, and implement monitoring. For simple two-step operations, a compensating transaction in a single service might suffice without full saga infrastructure. If your system is not distributed (single database), use database transactions instead. Sagas are justified when you have multiple services that must coordinate across service boundaries and cannot share a database transaction.
