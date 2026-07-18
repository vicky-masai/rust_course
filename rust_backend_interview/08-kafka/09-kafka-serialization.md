# Kafka Serialization

## Interview Question

How do you handle message serialization and schema evolution in Kafka, and what are the trade-offs between Avro, Protobuf, and JSON?

## Interview Answer

Kafka stores raw bytes and is schema-agnostic, so serialization is the producer's responsibility. The three common formats are Avro (compact, schema-registry-integrated), Protobuf (binary, strongly typed), and JSON (human-readable, flexible). A Schema Registry enforces schema compatibility and prevents producers from sending messages that break consumers. Schema evolution strategies (BACKWARD, FORWARD, FULL compatibility) control how schemas can change over time. In Rust, `apache-avro` and `prost` (for Protobuf) provide mature serialization, while JSON uses `serde_json`.

---

## Follow-up Questions & Answers

### Q1. What is the Confluent Schema Registry and how does it integrate with Kafka producers?

**Interview Answer**

The Schema Registry is a centralized service that stores and versions schemas. Producers register their schema before sending messages; the registry assigns a schema ID that is prepended to the message (5-byte magic byte + schema ID). Consumers fetch the schema by ID for deserialization. This decouples producers and consumers from schema knowledge and prevents incompatible messages. In Rust, use the `schema_registry_converter` crate or make HTTP calls to the registry API. The registry enforces compatibility modes and rejects schema changes that violate the configured compatibility level.

---

### Q2. What are the differences between BACKWARD, FORWARD, and FULL compatibility?

**Interview Answer**

**BACKWARD compatibility** means new consumers can read old data and old consumers can read new data. You can add optional fields or remove required fields with defaults. **FORWARD compatibility** means old consumers can read new data. You can add required fields (with defaults) or remove optional fields. **FULL compatibility** combines both, ensuring new and old versions can read each other's data. Most production systems use BACKWARD or FULL compatibility. In Rust, the schema registry API validates compatibility before allowing schema registration, preventing runtime deserialization failures.

---

### Q3. How does Avro serialization work with Kafka and what are its advantages?

**Interview Answer**

Avro encodes data as compact binary with a schema, enabling efficient serialization and deserialization. With Kafka, the Avro schema is registered in the Schema Registry, and each message carries the schema ID. Avro's advantages include: compact encoding (smaller payloads), schema evolution with defaults, and strong typing. The schema defines field types, defaults, and documentation. In Rust, the `apache-avro` crate handles Avro serialization. For high-throughput services, Avro's compact encoding reduces network and storage costs compared to JSON.

---

### Q4. How does Protobuf compare to Avro for Kafka message serialization?

**Interview Answer**

Protobuf uses a binary format with strongly typed messages defined in `.proto` files. Compared to Avro: Protobuf has smaller messages (no schema in the payload), explicit field numbering for evolution, and better support for nested structures. Avro embeds the schema reference, making it self-describing. Protobuf is preferred when: schema is known at compile time, nested messages are complex, or gRPC is used alongside Kafka. Avro is preferred for: schema evolution flexibility, data warehousing, and integration with the Confluent ecosystem. In Rust, `prost` provides idiomatic Protobuf support.

---

### Q5. When would you use JSON instead of Avro or Protobuf for Kafka?

**Interview Answer**

JSON is suitable when: (1) debugging and human readability are priorities; (2) schema evolution is minimal or managed externally; (3) the consumer ecosystem is non-JVM and schema registry integration is complex; (4) message volume is low enough that JSON's overhead is negligible. JSON with a schema (via JSON Schema or a registry) provides some protection against schema drift. For Rust services, `serde_json` is fast and idiomatic. JSON is not recommended for high-throughput production systems due to larger payloads and lack of built-in schema evolution guarantees.

---

### Q6. How do you handle schema evolution for existing topics in production?

**Interview Answer**

Schema evolution in production requires: (1) registering the new schema with the registry under the configured compatibility mode; (2) deploying the producer with the new schema; (3) ensuring consumers can handle both old and new schemas (dual-schema support); (4) gradually rolling out consumers with the new schema. Use a consumer that detects the schema ID in the message header and deserializes with the correct version. In Rust, maintain a `HashMap<schema_id, Schema>` cache of registered schemas. The key rule is to never make breaking changes without creating a new topic or using a compatibility mode that supports the change.

---

### Q7. What is the 5-byte header format used by the Confluent Schema Registry?

**Interview Answer**

The wire format prepends a 5-byte header to each serialized message: (1) 1 byte: magic byte `0x00` identifying the Confluent wire format; (2) 4 bytes: schema ID as a big-endian 32-bit integer. Consumers read this header, extract the schema ID, fetch the schema from the registry, and deserialize the remaining bytes. This means every message carries a reference to its schema, enabling the consumer to deserialize without prior schema knowledge. In Rust, parse the header manually or use the `schema_registry_converter` crate for automatic handling.

---

### Q8. How do you choose between Avro and Protobuf for a new Kafka-based Rust service?

**Interview Answer**

Choose Avro when: using the Confluent ecosystem, need schema evolution with defaults, or building data pipelines with schema registry. Choose Protobuf when: using gRPC alongside Kafka, have deeply nested message structures, or prefer compile-time type checking with `.proto` files. For Rust services, Protobuf with `prost` has better language integration (derive macros, type safety). Avro requires runtime schema parsing. If neither ecosystem is a constraint, Protobuf with a schema registry provides the best balance of performance, type safety, and schema evolution for Rust.

---

### Q9. What are common schema evolution anti-patterns in Kafka?

**Interview Answer**

Common anti-patterns: (1) **Renaming fields** - break consumers that reference old field names; use aliases instead; (2) **Removing required fields** - break old consumers unless BACKWARD compatibility with defaults is set; (3) **Changing field types** - e.g., int to string breaks deserialization; (4) **Not registering schemas** - leads to unversioned, unvalidated schemas; (5) **Ignoring compatibility levels** - using NONE allows any change, defeating the purpose. Best practice: always register schemas, always define compatibility mode, always test with old and new schemas before deploying. In Rust, CI should validate schema compatibility.

---

### Q10. How do you implement schema registry integration in a Rust Kafka service?

**Interview Answer**

In Rust, implement schema registry integration by: (1) using the `schema_registry_converter` crate or making HTTP requests to the registry API; (2) caching schemas locally to avoid repeated registry calls; (3) registering schemas on producer startup with the configured compatibility mode; (4) embedding the schema ID in the message header when producing; (5) deserializing messages by extracting the schema ID and fetching the schema for deserialization. The `apache-avro` crate handles Avro-specific serialization. For Protobuf, `prost` handles encoding; the schema registry integration adds the header. Always include schema registry health checks in your service's readiness probe.
