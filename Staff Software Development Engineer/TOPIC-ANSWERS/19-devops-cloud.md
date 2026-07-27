# LEVEL 19 — DevOps & Cloud

---

## Git

### 0350. Branch

Parallel line of development. Keep short-lived; name by intent. Long-lived branches diverge and hurt.

**Talk track:** *"Branches isolate change — merge early to avoid integration hell."*

---

### 0351. Merge

Combine histories; may create a merge commit. Simple and preserves true history of integration.

**Talk track:** *"Merges integrate branches — conflicts are communication, not just text."*

---

### 0352. Rebase

Replay commits on top of another base — linear history. Don't rebase shared published history without coordination.

**Talk track:** *"Rebase rewrites history for clarity — never rewrite others' public commits casually."*

---

### 0353. Cherry Pick

Apply one commit onto another branch. Useful for hotfixes; can duplicate patches and confuse history if overused.

**Talk track:** *"Cherry-pick copies a commit — great for urgent fixes, messy if routine."*

---

### 0354. Bisect

Binary search commits to find which introduced a bug. Requires a reliable test script. Staff debugging superpower.

**Talk track:** *"git bisect finds the breaking commit with log₂(n) checks."*

---

### 0355. Hooks

Scripts on git events (pre-commit, pre-push). Formatters/linters locally; don't rely as sole CI gate — users can skip.

**Talk track:** *"Hooks automate local quality gates — CI remains the source of truth."*

---

## Docker

### 0356. Images

Immutable filesystem layers + metadata + entrypoint. Build once, run many. Tag carefully; prefer digests in prod.

**Talk track:** *"Images are packaged snapshots of your app and its deps."*

---

### 0357. Containers

Running instances of images — isolated processes with their own filesystem view, not tiny VMs. Share the host kernel.

**Talk track:** *"Containers are isolated processes from images — fast start, kernel shared."*

---

### 0358. Volumes

Persistent / shared storage outside the container's ephemeral layer. Bind mounts for dev; named volumes for data.

**Talk track:** *"Volumes keep data alive when containers die."*

---

### 0359. Networking

Bridge, host, overlay networks; published ports; DNS between compose services. Understand localhost vs container IP confusion.

**Talk track:** *"Container networking is virtual links and DNS — publish only what you must."*

---

### 0360. Multi Stage Builds

Build in a fat image; copy artifacts into a slim runtime image. Smaller attack surface and faster pulls.

**Talk track:** *"Multi-stage builds discard build tools from the final image."*

---

### 0361. BuildKit

Modern Docker build engine: better caching, parallel builds, secrets mounts. Enable for faster safer builds.

**Talk track:** *"BuildKit is the modern builder — cache and secrets done right."*

---

### 0362. OCI

Open Container Initiative standards for image/runtime formats. Enables portable tooling across Docker/containerd/Podman.

**Talk track:** *"OCI is the standard that makes container images portable across engines."*

---

## Kubernetes

### 0363. Pods

Smallest deployable unit — one or more containers sharing network/namespace. Usually one app container + sidecars.

**Talk track:** *"Pods are the atom of scheduling — co-located containers with a shared network identity."*

---

### 0364. ReplicaSets

Ensure N copies of a pod template run. Rarely managed directly — Deployments own them.

**Talk track:** *"ReplicaSets maintain desired pod count."*

---

### 0365. Deployments

Declarative rolling updates/rollbacks for stateless apps. The default app workload API.

**Talk track:** *"Deployments roll out ReplicaSets safely — your everyday release object."*

---

### 0366. DaemonSets

One pod per node (or subset) — agents, log collectors, CNI helpers.

**Talk track:** *"DaemonSets run node-level agents everywhere you need them."*

---

### 0367. StatefulSets

Stable network identity + ordered deploy/scale + persistent volume claims per identity. Databases, queues with identity.

**Talk track:** *"StatefulSets give pods stable names and disks — for stateful clustered software."*

---

### 0368. Services

Stable virtual IP/DNS load-balancing to pods. ClusterIP/Internal, NodePort, LoadBalancer types.

**Talk track:** *"Services are stable front doors to ephemeral pods."*

---

### 0369. Ingress

HTTP(S) routing into the cluster — host/path rules, TLS. Needs an Ingress controller (nginx, ALB, Traefik).

**Talk track:** *"Ingress is L7 entry into the cluster — DNS and TLS live here."*

---

### 0370. ConfigMaps

Non-secret config as k8s objects injected as env/files. Version with app; don't stuff huge files.

**Talk track:** *"ConfigMaps externalize config from images."*

---

### 0371. Secrets

