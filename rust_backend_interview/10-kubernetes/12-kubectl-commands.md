# kubectl Commands

## Interview Question

What are the essential kubectl commands for managing Kubernetes resources, debugging issues, and inspecting logs in a production environment?

## Interview Answer

kubectl is the command-line interface for Kubernetes, and mastering it is essential for day-to-day operations. The most used commands include `kubectl get` (list resources), `kubectl describe` (detailed resource info including events), `kubectl logs` (view container logs), `kubectl exec` (run commands inside Pods), `kubectl apply` (declaratively apply manifests), and `kubectl delete` (remove resources). For debugging, `kubectl describe pod <name>` shows events (image pull failures, scheduling issues, probe failures), `kubectl logs <pod> --previous` shows logs from crashed containers, and `kubectl exec -it <pod> -- /bin/sh` gives you a shell inside the container. Production essentials include `kubectl port-forward` (access Services locally), `kubectl top` (view resource usage), and `kubectl rollout` (manage deployments). Always use `--namespace` or set a default namespace context with `kubectl config set-context --current --namespace=production` to avoid applying resources to the wrong namespace. For Rust services, kubectl commands are your primary tool for debugging startup failures, checking health probe results, and viewing application logs.

---

## Follow-up Questions & Answers

### Q1. How do you view logs from a specific container in a multi-container Pod?

**Interview Answer**

Use `kubectl logs <pod-name> -c <container-name>` to specify which container's logs to view — this is essential for multi-container Pods with sidecars. Add `--previous` to see logs from a crashed container that was restarted, or `-f` to follow logs in real-time (like `tail -f`). For Rust services with a Fluentd sidecar, `kubectl logs my-pod -c my-rust-service` shows application logs while `kubectl logs my-pod -c log-collector` shows the sidecar's logs. Use `--since=1h` to limit log output to recent entries, and `--max-log-requests=10` to increase the default limit when streaming logs from multiple Pods. For production debugging, combine `kubectl logs` with `kubectl describe pod` — the logs show what your Rust service output, while describe shows Kubernetes events (probe failures, OOM kills, scheduling issues).

---

### Q2. How do you debug a Pod that is stuck in CrashLoopBackOff?

**Interview Answer**

Start with `kubectl describe pod <pod-name>` to see events — look for OOMKilled (memory limit too low), image pull errors (wrong image name/tag), or failed liveness probes. Check logs with `kubectl logs <pod-name> --previous` to see output from the crashed container. For Rust services, common causes include: missing environment variables (your config crate panics on startup), database connection failures at initialization, wrong binary architecture (running amd64 on arm64), or segfaults in unsafe code. Use `kubectl exec -it <pod-name> -- /bin/sh` to inspect the running container if it's partially up, or `kubectl debug -it <pod-name> --image=busybox --target=<container>` for ephemeral debugging containers. Check the Pod's events, resource limits, and probe configuration — often the fix is as simple as correcting an environment variable or increasing the memory limit.

---

### Q3. How do you port-forward to a Service in Kubernetes?

**Interview Answer**

Use `kubectl port-forward svc/<service-name> <local-port>:<service-port>` to tunnel traffic from your local machine to a Kubernetes Service, or `kubectl port-forward pod/<pod-name> <local-port>:<container-port>` to connect to a specific Pod. For Rust services, this lets you access your Actix/Axum server at `localhost:8080` for local testing without exposing the Service externally. Add `--address 0.0.0.0` to allow external access (e.g., from another machine). Port-forward is invaluable for debugging because it bypasses Ingress and LoadBalancer, giving you direct access to the Pod. Note that port-forward only works when the Pod is Running and on the same cluster network — it won't work from outside the cluster to a node without the Kubernetes API server connection. For production, use `kubectl port-forward` only for temporary debugging, not as a persistent connection.

---

### Q4. How do you view resource usage across Pods and nodes?

**Interview Answer**

Use `kubectl top pods` to see CPU and memory usage across all Pods (add `-A` for all namespaces), and `kubectl top nodes` to see node-level resource consumption. These commands require the Metrics Server to be installed in the cluster. For Rust services, `kubectl top pods --sort-by=memory` identifies Pods consuming the most memory (potential memory leaks), and `kubectl top pods --sort-by=cpu` identifies CPU-hungry Pods. Compare actual usage to resource requests to identify over-provisioned Pods — if your Rust service requests 1Gi memory but only uses 200Mi, you're wasting cluster resources. Use `kubectl describe node <name>` to see allocated vs. available resources on each node. For production monitoring, integrate with Prometheus and Grafana for historical trends rather than relying on point-in-time `kubectl top` snapshots.

---

### Q5. How do you manage kubeconfig and switch between clusters?

**Interview Answer**

