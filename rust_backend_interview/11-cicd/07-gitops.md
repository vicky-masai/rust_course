# GitOps

## Interview Question

Explain GitOps principles, how ArgoCD and Flux work, and how to implement declarative deployment from Git for Kubernetes.

## Interview Answer

GitOps is an operational framework where Git repositories serve as the single source of truth for infrastructure and application deployment — developers update code and configuration in Git, and automated tools reconcile the cluster state to match Git. The core principles are: declarative configuration (Kubernetes manifests or Helm charts in Git), version control (every change is a Git commit), automated delivery (tools continuously sync Git state to the cluster), and reconciliation (tools detect and correct drift from Git state). ArgoCD is a Kubernetes-native GitOps tool that watches Git repositories and automatically syncs application manifests to the cluster, providing a web UI for visualizing deployment state. Flux (v2) is a set of controllers that do the same, with stronger focus on multi-tenancy and RBAC. For Rust backend services, GitOps means: your Helm chart lives in a Git repo, CI builds and tests the Rust binary, pushes the Docker image, then updates the chart's image tag in Git — Flux/ArgoCD detects the change and deploys it. This eliminates manual `kubectl apply` or `helm upgrade` commands and provides a complete audit trail of every deployment.

---

## Follow-up Questions & Answers

### Q1. What are the core principles of GitOps?

**Interview Answer**

GitOps has four core principles: (1) Declarative — system desired state is declared in configuration, not imperative scripts; (2) Versioned and immutable — all changes are Git commits with full history and audit trail; (3) Pulled automatically — agents in the cluster pull and apply changes from Git, rather than being pushed to by CI/CD; (4) Continuously reconciled — agents detect and correct drift between Git state and actual cluster state. This differs from traditional push-based CI/CD where pipelines push changes to the cluster. For Rust services, this means: declare your Deployment, Service, and ConfigMap in Helm charts committed to Git, and ArgoCD/Flux continuously ensure the cluster matches Git. If someone manually changes a resource in the cluster, the GitOps agent reverts it to match Git. This provides immutability, auditability, and self-healing for your infrastructure.

---

### Q2. How does ArgoCD work and what are its key features?

**Interview Answer**

ArgoCD is a Kubernetes controller that watches Git repositories for changes and automatically syncs the desired state (manifests, Helm charts, Kustomize overlays) to the cluster. Key features: automatic sync (detects Git changes and applies within minutes), manual sync (requires explicit trigger), sync waves (order dependencies between resources), health checks (verifies resources are healthy before proceeding), rollback (revert to previous Git commit), and web UI/CLI for visualization and management. ArgoCD stores application definitions as Application CRDs, specifying the Git repo, target revision, path, and destination cluster/namespace. For Rust services, create an ArgoCD Application pointing to your Helm chart repo — when CI updates the image tag in Git, ArgoCD detects the change and rolls out the new version. ArgoCD supports multi-cluster management, SSO integration, and RBAC for team access control.

---

### Q3. How does Flux work and how does it differ from ArgoCD?

**Interview Answer**

Flux (v2) is a set of GitOps controllers (source-controller, kustomize-controller, helm-controller, notification-controller) that continuously reconcile cluster state with Git. Flux is more "Kubernetes-native" than ArgoCD — it runs as a set of CRDs and controllers in the cluster, with no web UI (uses CLI and Kubernetes events). Key differences: Flux uses CRDs (GitRepository, Kustomization, HelmRelease) while ArgoCD uses Application objects; Flux has built-in image automation (automatically update image tags in Git when new versions are published); Flux supports multi-tenancy natively with namespace-scoped controllers. For Rust services, Flux's image automation is powerful — when CI pushes a new Docker image tag, Flux automatically updates the Helm chart values in Git and triggers a deployment. Choose Flux for Kubernetes-native workflows and automation, ArgoCD for web UI and multi-cluster visualization.

---

### Q4. How do you implement GitOps for Helm-based Rust services?

**Interview Answer**

Structure your repository with the Helm chart in a `chart/` directory and an ArgoCD Application or Flux HelmRelease that references it. CI pipeline: build Rust binary → create Docker image → push to registry → update image tag in `values.yaml` (or a separate values file) in Git. GitOps tool (ArgoCD/Flux) detects the Git change → renders Helm template → applies to cluster. For Rust services, the `values.yaml` contains configuration (replicas, resources, feature flags), and CI only updates the `image.tag` field on each release. Store the chart in the same repo as the application code for single-repo simplicity, or in a separate repo for separation of concerns. Use branch protection and PR reviews for production changes, and automatic sync for staging. This ensures every deployment is reviewed, auditable, and reversible.

---

### Q5. How do you handle Secrets in GitOps?

**Interview Answer**

