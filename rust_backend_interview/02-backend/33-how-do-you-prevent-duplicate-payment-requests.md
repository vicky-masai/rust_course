# How do you prevent duplicate payment requests?

## Interview Question

How do you prevent duplicate payment requests?

## Interview Answer

Use an **Idempotency-Key**, store the request status in Redis or the database, and return the original response for retries.

---

## Follow-up Questions & Answers

### Q1. How do you implement idempotency keys in an Axum payment endpoint?

**Interview Answer**

Generate a unique idempotency key client-side or server-side and include it in the request header. Before processing, check Redis or a database table for the key; if found, return the stored response. If not, process the payment, store the result with the key, and return the new response.

---

### Q2. How long should idempotency keys be stored?

**Interview Answer**

Store keys for at least 24-48 hours to cover client retries and network delays. Use Redis TTL with `SETEX` to automatically expire old keys. For critical financial systems, store in the database permanently for audit purposes and compliance requirements.

---

### Q3. What happens if two requests with the same idempotency key arrive simultaneously?

**Interview Answer**

Use Redis `SETNX` or database unique constraints to ensure only one request processes the key. The second request should wait for the first to complete and return the stored result. Implement this with distributed locking using `redis::SET` with NX and EX options.

---

### Q4. How do you handle idempotency key validation failures?

**Interview Answer**

Return `409 Conflict` if a key is reused with different request parameters, indicating a potential error. Return the original response if the key matches an existing successful request. Log the mismatch for monitoring and alert on repeated key conflicts.

---

### Q5. What is the difference between client-generated and server-generated idempotency keys?

**Interview Answer**

Client-generated keys allow clients to control retry behavior and prevent duplicates across network failures. Server-generated keys are simpler for clients but require the server to deduplicate based on request content. Most payment APIs use client-generated keys via `Idempotency-Key` header.

---

### Q6. How does idempotency interact with database transactions?

**Interview Answer**

Store the idempotency key in the same transaction as the payment record to ensure atomicity. If the transaction rolls back, the key isn't stored and retries will process correctly. Use `sqlx::Transaction` in Axum to coordinate key storage with payment creation.

---

### Q7. How do you prevent duplicate payments from Kafka message retries?

**Interview Answer**

Use idempotent consumers that check for processed message IDs before executing payment logic. Store processed IDs in Redis with TTL or in the database with unique constraints. Configure Kafka consumer `enable.auto.commit=false` and commit offsets only after successful processing.

---

### Q8. What monitoring should you set up for idempotency?

**Interview Answer**

Track idempotency key hit rates, conflict rates, and processing times using Prometheus metrics. Alert on high conflict rates which may indicate client bugs or malicious retries. Log idempotency events with `tracing` to correlate with payment audit trails.
