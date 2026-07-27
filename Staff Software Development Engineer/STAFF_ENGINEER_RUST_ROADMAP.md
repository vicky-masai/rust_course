# Staff Software Engineer — Rust Backend Master Roadmap

> **Goal:** Design Any System · Build Any Backend · Lead Architecture  
> **Version:** v1.0 (Recommended Learning Sequence)  
> **How to use:** Check `[ ]` → `[x]` as you finish each topic. Track daily by number.  
> **Answers:** Staff-level plain-English explanations for every topic → [`TOPIC-ANSWERS/`](./TOPIC-ANSWERS/README.md)

---

## How to study every topic

- [ ] Learn the theory
- [ ] Understand internal implementation
- [ ] Read the source code
- [ ] Implement it in Rust
- [ ] Benchmark it
- [ ] Debug it
- [ ] Add observability
- [ ] Scale it
- [ ] Explain the tradeoffs
- [ ] Teach it to someone else

---

## LEVEL 00 — Engineering Mindset (Think Like a Staff Engineer)

Always ask for every topic you study:

- [ ] 0001. Why does this exist?
- [ ] 0002. What problem does it solve?
- [ ] 0003. How does it work internally?
- [ ] 0004. What are the tradeoffs?
- [ ] 0005. What happens when it fails?
- [ ] 0006. How would Amazon / Uber / Stripe / OpenAI build this?
- [ ] 0007. How would I debug it in production?
- [ ] 0008. How would I scale it to 100M users?

---

## LEVEL 01 — Computer Science Foundation

### Computer Architecture

- [ ] 0009. Binary & Hexadecimal
- [ ] 0010. CPU Architecture
- [ ] 0011. Registers
- [ ] 0012. Instruction Pipeline
- [ ] 0013. CPU Cache (L1 / L2 / L3)
- [ ] 0014. Cache Lines
- [ ] 0015. RAM
- [ ] 0016. Stack Memory
- [ ] 0017. Heap Memory
- [ ] 0018. Virtual Memory
- [ ] 0019. Paging
- [ ] 0020. Memory Alignment
- [ ] 0021. Memory Padding
- [ ] 0022. Branch Prediction
- [ ] 0023. SIMD
- [ ] 0024. NUMA
- [ ] 0025. False Sharing
- [ ] 0026. Cache Locality
- [ ] 0027. Memory Barriers

### Operating Systems

- [ ] 0028. Linux Architecture
- [ ] 0029. Process
- [ ] 0030. Thread
- [ ] 0031. Scheduler
- [ ] 0032. Context Switching
- [ ] 0033. Synchronization
- [ ] 0034. System Calls
- [ ] 0035. File Systems
- [ ] 0036. mmap()
- [ ] 0037. epoll
- [ ] 0038. io_uring
- [ ] 0039. Signals
- [ ] 0040. IPC

### Networking

- [ ] 0041. OSI Model
- [ ] 0042. TCP
- [ ] 0043. UDP
- [ ] 0044. DNS
- [ ] 0045. TLS
- [ ] 0046. HTTP/1.1
- [ ] 0047. HTTP/2
- [ ] 0048. HTTP/3
- [ ] 0049. QUIC
- [ ] 0050. WebSocket
- [ ] 0051. SSE
- [ ] 0052. gRPC
- [ ] 0053. Reverse Proxy
- [ ] 0054. Load Balancing

### Linux Tools

- [ ] 0055. top
- [ ] 0056. htop
- [ ] 0057. ps
- [ ] 0058. vmstat
- [ ] 0059. iostat
- [ ] 0060. sar
- [ ] 0061. lsof
- [ ] 0062. ss
- [ ] 0063. tcpdump
- [ ] 0064. strace
- [ ] 0065. perf
- [ ] 0066. journalctl
- [ ] 0067. systemd

---

## LEVEL 02 — Data Structures & Algorithms

### Data Structures

- [ ] 0068. Arrays
- [ ] 0069. Strings
- [ ] 0070. Linked List
- [ ] 0071. Stack
- [ ] 0072. Queue
- [ ] 0073. Deque
- [ ] 0074. HashMap
- [ ] 0075. HashSet
- [ ] 0076. Heap
- [ ] 0077. Trie
- [ ] 0078. Tree
- [ ] 0079. BST
- [ ] 0080. AVL Tree
- [ ] 0081. Red Black Tree
- [ ] 0082. B Tree
- [ ] 0083. B+ Tree
- [ ] 0084. Graph

### Algorithms

