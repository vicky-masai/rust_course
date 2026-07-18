# channels in Rust

## Interview Question

Explain channels in Rust.

## Interview Answer

"Channels enable thread-safe communication between producers and consumers without shared mutable state."

---

## Follow-up Questions & Answers

### Q1. What are the main types of channels available in Tokio?

**Interview Answer**

Tokio provides `mpsc` for multi-producer single-consumer, `oneshot` for single-value delivery, `broadcast` for multi-producer multi-consumer where every receiver gets every message, and `watch` for a single producer with multiple receivers where only the latest value is retained. I choose the channel type based on how many senders and receivers I need and whether I care about every message or just the latest.

---

### Q2. How do channels compare to using `Arc<Mutex<T>>` for shared state?

**Interview Answer**

Channels follow the ownership model where data is moved between tasks without shared mutable state, so there are no locks or contention. `Arc<Mutex<T>>` requires both tasks to access the same memory, adding lock overhead and potential deadlocks. I prefer channels for message passing patterns like event processing and background workers, and mutexes for simple shared counters or caches.

---

### Q3. What happens when a channel buffer is full?

**Interview Answer**

By default, `send()` on a bounded channel blocks until the buffer has space. I use `.try_send()` for non-blocking sends that return an error immediately if full, which is useful for dropping messages under backpressure. For critical messages, I increase the buffer size or use `send().await` with a timeout to prevent indefinite blocking.

---

### Q4. How do you use channels for background task processing in an Axum backend?

**Interview Answer**

I create a bounded `mpsc` channel at application startup and pass the sender to handlers through Axum's `State`. Handlers send work messages to the channel, and a separate Tokio task consumes from the receiver and processes jobs like sending emails or updating search indexes. This decouples request handling from slow background work while keeping the system backpressure-aware.

---

### Q5. What is a oneshot channel and when would you use it?

**Interview Answer**

A oneshot channel carries exactly one value from a producer to a consumer, similar to a Future that resolves to a single result. I use it when a spawned task needs to return a value to the caller, like querying a cache asynchronously and returning the result. It's lighter than mpsc since it doesn't need a buffer and is automatically closed after the value is sent.

---

### Q6. What is channel backpressure and why is it important?

**Interview Answer**

Backpressure occurs when a producer sends messages faster than the consumer processes them, filling the channel buffer. Without backpressure, you'd either block the producer, drop messages, or exhaust memory. Tokio's bounded channels provide natural backpressure by making `send().await` wait when the buffer is full, which slows down producers to match consumer throughput.

---

### Q7. How do you handle channel closing and cleanup?

**Interview Answer**

When all senders are dropped, the channel closes and the receiver's iterator returns `None`. I check for channel closure in consumer loops to gracefully shut down background tasks. I use `tokio::select!` to listen for both incoming messages and a shutdown signal, ensuring clean termination during application shutdown.

---

### Q8. What is the `watch` channel and how is it different from `broadcast`?

**Interview Answer**

`watch` is designed for broadcasting configuration or state updates where only the latest value matters, like hot-reloading settings. If a receiver is slow, it skips intermediate values and only reads the most recent one. `broadcast` guarantees every receiver gets every message, which is better for event logs or audit trails where losing messages is unacceptable.

---

### Q9. How do you test code that uses channels?

**Interview Answer**

I create channels in the test function and inject them as dependencies instead of creating them internally. This lets me send test messages through the producer and assert on the consumer's output. I also test edge cases like sending on a closed channel and filling the buffer to verify backpressure behavior. For async tests, I use `#[tokio::test]` with `tokio::time::pause()` to control timing.
