# LEVEL 04 — Memory Management

---

## Smart Pointers

### 0143. Box

Heap allocation of a single value. Owns `T`, frees on drop. Puts large data on the heap, enables recursive types (`Box<Node>`), and is the simplest owned pointer.

**Talk track:** *"Box is owned heap memory for one value — recursive types and big moves go here."*

---

### 0144. Rc

Reference-counted shared ownership for single-threaded use. Clone bumps count; last drop frees. Not `Send`/`Sync` — don't share across threads.

Cycles leak unless you break them with `Weak`.

**Talk track:** *"Rc is shared ownership on one thread — cycles need Weak."*

---

### 0145. Arc

Atomic reference count — like `Rc` but thread-safe. The standard way to share owned data across threads/tasks. Cloning is cheap (bump counter); data itself isn't copied.

Pair with `Mutex`/`RwLock` for shared mutation.

**Talk track:** *"Arc is multi-threaded shared ownership — clone the pointer, not the data."*

---

### 0146. Weak

Non-owning reference into an `Rc`/`Arc` cycle graph. Doesn't keep the value alive; `upgrade()` may fail if already dropped.

Use for parent pointers in trees, caches, observers.

**Talk track:** *"Weak breaks cycles — you can try to upgrade, but the value may be gone."*

---

### 0147. Cell

Interior mutability for `Copy` types (and swap) with no runtime borrow flag — just replace values through `&Cell<T>`. Single-threaded.

Simple counters/flags behind shared refs.

**Talk track:** *"Cell lets you replace Copy values through a shared reference on one thread."*

---

### 0148. RefCell

Runtime-checked borrow rules for single-threaded shared mutation. `borrow()` / `borrow_mut()` panic (or return error variants) if rules violated.

Great for test doubles and graph structures; panic risk if logic wrong.

**Talk track:** *"RefCell moves borrow checking to runtime on one thread — panic if you alias mutably."*

---

### 0149. Mutex

Mutual exclusion lock: one thread at a time enters. Poisoning on panic (Rust std) surfaces crashed critical sections.

Hold locks briefly; don't await while holding a std mutex in async (use tokio's mutex or restructure).

**Talk track:** *"Mutex serializes access. Contended locks become your scalability ceiling."*

---

### 0150. RwLock

Many readers OR one writer. Good for read-heavy shared state. Writer starvation possible depending on implementation.

More overhead than Mutex when contention is low and writes are common — measure.

**Talk track:** *"RwLock optimizes read-mostly sharing — writers still need exclusivity."*

---

### 0151. OnceCell

Initialize exactly once, then immutable access. Lazy config, one-time computations without `mut` static hacks.

Std now has `OnceLock` as the thread-safe cousin naming evolved — know the "init once" pattern.

**Talk track:** *"OnceCell is write-once initialization — safe lazy globals without mut static UB."*

---

### 0152. LazyLock

Thread-safe lazy initialization of a value on first access (std). Perfect for global clients, compiled regexes, config.

**Talk track:** *"LazyLock builds global state on first use, safely across threads."*

---

### 0153. Atomic Types

`AtomicUsize`, `AtomicBool`, etc. — lock-free concurrent updates with memory orderings. Counters, flags, simple state machines.

Harder than Mutex to get right; easy to invent subtle bugs. Prefer Mutex until profiling needs atomics.

**Talk track:** *"Atomics are concurrent primitives with explicit memory ordering — power tools, not defaults."*

---

### 0154. Pin

(See also 0118.) Smart-pointer wrapper guaranteeing the pointee won't move. Needed for async self-referential futures. `Unpin` types can be moved out of Pin easily; most ordinary types are `Unpin`.

**Talk track:** *"Pin is an API contract: this address stays put."*

---

## Collections

### 0155. Vec

Growable contiguous array — Rust's default list. Amortized O(1) push, O(1) index. Reallocates when capacity exceeded (growth factor).

Prefer `Vec` over linked structures for almost everything in user space.

**Talk track:** *"Vec is the cache-friendly dynamic array — capacity management is its main cost."*

---

### 0156. VecDeque

Ring-buffer double-ended queue. Efficient push/pop front and back. Work queues, buffers, sliding structures.

**Talk track:** *"VecDeque is a ring buffer — fast at both ends."*

---

### 0157. HashMap

Hash table map (SipHash by default for HashDoS resistance — can switch hasher for perf). Unordered. The everyday associative array.

**Talk track:** *"HashMap is average O(1) keyed storage — hasher and load factor shape real performance."*

---

### 0158. HashSet

Unique keys via hashing. Deduplicate and membership test.

**Talk track:** *"HashSet is uniqueness as a data structure."*

---

### 0159. BTreeMap

Ordered map via B-tree. Range queries, sorted iteration, predictable performance. Slightly slower than HashMap for pure point lookups typically.

**Talk track:** *"BTreeMap when you need ordered keys and ranges, not just get/put."*

---

### 0160. BTreeSet

Ordered set. Sorted unique elements, range scans over values.

**Talk track:** *"BTreeSet is sorted uniqueness."*

---

### 0161. BinaryHeap

Max-heap priority queue (peek largest). Use `Reverse` for min-heap behavior. Scheduling, Dijkstra, top-k.

**Talk track:** *"BinaryHeap pops the highest priority item in log n."*

---

### 0162. Hash Algorithms

Turn keys into fixed-size bits for tables and integrity. Cryptographic hashes (SHA-256) ≠ fast table hashes (ahash, fxhash). SipHash balances DOS resistance vs speed for HashMap.

Never use a non-crypto hash for security tokens.

**Talk track:** *"Pick hash function by threat model: DoS resistance, crypto strength, or raw speed."*

---

### 0163. Allocator

The component that hands out heap memory. Global allocator can be swapped (jemalloc, mimalloc) for different fragmentation/perf profiles. Custom allocators for arenas/pools.

**Talk track:** *"The allocator is the heap's policy engine — swapping it can change latency tails."*

---

### 0164. Memory Allocation

The act of obtaining heap memory. Cost: time, contention, fragmentation. Strategies: pooling, arenas (bump allocate then free all), preallocation (`Vec::with_capacity`).

Staff habit: allocation in hot paths shows up in profiles — remove or batch them.

**Talk track:** *"Allocation is a system call path into the allocator — hot loops should reuse buffers."*
