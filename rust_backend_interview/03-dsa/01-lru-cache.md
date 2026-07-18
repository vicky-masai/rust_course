# LRU Cache

## Interview Question

Design and implement an LRU (Least Recently Used) Cache that supports `get` and `put` operations in O(1) time complexity.

## Interview Answer

An LRU Cache is implemented using a HashMap combined with a Doubly Linked List. The HashMap provides O(1) key lookup to nodes in the linked list, while the doubly linked list maintains access order — the most recently used item is at the head, and the least recently used is at the tail. On every `get` or `put`, the accessed node is moved to the head. When the cache exceeds capacity, the tail node (least recently used) is evicted.

**Time Complexity**
- Get: O(1)
- Put: O(1)

**Space Complexity**: O(n) where n is the capacity of the cache.

---

## Follow-up Questions & Answers

### Q1. Why do we need a Doubly Linked List instead of a Singly Linked List?

**Interview Answer**

A Doubly Linked List is required because we need to remove a node from any position in O(1) time when it is accessed. With a singly linked list, removal requires knowing the previous node, which takes O(n) time to traverse. A doubly linked list stores both `prev` and `next` pointers, so given a reference to any node, we can remove it in constant time by updating its neighbors' pointers directly.

---

### Q2. How would you implement an LRU Cache in Rust?

**Interview Answer**

In Rust, you can use a `HashMap<K, Rc<RefCell<DoublyLinkedListNode>>>` to map keys to nodes, and a custom doubly linked list struct. However, due to Rust's ownership model, a common approach is to use indices into a `Vec<Node>` instead of raw pointers. The `lru` crate provides a production-ready implementation. A simplified version stores entries in a `Vec` and a `HashMap<K, usize>` where the value is the index into the vec, with a free-list for reuse.

```rust
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    key: i32,
    value: i32,
    prev: Option<usize>,
    next: Option<usize>,
}

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, usize>,
    nodes: Vec<Node>,
    head: Option<usize>,
    tail: Option<usize>,
    free: Vec<usize>,
}
```

---

### Q3. What are real-world use cases of LRU Cache in backend systems?

**Interview Answer**

LRU Caches are widely used in backend systems. **Redis** uses an LRU-like eviction policy (`allkeys-lru`, `volatile-lru`) as its default maxmemory policy. **CPU caches** use LRU to decide which cache lines to evict. **Database buffer pools** (like PostgreSQL's shared buffers) use LRU to manage pages in memory. **CDNs** use LRU to keep the most frequently requested content in cache. **DNS resolvers** cache domain lookups using LRU eviction.

---

### Q4. What is the difference between LRU, LFU, and FIFO eviction policies?

**Interview Answer**

**LRU** (Least Recently Used) evicts the item that was accessed least recently, assuming recent access predicts future access. **LFU** (Least Frequently Used) evicts the item with the lowest access count, favoring frequently accessed items but suffering from stale entries. **FIFO** (First In First Out) evicts the oldest inserted item regardless of access patterns. LRU is the most common default in caches (Redis, Memcached), while LFU is better for workloads with clear hot/cold data separation. Redis 4.0+ added LFU support via `volatile-lfu` and `allkeys-lfu`.

---

### Q5. What is the time and space complexity of LRU Cache?

**Interview Answer**

Both `get` and `put` operations are **O(1)** time complexity — HashMap lookup is O(1) average case, and doubly linked list insertion/deletion at known positions is O(1). The space complexity is **O(n)** where n is the capacity of the cache, since we store at most `capacity` entries in both the HashMap and the linked list. In Rust, each entry also carries the overhead of the node struct (key, value, prev, next pointers), so the constant factor is higher than a simple HashMap.

---

### Q6. How does the LRU Cache handle hash collisions?

**Interview Answer**

The HashMap component of the LRU Cache handles collisions the same way a standard HashMap does — via chaining (linked lists at each bucket) or open addressing. In Rust's `std::collections::HashMap`, the default is robin-hood hashing with backward shift deletion. The LRU logic itself is agnostic to collisions because each key maps to exactly one node in the linked list. The worst case for `get` is still O(n) if all keys hash to the same bucket, but this is extremely rare in practice.

---

### Q7. What happens when the LRU Cache is at full capacity and we insert a new key?

**Interview Answer**

When the cache is at full capacity and a new key is inserted, the **tail node** of the doubly linked list (the least recently used item) is evicted. Its key is removed from the HashMap, and its memory is freed or recycled. The new key-value pair is then inserted at the **head** of the linked list and added to the HashMap. If the new key already exists in the cache, we simply update its value and move the node to the head without evicting anything.

---

### Q8. Can you implement an LRU Cache using only Rust's standard library?

**Interview Answer**

Yes, but it requires manual implementation of the doubly linked list since `std` does not provide one. You can use `VecDeque` for a simple queue-based approach, but for true O(1) arbitrary removal, you need a custom linked list with indices. A practical approach uses a `Vec<Option<Node>>` with a free-list for index reuse, a `HashMap<K, usize>` for key-to-index mapping, and `head`/`tail` indices for the LRU order. This avoids unsafe code while maintaining O(1) operations.

```rust
struct LRUCache {
    capacity: usize,
    map: HashMap<i32, usize>,
    entries: Vec<(i32, i32, Option<usize>, Option<usize>)>, // key, value, prev, next
    head: usize,
    tail: usize,
    free_list: Vec<usize>,
}
```

---

### Q9. How is LRU Cache different from a TTL-based cache?

**Interview Answer**

An **LRU Cache** evicts entries based on access patterns — the least recently accessed item is removed regardless of how long it has been stored. A **TTL-based cache** evicts entries after a fixed time-to-live duration expires, regardless of access frequency. LRU is memory-efficient and adapts to workload patterns, while TTL ensures data freshness. Many production systems combine both: Redis supports `maxmemory-policy` for LRU and `EXPIRE` commands for TTL. A common pattern is to use LRU for capacity management and TTL for data freshness guarantees.

---

### Q10. What are the thread-safety considerations when implementing LRU Cache?

**Interview Answer**

A standard LRU Cache is **not thread-safe** because concurrent `get` and `put` operations can corrupt the linked list pointers and HashMap state. In Rust, you can wrap it in `Mutex<LRUCache>` for coarse-grained locking, or use `RwLock` if reads significantly outnumber writes. For high-concurrency scenarios, a **concurrent LRU** can be built using lock-free data structures or sharded locking (partitioning the cache into segments). The `moka` crate in Rust provides a high-performance concurrent cache with LRU-like eviction. In production, distributed caches like Redis handle thread safety at the server level.
