# CORS Configuration

## Interview Question

What is CORS and how do you configure it securely in a Rust Axum backend?

## Interview Answer

CORS (Cross-Origin Resource Sharing) is a browser security mechanism that restricts web pages from making requests to a different origin (domain, protocol, or port) than the one that served the page. Without CORS, a malicious site could make API calls to your backend using the user's cookies. In a Rust Axum backend, CORS is configured using the tower-http CorsLayer, which adds appropriate Access-Control-Allow-Origin, Methods, and Headers headers. A secure configuration specifies exact allowed origins (never * in production), restricts allowed methods to what is actually needed, and limits exposed headers. Preflight requests (OPTIONS) must be handled to allow browsers to verify cross-origin access before sending the actual request.

---

## Follow-up Questions & Answers

### Q1. How does CORS work at a technical level?

**Interview Answer**

When a browser makes a cross-origin request, it first sends a preflight OPTIONS request to check if the server allows it. The preflight includes Origin, Access-Control-Request-Method, and Access-Control-Request-Headers. The server responds with Access-Control-Allow-Origin, Access-Control-Allow-Methods, Access-Control-Allow-Headers, and Access-Control-Max-Age (how long to cache the preflight response). Only after the preflight succeeds does the browser send the actual request. Simple requests (GET, HEAD, POST with simple headers) skip the preflight. In Axum, CorsLayer handles both preflight and actual requests automatically.

---

### Q2. Why should you never use Access-Control-Allow-Origin: * in production?

**Interview Answer**

A wildcard origin allows any website to make requests to your API, potentially with the user's cookies or credentials. This defeats the purpose of CORS as a security boundary. An attacker's site could make authenticated requests to your API on behalf of your users. Use specific origins instead: https://yourapp.com, https://admin.yourapp.com. In Axum, use CorsLayer::new().allow_origin("https://yourapp.com".parse().unwrap()) or allow_origin(vec!["https://app.com".parse().unwrap(), "https://admin.com".parse().unwrap()]). For development, use environment variables to configure allowed origins.

---

### Q3. What is the difference between simple and preflight requests?

**Interview Answer**

Simple requests are GET, HEAD, or POST requests with only simple headers (Accept, Content-Language, Content-Type with specific values). They are sent directly without a preflight. Non-simple requests (PUT, DELETE, PATCH, or requests with custom headers like Authorization) trigger a preflight OPTIONS request first. The browser sends the preflight, waits for approval, then sends the actual request. In Axum, the CorsLayer middleware automatically handles preflight responses. For API authentication with custom Authorization headers, every request triggers a preflight, so you must configure CORS to allow the Authorization header.

---

### Q4. How do you configure CORS in Axum with different environments?

**Interview Answer**

Use environment variables to configure allowed origins per environment. In development, allow localhost origins: CorsLayer::new().allow_origin(["http://localhost:3000".parse().unwrap()]). In production, restrict to your domain: allow_origin(["https://api.yourapp.com".parse().unwrap()]). Use tower-http's CorsLayer::permissive() only during local development — never in production. For multiple frontend apps, list all origins. Example: let allowed_origins = env::var("CORS_ORIGINS")?.split(',').map(|o| o.parse()).collect(); CorsLayer::new().allow_origin(allowed_origins).

---

### Q5. What are the security implications of misconfigured CORS?

**Interview Answer**

Misconfigured CORS can: allow credential theft (Access-Control-Allow-Credentials: true with wildcard origin), enable CSRF attacks (allowing POST from any origin), expose sensitive headers (Access-Control-Expose-Headers with sensitive data), and bypass same-origin policy protections. Common mistakes: using * with credentials (browsers reject this), allowing all methods when only GET is needed, and not validating the Origin header server-side. In production Rust backends, CORS misconfiguration is a frequent vulnerability — audit your CORS policy regularly and test with browser developer tools.

---

### Q6. How does CORS relate to CSRF protection?

**Interview Answer**

CORS and CSRF protection serve complementary purposes. CORS restricts which origins can make cross-origin requests from browsers. CSRF protection ensures that requests are intentionally initiated by the user (using CSRF tokens, SameSite cookies, or other mechanisms). CORS does not protect against CSRF because simple requests (GET, POST with form data) do not trigger preflights, and CORS does not apply to same-origin requests. For Rust backends, implement CSRF protection using SameSite=Strict cookies and CSRF tokens alongside CORS configuration. Both are needed for complete protection.

---

### Q7. What is Access-Control-Allow-Credentials and when should you use it?

**Interview Answer**

Access-Control-Allow-Credentials: true tells the browser that the server accepts requests with credentials (cookies, Authorization header). Without this header, the browser strips credentials from cross-origin requests. You need it when your API uses cookie-based authentication and the frontend is on a different origin. In Axum, set it with CorsLayer::new().allow_credentials(true). Important: when allow_credentials is true, you cannot use wildcard (*) for Access-Control-Allow-Origin — you must specify exact origins. This prevents a malicious site from reading authenticated responses.

---

### Q8. How do you handle CORS with WebSockets?

**Interview Answer**

WebSocket connections are initiated with an HTTP upgrade request, which is subject to CORS preflight. The browser checks Access-Control-Allow-Origin on the upgrade response. In Axum, the WebSocketUpgrade handler runs after CORS middleware, so CORS headers are included in the upgrade response. For development, ensure your CORS configuration allows the WebSocket origin. In production, CORS for WebSockets follows the same rules as regular HTTP — restrict origins to your domain. Note that CORS only applies to the initial handshake; WebSocket frames are not subject to CORS.

---

### Q9. How do you test CORS configuration?

**Interview Answer**

Test with browser developer tools (Network tab) to verify preflight responses include correct headers. Use curl to test with different Origin headers: curl -H "Origin: https://evil.com" -H "Access-Control-Request-Method: POST" -X OPTIONS https://yourapi.com/endpoint. Verify that: allowed origins match your frontend domains, allowed methods match your API methods, allowed headers include Authorization if used, credentials are handled correctly, and max-age is set to avoid excessive preflights. Write integration tests that simulate cross-origin requests and verify CORS headers. Use OWASP ZAP or similar tools for automated CORS testing.

---

### Q10. How do you handle CORS in a microservices architecture?

**Interview Answer**

In microservices, CORS should be handled at the API gateway or reverse proxy (Nginx, Traefik), not in individual services. This centralizes CORS policy and avoids inconsistent configurations. The gateway adds CORS headers before forwarding requests to backend services. If services are internal (not directly accessible from browsers), CORS is unnecessary — it only applies to browser-initiated requests. For Rust microservices behind Nginx, configure CORS in Nginx's location block with add_header directives. Individual Axum services should not configure CORS unless they are directly exposed to browsers.
