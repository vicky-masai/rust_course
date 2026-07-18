# Persistent Volumes

## Interview Question

Explain the Persistent Volume (PV), Persistent Volume Claim (PVC), and StorageClass abstractions in Kubernetes and when you need them for stateful workloads.

## Interview Answer

Persistent Volumes (PVs) represent storage resources in the cluster (like an EBS volume or NFS share), while Persistent Volume Claims (PVCs) are requests for storage by Pods — similar to how Pods request CPU and memory. StorageClasses define the types of storage available (e.g., SSD, HDD, encrypted) and handle dynamic provisioning, so when a PVC references a StorageClass, Kubernetes automatically creates the underlying storage. For stateful Rust workloads like databases or message queues, you use PVCs in your Pod/Deployment spec, and Kubernetes binds them to available PVs or provisions new ones. StatefulSets use volumeClaimTemplates to give each Pod its own persistent storage, essential for distributed databases where each replica needs independent storage. This abstraction separates storage provisioning from application deployment, allowing different teams to manage infrastructure and application concerns independently.

---

## Follow-up Questions & Answers

### Q1. What is the difference between a PV and a PVC?

**Interview Answer**

A Persistent Volume (PV) is a cluster-level resource representing actual storage (EBS volume, NFS mount, local disk) that an administrator provisions manually or that is dynamically provisioned by a StorageClass. A Persistent Volume Claim (PVC) is a namespace-scoped request for storage by a user — it specifies size, access mode, and optionally a StorageClass. Kubernetes binds the PVC to a compatible PV (matching size, access mode, and StorageClass), and the Pod mounts the PVC like a volume. This two-tier abstraction allows cluster administrators to manage storage infrastructure separately from developers who simply declare "I need 10Gi of SSD storage" via a PVC. For Rust services, you create a PVC in your manifest and reference it as a volume mount — the underlying PV details are abstracted away.

---

### Q2. What are access modes for Persistent Volumes?

**Interview Answer**

Access modes define how a PV can be mounted across nodes: ReadWriteOnce (RWO) allows read-write access by a single node, ReadOnlyMany (ROX) allows read-only access by multiple nodes, and ReadWriteMany (RWX) allows read-write access by multiple nodes simultaneously. RWO is the most common and is supported by all block storage (EBS, GCE Persistent Disks), while RWX requires file-based storage like NFS or EFS and is less commonly supported. For Rust services, use RWO for databases (only one replica should write), ROX for shared configuration files, and RWX for shared logging or caching layers. Note that some storage providers support only specific access modes — check your cloud provider's documentation before creating PVCs.

---

### Q3. What is dynamic provisioning and how does it work?

**Interview Answer**

Dynamic provisioning eliminates the need for cluster administrators to pre-create PVs — instead, when a PVC references a StorageClass, Kubernetes automatically provisions the underlying storage and creates a PV to bind to it. The StorageClass specifies the provisioner (e.g., `ebs.csi.aws.com` for AWS EBS, `pd.csi.gke.io` for GCP Persistent Disk), parameters (volume type, IOPS, encryption), and reclaim policy. When you delete the PVC, the reclaim policy determines what happens to the storage: Delete removes the underlying volume, Retain preserves it for manual cleanup. For production Rust services, always define a StorageClass with your preferred volume type and reclaim policy to ensure consistent storage provisioning across environments.

---

### Q4. What is a StatefulSet and how does it use Persistent Volumes?

**Interview Answer**

StatefulSets are Kubernetes resources designed for stateful applications (databases, message queues, distributed systems) that need stable network identities and persistent storage. Unlike Deployments, StatefulSets create Pods with deterministic names (`pod-0`, `pod-1`, `pod-2`) and stable DNS entries, and they use `volumeClaimTemplates` to give each Pod its own persistent storage. When a Pod is rescheduled (due to node failure), it retains its name and reattaches to its original PV, ensuring data continuity. For Rust services that manage state (like a distributed cache or database replica), StatefulSets provide the stable infrastructure needed — but most Rust web services are stateless and should use Deployments instead.

---

### Q5. What is the difference between EBS, EFS, and S3 for Kubernetes storage?

**Interview Answer**

