# B-Tree vs B+ Tree

## Interview Question

What is the difference between a B-Tree and a B+ Tree, and why do databases prefer B+ Trees?

## Interview Answer

A **B-Tree** stores key-value pairs in both internal and leaf nodes, with each node containing keys and pointers to children. A **B+ Tree** stores keys only in leaf nodes (which form a linked list), while internal nodes store only copy of keys for routing. Databases prefer **B+ Trees** because: **1)** Leaf nodes form a linked list enabling efficient range queries. **2)** Internal nodes are smaller (no values), fitting more keys per page and reducing tree height. **3)** All data access goes through leaves, providing consistent O(log n) performance. B+ Trees are used in **MySQL InnoDB**, **PostgreSQL**, and **SQLite** index structures.

---

## Follow-up Questions & Answers

### Q1. What is the exact structural difference between B-Tree and B+ Tree?

**Interview Answer**

In a **B-Tree** of order m: each node has at most m children and m-1 keys. Internal nodes store (key, value, child_pointer) triples. Searching can terminate at any node. In a **B+ Tree** of order m: internal nodes store (key, child_pointer) pairs — no values. All values are in leaf nodes. Leaf nodes are linked together in a sorted linked list. Internal nodes act purely as a routing index. The B+ Tree has two types of nodes, while B-Tree nodes are uniform. B+ Tree leaf nodes typically store more entries because internal nodes don't waste space on values.

---

### Q2. How does the linked list in B+ Tree leaf nodes enable efficient range queries?

**Interview Answer**

In a B+ Tree, after finding the first leaf node that matches the start of a range, we can simply follow the leaf-level linked list to iterate through all subsequent entries in sorted order. This avoids traversing back up and down the tree for each range element. For example, `SELECT * FROM users WHERE age BETWEEN 20 AND 30` — the database finds the leaf containing age=20, then scans forward through the linked list until age>30. This is O(log n + k) where k is the number of results. In a B-Tree, range queries require in-order traversal, which involves more random access patterns. The sequential leaf traversal also enables efficient disk I/O with sequential reads.

---

### Q3. Why are B+ Trees preferred for database indexing?

**Interview Answer**

B+ Trees are preferred because: **1) Higher branching factor** — internal nodes don't store values, so more keys fit per disk page (e.g., 4KB page), reducing tree height. A B+ Tree of order 1000 with height 3 can index billions of records. **2) Predictable performance** — all queries traverse the same path from root to leaf, giving consistent O(log n) behavior. **3) Sequential leaf access** — range queries and full scans are efficient due to leaf linking. **4) Better cache utilization** — smaller internal nodes mean more fit in CPU cache. **5) Simpler concurrency** — locking individual pages during split/merge is straightforward. MySQL InnoDB uses 16KB pages with B+ Tree indexes, achieving 3-4 levels for billions of rows.

---

### Q4. How does a B+ Tree handle node splits and merges?

**Interview Answer**

When a leaf node overflows during insertion, it splits into two nodes — the first half stays, the second half moves to a new node, and the median key is copied up to the parent. If the parent overflows, it splits recursively up to the root. If the root splits, a new root is created with one key, increasing tree height by 1. During deletion, if a node falls below the minimum occupancy (⌈m/2⌉ for leaves, ⌈m/2⌉ for internal nodes), it borrows from a sibling (rotation) or merges with a sibling. Merging propagates up if the parent loses a key. In Rust, you'd implement this with a `BTreeMap<K, V>` (std provides a B-Tree map) or a custom page-oriented implementation for disk-based trees.

---

### Q5. What is the time and space complexity of B+ Tree operations?

**Interview Answer**

**Search**: O(log_m n) — where m is the order (branching factor) and n is the number of entries. For a B+ Tree of order 1000 with 1 billion entries, the height is just 3. **Insert**: O(log_m n) — find the leaf, insert, possibly split up to root. **Delete**: O(log_m n) — find the leaf, delete, possibly merge up to root. **Range query**: O(log_m n + k) — find start leaf, scan k entries via linked list. **Space**: O(n) — each entry stored exactly once in leaf nodes, plus internal node overhead. The B+ Tree's high branching factor means the tree is very shallow, minimizing disk I/O. With 16KB pages, a B+ Tree can achieve 500-1000 entries per node.

---

