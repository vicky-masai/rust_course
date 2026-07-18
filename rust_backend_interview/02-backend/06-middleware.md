# Middleware

## Interview Question

Middleware.

## Interview Answer

"Middleware handles authentication, logging, request IDs, CORS, rate limiting, and error handling."

---

## Follow-up Questions & Answers

### Q1. How does middleware work in Axum specifically?

**Interview Answer**

Axum middleware is implemented as a `tower::Layer` that wraps the inner service. You apply it using `.layer()` on the `Router`, and it intercepts every request before it reaches the handler. The middleware can modify the request, short-circuit with a response, or run logic after the handler completes.

---

### Q2. What is the order of middleware execution in Axum?

**Interview Answer**

Middleware executes in the order they are added, with the outermost layer running first. For example, if you add logging first and then authentication, logging wraps authentication. This means logging captures every request including rejected ones, while authentication only runs after logging has started. I place CORS and logging as outer layers and auth closer to the handlers.

---

### Q3. How would you implement JWT authentication middleware in Axum?

**Interview Answer**

I create an extractor struct `AuthUser` that reads the `Authorization` header, validates the JWT using `jsonwebtoken`, and extracts the user claims. If the token is missing or invalid, it returns a `401 Unauthorized` response. Handlers that need authentication simply include `AuthUser` as a parameter, and Axum runs the extraction automatically.

---

### Q4. How do you implement rate limiting as middleware?

**Interview Answer**

I use a token bucket or sliding window algorithm stored in Redis to track request counts per IP or API key. The middleware checks the count before passing the request to the handler and returns `429 Too Many Requests` with a `Retry-After` header when the limit is exceeded. For single-server setups, I use an in-memory `DashMap` counter instead of Redis.

---

### Q5. What is the difference between middleware and interceptors in the context of backend services?

**Interview Answer**

Middleware wraps the entire request-response pipeline and can modify both the request before processing and the response after. Interceptors typically operate at a higher level, like in gRPC, and focus on cross-cutting concerns like logging or tracing. In Axum, tower layers serve both roles since they have full access to the request and response lifecycle.

---

### Q6. How do you handle CORS in an Axum application?

**Interview Answer**

I use the `tower-http::cors` layer with allowed origins, methods, and headers configured explicitly rather than using wildcards. For example, I allow `GET`, `POST`, `PUT`, `DELETE` methods and `Authorization` and `Content-Type` headers from the frontend domain. This prevents browsers from blocking legitimate cross-origin requests while maintaining security.

---

### Q7. Can middleware cause performance issues and how do you mitigate them?

**Interview Answer**

Yes, middleware that performs expensive operations like full request body parsing or external API calls on every request adds latency. I keep middleware lightweight and only apply it where needed using Axum's `.route_layer()` instead of `.layer()` to scope it to specific routes. Heavy processing like request logging with body inspection is gated behind feature flags or debug modes.

---

### Q8. How do you propagate request context through middleware layers?

**Interview Answer**

I use Axum's `Extension` or `request.extensions()` to insert values like request IDs or authenticated user info in middleware. Downstream handlers extract these values using the `Extension<T>` extractor. For tracing, I use the `tracing::Span` context so logs are automatically correlated with the correct request across all layers.

---
