# OAuth2 and OpenID Connect

## Interview Question

Explain OAuth2 and OpenID Connect and how you implement them in a Rust backend.

## Interview Answer

OAuth2 is an authorization framework that allows third-party applications to obtain limited access to a user's resources without exposing credentials. It defines several grant types for different scenarios: Authorization Code (for server-side apps), Client Credentials (for service-to-service), and PKCE (for SPAs and mobile apps). OpenID Connect (OIDC) is an authentication layer built on top of OAuth2, adding a standardized way to verify user identity through an ID token (JWT). In a Rust backend, you implement OAuth2 by acting as either the identity provider (issuing tokens) or the client (consuming tokens from providers like Google or GitHub). Libraries like oauth2-rs handle the protocol details, while you manage token storage, session handling, and user provisioning.

---

## Follow-up Questions & Answers

### Q1. What are the different OAuth2 grant types and when do you use each?

**Interview Answer**

Authorization Code is the most secure for server-side applications — the client redirects to the auth server, receives an authorization code, and exchanges it for tokens. Authorization Code with PKCE is for SPAs and mobile apps that cannot securely store client secrets. Client Credentials is for machine-to-machine communication where no user is involved. Implicit is deprecated due to security concerns (tokens exposed in URL fragments). Resource Owner Password Credentials is also deprecated (shares user credentials directly). In Rust backends, use Authorization Code with PKCE for frontend clients and Client Credentials for service-to-service.

---

### Q2. What is the difference between an access token and an ID token?

**Interview Answer**

An access token is a credential for accessing protected resources (APIs). It is typically an opaque string or JWT, and its content is not meant for the client to parse. An ID token is a JWT that contains user identity claims (sub, email, name, iss, aud, exp). It is meant for the client to verify the user's identity. After OAuth2 authentication, the client receives both: the ID token is used to establish a session, and the access token is used to call APIs. The server validates access tokens for API requests; the client validates ID tokens for login.

---

### Q3. How do you implement OAuth2 Authorization Code flow in Rust?

**Interview Answer**

Using the oauth2 crate, create a client with the auth URL, token URL, client ID, and redirect URL. Generate a PKCE code verifier and challenge. Redirect the user to the authorization endpoint with the code challenge. On callback, exchange the authorization code for tokens using the code verifier. Store the tokens securely (httpOnly cookie or server-side session). In Axum, implement this as a multi-step handler: GET /auth/login redirects to the provider, GET /auth/callback exchanges code for tokens and creates a session. The oauth2 crate handles URL construction, token exchange, and token refresh automatically.

---

### Q4. What is the OAuth2 PKCE extension and why is it required?

**Interview Answer**

PKCE (Proof Key for Code Exchange) prevents authorization code interception attacks. The client generates a random code_verifier, hashes it to create a code_challenge, and sends the challenge in the authorization request. When exchanging the code for tokens, the client sends the original code_verifier. The server verifies the hash matches. An attacker who intercepts the authorization code cannot exchange it without the code_verifier. PKCE is required for public clients (SPAs, mobile apps) that cannot store client secrets securely. In Rust, use the oauth2 crate's PKCE features to generate and validate challenges.

---

### Q5. What is OpenID Connect and how does it extend OAuth2?

**Interview Answer**

OpenID Connect (OIDC) adds authentication to OAuth2 by defining a standardized ID token format (JWT) with specific claims (sub, iss, aud, exp, iat). It also provides a UserInfo endpoint for retrieving additional profile information, a discovery endpoint (/.well-known/openid-configuration) for automatic provider configuration, and a session management protocol for logout. While OAuth2 only handles authorization (granting access), OIDC handles authentication (verifying identity). For a Rust backend acting as an OIDC provider, you issue ID tokens signed with RS256 and publish a discovery document.

---

### Q6. How do you handle token refresh in a Rust backend?

**Interview Answer**

When an access token expires, use the refresh token to obtain a new access token pair. Implement a refresh endpoint that validates the refresh token, checks it is not revoked (database lookup), issues new tokens, and optionally rotates the refresh token (issues a new refresh token and invalidates the old one). In Rust, use the oauth2 crate's TokenResponse to access the refresh_token and call the token endpoint. Store refresh tokens in the database with user association and expiry. Implement a background job to clean up expired refresh tokens periodically.

---

### Q7. What is token revocation and why is it important?

**Interview Answer**

Token revocation allows invalidating tokens before they expire — necessary for user logout, password changes, and compromised token detection. OAuth2 Token Revocation (RFC 7009) defines a standard endpoint for revoking tokens. In a Rust backend, implement revocation by deleting the token from the database and checking for token existence on every validation request (or using a short-lived blocklist in Redis for stateless tokens). For immediate effect, maintain a blocklist of revoked token IDs with their expiry times. The trade-off is between stateless validation (fast but cannot revoke) and stateful validation (allows revocation but requires database lookups).

---

### Q8. How do you implement OAuth2 as an identity provider in Rust?

**Interview Answer**

As an identity provider, your Rust backend issues OAuth2 tokens to third-party applications. Implement the authorization endpoint (validates user login, shows consent screen), token endpoint (exchanges authorization code for tokens), and JWKS endpoint (publishes public keys for token verification). Use the oxide-auth crate or implement manually with axum. The authorization endpoint generates codes with short expiry, the token endpoint validates codes and issues signed JWTs, and the JWKS endpoint publishes your public RSA keys. Store third-party application registrations (client ID, secret, redirect URIs) in PostgreSQL.

---

### Q9. What security considerations apply to OAuth2 implementations?

**Interview Answer**

Key security considerations: always use PKCE for authorization code flow, validate redirect URIs to prevent open redirect attacks, use state parameter to prevent CSRF, store tokens in httpOnly secure cookies (not localStorage), validate token signatures and expiry on every request, implement token rotation for refresh tokens, use HTTPS for all token endpoints, and validate the aud claim to prevent token misuse. In Rust, validate all claims using the jsonwebtoken crate, implement constant-time comparison for token validation, and log all token issuance and revocation events.

---

### Q10. How do you integrate OAuth2 with Axum middleware?

**Interview Answer**

Create an Axum middleware that extracts the Bearer token from the Authorization header, validates it using the OAuth2 provider's public keys (JWKS), and inserts the decoded claims into request.extensions(). The middleware runs before handlers, ensuring all protected routes have authenticated user context. Use tower::Layer for composability. For routes that require specific OAuth2 scopes, create a RequireScope extractor that checks the token's scope claim. The JWKS keys should be cached and refreshed periodically (every 24 hours) to avoid hitting the provider on every request.
