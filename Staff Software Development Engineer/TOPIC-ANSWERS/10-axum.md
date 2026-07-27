# LEVEL 10 — Axum

### 0240. Router

Maps HTTP paths/methods to handlers. Nesting, merging, and route parameters live here. The skeleton of an Axum app.

**Talk track:** *"The router is the table of contents for your HTTP surface."*

---

### 0241. Middleware

Functions that wrap requests/responses: logging, auth, timeouts, CORS. Run before/after handlers. Order matters.

**Talk track:** *"Middleware is the pipeline around handlers — cross-cutting concerns belong here."*

---

### 0242. Tower Layers

Axum is built on Tower — layered services (`layer`). Rate limits, tracing, load shed as composable `Service` wrappers.

**Talk track:** *"Tower layers are composable service middleware — Axum speaks that dialect."*

---

### 0243. Custom Extractors

Implement `FromRequest` / `FromRequestParts` to pull typed data from requests (auth user, config, validated JSON). Keeps handlers clean.

**Talk track:** *"Extractors turn raw requests into typed inputs — the Axum way to share parsing."*

---

### 0244. Request Extensions

Type-map baggage attached to a request (connection state, request IDs). Handy; don't overuse as a junk drawer — prefer extractors.

**Talk track:** *"Extensions are typed request-scoped scratch space."*

---

### 0245. Authentication

Prove who the caller is — sessions, JWT bearer tokens, mTLS, API keys. Happens before authorization.

**Talk track:** *"Authentication answers 'who are you?' — verify credentials/tokens at the edge of the app."*

---

### 0246. Authorization

Prove what they're allowed to do — roles, permissions, policies. After identity is known.

**Talk track:** *"Authorization answers 'are you allowed?' — never conflate with login alone."*

---

### 0247. RBAC

Role-Based Access Control: users get roles; roles get permissions. Simple mental model for many business apps.

Roles explode if overused — combine with careful permission design.

**Talk track:** *"RBAC maps users → roles → permissions — simple until role combinatorics explode."*

---

### 0248. ABAC

Attribute-Based Access Control: decisions from attributes (dept, resource owner, time, geo). More flexible, harder to reason/audit.

**Talk track:** *"ABAC decides from attributes and policies — powerful and operationally heavier."*

---

### 0249. Rate Limiting

Cap requests per key (IP, user, API key) to protect availability and control cost. Token bucket / sliding window common. Return `429` with clear headers when possible.

**Talk track:** *"Rate limits protect the system from abuse and accidental stampedes."*

---

### 0250. WebSockets

(See 0050.) In Axum: upgrade handler, then message loop with backpressure. Auth on upgrade; heartbeat timeouts.

**Talk track:** *"Axum WebSockets are long-lived tasks — plan auth, heartbeats, and fanout."*

---

### 0251. SSE

(See 0051.) Stream events with `text/event-stream`. Simpler push model for notifications and live feeds.

**Talk track:** *"SSE in Axum is an async stream of events to the client."*

---

### 0252. Background Workers

Work outside the request path: queues, `tokio::spawn`, separate consumer services. Don't do heavy jobs inside HTTP handlers if users wait.

**Talk track:** *"Handlers should be quick — push slow work to background workers with retries."*
