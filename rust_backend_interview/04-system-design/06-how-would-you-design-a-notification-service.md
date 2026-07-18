# How Would You Design a Notification Service?

## Interview Question

How would you design a scalable notification service in Rust?

## Interview Answer

I would design a notification service with an Axum API that receives notification requests, validates them, and publishes events to Kafka for asynchronous processing. Worker consumers (also written in Tokio) subscribe to Kafka topics partitioned by notification type (email, SMS, push) and route messages to the appropriate provider adapters — SendGrid for email, Twilio for SMS, and Firebase Cloud Messaging for push notifications. The service tracks delivery status by writing events to PostgreSQL, and implements retry logic with exponential backoff using a dedicated retry queue (Kafka topic with delayed messages). A dead letter queue captures permanently failed notifications for manual review, and an audit log records every notification attempt for compliance.

---

## Follow-up Questions & Answers

### Q1. Why use Kafka instead of a simpler queue like RabbitMQ?

**Interview Answer**

Kafka provides durable, ordered, replayable event logs which are essential for notification services. If a consumer crashes mid-processing, Kafka's consumer group rebalancing ensures messages are reprocessed from the last committed offset — no notifications are lost. The partitioning model allows parallel processing per notification type, and Kafka's retention policy (7+ days) means I can replay historical notifications for debugging or re-sending failed batches. RabbitMQ is simpler but doesn't support replay or ordered partitioning as naturally. The trade-off is Kafka's operational complexity, but at scale (millions of notifications/day), Kafka's throughput and durability advantages outweigh the added infrastructure cost.

---

### Q2. How do you implement retry logic for failed notifications?

**Interview Answer**

I implement a three-tier retry strategy using Kafka topics. When a worker fails to send a notification (e.g., Twilio returns a 503), it publishes the message to a retry topic with a delay header. The retry consumer reads messages and checks the delay — if the delay hasn't elapsed, it re-publishes with an incremented attempt count. After 3 attempts with exponential backoff (1s, 30s, 5min), the message moves to the dead letter queue. In Rust, I use `tokio::time::sleep` with `select!` to handle the delay without blocking the consumer thread. Each retry increments a counter on the message, and the PostgreSQL notification table tracks the attempt history for debugging.

---

### Q3. How do you handle provider-specific rate limits?

**Interview Answer**

Each provider has different rate limits — SendGrid allows 100 emails/second, Twilio allows 1 SMS/second per number. I implement per-provider token bucket rate limiters in Rust, using `tokio::sync::Semaphore` to control concurrency. Each provider adapter has its own semaphore initialized with the provider's rate limit, and workers must acquire a permit before calling the provider API. If the provider returns a 429 (rate limit exceeded), the worker releases the permit, waits for the `Retry-After` header duration, and re-acquires the permit. I also implement provider-level circuit breakers that open after 5 consecutive failures, stopping all requests to a provider for 30 seconds and falling back to an alternative provider if one is configured.

---

### Q4. How do you ensure notification ordering?

**Interview Answer**

Ordering matters for transactional notifications — a password reset email must arrive before a welcome email. I achieve ordering by partitioning the Kafka topic by user ID, ensuring all notifications for one user are processed by the same consumer in FIFO order. In Rust, the Kafka consumer uses `futures::stream` to process messages sequentially within a partition. For notifications that must be strictly ordered across users (rare), I use a global ordering topic with a single consumer, accepting lower throughput. For most cases, user-level ordering is sufficient. I also implement deduplication using a unique notification ID stored in Redis with a 1-hour TTL, preventing duplicate sends during consumer rebalancing.

---

### Q5. How do you track delivery status and provide analytics?

**Interview Answer**

I maintain a `notifications` PostgreSQL table with columns for notification_id, user_id, type, status (pending, sent, delivered, failed, opened), provider, and timestamps for each status change. Webhook endpoints receive delivery confirmations from providers — SendGrid sends open/click events, Twilio sends delivery status callbacks — and update the table. I use Kafka to publish status change events to an analytics pipeline that aggregates metrics: delivery rate by provider, open rate by notification type, and failure rate by hour. In the Axum admin API, I expose aggregated analytics endpoints that query materialized views refreshed every 5 minutes. Prometheus metrics track real-time delivery rates and failure percentages.

---

### Q6. How do you handle user notification preferences?

**Interview Answer**

I store user preferences in a `user_preferences` table with columns for user_id, notification_type (email, sms, push), and enabled (boolean). The Axum API allows users to toggle preferences via a `PUT /api/v1/notifications/preferences` endpoint. Before publishing a notification to Kafka, the service checks the user's preferences and skips publishing if the user has disabled that notification type. Preferences are cached in Redis with a 5-minute TTL to avoid database lookups on every notification. Critical notifications (security alerts, 2FA codes) bypass preferences — this is enforced at the service layer, not at the API level, to prevent users from accidentally disabling security notifications.

---

### Q7. How do you implement template management for notifications?

**Interview Answer**

I store notification templates in PostgreSQL with fields for template_id, type (email, sms, push), locale, subject, and body (using Handlebars syntax). Templates are versioned — when an operator updates a template, a new version is created and the old one is kept for audit. The Axum API provides CRUD endpoints for template management, and templates are cached in Redis for fast access. When a notification is sent, the worker fetches the template, interpolates variables using the `handlebars` crate, and sends the rendered content. I support multiple locales per template, with a fallback chain (e.g., en-IN → en → default). Template rendering is tested in CI with snapshot tests to catch formatting regressions.

---

### Q8. How do you handle the dead letter queue?

**Interview Answer**

The DLQ is a separate Kafka topic that captures notifications that failed all retry attempts. A dedicated consumer processes DLQ messages and writes them to a `failed_notifications` PostgreSQL table with the full error context (provider response, attempt history, timestamps). An Axum admin API endpoint allows operators to view, retry, or dismiss failed notifications. I implement a scheduled Tokio task that scans the DLQ table every hour and automatically retries notifications whose failure reason is transient (e.g., temporary provider outage). Permanent failures (invalid email address, user opted out) are marked as such and never retried. Grafana dashboards monitor DLQ depth, and an alert fires if the queue exceeds 100 messages.

---

### Q9. How do you scale the notification service?

**Interview Answer**

I scale the notification service horizontally by adding more Kafka consumer instances. Each consumer group member is assigned one or more Kafka partitions, so scaling from 3 to 6 consumers doubles the processing throughput. The Axum API scales independently — I run 3 replicas behind a load balancer for the ingestion API. The Kafka topics are partitioned by notification type (email, sms, push) to ensure even distribution. For the provider adapters, I scale the semaphore permits dynamically based on provider response times — if Twilio starts responding slower, I reduce the concurrency to stay within their rate limits. PostgreSQL read replicas handle analytics queries, keeping the primary free for writes.

---

### Q10. How do you ensure reliability and availability of the notification service?

**Interview Answer**

Reliability is achieved through multiple layers: Kafka provides at-least-once delivery guarantees, ensuring no messages are lost even during consumer failures. PostgreSQL uses synchronous replication for the primary write, preventing data loss. The Axum API runs across 2 availability zones with health checks every 10 seconds. I implement circuit breakers on provider calls to prevent cascading failures — if SendGrid goes down, email notifications fail gracefully into the retry queue instead of blocking all notifications. I also implement graceful degradation: if the preference lookup fails (Redis down), I default to sending the notification (better to over-notify than miss a critical alert). Full end-to-end testing with Chaos Monkey validates that the system handles node failures, network partitions, and provider outages without losing notifications.