- [ ] 0085. Sorting
- [ ] 0086. Searching
- [ ] 0087. Binary Search
- [ ] 0088. DFS
- [ ] 0089. BFS
- [ ] 0090. Sliding Window
- [ ] 0091. Two Pointer
- [ ] 0092. Prefix Sum
- [ ] 0093. Greedy
- [ ] 0094. Backtracking
- [ ] 0095. Dynamic Programming
- [ ] 0096. Union Find
- [ ] 0097. Topological Sort
- [ ] 0098. Dijkstra
- [ ] 0099. A*
- [ ] 0100. Consistent Hashing
- [ ] 0101. Bloom Filter
- [ ] 0102. HyperLogLog
- [ ] 0103. Skip List
- [ ] 0104. Bit Manipulation

---

## LEVEL 03 — Rust Language

### Ownership & Memory

- [ ] 0105. Ownership
- [ ] 0106. Borrow Checker
- [ ] 0107. References
- [ ] 0108. Move Semantics
- [ ] 0109. Copy Trait
- [ ] 0110. Clone Trait
- [ ] 0111. Drop Trait
- [ ] 0112. Drop Order
- [ ] 0113. Deref
- [ ] 0114. Deref Coercion
- [ ] 0115. Interior Mutability
- [ ] 0116. Memory Layout
- [ ] 0117. Zero Cost Abstractions
- [ ] 0118. Pinning
- [ ] 0119. Self Referential Types
- [ ] 0120. PhantomData

### Lifetimes

- [ ] 0121. Lifetime Elision
- [ ] 0122. Explicit Lifetimes
- [ ] 0123. Generic Lifetimes
- [ ] 0124. Lifetime Bounds
- [ ] 0125. `'static` Lifetime
- [ ] 0126. Higher Ranked Trait Bounds (HRTB)
- [ ] 0127. Lifetime Variance
- [ ] 0128. Lifetime Subtyping
- [ ] 0129. Async Lifetimes

### Traits & Generics

- [ ] 0130. Trait Bounds
- [ ] 0131. Associated Types
- [ ] 0132. Generic Associated Types (GAT)
- [ ] 0133. Trait Objects
- [ ] 0134. Static Dispatch
- [ ] 0135. Dynamic Dispatch
- [ ] 0136. Send
- [ ] 0137. Sync
- [ ] 0138. Sized
- [ ] 0139. Marker Traits
- [ ] 0140. Auto Traits
- [ ] 0141. Blanket Implementations
- [ ] 0142. Monomorphization

---

## LEVEL 04 — Memory Management

### Smart Pointers

- [ ] 0143. Box
- [ ] 0144. Rc
- [ ] 0145. Arc
- [ ] 0146. Weak
- [ ] 0147. Cell
- [ ] 0148. RefCell
- [ ] 0149. Mutex
- [ ] 0150. RwLock
- [ ] 0151. OnceCell
- [ ] 0152. LazyLock
- [ ] 0153. Atomic Types
- [ ] 0154. Pin

### Collections

- [ ] 0155. Vec
- [ ] 0156. VecDeque
- [ ] 0157. HashMap
- [ ] 0158. HashSet
- [ ] 0159. BTreeMap
- [ ] 0160. BTreeSet
- [ ] 0161. BinaryHeap
- [ ] 0162. Hash Algorithms
- [ ] 0163. Allocator
- [ ] 0164. Memory Allocation

---

## LEVEL 05 — Concurrency

- [ ] 0165. Threads
- [ ] 0166. Thread Pools
- [ ] 0167. Scoped Threads
- [ ] 0168. Shared State
- [ ] 0169. Arc + Mutex
- [ ] 0170. Arc + RwLock
- [ ] 0171. Condvar
- [ ] 0172. Message Passing
- [ ] 0173. Crossbeam
- [ ] 0174. MPSC Channels
- [ ] 0175. Lock-Free Programming
- [ ] 0176. Atomics
- [ ] 0177. Memory Ordering
- [ ] 0178. Compare-And-Swap (CAS)
- [ ] 0179. ABA Problem

---

## LEVEL 06 — Async Rust

- [ ] 0180. Future Trait
- [ ] 0181. Poll
- [ ] 0182. Waker
- [ ] 0183. Context
- [ ] 0184. Async / Await Internals
- [ ] 0185. Async State Machines
- [ ] 0186. Async Cancellation
- [ ] 0187. Async Streams
- [ ] 0188. Backpressure

---

## LEVEL 07 — Tokio

