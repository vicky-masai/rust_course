# Secrets Management

## Interview Question

How do you manage secrets (API keys, database credentials, encryption keys) in a production Rust backend?

## Interview Answer

Secrets management involves secure storage, access control, rotation, and audit of sensitive credentials. In production Rust backends, I use a layered approach: environment variables for simple deployments, a secrets manager (HashiCorp Vault, AWS Secrets Manager, or GCP Secret Manager) for production, and never commit secrets to version control. Secrets are injected at runtime — either through environment variables populated by the deployment platform, or fetched from the secrets manager on application startup. Database credentials are rotated regularly using short-lived tokens. All secrets access is logged for audit. The key principle is: secrets should be as dynamic as possible, with short lifetimes and automatic rotation.

---

## Follow-up Questions & Answers

### Q1. Why should you never store secrets in source code or environment variables in production?

**Interview Answer**

Secrets in source code are exposed in Git history, even if later removed — anyone with repository access can find them. Environment variables in Docker Compose or docker-compose.yml files are visible in the repository and to anyone with access to the running container. Production secrets should be injected by the deployment platform (Kubernetes Secrets, ECS task definitions, or a secrets manager) at runtime. In Rust, use std::env::var() to read secrets, but never set them in .env files committed to Git. Use .gitignore for .env files and provide .env.example with placeholder values.

---

### Q2. How does HashiCorp Vault work for secrets management?

**Interview Answer**

Vault stores secrets in an encrypted backend (Consul, PostgreSQL, or cloud storage) and provides a HTTP API for reading and writing secrets. Authentication can be via tokens, AppRole, Kubernetes service accounts, or cloud IAM. Secrets are organized in secret engines (KV, database, transit), with policies controlling who can access what. The database engine can generate dynamic, short-lived database credentials. In Rust, use the vaultrs crate to interact with Vault: read secrets on startup, cache them, and refresh periodically. Vault also provides encryption-as-a-service through the transit engine.

---

### Q3. What is the difference between static and dynamic secrets?

**Interview Answer**

Static secrets are fixed values that persist until manually changed (API keys, SSL certificates). Dynamic secrets are generated on-demand with short lifetimes (database credentials, cloud IAM tokens). Vault's database engine generates unique PostgreSQL credentials for each application instance, expiring after a configurable TTL. Dynamic secrets are more secure because: they are unique per consumer, they expire automatically, they can be revoked instantly, and compromise of one credential does not affect others. In Rust, fetch dynamic secrets from Vault at startup and refresh them before expiry.

---

### Q4. How do you handle secret rotation in a Rust backend?

**Interview Answer**

Implement secret rotation by: using dynamic secrets with automatic expiry (Vault database engine), watching for secret changes (Vault's lease renewal), and gracefully reloading credentials without restarting. In Rust, store secrets in an Arc<RwLock<Secrets>> that can be updated atomically. A background task periodically fetches new secrets from Vault and updates the shared state. For database connections, deadpool-postgres or sqlx pools can be reconfigured when credentials change. The key is zero-downtime rotation — the application continues serving requests while credentials are refreshed.

---

### Q5. What are the risks of logging secrets?

**Interview Answer**

Secrets in logs are exposed to anyone with log access — developers, operations staff, log aggregation systems, and potentially attackers who gain access to log storage. Common mistakes include logging database connection strings (which contain passwords), API request headers (which contain Authorization tokens), and error messages (which may include credential details). In Rust, use the tracing crate with sensitive field filtering: #[instrument(skip(password))] to exclude sensitive fields. Configure log redaction at the logging middleware level. Never log at DEBUG level in production.

---

### Q6. How do you manage secrets in Docker and Kubernetes?

**Interview Answer**

In Docker, use Docker secrets (docker secret create) or mount secrets as read-only volumes. Never use environment variables for secrets in docker-compose.yml. In Kubernetes, use Secrets resources (base64-encoded, not encrypted by default) or External Secrets Operator to sync from Vault/AWS Secrets Manager. Mount secrets as volumes (preferred over environment variables for security). Use RBAC to restrict which pods can access which secrets. Enable encryption at rest for Kubernetes secrets (EncryptionConfiguration). In Rust, read secrets from mounted files: std::fs::read_to_string("/run/secrets/db_password").

---

### Q7. What is the principle of least privilege for secrets access?

**Interview Answer**

Each service should only have access to the secrets it needs — a payment service needs payment API keys but not database admin credentials. Implement this through: Vault policies that restrict which paths each service can read, Kubernetes RBAC that limits secret access per namespace, and database roles with minimal required permissions. In Rust, the application authenticates to Vault with its own identity (Kubernetes service account) and receives only the secrets it is authorized to access. Regular audits verify that no service has excessive secret access.

---

### Q8. How do you handle secrets in CI/CD pipelines?

**Interview Answer**

CI/CD secrets are injected through the platform's secret mechanism (GitHub Actions secrets, GitLab CI variables, CircleCI environment). Never hardcode secrets in pipeline files. Use short-lived tokens for CI/CD (OIDC tokens for cloud providers instead of long-lived access keys). Limit secret access to specific pipeline steps using the secrets context. In Rust, cargo build does not need secrets — they are only needed at runtime. For testing, use test-specific credentials that have limited permissions. Audit all secret access in pipelines.

---

### Q9. What is the transit secrets engine in Vault and when is it used?

**Interview Answer**

Vault's transit engine provides encryption-as-a-service — applications send plaintext to Vault and receive ciphertext without ever handling encryption keys. The keys are managed entirely by Vault, with support for key rotation, versioning, and different algorithms (AES-256-GCM, RSA). This is useful when you need to encrypt data but cannot manage encryption keys in the application. In Rust, use the vaultrs crate to call transit/encrypt and transit/decrypt endpoints. The application sends user data, Vault encrypts it, and the ciphertext is stored in the database. Only Vault can decrypt it.

---

### Q10. How do you audit secrets access in production?

**Interview Answer**

Vault provides detailed audit logs of every secret access — who requested what, when, from where, and whether it was allowed or denied. Export these logs to a centralized system (ELK, Splunk, CloudWatch). In Rust, log all secrets access (reads and rotations) using structured logging with the tracing crate. Monitor for anomalies: unusual access patterns, access from unexpected IPs, or access to secrets a service does not normally use. Set up alerts for failed secret access attempts, which may indicate compromised service credentials. Regular access reviews verify that secret permissions are still appropriate.