GitOps and Secrets are challenging because Secrets are sensitive but Git is plaintext. Solutions: Sealed Secrets (encrypt Secrets so they're safe to commit to Git — the controller decrypts them in-cluster), External Secrets Operator (sync Secrets from external providers like AWS Secrets Manager or Vault into Kubernetes), SOPS (encrypt YAML files with KMS, committed encrypted to Git), and Bitnami Reloader (restart Pods when Secrets change). For Rust services, the recommended approach is External Secrets Operator — define an ExternalSecret CRD that references a Secret in AWS Secrets Manager, and the controller syncs it to a Kubernetes Secret. The secret value never touches Git. ArgoCD and Flux both support Sealed Secrets and External Secrets Operator. Never commit plaintext Secrets to Git, even in private repositories — the Git history persists even if you delete the file.

---

### Q6. What is drift detection and how do GitOps tools handle it?

**Interview Answer**

Drift detection is the process of identifying when actual cluster state deviates from the desired state declared in Git. GitOps tools continuously compare cluster state with Git state — Flux runs every 5 minutes by default, ArgoCD syncs every 3 minutes. When drift is detected, the tool either automatically corrects it (self-healing) or alerts operators, depending on configuration. For Rust services, this means if someone manually scales your Deployment to 20 replicas, GitOps reverts it to the value in Git (typically 3). If a Pod crashes and is restarted with different environment variables, GitOps restores the correct configuration. Drift detection provides immutability guarantees — the cluster state is always what Git says it should be. For critical resources, use `serverSideApply` in ArgoCD to avoid conflicts with other controllers.

---

### Q7. How do you implement multi-environment GitOps?

**Interview Answer**

Use separate directories or branches for each environment: `environments/dev/`, `environments/staging/`, `environments/prod/`, each containing environment-specific values files and ArgoCD Applications or Flux HelmReleases. Promotion moves changes from dev → staging → prod by updating Git: merge to dev branch auto-deploys to dev, merge to main auto-deploys to staging, tag creation or PR to prod branch deploys to production. For Rust services, environment differences are in `values-dev.yaml`, `values-staging.yaml`, `values-prod.yaml` (replica counts, resource limits, database URLs). Use ArgoCD Projects or Flux Kustomizations with RBAC to restrict which teams can modify which environments. For production, require PR reviews and approvals before merging. This ensures all deployments are audited, reviewed, and traceable to a Git commit.

---

### Q8. What is application rollback in GitOps and how does it work?

**Interview Answer**

GitOps rollback is simply reverting a Git commit — `git revert <commit>` creates a new commit that undoes the previous change, and the GitOps tool automatically syncs the reverted state to the cluster. For ArgoCD, you can also use the UI to roll back to a previous sync revision. For Helm-based services, GitOps rollback is equivalent to `helm rollback` but performed through Git rather than direct cluster commands. For Rust services, if a deployment causes errors, revert the Git commit that changed the image tag, and ArgoCD/Flux automatically rolls back to the previous version. This provides an auditable, reversible deployment history — every deployment and rollback is a Git commit with author, timestamp, and description. The simplicity of Git revert as rollback mechanism is one of GitOps' strongest advantages over traditional CI/CD rollback procedures.

---

### Q9. How do you handle pre-deployment tasks (migrations) in GitOps?

**Interview Answer**

Pre-deployment tasks like database migrations are challenging in GitOps because the GitOps tool applies all resources atomically. Solutions: Kubernetes Jobs with `helm.sh/hook: pre-upgrade` annotations (Helm hooks run before the upgrade), ArgoCD Sync Waves (order resource synchronization), or Init Containers that run migrations before the main container starts. For Rust services using Diesel or SQLx, create a migration Job that runs before the Deployment update — ArgoCD applies the Job first (lower sync wave), waits for completion, then applies the Deployment. Alternatively, run migrations as an Init Container in the same Pod as your Rust service. Ensure migrations are backward-compatible so the previous application version can still function during the rolling update window. Test migration jobs in CI against a test database to catch failures before production.

---

### Q10. What are the challenges of adopting GitOps and how do you address them?

**Interview Answer**

Common challenges: learning curve (team must understand GitOps tools and workflows), repository structure (deciding how to organize charts, values, and applications), handling Secrets (requires additional tooling like Sealed Secrets), managing CRDs and operators (GitOps tools themselves need to be deployed and maintained), and debugging sync failures (ArgoCD/Flux logs can be complex). Address these by: starting with a single non-critical service, documenting repository structure and promotion workflows, investing in Secret management early, using GitOps tool installer scripts (ArgoCD Helm chart, Flux bootstrap), and setting up notifications (Slack/Teams alerts on sync failures). For Rust services, start by putting your Helm chart in Git and setting up ArgoCD with auto-sync for staging — once comfortable, extend to production with manual sync approval. GitOps adoption is incremental — don't try to migrate everything at once.