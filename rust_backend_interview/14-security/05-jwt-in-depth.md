# JWT In Depth

## Interview Question

Explain the structure of JWTs and how you implement secure JWT handling in a Rust backend.

## Interview Answer

A JWT (JSON Web Token) consists of three Base64URL-encoded parts separated by dots: header (algorithm and key ID), payload (claims like sub, exp, iss, aud), and signature (verifies integrity). The signature is computed over header.payload using the specified algorithm (RS256 for asymmetric, HS256 for symmetric). In a Rust backend, I use the jsonwebtoken crate to create and validate JWTs with RS256 (RSA) for production, store private keys securely, validate all claims (exp, iss, aud) on every request, and implement refresh token rotation. JWTs are stateless — the server does not track them — so security depends entirely on proper signing, short expiry, and secure key management.

---

## Follow-up Questions & Answers

### Q1. What is the difference between symmetric and asymmetric JWT signing?

**Interview Answer**

Symmetric signing (HS256) uses a single shared secret for both signing and verification — the same key creates and validates the token. It is faster but problematic for distributed systems because every service that validates the token needs the secret. Asymmetric signing (RS256, ES256) uses a private key for signing and a public key for verification. Only the issuer needs the private key; validators only need the public key, which can be safely distributed. In production Rust backends, asymmetric signing is preferred because it enables third-party verification without exposing signing keys.

---

### Q2. What claims should a production JWT include?

**Interview Answer**

Standard claims: sub (user ID), iss (issuer — your API domain), aud (audience — the intended consumer), exp (expiry — 15 minutes for access tokens), iat (issued at), jti (unique token ID for revocation). Custom claims: roles, permissions, tenant_id, or any context needed by downstream services. The key is to keep the payload minimal — large payloads increase network overhead and exposure risk. Never include sensitive data (passwords, SSNs) in JWTs because the payload is only encoded, not encrypted. In Rust, use serde to map claims to a struct with #[serde(rename_all = "camelCase")].

---

### Q3. How do you validate a JWT in Rust using the jsonwebtoken crate?

**Interview Answer**

Use jsonwebtoken::decode with a Validation struct that specifies required algorithms, expected issuer, expected audience, and validates expiry. Example: let token_data = decode::<Claims>(token, &DecodingKey::from_rsa_pem(public_key_bytes)?, &Validation::new(Algorithm::RS256))?. The Validation struct automatically checks exp, iss, and aud if configured. Always specify allowed algorithms explicitly to prevent algorithm confusion attacks (never allow "none"). After decoding, perform additional business logic validation: check if the user still exists, if the token is revoked, or if the user's roles have changed.

---

### Q4. What is a JWT algorithm confusion attack and how do you prevent it?

**Interview Answer**

An algorithm confusion attack occurs when an attacker changes the JWT header's algorithm from RS256 to HS256, then signs the token with the public key (which is often publicly available) as the HMAC secret. If the server validates using HS256 with the public key as the secret, the forged token passes validation. Prevention: always specify the allowed algorithms explicitly in the Validation struct: Validation::new(Algorithm::RS256).never allow the "none" algorithm. In Rust, jsonwebtoken prevents this by requiring explicit algorithm specification. Additionally, store algorithm verification in a middleware layer that rejects tokens with unexpected algorithms.

---

### Q5. How do you implement JWT refresh token rotation?

**Interview Answer**

When a client uses a refresh token, issue a new access token AND a new refresh token, then invalidate the old refresh token. Store refresh tokens in the database with user_id, token_hash, and expiry. On refresh, validate the old token exists and is not expired, issue new tokens, delete the old refresh token, and insert the new one. If the old refresh token is reused after rotation (indicating theft), revoke all refresh tokens for that user and require re-authentication. In Rust, implement this in a dedicated refresh handler that performs atomic database operations: delete old token, insert new token, return new tokens.

---

### Q6. How do you handle JWT expiry in a scalable system?

**Interview Answer**

JWT expiry is validated by the exp claim — the server checks current_time < exp. In a distributed system, clock skew between servers can cause issues. Use NTP to synchronize clocks and allow a small clock skew (30-60 seconds) in validation. For access tokens, short expiry (15 minutes) limits exposure. For refresh tokens, longer expiry (7-30 days) with rotation on use. Background cleanup jobs remove expired tokens from the database. In Rust, use chrono::Utc::now() for consistent time handling and configure jsonwebtoken's Validation with leeway(Duration::from_secs(30)) to account for clock skew.

---

### Q7. What is the difference between JWE and JWS?

**Interview Answer**

JWS (JSON Web Signature) signs the JWT to ensure integrity and authenticity but does not encrypt the payload — anyone with the token can read the claims. JWE (JSON Web Encryption) encrypts the JWT payload so only the intended recipient can read it. JWS is sufficient for most use cases because JWT payloads should not contain sensitive data. JWE is needed when the JWT carries PII, financial data, or other sensitive information that should not be visible in transit. In Rust, the josekit crate supports both JWS and JWE. For most Axum backends, JWS with RS256 is the standard.

---

### Q8. How do you implement JWT-based single sign-out?

**Interview Answer**

Single sign-out requires revoking the user's tokens across all devices. Since JWTs are stateless, implement revocation by maintaining a token blocklist (Redis with TTL matching token expiry). When a user logs out, add their token's jti to the blocklist. During validation, check the blocklist before accepting the token. For a complete solution, also revoke all refresh tokens for the user, preventing token refresh. In Rust, implement a Redis-backed blocklist that the validation middleware checks on every request. The blocklist entry TTL should match the token's remaining expiry to avoid memory leaks.

---

### Q9. What are the security implications of storing JWTs in localStorage vs cookies?

**Interview Answer**

localStorage is accessible to JavaScript, making it vulnerable to XSS attacks — an attacker who injects script can steal the token. HttpOnly cookies are not accessible to JavaScript, mitigating XSS token theft, but are vulnerable to CSRF (Cross-Site Request Forgery). For API-only backends, localStorage with CORS restrictions is acceptable. For applications serving both HTML and API, use HttpOnly Secure SameSite=Strict cookies with CSRF tokens. In Axum, set cookies with axum::http::header::SET_COOKIE and validate CSRF tokens with tower-sessions middleware.

---

### Q10. How do you handle JWT in a microservices architecture?

**Interview Answer**

In microservices, the API gateway validates the JWT and forwards user claims to downstream services via headers (X-User-ID, X-User-Roles). Downstream services trust the gateway and do not re-validate the JWT's signature — this avoids redundant cryptographic operations. However, for zero-trust architectures, each service validates the JWT independently using the public key (JWKS). The trade-off is performance (validation overhead) vs security (no single point of failure). In Rust, implement a shared JWT validation crate that all services use, with cached JWKS keys refreshed every 24 hours.
