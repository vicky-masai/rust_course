# What is a Circuit Breaker?

## Interview Question

What is a circuit breaker and why is it important in distributed systems?

## Interview Answer

A circuit breaker is a resilience pattern that prevents an application from repeatedly calling a failing service, allowing the downstream service time to recover. It monitors the success/failure rate of requests and transitions between three states: Closed (normal operation, requests pass through), Open (too many failures, all requests are immediately rejected without calling the service), and Half-Open (after a cooldown, a limited number of test requests are sent to check if the service has recovered). Without circuit breakers, a failing service causes cascading failures as callers wait for timeouts, exhausting connection pools and thread resources across the entire system.

---

## Follow-up Questions & Answers

### Q1. What are the three states of a circuit breaker and what triggers transitions between them?

**Interview Answer**

**Closed** is the normal state — all requests pass through, and the breaker tracks failures. When the failure count exceeds a threshold (e.g., 5 consecutive failures or 50% failure rate in a window), it transitions to **Open**. In the **Open** state, all requests are immediately rejected with an error (no call to the downstream service) for a configurable timeout (e.g., 30 seconds). After the timeout, it transitions to **Half-Open**, where a limited number of test requests are sent. If they succeed, the breaker closes; if they fail, it reopens. This tri-state mechanism prevents cascade failures while allowing automatic recovery.

---

### Q2. How would you implement a circuit breaker in Rust with Axum?

**Interview Answer**

In Rust, you can implement a circuit breaker using an `AtomicU64` for the failure counter and an `AtomicU64` for the last failure timestamp, protected by `std::sync::atomic` ordering. Wrap outgoing HTTP calls (using `reqwest`) in a middleware that checks the circuit state before making the call. Libraries like `tower` allow you to create a `CircuitBreakerLayer` that sits in the Axum middleware stack. The state machine logic: in Closed state, call the service and increment the failure counter on error. In Open state, check if the cooldown has elapsed — if yes, transition to Half-Open and allow one test request. In Half-Open, if the test succeeds, reset the counter and close; if it fails, reopen.

---

### Q3. What metrics should a circuit breaker track for monitoring?

**Interview Answer**

Track the circuit state (closed/open/half-open) as a gauge, the number of requests allowed versus rejected, the failure rate percentage, the average response time of downstream calls, and the duration the circuit has been open. Record state transitions as events with timestamps for debugging. Use Prometheus metrics in your Rust service: `circuit_breaker_state{service="payment"} 0` (0=closed, 1=open, 2=half-open), `circuit_breaker_rejected_total{service="payment"} 150`, `circuit_breaker_allowed_total{service="payment"} 1000`. Create Grafana dashboards to visualize circuit breaker health across all downstream services and set alerts for circuits that remain open for extended periods.

---

### Q4. What is the difference between a circuit breaker and a rate limiter?

**Interview Answer**

A circuit breaker protects against failing downstream services by stopping all traffic when failures exceed a threshold — it is a binary on/off mechanism based on failure rate. A rate limiter protects against excessive traffic by capping the number of requests per time window regardless of whether they succeed or fail. They serve different purposes: a circuit breaker prevents cascade failures from unhealthy services, while a rate limiter prevents overload of healthy services. In practice, you often use both: a rate limiter at the API gateway to control incoming traffic, and circuit breakers on each outbound service call to handle downstream failures. They are complementary, not competing patterns.

---

### Q5. What are the common configuration pitfalls with circuit breakers?

**Interview Answer**

Setting the failure threshold too low causes the circuit to open on transient errors (a single network blip), while setting it too high allows prolonged cascade failures. The cooldown period must be long enough for the downstream service to recover but short enough to avoid unnecessary downtime. Ignoring error types — rejecting all errors equally including 4xx client errors — causes the circuit to open when the problem is with the caller, not the service. Not tracking the half-open state properly leads to recovery taking too long or too fast. In a Rust service, separate server errors (5xx) from client errors (4xx) when counting failures — only 5xx and timeouts should count toward the circuit breaker threshold.

---

### Q6. How do circuit breakers interact with retry policies?

**Interview Answer**

Retries and circuit breakers work together but must be configured carefully. Retries should only happen while the circuit is closed — once it opens, retries should be stopped immediately to avoid overwhelming the failing service. Use exponential backoff with jitter for retries to prevent thundering herd when the circuit transitions to half-open. The retry budget should be separate from the circuit breaker threshold — e.g., retry up to 3 times with exponential backoff, and the circuit breaker opens after 5 consecutive failures (across all retry attempts). In Rust, use `tower::retry::Retry` with a `RetryPolicy` that checks the circuit state before each attempt.

---

### Q7. What is a fallback strategy when the circuit breaker is open?

**Interview Answer**

When the circuit is open and requests are rejected, the service should return a meaningful fallback response rather than a generic error. Strategies include: returning cached data (e.g., a product catalog from Redis), providing a degraded response (showing a "temporarily unavailable" message with partial data), using a secondary service (failover to a backup), or queuing the request for later processing. In a Rust Axum backend, implement fallbacks as middleware that catches circuit breaker rejections and returns an appropriate response — for example, a cached product listing from Redis when the recommendation service is down, with a header indicating the response may be stale.

---

### Q8. How do distributed circuit breakers differ from local circuit breakers?

**Interview Answer**

A local circuit breaker tracks failures from a single service instance — it might be closed on one instance but open on another if only some instances are experiencing failures. A distributed circuit breaker shares state across all instances using a centralized store (Redis or a dedicated service mesh). In a microservice with 10 instances, local circuit breakers might allow 10 instances to each independently detect the failure at different times, causing inconsistent behavior. Distributed circuit breakers (implemented via Envoy/Istio service mesh or a shared Redis state) ensure all instances open the circuit simultaneously, providing uniform protection. The trade-off is the added latency and dependency on the shared state store.

---

### Q9. What is a bulkhead pattern and how does it complement circuit breakers?

**Interview Answer**

The bulkhead pattern isolates resources (threads, connection pools, memory) for each downstream service, preventing one failing service from consuming all resources. While circuit breakers stop requests to failing services, bulkheads limit the damage of a slow or partially failing service that has not yet tripped the circuit. For example, if the payment service is slow, without bulkheads it might consume all available threads, preventing requests to the notification service from being processed. In Rust, use separate Tokio runtime pools or bounded channels for each downstream service, so one slow service cannot starve others. Combined with circuit breakers, bulkheads provide defense in depth against cascade failures.

---

### Q10. How do you test circuit breaker behavior?

**Interview Answer**

Unit test the state machine transitions: simulate failures to verify the breaker opens at the threshold, time advancement to verify half-open transitions, and successful recovery to verify it closes. Use mock services that return configurable error rates. Integration test with real downstream services using fault injection (Toxiproxy to simulate latency and errors). Load test with concurrent requests to verify the circuit opens under sustained failure and recovers when the service heals. In Rust, use `tokio::time::pause()` to simulate time advancement without real delays, and `mockall` to create mock service clients that return errors at configurable rates. Verify metrics are correctly emitted at each state transition.
