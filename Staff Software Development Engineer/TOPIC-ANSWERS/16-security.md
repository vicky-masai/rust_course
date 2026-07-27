# LEVEL 16 — Security

### 0320. JWT

JSON Web Token: signed (sometimes encrypted) claims blob carried by clients. Stateless auth convenience; hard to revoke; size bloat; secret/alg mismanagement is common vuln (`none` alg, weak secrets).

Prefer short TTL + refresh rotation; validate issuer/audience/alg explicitly.

**Talk track:** *"JWTs are portable signed claims — great for stateless APIs, weak at instant revocation."*

---

### 0321. OAuth2

Delegation framework: apps get access tokens to act for users without passwords. Roles: resource owner, client, auth server, resource server. Flows: auth code (+PKCE), client credentials, etc.

**Talk track:** *"OAuth2 delegates access with tokens — use auth code+PKCE for user apps, client credentials for machine clients."*

---

### 0322. OpenID Connect

Identity layer on OAuth2 — `id_token` authenticates the user. Standard for "Log in with X."

**Talk track:** *"OIDC is login identity on top of OAuth2's access delegation."*

---

### 0323. TLS

(See 0045.) Encrypt and authenticate network connections. Always in transit for credentials and personal data.

**Talk track:** *"TLS is the baseline for data in transit — verify certificates, disable ancient versions."*

---

### 0324. mTLS

Both client and server present certificates. Strong service-to-service identity in zero-trust meshes. Cert rotation and PKI ops are the hard part.

**Talk track:** *"mTLS proves both sides' identity with certs — strong, ops-heavy."*

---

### 0325. Encryption

Confidentiality via crypto. At rest (disk/KMS), in transit (TLS), application-level field encryption. Key management is the real system.

Never invent crypto — use vetted libraries.

**Talk track:** *"Encryption is easy; key management and threat modeling are the work."*

---

### 0326. Hashing

One-way digests. Passwords need slow salted hashes (Argon2, bcrypt) — not SHA-256 alone. Integrity hashes differ from password hashes.

**Talk track:** *"Hash for integrity or passwords — password hashing must be slow and salted."*

---

### 0327. Secrets Management

Store API keys, DB passwords, private keys outside code — Vault, cloud secret managers, K8s secrets (with care). Rotate, audit access, least privilege.

**Talk track:** *"Secrets belong in a managed store with rotation — never in git."*

---

### 0328. OWASP Top 10

Living list of common web risks (injection, broken auth, SSRF, etc.). Use as a checklist for design reviews and testing — not a complete security program.

**Talk track:** *"OWASP Top 10 is the baseline vulnerability vocabulary for web apps."*

---

### 0329. SQL Injection

Untrusted input concatenated into SQL. Attackers change query structure. Fix: parameterized queries / bound arguments always.

**Talk track:** *"SQL injection is string-built queries — parameters eliminate it."*

---

### 0330. XSS

Cross-Site Scripting: inject JS into pages viewed by others. Steal sessions, deface, drive-by actions. Fix: context-aware output encoding, CSP, HttpOnly cookies.

**Talk track:** *"XSS runs attacker script as your user — encode output and tighten CSP."*

---

### 0331. CSRF

Cross-Site Request Forgery: trick a logged-in browser into hitting your site with cookies attached. Fix: anti-CSRF tokens, SameSite cookies, prefer Authorization headers over cookies for APIs.

**Talk track:** *"CSRF rides the browser's automatic cookie auth — tokens and SameSite blunt it."*

---

### 0332. SSRF

Server-Side Request Forgery: trick your server into requesting internal URLs (cloud metadata, admin panels). Validate/allowlist URLs; block link-local and metadata IPs.

**Talk track:** *"SSRF turns your server into an internal proxy for attackers — allowlist egress."*

---

### 0333. API Security

Authn/z, rate limits, input validation, least privilege tokens, audit logs, schema validation, no excessive data exposure. Treat every external input as hostile.

**Talk track:** *"API security is layered: identity, authorization, validation, rate limits, and least data returned."*
