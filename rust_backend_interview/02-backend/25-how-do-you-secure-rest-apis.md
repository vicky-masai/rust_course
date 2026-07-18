# How do you secure REST APIs?

## Interview Question

How do you secure REST APIs?

## Interview Answer

"JWT authentication, RBAC/ABAC authorization, HTTPS, input validation, SQL injection prevention, CORS, rate limiting, and audit logging."

---

## Follow-up Questions & Answers

### Q1. How do you implement JWT authentication in an Axum application?

**Interview Answer**

Use the `axum-extra` crate with `TypedHeader` to extract Bearer tokens from the `Authorization` header. Validate the JWT signature and claims using `jsonwebtoken`, then pass the claims to handlers via Axum's request extensions. Implement a middleware layer that checks token validity before reaching protected routes.

---

### Q2. What is the difference between RBAC and ABAC authorization?

**Interview Answer**

RBAC assigns roles like `admin` or `user` to principals and checks permissions based on role membership. ABAC evaluates attributes of the user, resource, and environment against policies for fine-grained access control. RBAC is simpler to implement in Axum middleware; ABAC requires a policy engine like Open Policy Agent.

---

### Q3. How do you prevent SQL injection in a Rust backend?

**Interview Answer**

Use sqlx's parameterized queries with `query!` or `query_as!` macros which enforce bind parameters at compile time. Never concatenate user input into SQL strings. sqlx's compile-time validation catches unsafe query construction during the build process, making injection prevention automatic.

---

### Q4. How should you handle CORS in an Axum application?

**Interview Answer**

Use `tower-http::cors::CorsLayer` to configure allowed origins, methods, and headers. Set `Access-Control-Allow-Credentials` for cookie-based authentication and restrict `Access-Control-Allow-Origin` to specific domains in production. In Axum, apply the CORS layer to the router before other middleware.

---

### Q5. What rate limiting strategy should you use for REST APIs?

**Interview Answer**

Implement token bucket or sliding window algorithms using `governor` or `tower_governor` for per-client rate limiting. Apply different limits for authenticated vs anonymous users based on API key or IP address. Return `429 Too Many Requests` with `Retry-After` header to allow clients to back off gracefully.

---

### Q6. How do you secure JWT tokens against common attacks?

**Interview Answer**

Use short expiration times (15-30 minutes) with refresh tokens stored in HTTP-only cookies. Validate the `iss`, `aud`, and `exp` claims on every request. Implement token revocation using a Redis blocklist for logout and compromised tokens. Use RS256 or ES256 instead of HS256 for production JWTs.

---

### Q7. What input validation strategy should you follow in Axum?

**Interview Answer**

Use `serde` with derive macros and validation attributes like `#[validate(length(min = 1))]` via the `validator` crate. Validate at the Axum handler boundary using `axum::extract::Json<T>` with validated structs. Return `400 Bad Request` with descriptive error messages for invalid inputs.

---

### Q8. How do you implement audit logging for API security?

**Interview Answer**

Log authentication events, authorization failures, and data modifications with structured fields using the `tracing` crate. Store audit logs in a separate append-only table or external service like ELK. In Axum, create a middleware that captures request metadata, user identity, and response status for each protected endpoint.
