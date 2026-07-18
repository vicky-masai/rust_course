# Contract Testing in Rust

## Interview Question

What is contract testing, and how do you implement API contract verification in a Rust backend?

## Interview Answer

Contract testing verifies that services agree on the shape and behavior of their interfaces. Instead of testing end-to-end integration, each service tests against a shared contract (schema, API spec). Consumer-driven contracts (like Pact) let the consumer define expected interactions, which the provider then verifies. For Rust, you can use the `pact` crate or validate against OpenAPI specs with `oas3` or `paperclip`. This approach catches breaking changes early without requiring full integration test environments.

---

## Follow-up Questions & Answers

### Q1. What is the difference between contract testing and integration testing?

**Interview Answer**

Integration testing verifies that services work together in a shared environment. Contract testing verifies that each service independently adheres to a shared interface agreement. Contract tests are faster, cheaper to run, and don't require all services to be deployed together. They catch API mismatches without the complexity of managing test environments with multiple running services.

---

### Q2. How does Pact-based contract testing work?

**Interview Answer**

The consumer writes a pact file describing expected HTTP interactions (request/response pairs). The provider runs pact verification tests that replay those interactions against the real service. If the provider's responses match the contract, the test passes. Pact brokers can manage contracts between teams, ensuring consumers and providers stay in sync. The `pact` Rust crate supports HTTP and message-based contracts.

---

### Q3. How do you validate against OpenAPI specifications in Rust?

**Interview Answer**

Use crates like `paperclip` to generate typed API clients from OpenAPI specs, or `oas3` to parse and validate spec files. You can write tests that make requests to your server and validate responses against the schema using JSON Schema validation. This ensures your implementation matches your documentation. Tools like `schemars` can generate JSON schemas from Rust types, enabling round-trip validation.

---

### Q4. What are the benefits of consumer-driven contracts?

**Interview Answer**

Consumer-driven contracts let the API consumer define what they need, ensuring providers don't break downstream services. Changes are negotiated before implementation, reducing surprises. Providers can see all consumers' expectations and assess the impact of changes. This approach scales well in microservice architectures where maintaining a central integration test suite is impractical.

---

### Q5. How do you handle contract versioning?

**Interview Answer**

Version contracts alongside the API using semantic versioning. Include the contract version in pact file metadata. Use a pact broker to track contract versions between consumers and providers. When a provider publishes a new version, verify it against all consumer contracts. Breaking changes require a new major version, and consumers must update their contracts before the provider removes old endpoints.

---

### Q6. How do you test message-based contracts (async APIs)?

**Interview Answer**

Pact supports asynchronous messaging contracts where the consumer defines expected message payloads. The provider verifies it publishes messages matching those payloads. In Rust, use message queues like RabbitMQ or Kafka in your tests. The consumer specifies the message topic and expected body structure. The provider tests that its event publishing code produces messages conforming to the contract.

---

### Q7. What tools support contract testing in Rust?

**Interview Answer**

The `pact` crate provides Rust-native pact support for both consumer and provider tests. `wiremock` can mock external APIs during contract verification. `schemars` generates JSON schemas from Rust types for schema validation. For OpenAPI, `paperclip` and `utoipa` can generate and validate specs. `reqwest` is used for making HTTP requests in contract verification tests.

---

### Q8. How do you integrate contract testing into CI/CD?

**Interview Answer**

Publish pact files to a pact broker during consumer CI builds. Provider CI pipelines pull contracts from the broker and run verification tests. Use the `pact_verifier` crate to programmatically verify contracts. Fail the provider build if any consumer contract is broken. Automate the feedback loop so providers know immediately when a change breaks a consumer.

---

### Q9. When should you use contract testing vs. end-to-end testing?

**Interview Answer**

Use contract testing when you have many microservices and need fast, independent verification of API compatibility. Use end-to-end testing for critical user journeys that span multiple services and require realistic data flow. Contract tests are cheaper and faster; e2e tests are more comprehensive but slower and more brittle. Most teams use a pyramid approach: many contract tests, fewer integration tests, and minimal e2e tests.

---

### Q10. How do you handle schema evolution in contract testing?

**Interview Answer**

Design contracts to be forward-compatible by making fields optional or adding new fields without removing old ones. Use additive changes where possible. When breaking changes are necessary, version the API and maintain backward compatibility during a transition period. Run contract tests against both old and new versions to ensure smooth migration. Document schema changes in changelogs so consumers can plan upgrades.
