# LEVEL 09 — API & Backend Engineering

### 0229. REST

Architectural style: resources identified by URLs, manipulated with HTTP methods, stateless requests. Not a strict standard — people say REST meaning "JSON over HTTP with resourcey paths."

Good public APIs: clear resources, proper status codes, idempotent methods where expected.

**Talk track:** *"REST models the API as resources and HTTP verbs — simplicity and universality beat purity debates."*

---

### 0230. GraphQL

Clients ask for exactly the fields they need via a single endpoint and schema. Powerful for flexible UIs; risks: N+1 queries, expensive arbitrary queries, caching harder than REST.

Needs query cost limits and dataloaders in serious systems.

**Talk track:** *"GraphQL shifts shape control to the client — you must gate cost and protect the data layer."*

---

### 0231. gRPC

(See also 0052.) Contract-first RPC with Protobuf. Strong for internal microservices; streaming support. Less friendly for anonymous public browser clients without extra layers.

**Talk track:** *"gRPC is high-efficiency internal RPC with schemas and streaming."*

---

### 0232. OpenAPI

Machine-readable HTTP API description (Swagger). Generates docs, clients, mocks, gateway config. Source of truth for public HTTP contracts.

**Talk track:** *"OpenAPI is the contract file for REST — generate docs and clients from it."*

---

### 0233. API Versioning

Evolve without breaking clients: URL (`/v1`), headers, or content negotiation. Deprecation policy matters more than the mechanism.

Breaking changes need a migration window.

**Talk track:** *"Versioning is a social contract with clients — plan deprecation, not only paths."*

---

### 0234. Pagination

Split large lists into pages. Offset/limit is simple but slow/unstable on deep pages and churning data.

Always return a stable ordering key.

**Talk track:** *"Pagination protects the server and the client from mega-payloads — pick a strategy that matches data churn."*

---

### 0235. Cursor Pagination

Use an opaque cursor (often encoded key/timestamp) for "next page." Stable under inserts, efficient with indexed queries. Preferred for feeds and large tables.

**Talk track:** *"Cursors scale better than OFFSET for big, changing datasets."*

---

### 0236. Filtering

Query params or POST bodies that narrow results. Validate fields; index common filters; prevent unrestricted scanning.

**Talk track:** *"Filters are user-controlled queries — bound them and index them."*

---

### 0237. Sorting

Order results by fields. Whitelist sortable columns to avoid injection and expensive sorts. Default sort must be deterministic for pagination.

**Talk track:** *"Sorting must be deterministic and indexed or it becomes a full scan."*

---

### 0238. Validation

Check inputs before business logic: types, ranges, formats, business rules. Fail fast with clear errors. Never trust clients.

Layer: parse → validate → authorize → execute.

**Talk track:** *"Validation is the immune system of an API — assume hostile or buggy clients."*

---

### 0239. Idempotency

Repeating the same request leaves the system in the same state (no double charge). Use idempotency keys for payments/POSTs. Safe methods (GET) should be idempotent by nature.

Essential with retries and at-least-once delivery.

**Talk track:** *"Idempotency makes retries safe — store keys and return the first result on replay."*