Kubeconfig files (`~/.kube/config`) store cluster connection details, user credentials, and context definitions. Use `kubectl config get-contexts` to list available contexts, `kubectl config use-context <context-name>` to switch clusters, and `kubectl config set-context --current --namespace=<ns>` to set the default namespace. For multiple clusters (dev, staging, prod), merge kubeconfig files with `KUBECONFIG=file1:file2 kubectl config view --flatten > merged`. Set `KUBECONFIG` environment variable to point to your merged config. For production Rust services, always verify you're in the correct context before applying changes — `kubectl config current-context` shows the active cluster. Use separate kubeconfig files for production and development, and set `CURRENT_NAMESPACE` to avoid accidentally deploying to production when you mean dev. Enable auto-completion with `source <(kubectl completion zsh)` for faster context switching.

---

### Q6. How do you debug network connectivity between Pods?

**Interview Answer**

Start with `kubectl exec -it <pod> -- ping <target-service>` to test basic connectivity, then `kubectl exec -it <pod> -- curl <target-service>:<port>` to test application-level communication. Use `kubectl exec -it <pod> -- nslookup <service-name>` to verify DNS resolution — if this fails, check CoreDNS Pods and Service. Inspect NetworkPolicies with `kubectl get networkpolicies -A` and verify that traffic is allowed between source and destination. For Cilium, use `kubectl exec -it <cilium-pod> -n kube-system -- hubble observe` to see real-time network flows. For Rust services, common network issues include: missing readiness probes (Pod not in Service endpoints), wrong port configuration (Service targeting port 8080 but Rust server listening on 3000), and NetworkPolicies blocking DNS egress. Always check `kubectl describe svc <service-name>` to verify the Endpoints list has healthy Pod IPs.

---

### Q7. How do you perform rolling restarts and manage deployment rollouts?

**Interview Answer**

Trigger a rolling restart (all Pods replaced without changing the image) with `kubectl rollout restart deployment/<name>`. Check rollout status with `kubectl rollout status deployment/<name>`, view history with `kubectl rollout history deployment/<name>`, and undo with `kubectl rollout undo deployment/<name>` (or `--to-revision=N` for a specific revision). Pause a rollout with `kubectl rollout pause deployment/<name>` and resume with `kubectl rollout resume deployment/<name>`. For Rust services, rolling restarts are useful when you need to pick up ConfigMap or Secret changes (which don't trigger automatic Pod restarts), or when you've updated environment variables. The restart creates a new ReplicaSet and gradually replaces Pods, respecting maxSurge and maxUnavailable settings. Always monitor with `kubectl rollout status` to verify the rollout completes successfully before proceeding.

---

### Q8. How do you export and backup Kubernetes resources?

**Interview Answer**

Export resources with `kubectl get deployment my-rust-api -o yaml > deployment.yaml` to capture the current state including status fields, or use `kubectl get deployment my-rust-api -o yaml --export` (deprecated but removes cluster-specific fields). For all resources in a namespace: `kubectl get all -n production -o yaml > production-backup.yaml`. Use `kubectl get ingress,service,configmap,secret -n production -o yaml` for a comprehensive backup. For Helm releases, backup with `helm get all my-release > helm-backup.yaml`. Store backups in version control or cloud storage, and test restore procedures regularly. For production Rust services, always backup before major changes (upgrades, migrations), and include both manifests and data (database dumps, PV snapshots). The `velero` tool provides cluster-wide backup including PVs and is essential for disaster recovery.

---

### Q9. How do you manage multiple namespaces and contexts efficiently?

**Interview Answer**

Create a default namespace with `kubectl config set-context --current --namespace=production` so all commands default to production without `--namespace` flags. Use aliases for common operations: `alias k=kubectl`, `alias kn='kubectl -n production'`, `alias kdp='kubectl describe pod'`. For multi-cluster management, use tools like `kubectx` and `kns` for fast context and namespace switching. Set up shell completions with `source <(kubectl completion zsh)` and create a `.kubectl_aliases` file with frequently used commands. For Rust backend teams, standardize kubeconfig structure across team members, use separate contexts for dev/staging/prod, and document the namespace convention (e.g., `rust-api-dev`, `rust-api-staging`, `rust-api-prod`). Always verify your current context before destructive operations with `kubectl config current-context`.

---

### Q10. How do you use kubectl debug for production troubleshooting?

**Interview Answer**

Use `kubectl debug` to attach ephemeral debug containers to running Pods without modifying the original container image. For example, `kubectl debug -it <pod> --image=nicolaka/netshoot --target=<rust-container>` gives you a network debugging container with tools like curl, tcpdump, and dig in the same network namespace as your Rust service. This is invaluable for production debugging because your distroless Rust image likely lacks debugging tools. You can also debug node-level issues with `kubectl debug node/<node-name> --image=busybox` which gives you a privileged Pod on the node for inspecting kubelet logs, container runtime state, and network configuration. For Rust services, use ephemeral containers to test network connectivity, inspect environment variables, check file system state, and diagnose issues without restarting the Pod or modifying the production image. Always clean up debug containers after troubleshooting.