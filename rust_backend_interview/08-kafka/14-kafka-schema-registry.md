# Kafka Schema Registry

## Interview Question

How does a Schema Registry work with Kafka, and why is it essential for production systems?

## Interview Answer

A Schema Registry is a centralized service that stores, versions, and validates schemas for Kafka messages. Producers register their schema before sending; the registry assigns a schema ID prepended to each message. Consumers fetch the schema by ID for deserialization, ensuring type safety without hardcoded schemas. The registry enforces compatibility modes (BACKWARD, FORWARD, FULL) to prevent breaking changes. Confluent Schema Registry is the standard implementation, supporting Avro, Protobuf, and JSON Schema. In Rust, integrate via HTTP API calls or the `schema_registry_converter` crate for automatic schema management.

---

## Follow-up Questions & Answers

### Q1. How does the Schema Registry wire format work in Kafka messages?

**Interview Answer**

The Confluent wire format prepends a 5-byte header to each serialized message: 1 byte (magic byte `0x00`) + 4 bytes (schema ID as big-endian integer). The consumer reads this header, extracts the schema ID, fetches the schema from the registry, and deserializes the remaining bytes. This means the schema reference travels with the message, enabling the consumer to deserialize without prior schema knowledge. In Rust, parse the header manually:

```rust
let magic = bytes[0];
let schema_id = i32::from_be_bytes(bytes[1..5].try_into()?);
let payload = &bytes[5..];
```

Or use `schema_registry_converter` for automatic handling.

---

### Q2. What are the different compatibility modes and when do you use each?

**Interview Answer**

Compatibility modes: **BACKWARD** - new schema can read old data, old schema can read new data (use default values for new fields). **FORWARD** - old schema can read new data (new fields must have defaults). **FULL** - both backward and forward compatible. **NONE** - no compatibility checks (dangerous). Use BACKWARD for most cases (add fields with defaults, remove fields). Use FULL when both old and new consumers must coexist during rolling deployments. NONE is only for development. Set at the subject level via the registry API. In Rust, validate compatibility before registering using the `/compatibility/subjects/{subject}/versions/{version}` endpoint.

---

### Q3. How do you integrate Schema Registry with a Rust Kafka producer?

**Interview Answer**

In Rust, integrate by: (1) using the `schema_registry_converter` crate which handles schema registration and wire format; (2) making HTTP calls to the registry API from your producer code; (3) caching schemas locally to avoid repeated registry fetches. The producer registers the schema on first use, retrieves the schema ID, and prepends the 5-byte header to each message. Example using `schema_registry_converter`:

```rust
let schema_registry = SchemaRegistry::new("http://schema-registry:8081");
let avro_schema = schema_registry.get_schema("my-topic-value").await?;
let bytes = to_avro(&value, &avro_schema)?;
producer.send(Record::new("my-topic", Key::None, bytes)).await?;
```

---

### Q4. How do you handle schema evolution without breaking existing consumers?

**Interview Answer**

Schema evolution without breaking consumers: (1) **Add optional fields** with defaults - backward compatible; (2) **Never rename fields** - add new field and deprecate old; (3) **Never change field types** - add new field with new type instead; (4) **Test compatibility** before registering using the registry's compatibility endpoint; (5) **Use FULL compatibility** mode to ensure both directions work. During rolling deployments, consumers may run different schema versions simultaneously. The registry ensures all versions are compatible. In Rust, handle multiple schema versions in the consumer by maintaining a `HashMap<SchemaId, Schema>` cache.

---

### Q5. What is the subject naming strategy in Schema Registry?

**Interview Answer**

Subject naming strategies determine how the schema registry subject name is derived from the topic. Three strategies: **TopicNameStrategy** (default) - subject = `{topic}-value` or `{topic}-key`; **RecordNameStrategy** - subject = `{record-name}` (schema name, not topic); **TopicRecordNameStrategy** - subject = `{topic}-{record-name}` (allows multiple schemas per topic). TopicNameStrategy is simplest and used for most cases. RecordNameStrategy is useful when the same schema is used across multiple topics. TopicRecordNameStrategy supports multiple message types per topic. In Rust, set the strategy via the registry client configuration.

---

### Q6. How does the Schema Registry handle high availability and fault tolerance?

**Interview Answer**

The Schema Registry runs as a cluster with one leader and optional followers. The leader handles all write operations (schema registration, compatibility checks); followers serve read requests. If the leader fails, a follower is elected leader. Schema data is stored in a Kafka topic (`_schemas`) or a database, ensuring durability. For production, run 2+ registry instances behind a load balancer. If the registry is unavailable, producers can use cached schemas (schema IDs are embedded in messages), but new registrations will fail. In Rust, cache schemas locally and handle registry unavailability gracefully.

---

### Q7. How do you migrate schemas from Avro to Protobuf in an existing Kafka system?

**Interview Answer**

Migration strategy: (1) Register the Protobuf schema in the registry under a new subject; (2) Deploy consumers that can deserialize both Avro and Protobuf (detect wire format by magic byte); (3) Deploy producers to produce Protobuf messages; (4) Gradually decommission Avro consumers once all messages are Protobuf. Alternatively, use a bridge consumer that reads Avro and produces Protobuf to a new topic. The migration requires careful coordination and backward-compatible consumers. In Rust, the consumer should check the magic byte (`0x00` for Confluent) and schema ID to determine the serialization format.

---

### Q8. What are the best practices for Schema Registry in production?

**Interview Answer**

Best practices: (1) **Always register schemas** - never allow unregistered messages; (2) **Use BACKWARD or FULL compatibility** - prevent breaking changes; (3) **Version schemas** - increment version for each change; (4) **Document schemas** - include descriptions in schema definitions; (5) **CI/CD validation** - test schema compatibility in CI pipelines; (6) **Cache schemas locally** - reduce registry load; (7) **Monitor registry health** - alert on unavailability; (8) **Use schema names** - descriptive record names for clarity; (9) **Never use NONE compatibility** in production; (10) **Back up the `_schemas` topic** - registry data lives in Kafka.

---

### Q9. How does Schema Registry interact with Kafka Connect?

**Interview Answer**

Kafka Connect integrates with Schema Registry for source and sink connectors. Source connectors (e.g., JDBC, Debezium) automatically register schemas from the source system. Sink connectors (e.g., Elasticsearch, S3) fetch schemas for deserialization. The `value.converter=io.confluent.connect.avro.AvroConverter` configuration handles schema-aware serialization. Connectors automatically evolve schemas as the source data changes. This integration eliminates manual schema management for data pipeline scenarios. For Rust services consuming from Connect-produced topics, use the same registry client to fetch schemas.

---

### Q10. How do you implement schema registry health checks in a Rust service?

**Interview Answer**

Implement health checks by: (1) **Readiness probe** - call `GET /subjects` on the registry; if it returns 200, the registry is ready; (2) **Liveness probe** - verify the registry is responsive; (3) **Schema cache validation** - periodically refresh cached schemas to detect registry-side changes; (4) **Circuit breaker** - if the registry is unreachable, use cached schemas but alert. In Rust, add a health check endpoint to your Axum service:

```rust
async fn kafka_health(State(client): State<SchemaRegistryClient>) -> impl IntoResponse {
    match client.check_health().await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}
```

This ensures your service fails fast if the registry is unavailable during startup.
