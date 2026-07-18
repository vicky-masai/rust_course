# Infrastructure as Code

## Interview Question

Explain Infrastructure as Code (IaC) and compare Terraform and Pulumi for managing cloud infrastructure in CI/CD pipelines.

## Interview Answer

Infrastructure as Code (IaC) defines and manages infrastructure (servers, networks, databases, Kubernetes clusters) using machine-readable configuration files rather than manual processes or interactive tools. IaC enables version control for infrastructure, reproducible environments, automated provisioning in CI/CD, and drift detection when actual infrastructure deviates from declared state. Terraform uses a declarative HCL (HashiCorp Configuration Language) to define resources, with a state file tracking what's been created — it's cloud-agnostic and has the largest ecosystem of providers. Pulumi uses general-purpose programming languages (TypeScript, Python, Go, Rust) to define infrastructure, leveraging familiar language features like loops, conditionals, and type safety. For Rust backend developers, Pulumi with Rust is appealing because you define infrastructure in the same language as your application, sharing types and logic. Terraform is more mature with broader provider support, while Pulumi offers better abstractions and testing capabilities. Both integrate with CI/CD pipelines to automatically provision and update infrastructure on every merge.

---

## Follow-up Questions & Answers

### Q1. What is Terraform and how does it work?

**Interview Answer**

Terraform is an IaC tool by HashiCorp that uses HCL to declare desired infrastructure state. You write `.tf` files defining resources (AWS EC2 instances, Kubernetes Deployments, DNS records), and Terraform creates a plan showing what will change before applying it. Terraform maintains a state file (`.tfstate`) that maps your configuration to real infrastructure — this enables it to detect drift and plan incremental changes. The workflow is: `terraform init` (download providers), `terraform plan` (preview changes), `terraform apply` (make changes). For Rust backend projects, Terraform provisions the infrastructure your services run on: VPCs, databases (RDS), Kubernetes clusters (EKS), and monitoring (CloudWatch). Store state in remote backends (S3 with DynamoDB locking) for team collaboration. Terraform's `for_each` and `modules` enable reusable infrastructure patterns across environments.

---

### Q2. What is Pulumi and how does it compare to Terraform?

**Interview Answer**

Pulumi is an IaC tool that uses general-purpose programming languages (TypeScript, Python, Go, and since recently, Rust) instead of HCL. You define infrastructure as code in a familiar language with real loops, conditionals, functions, and type checking — no new language to learn. Pulumi's state management is similar to Terraform (stored in Pulumi Cloud or self-hosted backends), and it supports the same cloud providers. The key difference is expressiveness: Pulumi allows you to use programming constructs to create abstractions, while Terraform requires workarounds for complex logic. For Rust developers, Pulumi-Rust lets you define infrastructure in Rust with type safety and shared libraries between application and infrastructure code. Terraform has broader community support and more mature provider ecosystem, while Pulumi offers better testing (you can unit-test infrastructure code) and more natural abstractions for complex deployments.

---

### Q3. How do you integrate IaC into CI/CD pipelines?

**Interview Answer**

IaC integrates into CI/CD as a deployment stage that runs `terraform plan` on PRs (showing infrastructure changes for review) and `terraform apply` on merge to main (making changes). In GitHub Actions, create a workflow triggered on changes to `terraform/` directory, run `terraform plan` and post the plan as a PR comment for review, then apply on merge. For Pulumi, use `pulumi preview` (plan) and `pulumi up` (apply). Store Terraform state in remote backends (S3, Terraform Cloud) with state locking to prevent concurrent applies. For Rust projects, IaC provisions the infrastructure your services deploy to — the CI/CD pipeline first applies Terraform changes (create/update database, Kubernetes namespace, DNS records), then deploys the application (Docker image, Helm chart). Use separate workspaces or state files for dev/staging/prod to prevent cross-environment interference.

---

### Q4. What is Terraform state and why is it important?

**Interview Answer**

Terraform state is a JSON file that maps your configuration to real infrastructure resources, tracking IDs, attributes, and dependencies. State is critical because Terraform uses it to determine what needs to be created, updated, or destroyed — without state, Terraform would recreate everything on every apply. Store state in remote backends (S3 with DynamoDB locking, Terraform Cloud) for team collaboration — local state files cause conflicts and are lost if your machine fails. State contains sensitive data (database passwords, API keys) so encrypt it at rest and restrict access. For production Rust infrastructure, use workspaces to separate state per environment, enable state versioning for rollback, and never commit state files to Git. Drift detection (comparing state to actual infrastructure) should run regularly in CI to catch manual changes that deviate from declared state.

---

### Q5. How do you manage secrets in IaC?

**Interview Answer**

