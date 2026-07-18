# HPA and VPA

## Interview Question

Explain Horizontal Pod Autoscaler (HPA) and Vertical Pod Autoscaler (VPA), including how they work and when to use each.

## Interview Answer

Horizontal Pod Autoscaler (HPA) automatically scales the number of Pod replicas based on observed metrics like CPU utilization, memory usage, or custom metrics — for example, scaling your Rust API from 3 to 10 replicas when CPU exceeds 70%. VPA automatically adjusts Pod resource requests (CPU and memory) based on historical usage, right-sizing containers that are over- or under-provisioned. HPA is for handling variable load by adding more instances, while VPA is for optimizing resource allocation for existing instances. You cannot use both HPA and VPA on the same resource for the same metric simultaneously because they'd conflict — HPA scales replicas while VPA adjusts per-Pod resources. For Rust services, HPA is the primary tool for handling traffic spikes (deploy it alongside your Deployment), while VPA is useful during capacity planning to identify right-sized resource requests. Both require the Metrics Server installed in the cluster and properly configured resource requests on your Pods.

---

## Follow-up Questions & Answers

### Q1. How does HPA calculate the desired replica count?

**Interview Answer**

HPA uses a simple formula: `desiredReplicas = ceil(currentReplicas * (currentMetricValue / targetMetricValue))`. For example, if you have 3 replicas at 90% CPU with a 60% target, the calculation is `ceil(3 * (90 / 60)) = 5` replicas. HPA polls metrics every 15 seconds (configurable via `--horizontal-pod-autoscaler-sync-period`) and applies a stabilization window (default 5 minutes for scaling up, 15 minutes for scaling down) to prevent thrashing. The `behavior` field (stable/v2 API) lets you customize scaling speed and cooldown periods. For Rust services, set conservative scale-down windows (10-15 minutes) to avoid scaling down during brief traffic lulls, and aggressive scale-up windows to handle sudden traffic spikes quickly.

---

### Q2. What metrics can HPA use for scaling decisions?

**Interview Answer**

HPA can scale based on resource metrics (CPU utilization, memory utilization) from the Metrics Server, or custom metrics from adapters like Prometheus Adapter or KEDA. Resource metrics are built-in but limited — CPU is the most commonly used because it scales well and is predictive. Custom metrics enable scaling based on application-specific signals like request queue depth, response latency, or requests per second — for example, scaling your Rust service when average request latency exceeds 200ms. External metrics (from cloud providers like SQS queue depth or CloudWatch alarms) are also supported. For production Rust services, CPU-based scaling is a good starting point, but custom metrics provide more accurate scaling for latency-sensitive APIs where CPU utilization doesn't correlate well with user-facing performance.

---

### Q3. How do you configure HPA for a Rust backend service?

**Interview Answer**

First, ensure your Deployment has `resources.requests` defined (HPA cannot scale without them — it uses requests as the baseline for utilization percentage). Create an HPA manifest targeting your Deployment with `minReplicas`, `maxReplicas`, and a metric target (e.g., CPU at 70%). Apply it with `kubectl apply -f hpa.yaml`, and verify with `kubectl get hpa` — the `TARGETS` column shows current vs. desired metrics. For Rust services with bursty traffic, set a lower `minReplicas` for cost efficiency and a higher `maxReplicas` to handle spikes. Use the `behavior` field to customize scaling: set `scaleUp.stabilizationWindowSeconds: 30` for fast scale-up and `scaleDown.stabilizationWindowSeconds: 300` for slow scale-down. Monitor HPA behavior with `kubectl describe hpa` and adjust targets based on actual performance testing.

---

### Q4. What is VPA and when should you use it?

**Interview Answer**

Vertical Pod Autoscaler (VPA) automatically adjusts Pod resource requests based on actual usage history — it observes your Rust service's memory and CPU consumption over time and recommends or applies right-sized resource values. VPA has three modes: Off (only provides recommendations), Initial (applies recommendations only when Pods are created), and Auto (evicts and recreates Pods with new resource values, causing brief downtime). Use VPA during capacity planning to identify that your Rust service actually needs 512Mi memory instead of the 2Gi you requested, or to automatically right-size after code changes alter resource consumption. VPA cannot be used alongside HPA on the same metric, so typically run VPA in "Off" mode to get recommendations, then manually apply the right values and use HPA for scaling.

---

### Q5. What is KEDA and how does it extend HPA?

**Interview Answer**

