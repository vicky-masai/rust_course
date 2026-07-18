# Testing Axum APIs

## Interview Question

How do you write tests for axum-based HTTP APIs without starting a real server?

## Interview Answer

Axum provides `axum::test` utilities and integrates with `tower::ServiceExt` to test handlers in-process. You can use `axum::Router` directly in tests by calling it with `tower::ServiceExt::oneshot` and a constructed `http::Request`. The `reqwest` crate can test a running server on a test port. For handler-level testing, you extract the router's service, send requests, and assert on responses without network overhead. Mock dependencies are injected via axum's state extraction using `Extension` or `State`.

---

## Follow-up Questions & Answers

### Q1. How do you use `axum::test` to test handlers?

**Interview Answer**

Create your `Router` in a function that accepts shared state as a parameter. In tests, construct test state (mocks, test DB connections), build the router, and use `oneshot` to send requests. The `axum::body::to_bytes` function helps extract the response body. For example: `let response = router.oneshot(request).await.unwrap(); assert_eq!(response.status(), 200);`.

---

### Q2. How do you inject mock dependencies into axum handlers?

**Interview Answer**

Use axum's `State` extractor to pass shared state into handlers. Define a state struct containing trait objects or mock implementations. In tests, construct the state with test doubles and pass it to `Router::new().route("/path", get(handler)).with_state(test_state)`. This avoids global state and makes handlers fully testable with different dependency behaviors.

---

### Q3. How do you test middleware in axum?

**Interview Answer**

Wrap your router with the middleware in tests just like in production. Use `tower::ServiceExt::oneshot` to send requests through the full middleware stack. For specific middleware testing, compose just that layer with a mock service. Check that headers, status codes, and response bodies are modified correctly by the middleware. Test edge cases like missing headers, expired tokens, and rate limiting.

---

### Q4. How do you set up test fixtures for axum API tests?

**Interview Answer**

Create a `test_app` function that returns both the router and test-specific handles (like database connection pools or shutdown signals). Use `#[ctor]` or `once_cell` for one-time test setup like database migrations. Generate unique test data per test using `uuid::Uuid::new_v4()`. Clean up after tests using `sqlx` transactions that get rolled back.

---

### Q5. How do you test JSON request/response bodies?

**Interview Answer**

Use `serde_json::json!` macro to construct request bodies and `serde_json::from_slice` to deserialize responses. Set the `Content-Type: application/json` header on test requests. Use `axum::Json` in your handlers and verify the response body matches expected JSON structures. The `pretty_assertions` crate makes JSON comparison failures much easier to debug.

---

### Q6. How do you test authentication and authorization in axum?

**Interview Answer**

Create test tokens using the same JWT library as production but with test secrets. Pass the token in the `Authorization: Bearer <token>` header. Test unauthorized access (missing token), forbidden access (valid token, insufficient permissions), and expired tokens. Mock the token validation service to return different claims for different tests. Use `Extension` to inject mock auth services.

---

### Q7. What is the difference between testing with `oneshot` and starting a server?

**Interview Answer**

`oneshot` sends a single request through the router's service without starting a TCP listener. It's fast, has no network overhead, and is ideal for unit-level handler tests. Starting a server (on a random port with `tokio::net::TcpListener`) tests the full stack including network, connection handling, and keep-alive. Use `oneshot` for most handler tests and server tests for critical end-to-end paths.

---

### Q8. How do you test WebSocket handlers in axum?

**Interview Answer**

Use the `axum-test` crate or manually establish a WebSocket connection to the test server. Start the server on a random port, connect with `tokio-tungstenite`, send messages, and assert on responses. Test connection establishment, message handling, disconnection, and error scenarios. Use `tokio::time::timeout` to prevent tests from hanging on unresponsive connections.

---

### Q9. How do you test file upload and download endpoints?

**Interview Answer**

Construct multipart requests using `reqwest::multipart::Form` with test file data. Verify the server correctly parses the multipart body and stores the file. For downloads, request the file endpoint and verify the response headers (`Content-Type`, `Content-Disposition`) and body bytes. Use temporary directories that get cleaned up after tests. Test edge cases like empty files, large files, and invalid content types.

---

### Q10. How do you test rate limiting and other cross-cutting concerns?

**Interview Answer**

Apply the rate limiting layer to your test router and send multiple requests in quick succession. Verify that requests beyond the limit receive 429 status codes. Test CORS by sending preflight OPTIONS requests and checking the `Access-Control-Allow-*` headers. For request ID middleware, verify that responses include the expected header and that the same ID appears in log output.
