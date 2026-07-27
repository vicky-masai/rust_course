# LEVEL 20 — Enterprise Backend Architecture

### 0392. Repository Pattern

Hide data access behind an interface (`UserRepository`) so domain logic doesn't know SQL/Redis details. Enables testing with fakes; can become anemic if over-abstracted.

**Talk track:** *"Repositories isolate persistence — domain talks to ports, not SQL strings everywhere."*

---

### 0393. Unit of Work

Track a business transaction's changes and commit once — one DB transaction spanning multiple repository calls. Prevents partial writes across aggregates.

**Talk track:** *"Unit of Work batches work into one commit — one business action, one atomic write when possible."*

---

### 0394. Domain Driven Design (DDD)

Model software around the business domain: ubiquitous language, bounded contexts, aggregates, entities vs value objects. Best when business rules are complex — overkill for simple CRUD.

**Talk track:** *"DDD aligns code with business language and boundaries — use it where complexity is in the domain, not the framework."*

---

### 0395. Hexagonal Architecture

Ports and adapters: core domain in the center; UI, DB, messaging are adapters. Dependencies point inward. Same as "ports & adapters."

**Talk track:** *"Hexagonal keeps the domain independent — swap DB or HTTP without rewriting business rules."*

---

### 0396. Onion Architecture

Layered rings: domain center, then application services, then infrastructure. Similar goals to hexagonal with a concentric visual model.

**Talk track:** *"Onion layers dependencies so infrastructure is outer and replaceable."*

---

### 0397. Clean Architecture

Uncle Bob's framing: entities, use cases, interface adapters, frameworks. Business rules don't depend on frameworks. Same dependency rule as hex/onion.

**Talk track:** *"Clean Architecture is the dependency rule: frameworks depend on use cases, not the reverse."*

---

### 0398. Modular Monolith

One deployable unit, strong internal module boundaries (packages/crates). Gets modularity without distributed systems tax. Often the right staff default before microservices.

**Talk track:** *"Modular monoliths give boundaries without network failure modes — scale the team first, the network later."*

---

### 0399. Microservices

Independently deployable services owning their data. Team autonomy and scale isolation; pay with latency, ops, distributed transactions, and debugging cost.

**Talk track:** *"Microservices buy independent deploy/scale — you pay with distributed complexity. Earn them."*

---

### 0400. Service Oriented Architecture

Older broader style of networked services with enterprise buses and contracts. Microservices are a cloud-native evolution with different cultural norms (smart endpoints, dumb pipes).

**Talk track:** *"SOA was service integration at enterprise scale — microservices refined the operational model."*

---

### 0401. Workflow Engines

Orchestrate long-running multi-step processes (Temporal, Cadence, Camunda). Durable state, retries, timers. Better than hand-rolled state machines for complex sagas.

**Talk track:** *"Workflow engines make long-running business processes durable and visible."*