KEDA (Kubernetes Event-Driven Autoscaler) is a CNCF project that extends HPA with event-driven scaling from 60+ scalers including Kafka lag, RabbitMQ queue depth, Prometheus queries, AWS SQS, and custom metrics. Unlike standard HPA which polls metrics, KEDA can scale to zero (when no events are pending) and scale from zero (when new events arrive), making it ideal for batch processing or async workloads. For Rust services, KEDA is powerful for worker services that process messages from Kafka or RabbitMQ — you define a ScaledObject that triggers scaling based on consumer lag, and KEDA creates an HPA with the appropriate custom metrics. It also provides `ScaledJobs` for batch workloads that need one-shot Pods per event rather than long-running replicas.

---

### Q6. What are the limitations of HPA?

**Interview Answer**

HPA has several limitations: it requires resource requests to be set on Pods (without them, utilization metrics are unavailable), it only supports a few built-in metrics (CPU, memory) without custom metrics adapters, and scaling is reactive (it responds to current metrics, not predicted future load). HPA cannot scale based on multiple metrics simultaneously in a weighted manner — it uses the metric that gives the highest replica count. It also cannot scale to zero (KEDA solves this) and may thrash if metrics are volatile (stabilization windows help but don't eliminate this). For Rust services, HPA works well for steady traffic patterns but may lag behind sudden traffic spikes because it polls every 15 seconds and applies stabilization windows. Combine HPA with PodDisruptionBudgets to ensure safe scaling operations.

---

### Q7. How do custom metrics work with HPA?

**Interview Answer**

Custom metrics allow HPA to scale based on application-specific signals like request rate, queue depth, or latency, not just CPU/memory. You need a metrics adapter (Prometheus Adapter, Datadog Agent, or cloud-specific adapters) that exposes custom metrics through the Kubernetes custom metrics API. In your Rust service, instrument code with Prometheus metrics (using the `prometheus` crate) and expose them on a `/metrics` endpoint. The Prometheus Adapter scrapes these metrics and makes them available to HPA via the Kubernetes API. For example, you could scale when `http_requests_per_second` exceeds 1000 per replica. Custom metrics provide more accurate scaling for Rust services because they reflect actual application load rather than resource utilization, which may not correlate with user-facing performance.

---

### Q8. What is the difference between HPA v1 and v2?

**Interview Answer**

HPA v1 (stable) supports only CPU utilization metrics with basic scaling behavior. HPA v2 (beta since K8s 1.23, stable in 1.26) adds support for memory metrics, custom metrics, multiple metrics with weights, and the `behavior` field for fine-grained control over scaling speed and stabilization. The `behavior` field lets you configure scale-up and scale-down separately: `scaleUp.stabilizationWindowSeconds` prevents rapid scaling up by looking at metrics over a window, `scalingPolicy` limits the rate of change (e.g., scale by at most 2 pods per minute), and `selectPolicy` chooses the most or least aggressive policy. For production Rust services, always use v2 because it provides the flexibility to scale aggressively during traffic spikes while scaling down conservatively to avoid disruption.

---

### Q9. How do you monitor HPA and VPA behavior in production?

**Interview Answer**

Monitor HPA with `kubectl get hpa -w` (watch mode) to see real-time scaling decisions, `kubectl describe hpa` for detailed events and metric values, and Prometheus/Grafana dashboards showing replica counts, metric values, and scaling events over time. For VPA, check recommendations with `kubectl describe vpa <name>` — look at the `Recommendation` section for target values and compare to your current resource requests. Set up alerts for HPA hitting maxReplicas (indicating insufficient capacity), frequent scaling events (indicating thrashing), and VPA recommendations significantly exceeding current requests (indicating under-provisioning). For Rust services, correlate HPA scaling with application metrics like p99 latency and error rates to verify that scaling decisions actually improve performance. Use tools like Kubecost to track resource costs associated with scaling behavior.

---

### Q10. Can you use HPA and VPA together for the same Deployment?

**Interview Answer**

You cannot use HPA and VPA for the same metric simultaneously — they'd conflict because HPA scales replicas while VPA adjusts per-Pod resources, and both would fight over the same utilization targets. The recommended approach is to run VPA in "Off" mode (recommendations only, no automatic action) alongside HPA, using VPA's recommendations to manually set appropriate resource requests that your HPA can then use for scaling decisions. Alternatively, use VPA for memory recommendations and HPA for CPU-based scaling, as they target different metrics. For production Rust services, this combined approach works well: VPA tells you that your service needs 512Mi memory (you set that as the request), and HPA scales replicas when CPU exceeds 70%. This gives you both right-sizing and elastic scaling without conflicts.