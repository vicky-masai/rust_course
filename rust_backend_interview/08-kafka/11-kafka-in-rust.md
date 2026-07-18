# Kafka in Rust

## Interview Question

How do you integrate Kafka into a Rust backend service using the rdkafka crate, and what are the production considerations?

## Interview Answer

The `rdkafka` crate wraps librdkafka (a C library) providing a mature, performant Kafka client for Rust. It supports async/await via Tokio, with `FutureProducer` for producing and `StreamConsumer` for consuming messages. Integration with Axum involves wrapping the producer in `axum::extract::State` and spawning consumer tasks in the application startup. Production considerations include graceful shutdown, error handling, consumer group management, and metrics instrumentation. `rdkafka` supports all Kafka features including transactions, exactly-once semantics, and cooperative rebalancing.

---

## Follow-up Questions & Answers

### Q1. How do you set up a Kafka producer in an Axum service?

**Interview Answer**

Create a `FutureProducer` with configuration properties, wrap it in `Arc<FutureProducer>`, and pass it as Axum state via `Router::new().with_state(producer)`. Use `axum::extract::State<Arc<FutureProducer>>` in handlers to access the producer. Call `producer.send(...)` which returns a `DeliveryFuture` that resolves asynchronously. For high-throughput, use `producer.send_result(...)` to avoid blocking the handler. Example setup:

```rust
let producer: FutureProducer = ClientConfig::new()
    .set("bootstrap.servers", "localhost:9092")
    .set("message.timeout.ms", "5000")
    .create()?;
let state = Arc::new(producer);
let app = Router::new().route("/events", post(handler)).with_state(state);
```

---

### Q2. How do you implement a Kafka consumer that runs alongside an Axum server?

**Interview Answer**

Spawn the Kafka consumer in a `tokio::spawn` task during application startup. The consumer runs an infinite loop calling `consumer.recv().await` and processing messages. Use a `tokio::sync::broadcast` channel for graceful shutdown signaling. When the shutdown signal is received, the consumer task exits its loop and commits final offsets. The consumer task is joined during server shutdown using `tokio::select!` between the Axum server and the consumer task. This ensures the consumer processes all messages before the service shuts down.

---

### Q3. How do you handle Kafka consumer rebalancing in a Rust async context?

**Interview Answer**

Implement the `ConsumerContext` trait for a custom context struct, then call `consumer.rebalance(...)` with a callback that handles `Rebalance::Assign` and `Rebalance::Revoke`. In the `Revoke` variant, commit offsets for all assigned partitions using `consumer.commit(...)` with `CommitMode::Sync`. In the `Assign` variant, initialize any per-partition state. Use `rdkafka::consumer::Rebalance` enum for pattern matching. For cooperative rebalancing, set `partition.assignment.strategy=CooperativeStickyAssignor`. In Tokio, the rebalance callback runs on the consumer's internal thread, so use channels to communicate rebalance events to async processing tasks.

---

### Q4. What are the error handling patterns for Kafka operations in Rust?

**Interview Answer**

Kafka operations return `KafkaError` which should be matched exhaustively. Common patterns: (1) **Producer errors** - `send_result()` returns `Err((KafkaError, OwnedMessage))` for immediate failures; retry with exponential backoff for transient errors like `MessageTimedOut`; send to DLQ for permanent failures; (2) **Consumer errors** - `recv()` returns `Err(KafkaError)`; log and continue for transient errors; handle `KafkaError::PartitionEOF` as a non-error; (3) **Connection errors** - `KafkaError::MetadataFetch` indicates broker issues; implement circuit breaker logic. Always use `tracing::error!` for Kafka errors and include error codes for debugging.

---

### Q5. How do you configure rdkafka for production resilience?

**Interview Answer**

