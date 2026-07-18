# ConfigMaps and Secrets

## Interview Question

How do you manage configuration and secrets in Kubernetes, and what are the security considerations for each?

## Interview Answer

ConfigMaps store non-sensitive configuration data (database hosts, feature flags, log levels) as key-value pairs, while Secrets store sensitive data (passwords, API keys, TLS certificates) encoded in base64 — though base64 is not encryption and provides no security by itself. Both can be mounted as volumes (files in the Pod) or injected as environment variables into containers. For a Rust backend, you'd mount a ConfigMap for `config.toml` settings and inject Secrets as environment variables for database credentials. Security considerations are critical: base64-encoded Secrets are visible in etcd (which must be encrypted at rest), in `kubectl get secrets` output, and in Pod descriptions — for true security, use external secret managers like AWS Secrets Manager with the Secrets Store CSI driver, enable etcd encryption at rest, and restrict Secret access via RBAC policies. For Rust services, the `secrets` crate can read environment variables injected by Kubernetes, keeping sensitive values out of source code and container images.

---

## Follow-up Questions & Answers

### Q1. What is a ConfigMap and how do you create one?

**Interview Answer**

A ConfigMap is a Kubernetes resource that stores non-sensitive configuration as key-value pairs or file content, decoupling configuration from container images. Create one with `kubectl create configmap my-config --from-literal=DATABASE_URL=postgres://... --from-file=config.yaml`, or define it declaratively in YAML and apply it with `kubectl apply`. ConfigMaps can be referenced in Pod specs as environment variables (`envFrom: [{configMapRef: {name: my-config}}]`) or as mounted volumes. For Rust services, you might store `log.level=info` and `server.port=8080` in a ConfigMap and mount it as a TOML file that your `config` crate reads at startup. ConfigMaps are namespace-scoped, so you need separate ConfigMaps for dev and prod environments.

---

### Q2. How do Secrets differ from ConfigMaps and what are their limitations?

**Interview Answer**

Secrets are similar to ConfigMaps but designed for sensitive data — they store values as base64-encoded strings and have additional access controls. The critical limitation is that base64 is NOT encryption; anyone who can read the Secret can decode it. Secrets are stored in etcd (which should be encrypted at rest), passed over the API server (which should use TLS), and visible in Pod specs if not carefully managed. For production Rust services, use the Secrets Store CSI driver to mount secrets from external providers (Vault, AWS Secrets Manager) instead of storing them in Kubernetes, enabling automatic rotation and audit trails. Never commit Secret YAML files to Git, and always use RBAC to restrict who can read Secrets in your cluster.

---

### Q3. What are the different ways to mount ConfigMaps and Secrets in a Pod?

**Interview Answer**

You can mount them as environment variables using `env` or `envFrom` in the container spec, which injects keys as environment variables accessible to your Rust process. Alternatively, mount them as volumes to create files at specific paths — this is better for configuration files (like `config.toml`) because Kubernetes automatically updates the files when the ConfigMap/Secret changes (with a brief delay). Volume mounts support subPath for mounting individual keys as files. For Rust services, environment variables are simpler for single values (database URLs, API keys), while volume mounts are better for structured config files. Note that environment variable updates require a Pod restart, while volume-mounted files update without restart (though your Rust code must re-read them).

---

### Q4. How do you rotate Secrets in Kubernetes?

**Interview Answer**

Secret rotation in Kubernetes requires coordination between updating the Secret resource and ensuring Pods pick up the new values. For volume-mounted Secrets, Kubernetes updates the files automatically (typically within 1-2 minutes), and your Rust code must re-read the configuration — some frameworks support hot-reloading. For environment variable-mounted Secrets, you must restart the Pod (e.g., by triggering a rolling update) for the new values to take effect. For production Rust services, use external secret operators (External Secrets Operator, Sealed Secrets) that sync from external stores, enabling automated rotation with your vault or cloud secrets manager. Always test rotation in staging first to ensure your Rust application handles credential changes gracefully without dropping connections.

---

### Q5. How do you encrypt Secrets at rest in etcd?

**Interview Answer**

