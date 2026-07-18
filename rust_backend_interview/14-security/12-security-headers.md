# Security Headers

## Interview Question

What security headers should you configure in a production Rust backend and why?

## Interview Answer

Security headers are HTTP response headers that instruct browsers to enforce security policies. The essential headers are: Content-Security-Policy (prevents XSS by controlling allowed resources), Strict-Transport-Security (forces HTTPS), X-Content-Type-Options (prevents MIME sniffing), X-Frame-Options (prevents clickjacking), Referrer-Policy (controls referrer information leakage), and Permissions-Policy (restricts browser features). In a Rust backend, these headers are added at the Nginx reverse proxy layer using add_header directives, or in Axum using tower-http middleware. Properly configured security headers significantly reduce the attack surface of web applications without requiring application code changes.

---

## Follow-up Questions & Answers

### Q1. What is Content-Security-Policy (CSP) and how do you configure it?

**Interview Answer**

CSP restricts which resources (scripts, styles, images, fonts, connections) the browser is allowed to load. A strict policy like default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline' prevents XSS by blocking inline scripts and external resources. Report-uri directive sends violation reports to your server. Start with Content-Security-Policy-Report-Only to test without breaking functionality. In Rust, set CSP via middleware or Nginx's add_header. A well-configured CSP is one of the most effective XSS defenses — it prevents injected scripts from executing even if they bypass input validation.

---

### Q2. What is HSTS and why is it important?

**Interview Answer**

Strict-Transport-Security (HSTS) tells browsers to only connect to your site via HTTPS, even if the user types HTTP. This prevents downgrade attacks where an attacker forces HTTP connections. The header includes max-age (how long to remember, typically 1 year), includeSubDomains (apply to all subdomains), and preload (include in browser HSTS preload list). In Nginx: add_header Strict-Transport-Security "max-age=31536000; includeSubDomains; preload" always. HSTS is critical because it eliminates the window for SSL stripping attacks on the very first connection (before the user has ever received the HSTS header).

---

### Q3. What is X-Content-Type-Options and why use nosniff?

**Interview Answer**

X-Content-Type-Options: nosniff prevents browsers from MIME-sniffing a response away from the declared Content-Type. Without this header, a browser might interpret a text/plain response as JavaScript and execute it, enabling XSS attacks. This is especially important for file uploads — if a user uploads an HTML file disguised as an image, nosniff forces the browser to respect Content-Type: image/png and not execute the HTML. In Nginx: add_header X-Content-Type-Options "nosniff" always. This is one of the simplest and most effective security headers to configure.

---

### Q4. What is X-Frame-Options and how does it prevent clickjacking?

**Interview Answer**

X-Frame-Options prevents your page from being embedded in iframes on other sites, which could be used for clickjacking attacks (overlaying invisible frames to trick users into clicking things). DENY prevents any framing, SAMEORIGIN allows framing by the same domain. In Nginx: add_header X-Frame-Options "DENY" always. For modern browsers, CSP frame-ancestors directive provides more granular control and supersedes X-Frame-Options. Use both for defense-in-depth. For applications that legitimately embed content (widgets, dashboards), use frame-ancestors with specific allowed origins.

---

### Q5. What is Referrer-Policy and why does it matter?

**Interview Answer**

Referrer-Policy controls how much information the browser sends in the Referer header when navigating between pages. Strict-origin-when-cross-origin sends the full URL for same-origin requests, only the origin for cross-origin requests, and nothing for downgrade (HTTPS to HTTP). This prevents leaking sensitive URLs (containing tokens, internal paths) to external sites. In Nginx: add_header Referrer-Policy "strict-origin-when-cross-origin" always. For applications with sensitive URLs (password reset pages, admin panels), use no-referrer to prevent any URL leakage.

---

### Q6. What is Permissions-Policy and what should you restrict?

**Interview Answer**

Permissions-Policy (formerly Feature-Policy) restricts which browser features your site can use: camera, microphone, geolocation, payment, USB, etc. A strict policy: Permissions-Policy: camera=(), microphone=(), geolocation=(), payment=() disables all these features. This reduces the attack surface — even if an XSS attack occurs, the attacker cannot access the camera or microphone. In Nginx: add_header Permissions-Policy "camera=(), microphone=(), geolocation=()" always. Only enable features your application actually needs.

---

### Q7. How do you implement security headers in Axum?

**Interview Answer**

Use tower-http middleware to add security headers to all responses. Create a SecurityHeadersLayer that adds each header: .layer(AddHeadersLayer::new(vec![("X-Content-Type-Options", "nosniff"), ("X-Frame-Options", "DENY"), ("Referrer-Policy", "strict-origin-when-cross-origin")])). For CSP, build the policy string based on your application's resource needs. Apply the layer to the Axum router so all routes receive the headers. Alternatively, configure headers in Nginx for the reverse proxy layer, which is simpler and ensures headers are added even for error responses.

---

### Q8. What are the security implications of missing these headers?

**Interview Answer**

Missing CSP allows XSS attacks to execute injected scripts. Missing HSTS allows downgrade to HTTP and SSL stripping. Missing X-Content-Type-Options allows MIME sniffing attacks. Missing X-Frame-Options allows clickjacking. Missing Referrer-Policy leaks sensitive URLs. Missing Permissions-Policy allows unauthorized feature access. Together, these headers form a defense-in-depth strategy. Even with perfect input validation and output encoding, missing headers create attack surface. Security headers are low-effort, high-impact improvements that should be configured on every production web application.

---

### Q9. How do you test security header configuration?

**Interview Answer**

Use securityheaders.com to scan your site and check for missing or misconfigured headers. Use OWASP ZAP for automated security scanning. In CI, add a test that verifies expected headers are present in responses: send a request to your staging environment and assert each header exists with the correct value. Write integration tests in Rust using reqwest that verify headers: let resp = client.get(url).send().await?; assert!(resp.headers().contains_key("x-content-type-options")); Monitor header configuration drift through regular scans.

---

### Q10. How do security headers interact with CSP for single-page applications?

**Interview Answer**

SPAs (React, Vue, Angular) require relaxed CSP because they use inline scripts (webpack bundles), eval() in some cases, and dynamic resource loading. Start with a strict CSP and add necessary exceptions: script-src 'self' 'nonce-{random}' for bundled scripts, style-src 'self' 'unsafe-inline' for CSS-in-JS. Use nonces or hashes instead of 'unsafe-inline' where possible. For Axum serving a SPA, the index.html should include the nonce in script tags. Report violations first with Content-Security-Policy-Report-Only, then enforce once the policy is tuned. The key is progressive strictness — start permissive and tighten over time.