EBS (Elastic Block Store) provides block storage that mounts to a single node — it's fast (low latency, high IOPS) but limited to RWO access, making it ideal for databases. EFS (Elastic File System) provides NFS-based shared storage with RWX access, suitable for shared files and logging but with higher latency than EBS. S3 is object storage accessed via API, not mountable as a traditional volume, but can be used via CSI drivers for specific workloads. For Rust services, use EBS for stateful workloads needing fast local storage, EFS for shared configuration or log files across multiple replicas, and S3 for backups and static assets. Choose based on your access pattern (single-writer vs. multi-reader) and performance requirements.

---

### Q6. How do you handle data migration between Persistent Volumes?

**Interview Answer**

Data migration between PVs requires careful planning because Kubernetes doesn't provide built-in data migration tools. For databases, use native replication and backup/restore mechanisms (e.g., PostgreSQL's pg_dump/pg_restore) rather than trying to copy PV data directly. For file-based data, you can use Jobs with init containers to copy data from a source PVC to a destination PVC, or use tools like rsync in a multi-container Pod. In production Rust services, the recommended approach is: back up data from the old PV, provision a new PV with the desired StorageClass, restore data to the new PV, and update your Deployment to reference the new PVC. Always test migration procedures in staging and ensure your Rust application can handle brief storage unavailability during the transition.

---

### Q7. What happens when a PVC is deleted in Kubernetes?

**Interview Answer**

When a PVC is deleted, what happens to the underlying data depends on the PV's reclaim policy: Delete (default for dynamically provisioned PVs) removes the PV and the underlying storage resource (e.g., EBS volume), destroying all data permanently. Retain keeps the PV and its data intact, but the PV enters a "Released" state and is not automatically available for new claims — an administrator must manually clean it up and make it available again. For production Rust services with important data, always use Retain reclaim policy and implement backup procedures. If you accidentally delete a PVC with Delete reclaim policy and no backups, the data is gone permanently — this is one of the most critical operational risks in Kubernetes.

---

### Q8. How do you resize a Persistent Volume in Kubernetes?

**Interview Answer**

Kubernetes supports online PVC resizing (since 1.11 for filesystems, 1.24 for block volumes) — you edit the PVC's `storageRequests` to increase the size, and the underlying storage is expanded automatically if the StorageClass allows expansion (`allowVolumeExpansion: true`). The filesystem inside the volume may also need resizing, which happens automatically on the next mount or can be triggered manually. Shrinking PVs is not supported — you can only grow them. For production Rust services, always enable volume expansion in your StorageClass and set reasonable initial sizes to avoid frequent resizing. Note that resizing may require the Pod to be restarted depending on the storage provider, so plan accordingly for stateful workloads.

---

### Q9. What are local Persistent Volumes and their trade-offs?

**Interview Answer**

Local Persistent Volumes provide high-performance storage directly attached to a node (like NVMe SSDs), offering lower latency than network storage but with a critical trade-off: data is only accessible on that specific node. If the node fails, the data is unavailable until the node recovers. Kubernetes binds Local PVs to specific nodes using node affinity, so Pods using them are constrained to that node. For Rust services, Local PVs are suitable for caching layers, temporary data processing, or databases with their own replication (like CockroachDB or TiKV). They're not suitable for data that must survive node failures unless the application implements its own replication. Use `hostPath` volumes only for development — Local PVs are the production-grade alternative.

---

### Q10. How do you backup and restore Persistent Volumes?

**Interview Answer**

Kubernetes doesn't provide built-in backup for PVs — you need external tools. For cloud storage (EBS, GCE PD), use cloud provider snapshots (AWS EBS Snapshots, GCP Disk Snapshots) scheduled via Velero, which also handles cluster resource backup. For databases, use native backup tools (pg_dump for PostgreSQL, mongodump for MongoDB) running as Kubernetes Jobs that write to S3 or another durable store. Velero can back up both Kubernetes resources and PV data, enabling full cluster restores. For production Rust services with databases, implement automated daily backups to S3 with retention policies, test restores regularly, and ensure your Rust application can handle database failover during backup operations. Document your restore procedure and practice it — untested backups are not backups.