Kubernetes supports encrypting Secrets in etcd using EncryptionConfiguration with providers like `aescbc` (AES-CBC), `aesgcm` (AES-GCM), or `kms` (AWS KMS, GCP KMS, Vault). To enable encryption, create an EncryptionConfiguration file specifying the encryption key and provider, then pass it to the API server with `--encryption-provider-config`. When you apply a Secret, Kubernetes encrypts it before storing in etcd and decrypts when reading. For managed Kubernetes (EKS, GKE), encryption at rest is often enabled by default or configurable through cloud provider settings. For Rust services, this means your database credentials and API keys stored as Secrets are encrypted even if someone gains access to etcd storage. Note that enabling encryption is a one-way operation — you need to re-encrypt all existing Secrets after enabling it.

---

### Q6. What are Sealed Secrets and how do they work?

**Interview Answer**

Sealed Secrets (from Bitnami) is a Kubernetes controller that allows you to safely store encrypted Secret YAML files in Git — the controller decrypts them using a key that only exists in the cluster. You encrypt secrets with `kubeseal` using the cluster's public key, producing a SealedSecret manifest that's safe to commit to Git. When applied, the controller decrypts it into a regular Secret that Pods can mount. This solves the GitOps problem of managing Secrets declaratively. For Rust services, you'd encrypt your database credentials with `kubeseal`, commit the SealedSecret to your repo, and the controller automatically creates the Secret when the manifest is applied. The private key is unique to each cluster, so a SealedSecret encrypted for production won't decrypt in staging.

---

### Q7. How do RBAC policies protect ConfigMaps and Secrets?

**Interview Answer**

RBAC (Role-Based Access Control) lets you define Roles and ClusterRoles with permissions on specific resources (get, list, watch, create, update, delete) and bind them to users, groups, or service accounts. For Secrets, you should create a Role that allows `get`, `list`, `watch` only on specific Secrets your Rust service needs, then bind it to the service account used by your Pod. Default service accounts often have too broad permissions — always create dedicated service accounts for each workload. For ConfigMaps, restrict access to only the ConfigMaps your service reads. Use `kubectl auth can-i get secrets --as=system:serviceaccount:production:rust-api-sa` to test permissions. In production, enforce least-privilege access to prevent compromised Pods from reading Secrets belonging to other services.

---

### Q8. What is the difference between env and envFrom when referencing ConfigMaps?

**Interview Answer**

`env` lets you map individual ConfigMap keys to specific environment variable names — for example, mapping ConfigMap key `database.host` to env var `DB_HOST`. `envFrom` injects ALL keys from a ConfigMap (or Secret) as environment variables, using the key names directly. `envFrom` is simpler when you have many configuration values, but `env` gives you control over naming and lets you cherry-pick specific keys. For Rust services, use `env` when you need custom environment variable names (e.g., mapping a ConfigMap key `url` to `DATABASE_URL` for the `dotenvy` crate), and `envFrom` when your ConfigMap keys already match your expected env var names. Be careful with `envFrom` — if a ConfigMap key conflicts with an existing env var, the `env` value takes precedence.

---

### Q9. How do you use ConfigMaps to manage Rust application configuration?

**Interview Answer**

A common pattern for Rust services is to create a ConfigMap containing a `config.toml` file, mount it as a volume at `/etc/app/config.toml`, and have your Rust code read it at startup using the `config` crate. This decouples configuration from the container image, allowing different settings per environment without rebuilding the image. For environment-specific overrides, combine ConfigMap-mounted files with environment variables — your Rust code can use `config::Config::builder().add_source(config::File::from(...)).add_source(config::Environment::new())` to layer sources. In Kubernetes manifests, use `configMapKeyRef` to reference specific keys or mount the entire ConfigMap as a directory. This approach follows twelve-factor app principles and makes your Rust services configurable across dev, staging, and prod environments.

---

### Q10. What are the security best practices for handling Secrets in Kubernetes?

**Interview Answer**

Best practices include: enable etcd encryption at rest, restrict Secret access via RBAC (dedicated service accounts per workload), never commit plaintext Secrets to Git (use Sealed Secrets or External Secrets Operator), use volume mounts instead of environment variables for Secrets (env vars can leak in logs and `/proc/<pid>/environ`), rotate Secrets regularly and verify your Rust application handles re-reads, use network policies to prevent Pods from accessing Secrets they don't need, and audit Secret access with Kubernetes audit logs. For production Rust services, consider using the Secrets Store CSI driver with AWS Secrets Manager or HashiCorp Vault to eliminate Kubernetes-managed Secrets entirely. Additionally, never log Secret values in your Rust application, and use the `secrets` crate's `zeroize` feature to clear sensitive data from memory when it's no longer needed.