- [ ] 0189. Runtime
- [ ] 0190. Scheduler Internals
- [ ] 0191. Work Stealing
- [ ] 0192. Cooperative Scheduling
- [ ] 0193. spawn()
- [ ] 0194. spawn_blocking()
- [ ] 0195. JoinHandle
- [ ] 0196. JoinSet
- [ ] 0197. select!
- [ ] 0198. timeout()
- [ ] 0199. Semaphore
- [ ] 0200. Notify
- [ ] 0201. Barrier
- [ ] 0202. MPSC
- [ ] 0203. Broadcast
- [ ] 0204. Watch Channels

---

## LEVEL 08 — Advanced Rust

### Unsafe Rust

- [ ] 0205. Unsafe Rust
- [ ] 0206. Raw Pointers
- [ ] 0207. Unsafe Functions
- [ ] 0208. Unsafe Traits
- [ ] 0209. Pointer Arithmetic
- [ ] 0210. Manual Memory Management

### FFI

- [ ] 0211. FFI
- [ ] 0212. Rust ↔ C
- [ ] 0213. Rust ↔ C++
- [ ] 0214. Rust ↔ Python
- [ ] 0215. extern "C"
- [ ] 0216. no_mangle

### Macros

- [ ] 0217. macro_rules!
- [ ] 0218. Procedural Macros
- [ ] 0219. Derive Macros
- [ ] 0220. Attribute Macros
- [ ] 0221. Function-like Macros
- [ ] 0222. syn
- [ ] 0223. quote

### Serialization

- [ ] 0224. Serde
- [ ] 0225. JSON
- [ ] 0226. Bincode
- [ ] 0227. Protobuf
- [ ] 0228. FlatBuffers

---

## LEVEL 09 — API & Backend Engineering

- [ ] 0229. REST
- [ ] 0230. GraphQL
- [ ] 0231. gRPC
- [ ] 0232. OpenAPI
- [ ] 0233. API Versioning
- [ ] 0234. Pagination
- [ ] 0235. Cursor Pagination
- [ ] 0236. Filtering
- [ ] 0237. Sorting
- [ ] 0238. Validation
- [ ] 0239. Idempotency

---

## LEVEL 10 — Axum

- [ ] 0240. Router
- [ ] 0241. Middleware
- [ ] 0242. Tower Layers
- [ ] 0243. Custom Extractors
- [ ] 0244. Request Extensions
- [ ] 0245. Authentication
- [ ] 0246. Authorization
- [ ] 0247. RBAC
- [ ] 0248. ABAC
- [ ] 0249. Rate Limiting
- [ ] 0250. WebSockets
- [ ] 0251. SSE
- [ ] 0252. Background Workers

---

## LEVEL 11 — Database Engineering

- [ ] 0253. SQLx
- [ ] 0254. Connection Pooling
- [ ] 0255. Transactions
- [ ] 0256. Isolation Levels
- [ ] 0257. MVCC
- [ ] 0258. Deadlocks
- [ ] 0259. Optimistic Locking
- [ ] 0260. Pessimistic Locking
- [ ] 0261. Indexes
- [ ] 0262. Query Planner
- [ ] 0263. Query Optimizer
- [ ] 0264. WAL
- [ ] 0265. VACUUM
- [ ] 0266. HOT Updates
- [ ] 0267. Buffer Cache
- [ ] 0268. PostgreSQL Internals
- [ ] 0269. Replication
- [ ] 0270. Partitioning

---

## LEVEL 12 — Caching

- [ ] 0271. Redis
- [ ] 0272. Cache Aside
- [ ] 0273. Read Through
- [ ] 0274. Write Through
- [ ] 0275. Write Back
- [ ] 0276. Write Around
- [ ] 0277. Cache Invalidation
- [ ] 0278. Distributed Cache
- [ ] 0279. Redis Cluster

---

## LEVEL 13 — Distributed Systems

- [ ] 0280. CAP Theorem
- [ ] 0281. Consistency Models
- [ ] 0282. Consensus
- [ ] 0283. Raft
- [ ] 0284. Leader Election
- [ ] 0285. Distributed Locking
- [ ] 0286. Replication
- [ ] 0287. Sharding
- [ ] 0288. Partitioning
- [ ] 0289. Vector Clock
- [ ] 0290. Lamport Clock
- [ ] 0291. Failure Detection
- [ ] 0292. Clock Drift

---

## LEVEL 14 — Event Driven Architecture

### Messaging Systems

- [ ] 0293. Kafka
- [ ] 0294. RabbitMQ
- [ ] 0295. NATS

### Kafka Internals

