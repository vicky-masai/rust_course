# What is Idempotency?

## Interview Question

What is idempotency and why is it important in distributed systems?

## Interview Answer

Idempotency means that performing an operation multiple times produces the same result as performing it once. In distributed systems, this is critical because network retries, message queue re-deliveries, and duplicate requests are inevitable. Without idempotency, a retried payment could charge a customer twice, or a duplicated order could create two shipments. Idempotent operations make systems resilient to exactly-once delivery failures, which are extremely difficult to guarantee in distributed environments. Every API endpoint and message handler that causes a state change should be designed to be idempotent.

---

## Follow-up Questions & Answers

### Q1. What is the difference between idempotent, safe, and read-only operations?

**Interview Answer**

A safe operation has no side effects — calling it once or many times produces no observable change (like a GET request fetching data). An idempotent operation has side effects, but repeating it yields the same result as calling it once (like a DELETE request or setting a value to a specific state). A read-only operation is both safe and idempotent. For example, `PUT /users/123 {name: "Alice"}` is idempotent because it always sets the name to Alice regardless of how many times it is called, while `POST /users {name: "Alice"}` is not idempotent because it creates a new user each time.

---

### Q2. How do HTTP methods relate to idempotency?

**Interview Answer**

The HTTP specification defines GET, PUT, DELETE, HEAD, and OPTIONS as idempotent, meaning multiple identical requests should have the same effect as a single request. POST and PATCH are not inherently idempotent — a POST to `/orders` might create duplicate orders if called twice. However, you can make POST idempotent by accepting a client-generated `Idempotency-Key` header, as Stripe does with its API. PUT is naturally idempotent because it replaces the resource entirely. Understanding this distinction is important when designing REST APIs — you should prefer PUT over POST for update operations to get idempotency for free.

---

### Q3. How do you implement idempotency with a client-generated UUID in Rust?

**Interview Answer**

In a Rust Axum backend, the client generates a UUID and sends it as a header or body field. On the server side, before processing the request, you check Redis or a database table for the idempotency key. If found, you return the stored response. If not found, you process the request, store the result keyed by the UUID, and return it. This uses `SET NX EX` in Redis for atomic check-and-set with a TTL for automatic expiration. The key insight is that the check, process, and store must happen atomically or within a transaction to prevent race conditions from duplicate concurrent requests.

---

### Q4. What happens when an idempotency key expires before a retry arrives?

**Interview Answer**

If the TTL on an idempotency key expires and a retry arrives, the system will treat it as a new request and process it again, potentially causing duplicate state changes. To mitigate this, the TTL should be generous — at least as long as the maximum retry window. For payment systems, this might be 24-72 hours. Alternatively, you can use the database as the source of truth rather than Redis, storing the idempotency key in a table with no TTL, though this requires cleanup. Some systems use a two-phase approach: store the key on first request, then finalize the TTL only after the operation completes successfully.

---

### Q5. How does idempotency interact with message queues like Kafka?

**Interview Answer**

Kafka provides at-least-once delivery by default, meaning messages can be delivered to consumers more than once after rebalancing or producer retries. Idempotent consumers handle this by tracking processed message IDs or using the message key as an idempotency reference. With Kafka exactly-once semantics (enabled via idempotent producers and transactional consumers), you get deduplication at the broker level, but it adds complexity. In practice, designing consumers to be idempotent is more reliable and portable — you can switch messaging systems without changing your application logic. Each message handler should check a processed-messages table before applying side effects.

---

### Q6. What is the difference between idempotency and deduplication?

**Interview Answer**

Deduplication is the process of detecting and removing duplicate messages or requests, typically at the infrastructure level (Kafka deduplication, database unique constraints). Idempotency is an application-level design property where operations naturally produce the same result when repeated. Deduplication prevents duplicates from being processed at all, while idempotency ensures that even if duplicates reach your application, they do not cause incorrect behavior. The best approach is defense in depth: use deduplication where possible at the infrastructure level, and design operations to be idempotent at the application level.

---

### Q7. How do database transactions help or hinder idempotency?

**Interview Answer**

Database transactions ensure atomicity but do not automatically provide idempotency. A transaction that inserts a row will fail with a unique constraint violation on the second attempt, which is a form of idempotency enforcement. However, conditional inserts using `INSERT ... ON CONFLICT DO NOTHING` are explicitly idempotent. Transactions hinder idempotency when you mix idempotent and non-idempotent operations within the same transaction — for example, inserting a record and sending an email. The email is not idempotent, so the transaction alone cannot solve the problem. The outbox pattern addresses this by ensuring the email is sent exactly once via a background worker.

---

### Q8. Why is idempotency critical for payment processing systems?

**Interview Answer**

Payment systems operate in unreliable network environments where requests may timeout, retries may occur, and infrastructure failures may cause duplicate processing. Without idempotency, a single payment attempt could result in multiple charges to a customer. Stripe uses idempotency keys for every API call, and their documentation explicitly states that you should reuse keys for retries. In a Rust payment service, you would generate an idempotency key per transaction, store the payment result (success/failure/reference) in PostgreSQL with a unique constraint on the key, and return the stored result for any duplicate requests. This guarantees exactly-once billing even with retries.

---

### Q9. What are the challenges of making read-modify-write operations idempotent?

**Interview Answer**

Read-modify-write operations (like incrementing a counter or appending to a list) are inherently non-idempotent because each execution changes the state. To make them idempotent, you need to track whether the operation has already been applied — for example, storing the operation ID in a table and checking before execution. Alternatively, you can transform them into pure writes: instead of "increment by 5", send "set to 15" which is naturally idempotent. Redis provides `SET` with a known value as an idempotent alternative to `INCR`. In distributed systems, vector clocks or operation logs can help distinguish new operations from retries of the same logical operation.

---

### Q10. How do you handle idempotency in a microservices architecture?

**Interview Answer**

In microservices, idempotency must be enforced at every service boundary because message re-delivery can occur at any inter-service communication layer. Each service should maintain its own idempotency key storage (typically Redis or a dedicated table). When Service A calls Service B with an idempotency key, Service B checks its own store before processing. For event-driven architectures, the event ID from Kafka or RabbitMQ serves as a natural idempotency key. In a Rust microservice, you might use a middleware layer in Axum that checks the idempotency key before the request reaches the handler, storing results in PostgreSQL with a unique index on the key.
