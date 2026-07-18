# Current Project Architecture

## Interview Question

Explain your current project architecture.

## Interview Answer

> "Currently, I'm working on an enterprise Warehouse Management System (WMS) built using Rust. The backend follows a modular architecture where each business domain, such as Inventory, Orders, Users, Warehouses, and Authentication, is organized into separate modules while sharing a single deployment and database.
>
> We expose REST APIs using Axum, and Tokio handles asynchronous operations. Every request first passes through middleware for logging, request IDs, authentication, and authorization before reaching the business logic.
>
> PostgreSQL is our primary database for transactional data, and Redis is used for caching frequently accessed information and improving response times. For long-running operations such as notifications and background processing, we use asynchronous workers instead of blocking API requests.
>
> The application is containerized using Docker, deployed behind Nginx, and integrated with CI/CD pipelines. We also use structured logging and monitoring to troubleshoot production issues.
>
> While designing the system, our main goals are maintainability, scalability, and performance."

---

## Follow-up Questions & Answers

### Q1. Why did you choose a modular monolith instead of microservices?

**Answer:**
"At our current stage, a modular monolith provides simpler deployments, easier debugging, shared transactions, and lower operational complexity. Since modules are clearly separated, we can extract them into microservices later if needed."

---

### Q2. Why PostgreSQL?

**Answer:**
"It provides strong ACID guarantees, supports complex queries, transactions, indexing, JSONB, and is well suited for enterprise transactional systems."

---

### Q3. Why Redis?

**Answer:**
"Redis reduces database load by caching frequently accessed data. We also use it for sessions, rate limiting, and distributed locking where needed."

---

### Q4. Why background workers?

**Answer:**
"Operations like sending emails or notifications can take time. Running them asynchronously prevents blocking the user's request and improves API response time."

---

### Q5. How do you secure the APIs?

**Answer:**
"We use HTTPS, JWT authentication, RBAC authorization, input validation, parameterized SQL queries, rate limiting, structured audit logs, and secure secret management."

---