- [ ] 0296. Broker
- [ ] 0297. Topic
- [ ] 0298. Partition
- [ ] 0299. Leader
- [ ] 0300. ISR
- [ ] 0301. Consumer Groups
- [ ] 0302. Offsets
- [ ] 0303. Rebalancing
- [ ] 0304. Retention
- [ ] 0305. Compaction
- [ ] 0306. Ordering

### Patterns

- [ ] 0307. CQRS
- [ ] 0308. Event Sourcing
- [ ] 0309. Outbox Pattern
- [ ] 0310. CDC
- [ ] 0311. Saga Pattern
- [ ] 0312. Idempotency
- [ ] 0313. Dead Letter Queue

---

## LEVEL 15 — Search Systems

- [ ] 0314. Elasticsearch
- [ ] 0315. OpenSearch
- [ ] 0316. Inverted Index
- [ ] 0317. Tokenization
- [ ] 0318. Ranking
- [ ] 0319. Full Text Search

---

## LEVEL 16 — Security

- [ ] 0320. JWT
- [ ] 0321. OAuth2
- [ ] 0322. OpenID Connect
- [ ] 0323. TLS
- [ ] 0324. mTLS
- [ ] 0325. Encryption
- [ ] 0326. Hashing
- [ ] 0327. Secrets Management
- [ ] 0328. OWASP Top 10
- [ ] 0329. SQL Injection
- [ ] 0330. XSS
- [ ] 0331. CSRF
- [ ] 0332. SSRF
- [ ] 0333. API Security

---

## LEVEL 17 — Observability

- [ ] 0334. tracing
- [ ] 0335. Structured Logging
- [ ] 0336. OpenTelemetry
- [ ] 0337. Metrics
- [ ] 0338. Prometheus
- [ ] 0339. Grafana
- [ ] 0340. Jaeger
- [ ] 0341. Distributed Tracing

---

## LEVEL 18 — Performance Engineering

- [ ] 0342. Benchmarking
- [ ] 0343. Criterion
- [ ] 0344. CPU Profiling
- [ ] 0345. Memory Profiling
- [ ] 0346. Flamegraphs
- [ ] 0347. Allocation Reduction
- [ ] 0348. Cache Optimization
- [ ] 0349. Latency Analysis

---

## LEVEL 19 — DevOps & Cloud

### Git

- [ ] 0350. Branch
- [ ] 0351. Merge
- [ ] 0352. Rebase
- [ ] 0353. Cherry Pick
- [ ] 0354. Bisect
- [ ] 0355. Hooks

### Docker

- [ ] 0356. Images
- [ ] 0357. Containers
- [ ] 0358. Volumes
- [ ] 0359. Networking
- [ ] 0360. Multi Stage Builds
- [ ] 0361. BuildKit
- [ ] 0362. OCI

### Kubernetes

- [ ] 0363. Pods
- [ ] 0364. ReplicaSets
- [ ] 0365. Deployments
- [ ] 0366. DaemonSets
- [ ] 0367. StatefulSets
- [ ] 0368. Services
- [ ] 0369. Ingress
- [ ] 0370. ConfigMaps
- [ ] 0371. Secrets
- [ ] 0372. PVC
- [ ] 0373. PV
- [ ] 0374. HPA
- [ ] 0375. Operators
- [ ] 0376. CRDs

### AWS

- [ ] 0377. IAM
- [ ] 0378. EC2
- [ ] 0379. S3
- [ ] 0380. VPC
- [ ] 0381. ALB
- [ ] 0382. NLB
- [ ] 0383. Route53
- [ ] 0384. CloudWatch

### Infrastructure

- [ ] 0385. Terraform
- [ ] 0386. Nginx
- [ ] 0387. HAProxy
- [ ] 0388. Envoy
- [ ] 0389. Helm
- [ ] 0390. GitHub Actions
- [ ] 0391. ArgoCD

---

## LEVEL 20 — Enterprise Backend Architecture

- [ ] 0392. Repository Pattern
- [ ] 0393. Unit of Work
- [ ] 0394. Domain Driven Design (DDD)
- [ ] 0395. Hexagonal Architecture
- [ ] 0396. Onion Architecture
- [ ] 0397. Clean Architecture
- [ ] 0398. Modular Monolith
- [ ] 0399. Microservices
- [ ] 0400. Service Oriented Architecture
- [ ] 0401. Workflow Engines

---

## LEVEL 21 — Enterprise Platform Engineering

### Platform Topics

- [ ] 0402. Multi-Tenancy
- [ ] 0403. Audit Logging
- [ ] 0404. Event Framework
- [ ] 0405. Inventory Reservation
- [ ] 0406. Permission Systems
- [ ] 0407. Feature Flags
- [ ] 0408. API Gateway
- [ ] 0409. Service Discovery
- [ ] 0410. Distributed Transactions

