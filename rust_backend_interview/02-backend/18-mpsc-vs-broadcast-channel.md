# mpsc vs broadcast channel

## Interview Question

mpsc vs broadcast channel.

## Interview Answer

"mpsc delivers each message to a single receiver. Broadcast sends every message to all subscribers."

---

## Follow-up Questions & Answers

### Q1. What does MPSC stand for and what are its use cases?

**Interview Answer**

MPSC stands for Multi-Producer, Single-Consumer, meaning multiple tasks can send messages but only one task receives each message. I use it for work queues where multiple API handlers dispatch background jobs to a single worker pool. It's also useful for aggregating events from multiple sources into a single processing pipeline.

---

### Q2. What does the broadcast channel guarantee about message delivery?

**Interview Answer**

Broadcast guarantees that every connected receiver gets every message, as long as the receiver is keeping up. If a receiver falls behind and the buffer fills, old messages are overwritten and the receiver gets a `RecvError::Lagged` error indicating how many messages were missed. I handle this by logging the lag and retrying the operation that depended on the missed data.

---

### Q3. When would you choose broadcast over mpsc?

**Interview Answer**

I use broadcast when multiple tasks need to react to the same events, like notifying all WebSocket connections about a state change or pushing real-time updates to multiple subscribers. I use mpsc when work should be distributed among consumers, like a task queue where only one worker should process each job. The key distinction is fan-out (broadcast) vs load-balancing (mpsc).

---

### Q4. How do you handle the `Lagged` error in a broadcast channel?

**Interview Answer**

The `Lagged` error means the receiver was too slow and missed messages because the buffer was overwritten. I catch this error, log how many messages were lost, and decide whether to resync the state from the database or continue with partial data. For critical event streams, I increase the buffer size or switch to a persistent message broker like Kafka to avoid lag entirely.

---

### Q5. Can you combine mpsc and broadcast in a single system?

**Interview Answer**

Yes, I use mpsc for the primary work dispatch where a single consumer processes jobs, and broadcast to fan out status updates or events to multiple subscribers. For example, a file upload worker receives jobs through mpsc and broadcasts progress events through a broadcast channel so multiple WebSocket clients can track the upload. This gives both efficient work distribution and real-time notification.

---

### Q6. What is the memory overhead of broadcast vs mpsc channels?

**Interview Answer**

Broadcast stores each message once in a shared buffer and gives receivers shared references, so memory usage is proportional to buffer size regardless of receiver count. Mpsc duplicates messages when multiple senders are involved and each message is moved to the receiver. For high-frequency events with many subscribers, broadcast is more memory-efficient because it avoids cloning the message for each receiver.

---

### Q7. How do you decide the buffer size for a broadcast channel?

**Interview Answer**

I size the buffer based on the expected message rate and the maximum tolerable lag of slow consumers. For low-frequency config updates, a buffer of 10-50 is sufficient. For high-frequency event streams, I use 1000+ and monitor the `Lagged` errors in production. If lag is consistently occurring, I either increase the buffer, optimize the consumer, or switch to a persistent broker.

---

### Q8. What happens when all receivers are dropped in each channel type?

**Interview Answer**

For mpsc, when the receiver is dropped, any subsequent `send()` call returns an error and existing buffered messages are lost. For broadcast, when all receivers are dropped, the channel remains usable and new receivers can still subscribe. I handle cleanup by using `tokio::select!` to gracefully shut down consumers and ensure producers detect closed channels promptly.

---

### Q9. How do broadcast channels handle slow consumer scenarios in production?

**Interview Answer**

In production, a slow consumer can cause message loss since broadcast overwrites old messages. I monitor consumer lag metrics and set alerts when it exceeds thresholds. For critical paths, I use a persistent message queue like Kafka or RabbitMQ instead of broadcast. For non-critical notifications like cache invalidation, I accept occasional lag and have the consumer refresh its state from the source of truth.

---
