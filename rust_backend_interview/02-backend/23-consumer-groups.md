# consumer groups

## Interview Question

Explain consumer groups.

## Interview Answer

"Consumers in the same group divide partitions so each message is processed only once within the group."

---

## Follow-up Questions & Answers

### Q1. How do consumer groups achieve load balancing?

**Interview Answer**

Kafka assigns partitions to consumers within a group, ensuring each partition is consumed by exactly one consumer. When a consumer joins or leaves, Kafka rebalances partition assignments across remaining consumers. This distributes processing load automatically without manual partition management.

---

### Q2. What happens when a consumer crashes in a consumer group?

**Interview Answer**

Kafka detects the failure via heartbeat timeouts and triggers a rebalance, reassigning the crashed consumer's partitions to surviving consumers. Messages in those partitions are not lost but processing pauses during rebalance. Use `session.timeout.ms` and `heartbeat.interval.ms` to tune failure detection speed.

---

### Q3. How do you implement consumer groups in Rust?

**Interview Answer**

Use the `rdkafka` crate which provides a `BaseConsumer` or `StreamConsumer` with group ID configuration. Set `group.id` in the consumer config and call `subscribe()` to topic partitions. For Axum integration, run the consumer in a dedicated Tokio task and forward processed messages to handlers via channels.

---

### Q4. What is the difference between static and dynamic consumer group membership?

**Interview Answer**

Static membership uses a fixed `group.instance.id` for each consumer, so Kafka avoids full rebalances when consumers temporarily disconnect. Dynamic membership triggers rebalance on every join/leave event. Static membership is better for Kubernetes deployments where pods restart frequently.

---

### Q5. How many consumer groups can read from the same Kafka topic?

**Interview Answer**

Unlimited. Each consumer group maintains its own offset tracking, so multiple groups can independently consume the same topic without interfering. This enables use cases like one group for real-time processing and another for archival, each at its own pace.

---

### Q6. How do you handle rebalancing gracefully in a Rust Kafka consumer?

**Interview Answer**

Implement a rebalance callback using `ConsumerContext` in rdkafka that commits offsets before partitions are revoked. In Axum, pause message processing during rebalance and resume after new partitions are assigned. Use cooperative rebalancing protocol to minimize processing interruption.

---

### Q7. What is the relationship between consumer group lag and partition count?

**Interview Answer**

Lag is the difference between the latest offset and the consumer's current offset across all assigned partitions. More partitions allow more parallel consumers within a group, reducing lag. However, too many partitions increase metadata overhead and rebalance time, so tune partition count based on throughput requirements.

---

### Q8. How do you monitor consumer group health in production?

**Interview Answer**

Use Kafka's admin API or tools like `kafka-consumer-groups.sh` to check lag, member count, and rebalance status. Export metrics to Prometheus and alert on growing lag or frequent rebalances. In Rust, log consumer lag metrics periodically and integrate with your observability stack.