### Resilience Patterns

- [ ] 0411. Retry
- [ ] 0412. Exponential Backoff
- [ ] 0413. Circuit Breaker
- [ ] 0414. Bulkhead
- [ ] 0415. Timeout
- [ ] 0416. Fallback
- [ ] 0417. Chaos Engineering

---

## LEVEL 22 — System Design

- [ ] 0418. Scalability
- [ ] 0419. Reliability
- [ ] 0420. Availability
- [ ] 0421. Fault Tolerance
- [ ] 0422. Capacity Planning
- [ ] 0423. Load Balancing
- [ ] 0424. Caching Strategy
- [ ] 0425. High Availability
- [ ] 0426. Disaster Recovery
- [ ] 0427. Zero Downtime Deployment
- [ ] 0428. Multi Region Architecture
- [ ] 0429. Cost Optimization

---

## LEVEL 23 — AI Backend Engineering (Modern Backend)

- [ ] 0430. Embeddings
- [ ] 0431. Vector Databases
- [ ] 0432. RAG
- [ ] 0433. Model Context Protocol (MCP)
- [ ] 0434. Prompt Engineering
- [ ] 0435. Context Engineering
- [ ] 0436. Streaming LLM Responses
- [ ] 0437. AI Agent Architecture
- [ ] 0438. LLM Gateway
- [ ] 0439. Model Routing
- [ ] 0440. Inference Optimization

---

## LEVEL 24 — Staff Engineering

- [ ] 0441. Architecture Decision Records (ADR)
- [ ] 0442. Technical RFC Writing
- [ ] 0443. Design Reviews
- [ ] 0444. Tradeoff Analysis
- [ ] 0445. Incident Response
- [ ] 0446. Root Cause Analysis (RCA)
- [ ] 0447. Postmortems
- [ ] 0448. SLO
- [ ] 0449. SLI
- [ ] 0450. SLA
- [ ] 0451. Error Budgets
- [ ] 0452. Platform Engineering
- [ ] 0453. Developer Experience (DX)
- [ ] 0454. Technical Mentoring
- [ ] 0455. Cross Team Architecture
- [ ] 0456. Engineering Leadership

---

## LEVEL 25 — Source Code Reading (Mandatory)

Read internals of:

- [ ] 0457. Rust std
- [ ] 0458. Tokio
- [ ] 0459. Axum
- [ ] 0460. Tower
- [ ] 0461. Hyper
- [ ] 0462. Serde
- [ ] 0463. SQLx
- [ ] 0464. HashBrown
- [ ] 0465. Redis
- [ ] 0466. PostgreSQL
- [ ] 0467. Kafka
- [ ] 0468. etcd

---

## LEVEL 26 — Build Production Projects

- [ ] 0469. Authentication Service
- [ ] 0470. User Service
- [ ] 0471. Notification Service
- [ ] 0472. File Storage Service
- [ ] 0473. API Gateway
- [ ] 0474. Inventory Service
- [ ] 0475. Order Service
- [ ] 0476. Payment Service
- [ ] 0477. Search Service
- [ ] 0478. Workflow Engine
- [ ] 0479. Distributed Scheduler
- [ ] 0480. Distributed Cache
- [ ] 0481. Feature Flag Service
- [ ] 0482. Identity Provider
- [ ] 0483. Warehouse Management System (WMS)
- [ ] 0484. ERP Backend
- [ ] 0485. Multi-Tenant SaaS Platform
- [ ] 0486. AI Agent Platform
- [ ] 0487. RAG Platform
- [ ] 0488. Build Your Own Redis
- [ ] 0489. Build Your Own Kafka (Simplified)

---

## Final Goal Checklist

- [ ] 0490. Design Any Backend
- [ ] 0491. Build Any Backend In Rust
- [ ] 0492. Design Distributed Systems
- [ ] 0493. Build Enterprise SaaS Platforms
- [ ] 0494. Build AI Platforms
- [ ] 0495. Build High Performance Systems
- [ ] 0496. Lead Architecture Reviews
- [ ] 0497. Debug Production Incidents
- [ ] 0498. Optimize Performance
- [ ] 0499. Clear Senior / Staff Backend Interviews
- [ ] 0500. Think In Tradeoffs, Not Frameworks
- [ ] 0501. Become A Staff Software Engineer

---

## Daily study tip

Mark one or more topic numbers each day, e.g. **Today: 0105–0110**. Change `[ ]` to `[x]` when done:

```md
- [x] 0105. Ownership
```
