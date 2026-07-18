# Union-Find (Disjoint Set Union)

## Interview Question

What is Union-Find and when would you use it in backend or system engineering?

## Interview Answer

Union-Find (Disjoint Set Union, DSU) is a data structure that tracks a set of elements partitioned into disjoint (non-overlapping) subsets. It supports two operations: **Find** (determine which set an element belongs to) and **Union** (merge two sets into one). With **path compression** and **union by rank**, both operations run in **O(α(n))** amortized time — effectively constant. Union-Find is essential for: cycle detection in undirected graphs, connected component tracking, network connectivity, percolation problems, and Kruskal's minimum spanning tree algorithm. In backend systems, it's used for group membership (chat groups, friend circles), social network analysis, and distributed system component tracking.

**Time Complexity**: O(α(n)) amortized per operation (α = inverse Ackermann)
**Space Complexity**: O(n)

---

## Follow-up Questions & Answers

### Q1. What are path compression and union by rank?

**Interview Answer**

**Path compression** flattens the tree during `find` by making every node on the path point directly to the root. This dramatically reduces future query times. Without it, the tree can become a long chain with O(n) find operations. **Union by rank** attaches the shorter tree under the taller tree during `union`, keeping the overall tree balanced. Without it, unions can create degenerate chains. Together, they ensure amortized O(α(n)) per operation. A simpler alternative to union by rank is **union by size** (attach smaller tree under larger). Without these optimizations, operations degrade to O(n) worst case.

---

### Q2. How would you implement Union-Find in Rust?

**Interview Answer**

```rust
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y { return false; }

        // union by rank
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
        true
    }

    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
```

Note `find` takes `&mut self` due to path compression modifying the parent array.

---

### Q3. How is Union-Find used in Kruskal's Minimum Spanning Tree?

**Interview Answer**

Kruskal's algorithm sorts all edges by weight, then processes them in ascending order. For each edge (u, v), it checks if u and v are already in the same set using Union-Find. If not, the edge is added to the MST and the sets are united. This ensures no cycles are formed (since adding an edge within the same set would create a cycle). With sorting, the total time is **O(E log E + E × α(V))** ≈ **O(E log E)**. In backend systems, MST is used for network design (connecting servers with minimum cable), clustering (single-linkage clustering), and circuit design.

---

### Q4. How is Union-Find used for connected components in social networks?

**Interview Answer**

In social networks, users are nodes and friendships are edges. Union-Find efficiently tracks friend circles (connected components). When a new friendship is formed, `union` merges two circles. `find` determines which circle a user belongs to. `connected` checks if two users are in the same circle. For Facebook's "People You May Know" feature, finding users in the same connected component but not directly connected requires additional bookkeeping (storing component members). The amortized O(α(n)) time means millions of friendship operations can be processed in real-time. In Rust, each user maps to an integer index, and Union-Find tracks the social graph structure.

---

### Q5. What is a weighted Union-Find and when is it used?

**Interview Answer**

A weighted Union-Find (also called Union-Find with distances) tracks the relationship (weight/distance) between each node and its parent. In addition to parent and rank, store a `weight` array where `weight[x]` is the relationship between x and `parent[x]`. The `find` operation accumulates weights along the path. This supports queries like "what is the relative weight between x and y?" Used in: **Differential constraints** (maintaining relative differences between variables), **online graph coloring** (detecting bipartiteness), and **network latency tracking** (querying relative latency between nodes). In distributed systems, weighted Union-Find tracks relative clock offsets between servers.

---

### Q6. How does Union-Find handle dynamic connectivity queries?

**Interview Answer**

Dynamic connectivity asks: after a series of edge additions and deletions, are two nodes connected? Union-Find handles **offline** dynamic connectivity (all queries known in advance) using a technique called **divide and conquer on segments** or **Euler tour tree**. For **online** dynamic connectivity (queries arrive one at a time), Union-Find alone isn't sufficient for deletions — you need a **link-cut tree** or **Euler tour tree**. However, for edge additions only (incremental connectivity), Union-Find is perfect. In backend systems, this is common: services are only added, never removed from a cluster without full recomputation.

---

### Q7. What is the difference between Union-Find and BFS/DFS for connectivity?

**Interview Answer**

**Union-Find** is optimal for incremental connectivity (adding edges one at a time with queries between). It handles each edge in O(α(n)) time and doesn't require storing the full graph. **BFS/DFS** is better when you need to enumerate all nodes in a component, find shortest paths, or process the full graph at once. BFS/DFS requires O(V + E) time per query. Union-Find is preferred for: streaming edge additions, Kruskal's MST, and real-time connectivity checks. BFS/DFS is preferred for: component enumeration, path finding, and when the graph fits in memory and is processed batch-wise.

---

### Q8. How is Union-Find used in image processing (percolation)?

**Interview Answer**

The percolation problem asks: in an N×N grid where each cell is open (1) or blocked (0), does a path exist from top to bottom through open cells? Union-Find solves this efficiently: create a virtual top node and virtual bottom node. For each open cell, union it with adjacent open cells and with the top/bottom virtual nodes if on the first/last row. Check if top and bottom are connected. This runs in O(N² × α(N²)) time. Used in: modeling fluid flow, network reliability analysis, and statistical physics simulations. In Rust, flatten the 2D grid to 1D indices (row * N + col) and use Union-Find on the flat array.

---

### Q9. Can Union-Find be made concurrent/thread-safe?

**Interview Answer**

A standard Union-Find is **not thread-safe** because `find` modifies the parent array via path compression. For concurrent use: **Sharded Union-Find** partitions elements into shards, each with its own Union-Find, with cross-shard unions using locks. **Read-heavy workloads** can use `RwLock<UnionFind>` for concurrent reads with exclusive writes. **Lock-free approaches** use atomic operations for parent updates, but path compression becomes complex. In practice, for distributed systems, each node maintains a local Union-Find and periodically synchronizes with others. The `concurrent-union-find` crate provides lock-free implementations using CAS operations.

---

### Q10. What is the space-time tradeoff in Union-Find implementations?

**Interview Answer**

**Space optimization**: Store parent and rank in `Vec<usize>` (8 bytes per node on 64-bit). Using `Vec<u8>` for rank saves space (rank is always small). Path compression can be made iterative instead of recursive to avoid stack overflow for deep trees. **Time optimization**: Path compression with union by rank gives O(α(n)) amortized. Without compression, O(log n) with union by rank only. Without either, O(n) worst case. **Persistent Union-Find** (supporting rollbacks) requires O(log n) per operation using a stack of modifications. In Rust, the `union-find` crate provides both compact and high-performance variants. For very large datasets (billions of elements), consider sharding across memory regions.