Sensitive config objects (base64, not encryption by default — enable encryption at rest). Mount carefully; prefer external secret managers at scale.

**Talk track:** *"K8s Secrets are convenience, not a full vault — lock down RBAC and encryption."*

---

### 0372. PVC

PersistentVolumeClaim — request for storage by a pod. Decouples app from underlying disk tech.

**Talk track:** *"PVCs are storage requests apps make."*

---

### 0373. PV

PersistentVolume — actual storage resource in the cluster. Provisioned statically or dynamically via StorageClass.

**Talk track:** *"PVs are the real disks; PVCs bind to them."*

---

### 0374. HPA

Horizontal Pod Autoscaler — scale replicas from CPU/memory/custom metrics. Needs good metrics and headroom; scale-to-zero/cold start awareness.

**Talk track:** *"HPA scales out on signals — bad metrics make bad scaling."*

---

### 0375. Operators

Controllers that encode operational knowledge (deploy, backup, failover) for complex software via custom resources.

**Talk track:** *"Operators automate human runbooks for complex stateful systems."*

---

### 0376. CRDs

Custom Resource Definitions — extend the Kubernetes API with your types. Operators watch CRDs.

**Talk track:** *"CRDs let you teach Kubernetes new nouns."*

---

## AWS

### 0377. IAM

Identity and Access Management — users/roles/policies. Least privilege, prefer roles over long-lived keys, condition keys matter.

**Talk track:** *"IAM is the security boundary of AWS — roles over keys, least privilege always."*

---

### 0378. EC2

Virtual machines in AWS. Full control; you patch and scale. Baseline compute primitive.

**Talk track:** *"EC2 is rentable VMs — maximum control, maximum ops."*

---

### 0379. S3

Durable object storage. Infinite-ish scale, HTTP API, storage classes, versioning, lifecycle policies. Foundation for backups, data lakes, static assets.

**Talk track:** *"S3 is durable object storage — design keys, lifecycle, and access patterns."*

---

### 0380. VPC

Virtual Private Cloud — your isolated network: subnets, route tables, gateways, security groups. Public vs private subnet design is security architecture.

**Talk track:** *"VPC is your private network in AWS — subnet layout is a security decision."*

---

### 0381. ALB

Application Load Balancer — L7 HTTP routing, path/host based, WAF integration, target groups.

**Talk track:** *"ALB is HTTP-smart load balancing."*

---

### 0382. NLB

Network Load Balancer — L4 ultra-high performance, static IPs, TCP/UDP. When you need raw connection scale or non-HTTP.

**Talk track:** *"NLB is connection-level load balancing at massive scale."*

---

### 0383. Route53

DNS service — public/private zones, routing policies (latency, weighted, failover), health checks.

**Talk track:** *"Route53 is DNS with cloud routing smarts."*

---

### 0384. CloudWatch

Metrics, logs, alarms for AWS resources and custom metrics. First place many incidents surface.

**Talk track:** *"CloudWatch is AWS-native telemetry and alarming."*

---

## Infrastructure

### 0385. Terraform

Declarative IaC — plan/apply infrastructure changes, state file is critical. Modules for reuse; careful with state locking and secrets.

**Talk track:** *"Terraform makes infra reviewable code — protect state like production data."*

---

### 0386. Nginx

High-performance reverse proxy/web server. TLS termination, static files, reverse proxy, basic LB. Everywhere in legacy and edge setups.

**Talk track:** *"Nginx is the classic proxy/web server — simple, fast, ubiquitous."*

---

### 0387. HAProxy

Battle-tested load balancer (L4/L7). Fine-grained control, excellent for TCP and HTTP fleets.

**Talk track:** *"HAProxy is a precise, proven load balancer."*

---

### 0388. Envoy

Modern proxy designed for clouds/meshes — rich observability, retries, timeouts, L7 filters. Sidecar in service meshes; also edge proxy.

**Talk track:** *"Envoy is the programmable cloud-native proxy — retries, metrics, and L7 policy."*

---

### 0389. Helm

Kubernetes package manager — charts template k8s manifests. Values files per environment. Templating complexity is the downside.

**Talk track:** *"Helm packages Kubernetes apps — values configure environments."*

---

### 0390. GitHub Actions

CI/CD in GitHub — workflows on events. Build, test, scan, deploy. Keep secrets in OIDC to cloud when possible; pin action versions.

**Talk track:** *"Actions automate pipelines next to code — treat workflows as production software."*

---

### 0391. ArgoCD

GitOps continuous delivery for Kubernetes — desired state in git, cluster reconciled to match. PRs become deploys.

**Talk track:** *"ArgoCD makes git the source of truth for cluster state."*
