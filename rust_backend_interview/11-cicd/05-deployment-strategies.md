# Deployment Strategies

## Interview Question

Explain the different deployment strategies — blue-green, canary, rolling, and feature flags — including their trade-offs and when to use each.

## Interview Answer

Blue-green deployment maintains two identical environments (blue=current, green=new) and switches traffic atomically after validating the green environment — zero downtime, instant rollback by switching back. Canary deployment gradually routes a small percentage of traffic (1-10%) to the new version, monitors metrics, and promotes or rolls back based on results — minimal risk but complex traffic management. Rolling deployment incrementally replaces old Pods with new ones (Kubernetes default) — simple but no instant rollback and version skew during rollout. Feature flags decouple deployment from release by wrapping new code in conditional checks, allowing deployment to production without exposing features to users — enables trunk-based development but adds code complexity. For Rust backend services, blue-green is ideal for database schema changes where you need both versions running simultaneously, canary is best for performance-sensitive APIs where you need to validate under real traffic, rolling is the default for standard deployments, and feature flags are valuable for gradual rollouts and A/B testing. Most production systems combine strategies — rolling deployments with feature flags for code, canary for infrastructure changes.

---

## Follow-up Questions & Answers

### Q1. How does blue-green deployment work in Kubernetes?

**Interview Answer**

In Kubernetes, blue-green deployment creates two Deployments (blue and green) with different Pod labels but the same Service selector. To deploy, update the green Deployment with the new image, wait for all Pods to be ready, then switch the Service selector from blue to green labels. This provides atomic traffic switching — all traffic moves to the new version instantly. For Rust services, this is valuable when database migrations are not backward-compatible — run both versions temporarily, migrate the database, then switch traffic. Rollback is instant: switch the Service selector back to blue. The downside is resource cost (running double capacity during deployment) and the need for a mechanism to switch the Service selector (manual `kubectl patch`, a deployment script, or a tool like Argo Rollouts). Ensure both versions can coexist with the current database schema during the transition window.

---

### Q2. How does canary deployment work and how do you implement it?

**Interview Answer**

Canary deployment routes a small percentage of traffic to the new version while the majority stays on the stable version. In Kubernetes, implement with two Deployments (stable with 9 replicas, canary with 1) behind the same Service, giving ~10% traffic to canary. For more precise control, use a service mesh (Istio VirtualService with weight-based routing) or Ingress annotations (NGINX canary with `nginx.ingress.kubernetes.io/canary-weight: "10"`). Monitor canary metrics (error rate, latency, CPU) and gradually increase traffic if healthy, or rollback if issues appear. For Rust services, canary deployments are particularly valuable because Rust's performance characteristics make latency regressions easy to detect. Use Prometheus metrics from your Rust service to drive automated canary analysis — tools like Flagger or Argo Rollouts can automate the promote/rollback decision based on metrics thresholds.

---

### Q3. What are the advantages and disadvantages of rolling deployment?

**Interview Answer**

Rolling deployment (Kubernetes default with `RollingUpdate` strategy) incrementally replaces old Pods with new ones, maintaining availability throughout. Advantages: simple to configure, no extra infrastructure, maintains capacity during deployment. Disadvantages: no instant rollback (must roll forward with a new revision), version skew (old and new versions run simultaneously, requiring backward compatibility), and gradual rollout means partial user impact if the new version has issues. For Rust services, rolling deployment works well because Rust's fast startup time (typically under 100ms) means new Pods become ready quickly, minimizing the window where old and new versions coexist. Configure `maxUnavailable: 0` and `maxSurge: 1` for zero-downtime rolling updates. Ensure your Rust service is backward-compatible with the current database schema during the rolling window. Use PodDisruptionBudgets to guarantee minimum availability during rollouts.

---

### Q4. What are feature flags and how do they work?

**Interview Answer**

Feature flags (also called feature toggles) are conditional checks in code that enable or disable features without deploying new code. Implement as a configuration value (from ConfigMap, environment variable, or feature flag service like LaunchDarkly) that your Rust code checks: `if feature_flag_enabled("new_checkout_flow") { new_checkout() } else { old_checkout() }`. Deploy code with the feature flag disabled, then enable it for specific users, percentages, or environments without redeployment. Feature flags enable trunk-based development (all developers work on main, features are hidden until ready), gradual rollouts (enable for 5% of users, monitor, then 100%), and A/B testing. For Rust services, use a `once_cell::Lazy` for the flag value to avoid repeated config lookups, and implement flag evaluation with minimal overhead. Clean up old flags regularly to avoid code complexity accumulation.

