# Saga Orchestration with Kafka

## Interview Question

How do you implement the Saga pattern for distributed transactions using Kafka, and what are the trade-offs?

## Interview Answer

The Saga pattern manages distributed transactions as a sequence of local transactions, each with a compensating action for rollback. In Kafka, an orchestrator service coordinates the saga by publishing commands to topic-per-step and listening for success/failure events. On failure, the orchestrator publishes compensating commands to undo completed steps. Kafka's event log provides an audit trail of saga execution and compensations. The orchestration approach uses a central coordinator; the choreography approach uses events to trigger next steps without a coordinator. In Rust, the orchestrator is an Axum service producing to step-specific topics.

---

## Follow-up Questions & Answers

### Q1. What is the difference between orchestration and choreography in the Saga pattern?

**Interview Answer**

**Orchestration** uses a central coordinator (orchestrator) that explicitly tells each service what to do and handles compensations. The orchestrator maintains the saga state and knows the full workflow. **Choreography** uses events; each service publishes an event that triggers the next service's action, with no central coordinator. Orchestration is easier to understand, debug, and modify; choreography is more decoupled but harder to track and troubleshoot. For complex sagas with many steps, orchestration is preferred. Kafka's event log naturally supports choreography (events trigger next steps) but orchestration requires a dedicated saga service.

---

### Q2. How do you implement compensation actions in Kafka-based sagas?

**Interview Answer**

Each saga step defines a forward action (e.g., `ReserveInventory`) and a compensating action (e.g., `ReleaseInventory`). When a step fails, the orchestrator publishes compensating commands in reverse order of completion. For example, if steps A, B, C succeed and D fails, compensate C, then B, then A. Each service must be idempotent and handle both forward and compensating commands. In Kafka, compensation commands go to the same topic as forward commands, differentiated by message type. In Rust, the orchestrator maintains saga state (which steps completed) and publishes compensation events on failure.

---

### Q3. What happens if a compensation action fails?

**Interview Answer**

If a compensation fails, the saga is in an inconsistent state. Strategies: (1) **Retry with exponential backoff** - most compensation failures are transient (network, database lock); (2) **Dead letter queue** - route failed compensations to a DLQ for manual intervention; (3) **Saga log** - maintain a persistent log of saga state and compensation status; (4) **Alerting** - trigger immediate alerts for failed compensations. The key is to never give up on compensation; a failed compensation means the system is inconsistent. In Rust, implement a retry loop with a DLQ fallback and persistent saga state in the database.

---

### Q4. How do you track saga state and progress in Kafka?

**Interview Answer**

Saga state can be tracked by: (1) **Saga log** - a dedicated Kafka topic where the orchestrator publishes state transitions (StepStarted, StepCompleted, StepFailed, Compensating, Completed); (2) **Database table** - store saga ID, current step, status, and timestamps; (3) **Event sourcing** - reconstruct saga state by replaying its events. The saga log approach leverages Kafka's retention for audit trails. In Rust, publish state transitions to a `saga-log` topic and maintain a local cache for fast access. The saga log enables debugging by replaying the full execution history.

---

### Q5. How do you handle timeouts in Kafka-based sagas?

**Interview Answer**

Saga step timeouts prevent indefinite waiting. Implement with: (1) **Per-step timeout** - the orchestrator starts a timer when issuing a step command; if no response within timeout, trigger compensation; (2) **Kafka message timestamps** - embed deadline in message headers; consumer checks if expired; (3) **Scheduled compensation** - a timer task checks for stalled sagas. The `transaction.timeout.ms` in Kafka transactions limits how long a saga step can take. In Rust, use `tokio::time::timeout` around consumer processing and `tokio::spawn` for background compensation timers.

---

### Q6. How do you ensure idempotency in saga participants?

**Interview Answer**

Each saga participant must handle duplicate commands idempotently because Kafka provides at-least-once delivery. Implement: (1) **Unique saga step ID** - each command carries a unique step ID; the participant checks if already processed; (2) **Database upserts** - use unique constraints on saga step ID to prevent duplicates; (3) **Idempotency keys** - store processed keys in Redis with TTL. In Rust, include `saga_id + step_id` in the message key and use database `INSERT ... ON CONFLICT DO NOTHING`. This ensures that retries during compensation don't cause duplicate side effects.

---

### Q7. How do you handle the orchestrator itself failing mid-saga?

**Interview Answer**

If the orchestrator crashes: (1) **Saga log in Kafka** - the new orchestrator instance reads the saga log to determine which sagas were in progress; (2) **Database state** - the orchestrator's database tracks saga state and can resume; (3) **Timeout-based recovery** - other participants time out and trigger compensations independently. The orchestrator must be stateless or persist state externally. In Rust, the orchestrator reads the `saga-log` topic on startup to identify incomplete sagas and resumes or compensates them. This requires the orchestrator to handle duplicate processing of its own commands.

---

### Q8. How do you design the Kafka topics for a saga-based system?

**Interview Answer**

Topic design: (1) **Step topics** - one topic per saga step (e.g., `saga.reserve-inventory`, `saga.process-payment`); (2) **Saga log topic** - stores all saga state transitions for audit; (3) **Compensation topic** - dedicated topic for compensation commands (optional, can use step topics); (4) **DLQ topics** - per-step DLQ for failed messages. Use message keys with `saga_id` for ordering. Each step topic has a consumer group for the participant service. In Rust, the orchestrator produces to step topics and consumes from the saga log topic for state tracking.

---

### Q9. What are the alternatives to Saga pattern in distributed systems?

**Interview Answer**

Alternatives: (1) **Two-phase commit (2PC)** - strong consistency but blocking and coordinator-dependent; (2) **Event sourcing** - append-only event log with rebuild capability; (3) **TCC (Try-Confirm-Cancel)** - two-phase with explicit try/confirm/cancel; (4) **Process orchestration** - workflow engines like Camunda or Temporal. 2PC is rarely suitable for microservices due to tight coupling. Event sourcing with Kafka provides similar guarantees to sagas without explicit compensation. Temporal provides saga-like workflows with better developer experience. For Kafka-based systems, sagas or event sourcing are the most natural patterns.

---

### Q10. How do you test Kafka-based sagas in Rust?

**Interview Answer**

Test sagas using: (1) **Integration tests with testcontainers** - spin up Kafka and run the full saga flow; (2) **Unit test orchestrator logic** by mocking the Kafka producer/consumer; (3) **Chaos testing** - inject failures at each step to verify compensation triggers; (4) **State machine tests** - verify all valid saga state transitions. Use `rstest` for parameterized testing of success, failure, and partial failure scenarios. Test compensation by verifying database state after simulated failures. In Rust, abstract the Kafka client behind a trait and use `mockall` to simulate failures and verify the orchestrator's compensation logic.
