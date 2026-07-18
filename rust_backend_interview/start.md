# Rust Backend Interview Q&A

Source: [ChatGPT — Backend Rust Interview Guide](https://chatgpt.com/share/6a5b28c8-de30-83ee-8f16-86a98cd17e80)

Each file contains only:
1. **Interview Question**
2. **Interview Answer**
3. **Follow-up Questions & Answers** (when present in the share)

---

## Rust

1. [Ownership](01-rust/01-ownership.md) · +follow-ups
2. [Borrowing](01-rust/02-borrowing.md) · +follow-ups
3. [References (`&T` and `&mut T`) in Rust](01-rust/03-references-t-and-mut-t-in-rust.md) · +follow-ups
4. [Mutable Borrowing vs Immutable Borrowing in Rust](01-rust/04-mutable-borrowing-vs-immutable-borrowing-in-rust.md) · +follow-ups
5. [Lifetimes (`'a`) in Rust](01-rust/05-lifetimes-a-in-rust.md) · +follow-ups
6. [Stack vs Heap Memory in Rust](01-rust/06-stack-vs-heap-memory-in-rust.md) · +follow-ups
7. [Move vs Copy vs Clone in Rust](01-rust/07-move-vs-copy-vs-clone.md) · +follow-ups
8. [`Drop` Trait in Rust](01-rust/08-drop-trait.md) · +follow-ups
9. [RAII (Resource Acquisition Is Initialization) in Rust](01-rust/09-raii-resource-acquisition-is-initialization-in-rust.md) · +follow-ups
10. [`String` vs `&str` in Rust](01-rust/10-string-vs-str.md) · +follow-ups
11. [`Option<T>` in Rust](01-rust/11-optiont-in-rust.md) · +follow-ups
12. [`Result<T, E>` in Rust](01-rust/12-resultt-e-in-rust.md) · +follow-ups
13. [Pattern Matching (`match`) in Rust](01-rust/13-pattern-matching-match-in-rust.md) · +follow-ups
14. [Traits in Rust](01-rust/14-traits-in-rust.md) · +follow-ups
15. [Generics in Rust](01-rust/15-generics-in-rust.md) · +follow-ups
16. [`Send` and `Sync` Traits in Rust](01-rust/16-send-and-sync-traits-in-rust.md) · +follow-ups
17. [`Box<T>` Smart Pointer in Rust](01-rust/17-boxt-smart-pointer-in-rust.md) · +follow-ups
18. [`Rc<T>` and `Arc<T>` in Rust](01-rust/18-rct-and-arct-in-rust.md) · +follow-ups
19. [`Mutex<T>` and `RwLock<T>` in Rust](01-rust/19-mutext-and-rwlockt-in-rust.md) · +follow-ups
20. [Async/Await in Rust](01-rust/20-async-await-in-rust.md) · +follow-ups
21. [Future in Rust](01-rust/21-future-in-rust.md) · +follow-ups
22. [Tokio Runtime in Rust](01-rust/22-tokio-runtime-in-rust.md) · +follow-ups
23. [`tokio::spawn()` vs `std::thread::spawn()`](01-rust/23-tokio-spawn-vs-std-thread-spawn.md) · +follow-ups
24. [`spawn_blocking()` in Tokio](01-rust/24-spawn-blocking-in-tokio.md) · +follow-ups
25. [`Pin` and Why Rust Needs It](01-rust/25-pin-and-why-rust-needs-it.md) · +follow-ups
26. [`unsafe` Rust](01-rust/26-unsafe-rust.md) · +follow-ups
27. [Zero-Cost Abstractions in Rust](01-rust/27-zero-cost-abstractions-in-rust.md) · +follow-ups
28. [Static Dispatch vs Dynamic Dispatch in Rust](01-rust/28-static-dispatch-vs-dynamic-dispatch-in-rust.md) · +follow-ups
29. [`Fn`, `FnMut`, and `FnOnce` Closures in Rust](01-rust/29-fn-fnmut-and-fnonce-closures-in-rust.md) · +follow-ups
30. [`Arc<Mutex<T>>` vs `Arc<RwLock<T>>` in Backend Applications](01-rust/30-arcmutext-vs-arcrwlockt-in-backend-applications.md) · +follow-ups
31. [Interior Mutability (`Cell<T>` and `RefCell<T>`) in Rust](01-rust/31-interior-mutability-cellt-and-refcellt-in-rust.md) · +follow-ups
32. [`Cow<T>` (Copy-on-Write) in Rust](01-rust/32-cowt-copy-on-write-in-rust.md) · +follow-ups
33. [`PhantomData<T>` in Rust](01-rust/33-phantomdatat-in-rust.md) · +follow-ups
34. [`Clone` vs `Copy` in Rust](01-rust/34-clone-vs-copy.md) · +follow-ups
35. [unwrap() vs ?](01-rust/35-unwrap-vs.md)
36. [Arc](01-rust/36-arc.md)
37. [Arc vs Rc](01-rust/37-arc-vs-rc.md)
38. [What is Send?](01-rust/38-what-is-send.md)
39. [What is Sync?](01-rust/39-what-is-sync.md)
40. [Traits](01-rust/40-traits.md)
41. [What is Generics?](01-rust/41-what-is-generics.md)
42. [Error Handling](01-rust/42-error-handling.md)
43. [backpressure](01-rust/43-backpressure.md)
44. [What is lock contention?](01-rust/44-what-is-lock-contention.md)
45. [Send and Sync in depth](01-rust/45-send-and-sync-in-depth.md)
46. [Describe a backend architecture you have built](01-rust/46-describe-a-backend-architecture-you-have-built.md)
47. [How does the Rust borrow checker work internally?](01-rust/47-how-does-the-rust-borrow-checker-work-internally.md)
48. [What exactly happens when `rustc` compiles a program?](01-rust/48-what-exactly-happens-when-rustc-compiles-a-program.md)
49. [MIR](01-rust/49-mir.md)
50. [What is a self-referential struct, and why is it difficult?](01-rust/50-what-is-a-self-referential-struct-and-why-is-it-difficult.md)
51. [the Waker](01-rust/51-the-waker.md)
52. [What is the ABA problem?](01-rust/52-what-is-the-aba-problem.md)
53. [What memory ordering would you use for atomics?](01-rust/53-what-memory-ordering-would-you-use-for-atomics.md)
54. [cache line false sharing](01-rust/54-cache-line-false-sharing.md)
55. [What is work stealing?](01-rust/55-what-is-work-stealing.md)
56. [What happens when you call `Arc::clone()`?](01-rust/56-what-happens-when-you-call-arc-clone.md)
57. [How would you build a database connection pool from scratch?](01-rust/57-how-would-you-build-a-database-connection-pool-from-scratch.md)
58. [What is hazard pointer memory reclamation?](01-rust/58-what-is-hazard-pointer-memory-reclamation.md)
59. [What are epoch-based garbage collection techniques?](01-rust/59-what-are-epoch-based-garbage-collection-techniques.md)
60. [database sharding](01-rust/60-database-sharding.md)

## Backend

1. [Current Project Architecture](02-backend/01-project-architecture.md) · +follow-ups
2. [Tell me about yourself](02-backend/02-tell-me-about-yourself.md)
3. [Why Rust?](02-backend/03-why-rust.md)
4. [REST API](02-backend/04-rest-api.md)
5. [PUT vs PATCH](02-backend/05-put-vs-patch.md)
6. [Middleware](02-backend/06-middleware.md)
7. [Transactions](02-backend/07-transactions.md)
8. [Pagination](02-backend/08-pagination.md)
9. [Microservices vs Monolith](02-backend/09-microservices-vs-monolith.md)
10. [Horizontal vs Vertical Scaling](02-backend/10-horizontal-vs-vertical-scaling.md)
11. [Caching Strategy](02-backend/11-caching-strategy.md)
12. [Logging](02-backend/12-logging.md)
13. [Tell me about your backend project](02-backend/13-tell-me-about-your-backend-project.md)
14. [Why is Rust faster than Go or Java?](02-backend/14-why-is-rust-faster-than-go-or-java.md)
15. [What happens when you call `.await`?](02-backend/15-what-happens-when-you-call-await.md)
16. [What causes deadlocks?](02-backend/16-what-causes-deadlocks.md)
17. [channels in Rust](02-backend/17-channels-in-rust.md)
18. [mpsc vs broadcast channel](02-backend/18-mpsc-vs-broadcast-channel.md)
19. [connection pooling](02-backend/19-connection-pooling.md)
20. [Why use sqlx instead of Diesel?](02-backend/20-why-use-sqlx-instead-of-diesel.md)
21. [optimistic locking](02-backend/21-optimistic-locking.md)
22. [pessimistic locking](02-backend/22-pessimistic-locking.md)
23. [consumer groups](02-backend/23-consumer-groups.md)
24. [What is exactly-once processing?](02-backend/24-what-is-exactly-once-processing.md)
25. [How do you secure REST APIs?](02-backend/25-how-do-you-secure-rest-apis.md)
26. [zero-copy in Rust](02-backend/26-zero-copy-in-rust.md)
27. [Memory leak in Rust—is it possible?](02-backend/27-memory-leak-in-rust-is-it-possible.md)
28. [How do you profile Rust applications?](02-backend/28-how-do-you-profile-rust-applications.md)
29. [tracing](02-backend/29-tracing.md)
30. [Why doesn't Rust have a garbage collector?](02-backend/30-why-doesnt-rust-have-a-garbage-collector.md)
31. [lock-free programming](02-backend/31-lock-free-programming.md)
32. [How do you reduce API latency from 800ms to 50ms?](02-backend/32-how-do-you-reduce-api-latency-from-800ms-to-50ms.md)
33. [How do you prevent duplicate payment requests?](02-backend/33-how-do-you-prevent-duplicate-payment-requests.md)
34. [optimistic vs pessimistic locking](02-backend/34-optimistic-vs-pessimistic-locking.md)
35. [How do you guarantee exactly-once processing?](02-backend/35-how-do-you-guarantee-exactly-once-processing.md)
36. [replication](02-backend/36-replication.md)
37. [partitioning](02-backend/37-partitioning.md)
38. [Why is your query slow?](02-backend/38-why-is-your-query-slow.md)

## DSA

1. [LRU Cache](03-dsa/01-lru-cache.md)
2. [Top K Frequent Elements](03-dsa/02-top-k-frequent-elements.md)
3. [Merge K Sorted Lists](03-dsa/03-merge-k-sorted-lists.md)
4. [Detect Cycle in Graph](03-dsa/04-detect-cycle-in-graph.md)
5. [Shortest Path](03-dsa/05-shortest-path.md)
6. [Find Strongly Connected Components](03-dsa/06-find-strongly-connected-components.md)
7. [Bloom Filter](03-dsa/07-bloom-filter.md)
8. [Skip List](03-dsa/08-skip-list.md)
9. [B-Tree vs B+ Tree](03-dsa/09-b-tree-vs-b-tree.md)
10. [Time Complexity of HashMap](03-dsa/10-time-complexity-of-hashmap.md)

## System Design

1. [Rate limiting implementation](04-system-design/01-rate-limiting-implementation.md)
2. [How would you design a high-performance API?](04-system-design/02-how-would-you-design-a-high-performance-api.md)
3. [Design an API handling 1 million requests per second](04-system-design/03-design-an-api-handling-1-million-requests-per-second.md)
4. [How would you design a distributed rate limiter?](04-system-design/04-how-would-you-design-a-distributed-rate-limiter.md)
5. [How would you design a backend that handles 10 million requests/day?](04-system-design/05-how-would-you-design-a-backend-that-handles-10-million-requests-day.md)
6. [How would you design a notification service?](04-system-design/06-how-would-you-design-a-notification-service.md)
7. [Design WhatsApp](04-system-design/07-design-whatsapp.md)
8. [Design Uber](04-system-design/08-design-uber.md)
9. [Design YouTube](04-system-design/09-design-youtube.md)
10. [Design URL Shortener](04-system-design/10-design-url-shortener.md)
11. [Design a distributed cache](04-system-design/11-design-a-distributed-cache.md)
12. [Design Rate Limiter](04-system-design/12-design-rate-limiter.md)
13. [Design an LRU Cache in Rust](04-system-design/13-design-an-lru-cache-in-rust.md)

## Distributed Systems

1. [CAP theorem](05-distributed-systems/01-cap-theorem.md)
2. [Eventual consistency](05-distributed-systems/02-eventual-consistency.md)
3. [What is idempotency?](05-distributed-systems/03-what-is-idempotency.md)
4. [How do you implement idempotency?](05-distributed-systems/04-how-do-you-implement-idempotency.md)
5. [the Outbox Pattern](05-distributed-systems/05-the-outbox-pattern.md)
6. [CQRS](05-distributed-systems/06-cqrs.md)
7. [Saga Pattern](05-distributed-systems/07-saga-pattern.md)
8. [What is a circuit breaker?](05-distributed-systems/08-what-is-a-circuit-breaker.md)
9. [Raft](05-distributed-systems/09-raft.md)
10. [Paxos vs Raft](05-distributed-systems/10-paxos-vs-raft.md)
11. [What is split-brain?](05-distributed-systems/11-what-is-split-brain.md)
12. [How would you design a distributed lock?](05-distributed-systems/12-how-would-you-design-a-distributed-lock.md)
13. [vector clocks](05-distributed-systems/13-vector-clocks.md)
14. [Outbox Pattern](05-distributed-systems/14-outbox-pattern.md)
15. [Event Sourcing](05-distributed-systems/15-event-sourcing.md)
16. [Distributed Lock](05-distributed-systems/16-distributed-lock.md)
17. [consistent hashing](05-distributed-systems/17-consistent-hashing.md)

## PostgreSQL

1. [PostgreSQL Index](06-postgresql/01-postgresql-index.md)
2. [SQL JOIN Types](06-postgresql/02-sql-join-types.md)
3. [How do you optimize PostgreSQL queries?](06-postgresql/03-how-do-you-optimize-postgresql-queries.md)
4. [What is MVCC?](06-postgresql/04-what-is-mvcc.md)
5. [MVCC](06-postgresql/05-mvcc.md)

## Redis

1. [Why Redis?](07-redis/01-why-redis.md)
2. [Redis vs PostgreSQL](07-redis/02-redis-vs-postgresql.md)

## Kafka

1. [Kafka](08-kafka/01-kafka.md)
2. [Event-Driven Architecture](08-kafka/02-event-driven-architecture.md)
3. [Why Kafka instead of RabbitMQ?](08-kafka/03-why-kafka-instead-of-rabbitmq.md)
4. [Why Kafka over RabbitMQ?](08-kafka/04-why-kafka-over-rabbitmq.md)

## Docker

1. [Docker](09-docker/01-docker.md)

## Networking

1. [Nginx](13-networking/01-nginx.md)
2. [epoll, kqueue, and IOCP](13-networking/02-epoll-kqueue-and-iocp.md)

## Security

1. [Authentication](14-security/01-authentication.md)
2. [Authorization](14-security/02-authorization.md)
3. [RBAC](14-security/03-rbac.md)

---

**Total: 155 topics**