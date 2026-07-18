# Authentication

## Interview Question

How do you implement authentication in a production Rust backend using Axum?

## Interview Answer

Authentication verifies that a user is who they claim to be. In a production Rust backend, I implement JWT-based authentication using access tokens and refresh tokens, password hashing with Argon2id for storage, and middleware extractors in Axum to validate tokens on protected routes. The login flow validates credentials against the database, issues a short-lived access token (15 minutes) and a long-lived refresh token (7 days), and the client includes the access token in the Authorization header. Axum extractors parse and validate the JWT, rejecting unauthorized requests before they reach handler logic. Passwords are never stored in plaintext — Argon2id provides memory-hard hashing that resists GPU and ASIC attacks.

---

## Follow-up Questions & Answers

### Q1. What is the difference between authentication and authorization?

**Interview Answer**

Authentication answers "who are you?" while authorization answers "what are you allowed to do?" Authentication verifies identity through credentials (passwords, tokens, biometrics). Authorization determines permissions after identity is established (role-based access, resource ownership). In a Rust backend, authentication is handled first (JWT validation middleware), then authorization checks permissions (RBAC middleware). Both are separate concerns that should not be conflated — an unauthenticated request is rejected before authorization checks, and an authenticated request may still be unauthorized for specific resources.

---

### Q2. Why use Argon2id over bcrypt for password hashing?

**Interview Answer**

Argon2id won the Password Hashing Competition and is the recommended algorithm for new applications. It is memory-hard, meaning it requires a configurable amount of memory to compute, making GPU and ASIC attacks more expensive. Argon2id also provides configurable time cost, memory cost, and parallelism, allowing you to tune security as hardware improves. bcrypt is still secure but has a fixed memory cost and is limited to 72 bytes of input. In Rust, the argon2 crate provides a safe implementation. Example: Argon2::default().hash_password(b"password", &salt).unwrap().

---

### Q3. How do JWT access tokens and refresh tokens work together?

**Interview Answer**

Access tokens are short-lived (15-60 minutes) and carry user identity and permissions in their payload. They are sent with every request and validated without database lookups. Refresh tokens are long-lived (days to weeks) and stored securely (httpOnly cookie or secure storage). When an access token expires, the client sends the refresh token to a dedicated endpoint to get a new access token pair. This limits the window of exposure if an access token is compromised. Refresh tokens can be revoked server-side (stored in a database) to immediately invalidate a user's session, which is not possible with stateless access tokens alone.

---

### Q4. How do you implement password reset securely?

**Interview Answer**

Generate a cryptographically random token using rand::thread_rng().gen::<[u8; 32>()> and store its SHA-256 hash in the database with a 1-hour expiry. Send the raw token to the user's email (never the hash). When the user submits the token and new password, hash the submitted token, compare with the stored hash, and if valid, hash the new password with Argon2id and update the database. Delete the reset token after use. Never reveal whether an email exists in the system (return a generic message). Rate limit reset requests to prevent enumeration and email flooding.

---

### Q5. What is session fixation and how do you prevent it?

**Interview Answer**

Session fixation occurs when an attacker sets a known session ID before the user authenticates, then hijacks the session after the user logs in. Prevention requires regenerating the session ID upon successful authentication — the old ID is invalidated and a new one is issued. In Rust with Axum, if using server-side sessions (tower-sessions), regenerate the session ID in the login handler. For JWT-based auth, session fixation is not applicable because tokens are stateless and signed — the server does not track session IDs. However, refresh token rotation (issuing new refresh tokens on each use) provides similar protection.

---

### Q6. How do you store passwords securely in PostgreSQL?

**Interview Answer**

Store the Argon2id hash, salt, and configuration parameters in a single column (e.g., TEXT or BYTEA). The hash format includes all parameters: $argon2id$v=19$m=65536,t=3,p=1$salt$hash. When verifying, the argon2 crate parses the stored hash string and uses its embedded parameters. Never store plaintext passwords, use reversible encryption, or use fast hashes like MD5/SHA-256 without salting. The salt should be randomly generated per password (Argon2id does this internally). In sqlx, use argon2::PasswordHash and verify_password for checking: Argon2::default().verify_password(password.as_bytes(), &hash).

---

### Q7. What are common authentication vulnerabilities and how do you prevent them?

**Interview Answer**

Common vulnerabilities include: brute force attacks (prevent with rate limiting and account lockout), credential stuffing (prevent with MFA and breached password checks), token theft (mitigate with short expiry, secure storage, and HTTPS), session fixation (prevent with session regeneration), timing attacks on token comparison (use constant-time comparison via the constant_time_eq function), and password enumeration (return generic error messages). In Rust, use the subtle crate for constant-time operations, implement rate limiting with tower::limit, and always enforce HTTPS in production.

---

### Q8. How do you implement multi-factor authentication (MFA) in Rust?

**Interview Answer**

MFA adds a second verification factor after password authentication. TOTP (Time-based One-Time Password) is the most common method — the server generates a shared secret during setup, stores it encrypted in the database, and validates the 6-digit code from the user's authenticator app using the totp-rs crate. During login, after password verification, prompt for the TOTP code. Backup codes should be generated as a set of one-time use codes, hashed before storage. WebAuthn/FIDO2 provides hardware key support using the webauthn-rs crate. MFA significantly reduces account compromise even if passwords are leaked.

---

### Q9. How do you handle authentication in a microservices architecture?

**Interview Answer**

In microservices, authentication is centralized at the API gateway or reverse proxy, which validates tokens and forwards user identity to downstream services via headers (e.g., X-User-ID). Each service trusts the gateway and does not re-validate tokens, avoiding redundant cryptographic operations. Alternatively, each service validates tokens independently using shared signing keys or a public key infrastructure. The JWT's claims carry the user's identity and roles, so services can make authorization decisions without calling an authentication service. In Rust, implement a shared authentication middleware crate that all services import.

---

### Q10. What is the difference between stateful and stateless authentication?

**Interview Answer**

Stateful authentication stores session data on the server (database, Redis), requiring a lookup on every request but allowing immediate session revocation. Stateless authentication (JWT) carries all necessary data in the token itself, requiring no server-side storage or lookup but making revocation difficult. JWTs can be revoked by maintaining a blocklist (adding state back) or by using short expiry with refresh tokens. In production Rust backends, the hybrid approach is common: stateless JWTs for fast validation with a short-lived blocklist for revoked tokens stored in Redis.
