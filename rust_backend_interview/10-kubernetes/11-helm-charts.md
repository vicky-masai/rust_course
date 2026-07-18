# Helm Charts

## Interview Question

What are Helm charts, how do they work for Kubernetes package management, and what are the best practices for structuring them?

## Interview Answer

Helm is the package manager for Kubernetes, similar to apt or npm but for cluster resources. A Helm chart is a collection of YAML templates and default values that describe a set of Kubernetes resources — Deployments, Services, ConfigMaps, and more — as a single, versioned package. Helm renders templates by combining the chart's `values.yaml` defaults with user overrides, producing complete manifests that `kubectl apply` would apply. Charts are stored in repositories (like Artifact Hub) or as OCI artifacts in container registries, and Helm manages releases (installed instances of a chart) with built-in upgrade, rollback, and history tracking. For a Rust backend, you'd create a chart containing your Deployment, Service, Ingress, ConfigMap, and HPA, parameterizing values like image tag, replica count, and resource limits. This makes deploying across environments (dev, staging, prod) as simple as overriding values per environment. Helm's `helmfile` or `Flux` can then manage multiple releases declaratively for GitOps workflows.

---

## Follow-up Questions & Answers

### Q1. What is the structure of a Helm chart?

**Interview Answer**

A Helm chart directory contains: `Chart.yaml` (metadata: name, version, description), `values.yaml` (default configuration values), `templates/` (Kubernetes manifest templates using Go templating), `templates/_helpers.tpl` (reusable template functions), and `templates/NOTES.txt` (post-install usage instructions). Templates use `{{ .Values.key }}` syntax to inject values from `values.yaml` or user overrides. For a Rust service chart, `templates/` would contain `deployment.yaml`, `service.yaml`, `ingress.yaml`, `configmap.yaml`, and `hpa.yaml`. The `charts/` directory holds dependency charts. Structure your chart with clear separation: one chart per service, sub-charts for shared libraries, and values organized by environment (values-dev.yaml, values-prod.yaml). This structure makes charts maintainable and testable across environments.

---

### Q2. How do Helm values work and how do you override them?

**Interview Answer**

Helm values are key-value pairs defined in `values.yaml` that provide default configuration for your chart templates. Users override values during install/upgrade with `--set key=value` flags or by providing a custom values file with `-f custom-values.yaml`. For example, `helm install my-rust-api ./chart --set image.tag=v1.2.3 --set replicas=5` overrides the image tag and replica count. Values can be nested: `--set resources.limits.memory=512Mi`. For Rust services, common values include `image.repository`, `image.tag`, `replicaCount`, `service.type`, `ingress.host`, `resources.requests.cpu`, and `config.logLevel`. Use environment-specific values files (values-staging.yaml, values-prod.yaml) to maintain different configurations per environment. Helm's value precedence is: `--set` > custom values file > `values.yaml` defaults.

---

### Q3. What are Helm hooks and when would you use them?

**Interview Answer**

Helm hooks are special annotations on Kubernetes resources that execute at specific points in the release lifecycle: `pre-install`, `post-install`, `pre-upgrade`, `post-upgrade`, `pre-delete`, `post-delete`, and `test`. Common use cases include database migrations (run `pre-upgrade` hook as a Job before deploying a new version), certificate generation (`post-install` hook), and smoke tests (`test` hook). For Rust services, a migration hook would run `diesel migration run` or `sqlx migrate run` before deploying your Axum server, ensuring the database schema is ready. Hooks are implemented as Kubernetes Jobs or Pods with the annotation `helm.sh/hook: pre-upgrade`. Helm tracks hook execution and can wait for completion, but failed hooks block the release — use `helm.sh/hook-delete-policy` to control cleanup. Hooks add operational complexity, so use them judiciously and document their purpose clearly.

---

### Q4. What is Chartmuseum and how do you host Helm charts privately?

**Interview Answer**

ChartMuseum is an open-source Helm chart repository server that hosts charts locally or in cloud storage (S3, GCS, Azure Blob). It provides a REST API for uploading, downloading, and managing charts, and serves as a Helm-compatible repository that teams can add with `helm repo add`. For private charts, host ChartMuseum behind authentication (basic auth, OIDC) and store charts in encrypted cloud storage. Alternatives include using OCI registries (Docker Hub, ECR, GCR) to store charts as OCI artifacts — Helm 3 natively supports `helm push` to OCI registries. For Rust backend teams, hosting a private chart repository ensures that your service charts (containing deployment configurations and potentially sensitive defaults) are not publicly accessible while remaining easily deployable across environments.

---

### Q5. How do you test Helm charts?

**Interview Answer**