---

### Q5. How do you implement canary analysis automatically?

**Interview Answer**

Automated canary analysis compares metrics between canary and stable versions to make promote/rollback decisions. Tools like Flagger (works with Istio, Linkerd, or NGINX) or Argo Rollouts define success criteria: error rate below 1%, p99 latency below 200ms, and no increase in 5xx responses. Flagger gradually increases canary traffic while querying Prometheus metrics, comparing canary against baseline. If all metrics pass the analysis, the canary is promoted to 100% and the old version is scaled down. If any metric fails, automatic rollback occurs. For Rust services, instrument your application with Prometheus metrics (request count, latency histogram, error count) and define SLOs that the canary analysis enforces. This eliminates manual monitoring during deployments and provides consistent, data-driven release decisions.

---

### Q6. What is the difference between deployment and release?

**Interview Answer**

Deployment is the act of putting new code into an environment (building a Docker image, updating a Kubernetes Deployment), while release is making that code available to users (enabling the feature, routing traffic to it). With feature flags, deployment and release are decoupled — code is deployed to production but not released (feature flag is off). With canary deployment, code is deployed to a small percentage of users (partial release) before full release. Understanding this distinction enables practices like trunk-based development (deploy frequently, release selectively) and progressive delivery (gradually expose features to users). For Rust services, this means your CI/CD pipeline handles deployment (build, test, deploy to production with flags off), while feature flag services or traffic routing handles release (enable flags, increase canary percentage).

---

### Q7. How do you handle database migrations with deployment strategies?

**Interview Answer**

Database migrations must be compatible with both old and new application versions during rolling or canary deployments. Use expand-and-contract patterns: first deploy code that works with the new schema (add columns, don't remove), migrate the database, then deploy code that uses only the new schema (remove old columns). For blue-green deployments, migrate the database while both versions are running — ensure both versions can read the new schema. For feature flags, wrap migration-dependent code in flags so you can deploy migration code without using it until the migration runs. For Rust services using Diesel or SQLx, always test migrations against a production-like database in CI, and ensure migrations are idempotent. Never deploy a migration and application change simultaneously — deploy the migration first, verify it succeeds, then deploy the application change.

---

### Q8. What is progressive delivery and how does it relate to deployment strategies?

**Interview Answer**

Progressive delivery is an evolution of continuous delivery that combines deployment strategies with automated verification — deploying changes to a small subset of users, automatically analyzing the impact, and gradually expanding if healthy. It combines canary deployments (small traffic percentage), feature flags (selective feature exposure), and automated analysis (metrics-based promote/rollback decisions). Tools like Flagger, Argo Rollouts, and LaunchDarkly implement progressive delivery workflows. For Rust services, progressive delivery means: deploy to 1% of traffic, verify latency and error metrics against SLOs, increase to 10%, verify again, increase to 50%, then full rollout. If any stage fails, automatic rollback occurs. This approach reduces deployment risk to near-zero because changes are validated under real production traffic before full exposure.

---

### Q9. How do you implement rollback with each deployment strategy?

**Interview Answer**

Rollback varies by strategy: blue-green rollback is instant (switch Service selector back to blue), canary rollback is fast (scale down canary, route all traffic to stable), rolling rollback requires `kubectl rollout undo` (creates new ReplicaSet from previous revision, takes minutes), and feature flag rollback is instant (disable the flag in configuration). For rolling deployments, Helm provides `helm rollback` to revert to previous release state. For canary deployments with Argo Rollouts, automatic rollback occurs if metrics fail analysis. For Rust services, ensure your application handles both old and new database schemas during rollback windows, and test rollback procedures regularly. The key principle is that rollback should be faster and safer than forward-fixing — design your deployment strategy with this in mind.

---

### Q10. When should you choose one deployment strategy over another?

**Interview Answer**

Choose based on risk tolerance, infrastructure complexity, and release requirements: blue-green for critical services needing instant rollback and zero-downtime (payment processing), canary for performance-sensitive services where you need real-traffic validation (user-facing APIs), rolling for standard services with backward-compatible changes (internal microservices), and feature flags for gradual rollouts and A/B testing (new features). For Rust services, start with rolling deployments (simplest), add feature flags for new features, and adopt canary for high-traffic services where performance regression detection is critical. Blue-green is rarely needed for Rust services because Rust's fast startup and backward-compatible deployments make rolling updates safe. The best choice often combines strategies: rolling deployment with feature flags is the most common production pattern.