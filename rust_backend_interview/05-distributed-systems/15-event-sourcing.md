# Event Sourcing

## Interview Question

What is Event Sourcing and when should you use it?

## Interview Answer

Event Sourcing is an architectural pattern where every state change is stored as an immutable event in an append-only log, rather than storing only the current state. The current state is derived by replaying events from the beginning (or from a snapshot). This provides a complete audit trail, enables temporal queries (state at any point in time), and pairs naturally with CQRS for separate read/write optimization. Event Sourcing is valuable in domains where the history of changes is as important as the current state — financial systems, collaboration tools, and audit-critical applications. The trade-off is increased complexity in event storage, schema evolution, and query patterns.

---

## Follow-up Questions & Answers

### Q1. What is the difference between Event Sourcing and traditional CRUD?

**Interview Answer**

In CRUD, you store only the current state — if a user's address changes, you overwrite the old address. The history of changes is lost (unless you add separate audit logging). In Event Sourcing, you store every change as an event: `AddressChanged { old: "...", new: "..." }`. The current address is derived by replaying all address-related events. CRUD is simpler and sufficient for most applications. Event Sourcing adds complexity (event storage, replay, snapshotting) but provides benefits like complete audit trails, temporal queries, and the ability to rebuild any read model from events. Use Event Sourcing when the history matters; use CRUD when only the current state matters.

---

### Q2. What is the role of snapshots in Event Sourcing?

**Interview Answer**

Replaying all events from the beginning becomes slow as the event log grows — replaying 1 million events to get the current state of an order is impractical. Snapshots capture the state at a particular point in time (e.g., every 1000 events or every hour). When rebuilding state, you load the most recent snapshot and only replay events after it. Snapshots are optional optimizations — the system works without them but is slower. In a Rust implementation, store snapshots in a separate table with the aggregate ID and version. When the number of events since the last snapshot exceeds a threshold, take a new snapshot. This reduces replay time from O(n) to O(snapshot_size).

---

### Q3. How do you handle event schema evolution in Event Sourcing?

**Interview Answer**

Since events are immutable, you cannot change their schema after they are stored. Schema evolution is handled through upcasting — transforming old event versions to the current version during replay. For example, if `OrderCreatedV1` has fields `{order_id, amount}` and `OrderCreatedV2` adds `{currency}`, you write an upcaster that adds `currency: "USD"` as a default when replaying V1 events. Store events with their version number. In Rust, use an enum with version variants: `enum OrderCreated { V1 { order_id: String, amount: f64 }, V2 { order_id: String, amount: f64, currency: String } }`. The upcaster converts V1 to V2 during replay.

---

### Q4. How would you implement Event Sourcing in a Rust/Axum backend?

**Interview Answer**

Use PostgreSQL with an `events` table: `id UUID, aggregate_type VARCHAR, aggregate_id UUID, event_type VARCHAR, payload JSONB, version INT, created_at TIMESTAMP`. Write events within a transaction: insert the event with `version = last_version + 1` using a unique constraint on `(aggregate_id, version)` to prevent concurrent writes. For reads, either replay events to build a current state model (CQRS read side) or use projections stored in separate tables. In Rust, define events as `serde` serializable structs, use `sqlx` for database operations, and implement an `EventStore` trait with `append_events` and `load_events` methods. Use Axum handlers for commands that append events and queries that read from projections.

---

### Q5. What are the challenges of querying in Event Sourcing?

**Interview Answer**

Event stores are optimized for appending and replaying by aggregate, not for complex queries. You cannot efficiently query "show me all orders over $100" from the event log — you need a projection (read model) that maintains this view. This is where CQRS complements Event Sourcing: the write side appends events, and the read side maintains query-optimized projections. Challenges include: keeping projections up-to-date (they lag behind events during high load), rebuilding projections after schema changes (replay all events), and ensuring projection consistency (handling out-of-order events). In a Rust backend, use a background task that consumes events and updates projection tables in PostgreSQL or Elasticsearch.

---

### Q6. What is the difference between Event Sourcing and the Outbox Pattern?

**Interview Answer**

Event Sourcing stores all state changes as events as the primary data model — the event log is the source of truth. The Outbox Pattern stores business data normally and uses an outbox table as a mechanism to reliably publish events to other services. In Event Sourcing, the event IS the data; in the Outbox Pattern, the event is a notification about data changes. You can combine both: use Event Sourcing for the primary data model and the Outbox Pattern to publish events from the event store to Kafka for consumption by other services. Event Sourcing is an architectural choice; the Outbox Pattern is a reliability mechanism.

---

### Q7. How do you handle event replay for long-running aggregates?

**Interview Answer**

Aggregates with millions of events (e.g., a bank account with years of transactions) are expensive to replay. Solutions include: periodic snapshots (store state every N events), event compaction (summarize old events into a single snapshot event, e.g., "balance up to January: $5000"), and partitioned event storage (store events in time-based partitions so only recent partitions need replaying). In Rust, implement a snapshot manager that checks the event count since the last snapshot and triggers a snapshot if the threshold is exceeded. For compaction, create a `SnapshotApplied` event that summarizes the previous events, and start replay from the snapshot.

---

### Q8. What is the relationship between Event Sourcing and CQRS?

**Interview Answer**

Event Sourcing and CQRS are complementary patterns often used together. Event Sourcing provides the write model (events are appended to the event store), and CQRS provides the read model (projections derived from events are optimized for queries). The event store is the write side, and projections are the read side. However, you can use Event Sourcing without CQRS (query the event store directly for simple queries) or CQRS without Event Sourcing (write to a relational DB, read from Elasticsearch). When combined, they provide a powerful architecture: complete audit trail from Event Sourcing, and optimized, independent read models from CQRS.

---

### Q9. When is Event Sourcing the wrong choice?

**Interview Answer**

Event Sourcing is wrong for simple CRUD applications where only the current state matters and the domain is straightforward. It adds significant complexity: event storage, schema evolution, snapshotting, projection management, and eventual consistency between write and read models. Small teams may struggle with the operational overhead. It is also inappropriate when you need strong consistency across aggregates — events within an aggregate are consistent, but cross-aggregate queries require projections that may lag. If your domain does not benefit from temporal queries or audit trails, Event Sourcing is unnecessary overhead. Start with CRUD and migrate to Event Sourcing only when the pain of losing history becomes significant.

---

### Q10. How do you test Event Sourcing systems?

**Interview Answer**

Test at two levels: event production and projection building. For event production, verify that commands produce the correct events by loading an aggregate, executing a command, and asserting the events produced. Use a pattern: `given(events).when(command).then(expected_events)`. For projections, verify that a sequence of events produces the correct read model state. Test event replay by storing events, replaying them, and asserting the resulting state matches expectations. Test snapshot consistency by comparing the state built from replay versus snapshot. In Rust, use a testing pattern with an `EventStore` trait that can be mocked, and test aggregates as pure functions of events and commands.
