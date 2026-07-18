# Microservices vs Monolith

## Interview Question

Microservices vs Monolith.

## Interview Answer

"Monoliths are simpler to build and deploy initially. Microservices provide independent scalability and deployments but add operational complexity."

---

## Follow-up Questions & Answers

### Q1. When would you recommend starting with a monolith over microservices?

**Interview Answer**

I recommend a monolith when the team is small, the domain is not yet fully understood, or the product is in early stages. A well-structured monolith with clear module boundaries can be decomposed into services later when scaling demands it. Starting with microservices too early adds operational overhead like service discovery, distributed tracing, and inter-service communication that slows down development.

---

### Q2. What does a "modular monolith" look like in a Rust/Axum project?

**Interview Answer**

I organize the codebase into domain modules like `users`, `orders`, and `payments` with clear boundaries enforced through Rust's module system. Each module has its own handlers, services, and repositories, and modules communicate through well-defined service interfaces rather than reaching into each other's internals. This makes it straightforward to extract a module into a separate service when needed.

---

### Q3. What are the biggest operational challenges with microservices?

**Interview Answer**

You need distributed tracing to follow requests across services, centralized logging to aggregate logs from multiple instances, service discovery for dynamic endpoints, and circuit breakers to handle partial failures. Debugging is harder because a single user request may touch five services. You also need separate CI/CD pipelines, database per service, and monitoring for each deployment.

---

### Q4. How do you handle inter-service communication in a microservices architecture?

**Interview Answer**

I prefer asynchronous communication through message brokers like RabbitMQ or Kafka for most interactions because it decouples services and handles failures gracefully. For synchronous calls where the client needs an immediate response, I use HTTP or gRPC with timeouts and retries. I avoid synchronous chains deeper than two services to prevent cascading failures.

---

### Q5. What is the strangler fig pattern and how does it relate to migrating from monolith to microservices?

**Interview Answer**

The strangler fig pattern involves gradually replacing parts of a monolith with new services while the old code continues to run. You route specific requests to the new service and keep the rest in the monolith. Over time, more functionality migrates until the monolith can be decommissioned. This reduces risk compared to a full rewrite because you can validate each piece independently.

---

### Q6. How do you decide which pieces of a monolith to extract into services?

**Interview Answer**

I look for modules with clear boundaries, independent data models, and different scaling requirements. For example, an image processing service that needs GPU resources is a good extraction candidate. I avoid extracting tightly coupled modules that share the same database tables, as that creates distributed transactions which are complex and slow.

---

### Q7. How does Rust fit into a microservices architecture?

**Interview Answer**

Rust's small binary size and fast startup make it ideal for containerized microservices where quick scaling matters. The low memory footprint means you can pack more service instances onto the same infrastructure compared to Java or Go. Axum with Tokio handles high-concurrency workloads efficiently, and the type system catches integration errors at compile time when sharing types across services.

---

### Q8. What monitoring do you set up for microservices?

**Interview Answer**

I use Prometheus for metrics like request latency, error rates, and throughput per service, Grafana for dashboards, and structured JSON logs aggregated into something like Loki or Elasticsearch. Distributed tracing with OpenTelemetry follows requests across service boundaries. Alerting on error rate spikes and p99 latency ensures issues are caught before users are significantly impacted.

---
