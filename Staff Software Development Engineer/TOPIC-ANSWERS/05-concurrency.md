# LEVEL 05 — Concurrency

### 0165. Threads

OS threads run code truly in parallel on multi-core machines (when scheduled). Each has a stack and scheduling cost. Rust requires `Send` data moved into threads.

Use for CPU-bound parallelism or blocking work. Don't spawn unbounded threads per request.

**Talk track:** *"Threads give real parallelism and real cost — bound them and share state carefully."*

---

### 0166. Thread Pools

Fixed (or bounded) set of worker threads pulling jobs from a queue. Cap resource use, reuse stacks, control concurrency.

Tokio's blocking pool, Rayon, and custom pools are variants of this idea.

**Talk track:** *"Thread pools replace 'spawn forever' with a bounded workforce."*

---

### 0167. Scoped Threads

Threads that must join before a scope ends — allowing them to borrow local data safely (`std::thread::scope`). No `'static` requirement for borrows.

**Talk track:** *"Scoped threads borrow locals safely because the scope joins before locals die."*

---

### 0168. Shared State

Multiple threads accessing the same data. Requires synchronization or immutability. Hardest concurrency model to get right — prefer channels when architecture allows.

**Talk track:** *"Shared state concurrency needs a clear ownership and locking story — or you'll race."*

---

### 0169. Arc + Mutex

Classic Rust shared mutable state: `Arc` shares the mutex; `Mutex` protects the data. Clone `Arc` across threads, lock to mutate.

Watch lock scope and deadlock (lock ordering).

**Talk track:** *"Arc\<Mutex\<T\>\> is the default shared-mutability pattern across threads."*

---

### 0170. Arc + RwLock

Same sharing, but read-optimized locking. Many concurrent readers; writers exclusive.

**Talk track:** *"Arc\<RwLock\<T\>\> when reads dominate and deserve parallelism."*

---

### 0171. Condvar

Condition variable: wait for a condition while releasing a mutex; another thread notifies. Classic "queue not empty" pattern.

Easy to misuse (spurious wakeups — always wait in a loop checking the condition).

**Talk track:** *"Condvar sleeps until signaled — always re-check the predicate in a loop."*

---

### 0172. Message Passing

Send data between threads/tasks via channels instead of sharing. Ownership moves with the message — Rust makes this natural.

Actor-style designs scale reasoning: no shared mutability, clearer failure boundaries.

**Talk track:** *"Don't communicate by sharing; share by communicating — channels move ownership."*

---

### 0173. Crossbeam

Crate ecosystem for advanced concurrency: scoped threads (historically), lock-free structures, channels. Used when std isn't enough.

**Talk track:** *"Crossbeam fills gaps: lock-free structures and richer concurrency utilities."*

---

### 0174. MPSC Channels

Multi-producer, single-consumer queues. Many workers send to one aggregator. Std `mpsc` and Tokio/crossbeam variants.

Backpressure depends on bounded vs unbounded channels — unbounded can OOM.

**Talk track:** *"MPSC fans in work to one consumer — bound the channel or risk memory blowups."*

---

### 0175. Lock-Free Programming

Algorithms using atomics/CAS instead of OS locks. Can reduce latency under contention; extremely hard to prove correct. ABA, memory ordering, and reclamation (GC/hazard pointers/epochs) haunt you.

Staff advice: don't go lock-free until locks are proven to be the bottleneck and you have experts/tests.

**Talk track:** *"Lock-free is not free — you pay with complexity and subtle correctness hazards."*

---

### 0176. Atomics

CPU-supported atomic read-modify-write operations. Building blocks for counters, flags, lock-free structures. Pair with memory orderings.

**Talk track:** *"Atomics are indivisible updates visible across cores under chosen orderings."*

---

### 0177. Memory Ordering

`Relaxed`, `Acquire`, `Release`, `AcqRel`, `SeqCst` — how much reordering you allow. Stronger = easier to reason, potentially slower.

Most app code should use Mutex (implicit safe ordering) or `SeqCst`/`AcqRel` when unsure; experts tune down.

**Talk track:** *"Ordering is the contract of what another thread is allowed to see and when."*

---

### 0178. Compare-And-Swap (CAS)

Atomic: "if value is still X, replace with Y." Retry loops on failure. Foundation of lock-free updates.

Contention causes CAS failures → livelock risk if poorly designed.

**Talk track:** *"CAS is optimistic concurrency — try update, retry if someone else won."*

---

### 0179. ABA Problem

CAS sees value A→…→A again and assumes nothing changed, but intermediate state did. Classic lock-free bug. Mitigations: tags/version counters, hazard pointers, immutable nodes.

**Talk track:** *"ABA is CAS fooling itself because the bit pattern returned — versions fix the illusion."*