### Q6. How do B+ Trees relate to real-world database systems?

**Interview Answer**

**MySQL InnoDB** — uses B+ Tree for primary key indexes and secondary indexes. Leaf pages contain the actual row data (clustered index) or primary key pointers (secondary index). Page size is 16KB. **PostgreSQL** — uses B+ Tree (called "BTREE") as the default index type. Leaf pages store (key, ctid) pairs pointing to heap tuples. **SQLite** — uses B+ Tree for both tables (row storage) and indexes. **Oracle** — uses B-Tree variants (called "balanced tree indexes"). **MongoDB** — WiredTiger storage engine uses B+ Tree for indexes. In all cases, the B+ Tree's sequential leaf access enables efficient `ORDER BY`, `RANGE`, and `JOIN` operations.

---

### Q7. What are the differences between clustered and non-clustered B+ Tree indexes?

**Interview Answer**

In a **clustered index** (MySQL InnoDB primary key), the leaf nodes contain the actual row data — the table IS the B+ Tree sorted by the primary key. Only one clustered index exists per table. In a **non-clustered index** (secondary index), leaf nodes contain (indexed_column, primary_key) pairs — an extra lookup to the clustered index is needed to fetch the full row. Clustered indexes provide faster range scans and sequential access because rows are physically stored in index order. Non-clustered indexes are useful for filtering on non-primary columns but incur an extra I/O for the row lookup. PostgreSQL uses heap-organized tables by default, with all indexes being non-clustered.

---

### B8. How would you implement a simplified B+ Tree in Rust?

**Interview Answer**

```rust
const MAX_KEYS: usize = 3;

enum BPlusNode {
    Internal {
        keys: Vec<i32>,
        children: Vec<Box<BPlusNode>>,
    },
    Leaf {
        keys: Vec<i32>,
        values: Vec<String>,
        next: Option<Box<BPlusNode>>,
    },
}

struct BPlusTree {
    root: BPlusNode,
    order: usize,
}

impl BPlusTree {
    fn new(order: usize) -> Self {
        BPlusTree {
            root: BPlusNode::Leaf {
                keys: Vec::new(),
                values: Vec::new(),
                next: None,
            },
            order,
        }
    }

    fn search(&self, key: i32) -> Option<String> {
        match &self.root {
            BPlusNode::Leaf { keys, values, .. } => {
                keys.iter().position(|&k| k == key)
                    .map(|i| values[i].clone())
            }
            BPlusNode::Internal { keys, children } => {
                let idx = keys.iter().position(|&k| k > key).unwrap_or(keys.len());
                children[idx].search(key)
            }
        }
    }
}
```

This simplified version demonstrates the key structural difference: internal nodes route to children, leaf nodes store values with a `next` pointer for linked-list traversal.

---

### Q9. What is the difference between B+ Tree and LSM Tree?

**Interview Answer**

**B+ Tree** is an update-in-place structure — writes modify existing pages. **LSM Tree** (Log-Structured Merge Tree) is an append-only structure — writes go to an in-memory buffer (memtable), then flush to sorted disk files (SSTables) that are periodically compacted. B+ Trees have better read performance (single tree traversal) but slower writes (random I/O, page splits). LSM Trees have faster writes (sequential I/O) but slower reads (may check multiple levels). LSM Trees are used in **RocksDB**, **Cassandra**, **LevelDB** — write-heavy workloads. B+ Trees are used in **MySQL**, **PostgreSQL** — balanced read/write workloads. In Rust, `sled` implements an LSM-like structure, while `reedbloom` implements B+ Tree-like indexing.

---

### Q10. When would you choose a B-Tree over a B+ Tree?

**Interview Answer**

Choose a **B-Tree** when: **1)** You need fast point queries and rarely do range queries — B-Tree can terminate early if the value is found in an internal node. **2)** You want to store different-sized values efficiently — B-Tree nodes can hold variable-length data inline. **3)** You're implementing a general-purpose sorted map (like Rust's `std::collections::BTreeMap`). Choose a **B+ Tree** when: **1)** Range queries are common (database indexes). **2)** You need predictable O(log n) performance (all access through leaves). **3)** Sequential disk I/O is important (leaf linking enables sequential reads). **4)** You want higher branching factor (smaller internal nodes). For most database indexing use cases, B+ Tree is the better choice.
