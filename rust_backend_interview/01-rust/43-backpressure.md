# backpressure

## Interview Question

Explain backpressure.

## Interview Answer

"Backpressure prevents producers from overwhelming consumers by slowing message production or buffering until consumers catch up."

---

## Follow-up Questions & Answers

### Q1. What are the main strategies for implementing backpressure?

**Interview Answer**

Common strategies include bounded channels that block or drop when full, rate limiting at the producer, load shedding to reject excess requests, and adaptive concurrency limits. In Rust, `tokio::sync::mpsc::channel(cap)` provides a bounded channel that naturally applies backpressure when the buffer fills up and the sender awaits.

---

### Q2. How does `tokio::sync::mpsc::bounded` provide backpressure?

**Interview Answer**

A bounded channel has a fixed capacity. When the buffer is full, `sender.send().await` suspends until the receiver consumes a message. This naturally slows down producers to match consumer throughput. The capacity should be tuned—too small wastes CPU on context switching, too large causes memory bloat under load spikes.

---

### Q3. What is load shedding and when should you use it?

**Interview Answer**

Load shedding explicitly rejects requests when the system is overloaded, rather than queuing them. It's useful when requests have deadlines—queuing a request that will timeout anyway wastes resources. In Rust, you can use `tokio::sync::Semaphore` to limit concurrent requests and return 530 Service Unavailable when the semaphore is full.

---

### Q4. How do you implement backpressure in a web service?

**Interview Answer**

Use a semaphore to limit concurrent request handlers: `let sem = Arc::new(Semaphore::new(MAX_CONCURRENT));`. Each handler acquires a permit before processing. Alternatively, use `tower::limit::ConcurrencyLimitLayer` in Axum to cap concurrent requests. For downstream services, implement retry with exponential backoff and circuit breakers.

---

### Q5. What is the difference between bounded and unbounded channels?

**Interview Answer**

Bounded channels have a fixed capacity and provide backpressure by suspending senders when full. Unbounded channels grow without limit, which can cause memory exhaustion under load spikes. Always prefer bounded channels in production—they force you to think about capacity planning and prevent OOM scenarios.

---

### Q6. How does backpressure relate to rate limiting?

**Interview Answer**

Rate limiting controls the rate of incoming requests (e.g., 1000 req/s), while backpressure controls the rate of processing based on consumer capacity. Rate limiting is a form of backpressure at the API gateway level. In Rust, use `tower::limit::RateLimitLayer` for rate limiting and bounded channels for internal backpressure.

---

### Q7. What happens if you ignore backpressure?

**Interview Answer**

Without backpressure, producers fill memory with unprocessed messages, leading to OOM crashes or extreme latency. In a web service, unbounded request queues cause response times to degrade silently before the service crashes. Bounded channels and semaphores force explicit handling of overload conditions.

---

### Q8. How do you monitor backpressure in a Rust service?

**Interview Answer**

Expose metrics like channel length, semaphore available permits, and request queue depth using `prometheus` or `metrics` crates. Alert when channel fill ratio exceeds thresholds. In Tokio, instrument channels with wrapper types that emit metrics on send/receive. Log warnings when backpressure is actively being applied.

---

### Q9. How does backpressure work in a message queue like Kafka?

**Interview Answer**

Kafka provides backpressure through consumer polling—consumers only pull messages when they're ready. In Rust, use `rdkafka` with a consumer group and tune `max.poll.interval.ms` and `fetch.min.bytes`. If processing is slow, the consumer falls behind but doesn't crash, and Kafka retains messages until the consumer catches up.

---

### Q10. Can you implement adaptive backpressure?

**Interview Answer**

Yes, adjust concurrency limits based on observed latency or error rates. For example, start with 100 concurrent requests, and if p99 latency exceeds a threshold, reduce to 50. Use `tower::limit::ConcurrencyLimit` with dynamic adjustments, or implement a PID controller that adjusts the semaphore count based on response time metrics.

---