Key production configurations for `rdkafka`: `session.timeout.ms=30000` (consumer heartbeat timeout), `heartbeat.interval.ms=10000` (heartbeat frequency), `max.poll.interval.ms=300000` (max processing time), `retries=2147483647` (infinite retries), `retry.backoff.ms=100` (backoff between retries), `enable.idempotence=true` (deduplication), `acks=all` (maximum durability), `delivery.timeout.ms=120000` (maximum time for delivery). For consumers, set `auto.offset.reset=earliest` and `isolation.level=read_committed`. Always configure `statistics.interval.ms` for monitoring. Use connection timeouts and set `metadata.max.age.ms` for metadata refresh.

---

### Q6. How do you test Kafka-based Rust services?

**Interview Answer**

Testing Kafka services uses: (1) **Integration tests** with a real Kafka instance using `testcontainers` crate to spin up a Kafka container; (2) **Mock producers/consumers** by trait abstraction - define a `MessageBroker` trait and implement a mock for tests; (3) **Embedded Kafka** using `kafka-embedded` or `redpanda` in testcontainers for faster startup. For unit tests, abstract the Kafka client behind a trait and mock it with `mockall`. Test consumer error handling by injecting `KafkaError` variants. Test rebalancing by running multiple consumer instances in tests. Use `rstest` for parameterized test cases across different Kafka configurations.

---

### Q7. How do you integrate OpenTelemetry tracing with Kafka in Rust?

**Interview Answer**

Inject trace context into Kafka message headers using OpenTelemetry's `TraceContext`. When producing, extract the current span context and add it as a header (`traceparent`). When consuming, extract the header and create a child span. Use `opentelemetry` and `tracing-opentelemetry` crates for span management. In `rdkafka`, access headers via `message.headers()` and set them via `ProducerRecord::headers`. This enables end-to-end distributed tracing across Kafka boundaries. Example header key: `traceparent` following W3C Trace Context format.

---

### Q8. What are the memory and performance considerations for rdkafka in Rust?

**Interview Answer**

`rdkafka` wraps librdkafka which uses its own memory management. Key considerations: (1) **Producer buffer** - `buffer.memory` (default 32MB) limits total buffered messages; increase for high-throughput; (2) **Consumer fetch** - `fetch.min.bytes` and `fetch.max.wait.ms` control fetch batching; (3) **Batch size** - `batch.size` (default 16KB) controls per-request batching; (4) **Compression** - reduces network and disk usage but adds CPU overhead; (5) **Zero-copy** - `rdkafka` avoids copies where possible. Profile memory usage with `jemalloc` or `dhat` and ensure consumer `max.partition.fetch.bytes` doesn't cause excessive memory consumption per partition.

---

### Q9. How do you implement graceful shutdown for Kafka consumers in Rust?

**Interview Answer**

Use `tokio::signal::ctrl_c()` or a shutdown channel to signal the consumer to stop. In the consumer task loop, use `tokio::select!` between `consumer.recv()` and the shutdown signal. When shutdown is received, break the loop and commit final offsets using `consumer.commit(...)` with `CommitMode::Sync`. Wait for the consumer task to complete before exiting. In Axum, use `axum::serve(...).with_graceful_shutdown(shutdown_signal)` and join both the server and consumer tasks. This ensures no messages are lost during deployment or container termination.

---

### Q10. How do you structure a production Kafka module in a Rust codebase?

**Interview Answer**

Structure a `kafka` module with: `producer.rs` (producer initialization, send helpers, error handling), `consumer.rs` (consumer setup, message processing, rebalance handling), `config.rs` (Kafka configuration from environment variables), `error.rs` (Kafka-specific error types wrapping `KafkaError`), `metrics.rs` (Prometheus metrics for Kafka operations), and `context.rs` (custom consumer context for rebalance). Use a `KafkaClient` struct encapsulating both producer and consumer with async methods. Configuration should come from environment variables (`BOOTSTRAP_SERVERS`, `GROUP_ID`) using `config` crate. This module is imported by Axum handlers that need to produce or consume messages.