Test Helm charts using `helm lint chart/` to validate template syntax and value references, `helm template chart/` to render templates locally without deploying (verifying the generated YAML), and `helm test release-name` to run tests defined in `templates/tests/`. For comprehensive testing, use tools like `chart-testing` (ct) which lints and installs charts in a temporary kind cluster, or `conftest` which tests rendered YAML against Rego policies. Write unit tests for complex template logic using `helm-unittest`. For Rust services, test that your chart produces correct manifests for different value combinations: verify the Deployment references the correct image, the Service exposes the right port, and the HPA targets the correct metric. CI/CD pipelines should lint charts on every PR and run integration tests against a test cluster before merging.

---

### Q6. What is the difference between Helm 2 and Helm 3?

**Interview Answer**

Helm 2 used Tiller (a server-side component in the cluster) to manage releases, which required cluster-admin RBAC permissions and created security concerns. Helm 3 removed Tiller entirely, making Helm a client-only tool that stores release data in Kubernetes Secrets (or ConfigMaps). Helm 3 also added: OCI registry support for chart storage, built-in chart dependency management, improved rollback capabilities, and the `helm test` command as a first-class feature. For production Rust services, Helm 3 is the only version you should use — Helm 2 is end-of-life. The migration from Helm 2 to 3 is generally straightforward with the `helm-2to3` plugin, but test thoroughly in staging before migrating production releases.

---

### Q7. How do you manage multiple environments with Helm?

**Interview Answer**

Use separate values files for each environment: `values-dev.yaml`, `values-staging.yaml`, `values-prod.yaml`, each overriding environment-specific settings (replica counts, resource limits, ingress hosts, database URLs). Deploy with `helm upgrade --install my-service ./chart -f values-prod.yaml --namespace production`. For GitOps, store values files in Git and use tools like Flux or ArgoCD to automatically deploy when values change. Helmfile provides a declarative way to manage multiple releases across environments in a single `helmfile.yaml` file, specifying which values file to use for each environment. For Rust services, this means one chart maintained in a single repo with per-environment values — dev might use 2 replicas with 256Mi memory, while prod uses 10 replicas with 1Gi memory and HPA enabled.

---

### Q8. What are Helm sub-charts and dependencies?

**Interview Answer**

Sub-charts are dependencies that a parent chart includes from the `charts/` directory or by specifying them in `Chart.yaml` dependencies. For example, your Rust service chart might depend on a PostgreSQL sub-chart (for dev/staging only) or a Redis sub-chart for caching. Dependencies are specified with name, version, and repository, and `helm dependency update` downloads them to the `charts/` directory. Sub-charts inherit values from the parent chart's `values.yaml` under a namespaced key (e.g., `postgresql.auth.password`). For production Rust services, sub-charts are useful for shared infrastructure components, but avoid over-coupling — use independent charts for databases and microservices, only using sub-charts for tightly coupled dependencies. Document dependency versions clearly because upgrading a sub-chart can break your service.

---

### Q9. How do you handle Secrets in Helm charts?

**Interview Answer**

Helm charts should NOT store Secrets in `values.yaml` because they'd be committed to Git in plaintext. Instead, use `--set` flags to inject Secrets at deploy time, reference Kubernetes Secrets created outside Helm (e.g., `existingSecret: my-db-secret`), or use external secret operators (Sealed Secrets, External Secrets Operator). Helm's templating can reference existing Secrets: `secretKeyRef: { name: {{ .Values.existingSecret }}, key: password }`. For Rust services, the pattern is: create the Secret manually or via an operator, then reference it in your Helm values. Helm hooks can also create Secrets during install/upgrade, but this complicates management. The safest approach is Sealed Secrets — encrypt the Secret YAML, commit it to Git, and the controller decrypts it in the cluster.

---

### Q10. What is the difference between Helm charts and Kubernetes operators?

**Interview Answer**

Helm charts are templated Kubernetes manifests for deploying static resources — they define what to create but don't manage ongoing operational logic. Kubernetes operators (built with the Operator SDK or Kubebuilder) include custom controllers that implement domain-specific operational knowledge — they watch custom resources and take actions like automated backups, scaling, failover, or schema migrations. Use Helm charts for deploying your Rust web services (Deployments, Services, Ingress) where the operational logic is straightforward. Use operators for stateful, complex infrastructure like databases (Postgres Operator, MySQL Operator) where ongoing management (backups, failover, upgrades) requires application-specific intelligence. Some projects provide both: a Helm chart for initial deployment and an operator for lifecycle management. For most Rust backend services, Helm charts are sufficient — operators are for infrastructure components that need continuous reconciliation.