Never hardcode secrets in Terraform or Pulumi files — use your cloud provider's secret management (AWS Secrets Manager, GCP Secret Manager, Azure Key Vault) or a dedicated tool (HashiCorp Vault). In Terraform, use `aws_secretsmanager_secret` to create secrets and reference them with `data "aws_secretsmanager_secret_version"`. In Pulumi, use `pulumi.Secret` to mark values as sensitive (encrypted in state). For CI/CD pipelines, use OIDC tokens for cloud provider authentication instead of static credentials, and inject secrets as environment variables during the apply step. For Rust services, Terraform might provision a database and store the password in Secrets Manager, then your Helm chart references the secret. Audit IaC secret access and rotate regularly — IaC state files may contain sensitive data, so protect them with encryption and access controls.

---

### Q6. What are Terraform modules and when should you use them?

**Interview Answer**

Terraform modules are reusable, composable packages of Terraform configuration that encapsulate infrastructure patterns. Create a module for common patterns: `modules/rust-service` that provisions an ECS service, load balancer, database, and DNS records. Modules accept input variables and expose output values, enabling consistent infrastructure across services. Use modules when: you repeat infrastructure patterns across multiple services, you want to enforce organizational standards (security groups, tagging), or you need to share infrastructure code across teams. For Rust backend teams, create modules for: Kubernetes namespace setup, database provisioning, monitoring/alerting, and CI/CD infrastructure. Publish modules to a private registry (Terraform Cloud, S3-backed registry) for team consumption. Modules improve maintainability — a change to the module propagates to all services using it.

---

### Q7. How do you test Infrastructure as Code?

**Interview Answer**

Test IaC at multiple levels: static analysis (`terraform validate`, `tflint` for HCL linting), plan verification (parse `terraform plan` output to check for unexpected changes), integration tests (deploy to a test environment and verify resources exist), and compliance tests (check that resources meet security standards). For Pulumi, use built-in testing: write unit tests that mock cloud providers and verify resource definitions. Tools like `terratest` (Go-based) deploy real infrastructure and assert on outputs. For Rust infrastructure, test that Terraform provisions a working Kubernetes cluster, that database connections work, and that monitoring alerts fire correctly. In CI, run `terraform plan` on PRs and validate the plan output against policies (using Sentinel or Open Policy Agent). The goal is to catch infrastructure issues before they reach production, similar to how tests catch application bugs.

---

### Q8. What is infrastructure drift and how do you handle it?

**Interview Answer**

Drift occurs when actual infrastructure deviates from declared Terraform state — someone manually changes a security group, adds a tag, or modifies a configuration outside of IaC. Drift makes future `terraform apply` unpredictable because Terraform tries to reconcile actual state with declared state. Handle drift by: running `terraform plan` regularly in CI to detect changes, enforcing that all changes go through IaC (disable manual console access in production), and using Terraform's `import` to bring manually-created resources under IaC management. For production Rust infrastructure, run drift detection daily and alert on unexpected changes. When drift is detected, either update the Terraform code to match the actual state (if the manual change was intentional) or revert the manual change (if it was unauthorized). Never ignore drift — it accumulates and eventually causes major issues during deployments.

---

### Q9. How do you manage multi-environment infrastructure with IaC?

**Interview Answer**

Use separate Terraform workspaces or state files per environment (dev, staging, prod) with shared modules and environment-specific variables. Create a directory structure: `environments/dev/main.tf`, `environments/staging/main.tf`, `environments/prod/main.tf`, each calling shared modules with different variable values. In CI/CD, run `terraform apply` for dev on every merge, staging on approval, and production on release. For Pulumi, use separate stacks per environment with configuration files. For Rust services, this means dev gets smaller instances and fewer replicas, staging mirrors production, and prod gets full resources. Use a promotion pipeline: deploy to dev first, run tests, promote to staging, validate, then promote to production. Ensure environments are isolated (separate VPCs, databases, credentials) to prevent a dev deployment from affecting production.

---

### Q10. What are the emerging trends in Infrastructure as Code?

**Interview Answer**

Emerging trends include: GitOps as the operational model (ArgoCD/Flux reconcile infrastructure from Git), ephemeral environments (spin up full environments per PR, destroy after merge), policy-as-code (OPA/Sentinel enforce security and compliance in IaC), and platform engineering (internal developer platforms built on IaC abstractions). For Rust backend teams, the most impactful trends are: ephemeral preview environments (each PR gets a temporary environment with database, cache, and service for testing), GitOps for Kubernetes (Flux watches Git and applies Helm chart changes automatically), and Pulumi with Rust (define both application and infrastructure in the same language). Also watch: OpenTofu (open-source Terraform fork), Crossplane (Kubernetes-native infrastructure management), and Backstage (developer portal for managing infrastructure). The direction is toward more abstraction, better testing, and tighter integration between application and infrastructure code.