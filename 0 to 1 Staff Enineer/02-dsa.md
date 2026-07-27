# LEVEL 02 — Data Structures & Algorithms

---

## Data Structures

### 0068. Arrays

Contiguous elements in memory, indexed by position in O(1). Best cache locality of the common structures. Fixed size in some languages; Rust's `[T; N]` is fixed, `Vec` is growable array.

Use when you need indexed access and scanning. Insert/delete in the middle is O(n) because elements shift.

**Talk track:** *"Arrays win on locality and indexing. Pay with expensive middle inserts and a fixed logical model."*

---

### 0069. Strings

Sequences of characters — but encoding matters (UTF-8 in Rust). Byte length ≠ character length. Manipulation can allocate.

Hot path tip: avoid repeated concatenation that reallocates; use builders. Validate at boundaries; treat strings as hostile user input in APIs.

**Talk track:** *"Strings are data plus encoding rules. In Rust, UTF-8 safety is part of the type story."*

---

### 0070. Linked List

Nodes with pointers to next (and maybe prev). O(1) insert/delete if you hold the node pointer; O(n) scan; poor cache locality.

Rarely the right default in modern systems — `Vec` usually wins. Still taught for pointer reasoning and interview basics; used inside some OS/allocator structures.

**Talk track:** *"Linked lists trade locality for flexible splicing. In app code, prefer contiguous storage unless profiling says otherwise."*

---

### 0071. Stack

LIFO: push/pop from one end. Function call stacks, undo history, DFS, expression evaluation, parsing.

Simple invariant, easy to reason about. Implement with `Vec`.

**Talk track:** *"Stacks model nested work — last problem you opened is first you close."*

---

### 0072. Queue

FIFO: enqueue back, dequeue front. Request buffering, BFS, fair processing, rate-limited pipelines.

Bounded queues add backpressure — when full, producers block or drop. That's a production feature, not just an interview topic.

**Talk track:** *"Queues absorb rate differences between producers and consumers. Bounds create backpressure."*

---

### 0073. Deque

Double-ended queue — push/pop both ends efficiently. Sliding window maximums, work stealing queues, palindrome checks, certain scheduling structures.

Rust: `VecDeque`.

**Talk track:** *"Deque is a queue that can act on both ends — useful for windowing and work-stealing patterns."*

---

### 0074. HashMap

Key → value with average O(1) lookup via hashing. Backbone of indexes, caches, counters, language maps.

Worst case O(n) if hashing collapses (attacks) — use good hashers. Iteration order is usually unordered. Load factor triggers resize.

**Talk track:** *"HashMaps are the workhorse index. Watch hash quality, memory overhead, and resize spikes."*

---

### 0075. HashSet

HashMap without values — unique membership tests. Deduping, visited sets in graphs, allow/deny lists.

Same hashing/resize tradeoffs as HashMap.

**Talk track:** *"HashSet answers 'have I seen this?' fast."*

---

### 0076. Heap

Binary heap: parent ordered vs children (min or max). O(log n) push/pop, O(1) peek extreme. Priority queues, Dijkstra, scheduling by priority.

Not a sorted structure for arbitrary queries — only efficient for the top element.

**Talk track:** *"Heaps give you the next-most-important item quickly — priority queues in practice."*

---

### 0077. Trie

Tree of prefixes (often characters). Autocomplete, routing tables, dictionary search. Memory heavy if naive; compressed tries exist.

Great when shared prefixes dominate.

**Talk track:** *"Tries index by prefix — perfect for autocomplete-style problems."*

---

### 0078. Tree

Hierarchical nodes with parent/child links. File systems, UI, org charts, syntax trees, decision trees. Shape determines performance (balanced vs skewed).

Many "tree problems" are recursion + invariants.

**Talk track:** *"Trees model hierarchy. Balance and branching factor decide whether operations stay fast."*

---

### 0079. BST

Binary Search Tree: left < node < right. Average O(log n) ops if balanced; degrades to O(n) if skewed (sorted inserts).

Foundation for ordered maps. In practice use self-balancing variants.

**Talk track:** *"BSTs give ordered key search — but only if you keep them balanced."*

---

### 0080. AVL Tree

Self-balancing BST using height differences. Stricter balance than red-black → slightly faster lookups, slower inserts/deletes.

Used when lookups dominate and you need guaranteed log n.

**Talk track:** *"AVL is aggressively balanced — read-heavy ordered maps like it."*

---

### 0081. Red Black Tree

Self-balancing BST with color invariants; looser balance than AVL → often faster modifications. Java `TreeMap`, C++ `std::map` historically, many OS structures.

Rust's `BTreeMap` is a B-tree, not RB — similar role (ordered map).

**Talk track:** *"Red-black trees power classic ordered maps with good update performance."*

---

### 0082. B Tree

Multi-way balanced tree optimized for block storage — many keys per node to match disk pages. Databases and filesystems live here.

Fewer I/O ops than binary trees on disk because fanout is high.

**Talk track:** *"B-trees are how databases index disk — high fanout to minimize page reads."*

---

### 0083. B+ Tree

B-tree variant where all values sit in leaves linked for range scans. Dominant in DB indexes (PostgreSQL-style B-trees are B+ family ideas).

Point lookup + range query friendly.

**Talk track:** *"B+ trees make range scans sequential at the leaf level — why SQL indexes range well."*

---

### 0084. Graph

Nodes + edges (directed/undirected, weighted/unweighted). Networks, dependencies, social graphs, routing, microservice call graphs.

Represent as adjacency list (common) or matrix (dense). Algorithms depend on representation.

**Talk track:** *"Graphs model relationships. Most system dependency problems are graph problems in disguise."*

---

## Algorithms

### 0085. Sorting

Ordering elements — many algorithms (quick, merge, heap, radix). Libraries use hybrid tuned sorts. Stability matters when equal keys must keep relative order.

Know costs: comparison sorts Ω(n log n) worst-case lower bound generally; exploit structure (integers) for linear sorts.

**Talk track:** *"Sorting enables binary search, merging, and human-readable output. Use std sorts unless you know a special case."*

---

### 0086. Searching

Find an element in a collection. Linear scan vs indexed structures vs binary search on sorted data. Choose based on frequency, size, and mutability.

In systems: search often means "which index do I need?"

**Talk track:** *"Searching is an index problem first, an algorithm problem second."*

---

### 0087. Binary Search

Halve a sorted search space each step — O(log n). Easy to get off-by-one wrong. Also used on answer spaces ("binary search the minimum feasible capacity").

Invariant thinking: what range still might hold the answer?

**Talk track:** *"Binary search needs sorted order or monotonic predicates. The hard part is the invariant, not the mid index."*

---

### 0088. DFS

Depth-First Search — go deep, backtrack. Recursion or explicit stack. Detect cycles, topological prep, path existence, maze solving.

Can blow recursion stack on deep graphs — use iterative form carefully.

**Talk track:** *"DFS explores paths deeply — natural for path/cycle structure."*

---

### 0089. BFS

Breadth-First Search — layer by layer with a queue. Shortest path in unweighted graphs, level-order traversal, nearest source problems.

Memory can be large on bushy graphs (queue holds a whole frontier).

**Talk track:** *"BFS finds shortest paths when every edge weighs the same."*

---

### 0090. Sliding Window

Maintain a window [left, right] over a sequence to find subarrays/substrings meeting a constraint in O(n). Classic: longest substring without repeats, max sum of size k.

Two pointers move forward only — amortized linear.

**Talk track:** *"Sliding window turns many O(n²) subarray scans into O(n) with a moving range."*

---

### 0091. Two Pointer

Two indices walking a structure (often from ends or same direction) to find pairs/partitions. Sorted arrays make this powerful.

Related to sliding window but also pair-sum, partitioning, palindrome checks.

**Talk track:** *"Two pointers exploit order so you don't restart scanning from scratch."*

---

### 0092. Prefix Sum

Precompute cumulative sums so range sum queries become O(1): `sum(l..r) = pref[r] - pref[l-1]`. Extend to difference arrays for range updates.

Common in contests and analytics pipelines.

**Talk track:** *"Prefix sums trade an O(n) preprocess for fast range sums."*

---

### 0093. Greedy

Build solution choice-by-choice using a local rule that proves globally optimal for specific problems (activity selection, Huffman, Dijkstra with non-negative weights).

Wrong on many problems — need a proof or known pattern. Interviewers listen for "why greedy is safe here."

**Talk track:** *"Greedy works only when a local choice never blocks a better global answer — prove it."*

---

### 0094. Backtracking

Try a choice, recurse, undo (take/leave). N-queens, sudokus, subset generation, constraint search.

Prune early when a partial solution can't succeed. Exponential worst case — structure and pruning save you.

**Talk track:** *"Backtracking is disciplined trial and error with undo — prune or die exponentially."*

---

### 0095. Dynamic Programming

Break problems into overlapping subproblems; store answers (memo/table). Optimal substructure required. Knapsack, LCS, path counts, many interview classics.

Staff angle: DP appears in pricing engines, routers, and parsers — not only interviews. Identify state → transition → base case.

**Talk track:** *"DP is recursion + caching with a defined state. Name the state clearly and you're halfway there."*

---

### 0096. Union Find

Disjoint Set Union (DSU): track which elements share a component. Nearly O(1) with path compression + union by rank. Kruskal MST, connectivity, friend circles, cycle detection in undirected graphs.

**Talk track:** *"Union-find answers 'are these in the same set?' while sets merge — connectivity workhorse."*

---

### 0097. Topological Sort

Order nodes in a DAG so every edge goes forward. Build order for compile deps, job scheduling, course prerequisites.

Cycle ⇒ impossible — detect via Kahn (BFS indegrees) or DFS colors.

**Talk track:** *"Topo sort schedules dependent work. A cycle means your dependency graph is broken."*

---

### 0098. Dijkstra

Shortest paths from a source on non-negative weighted graphs using a priority queue. O((V+E) log V) typical.

Negative edges need Bellman-Ford. A* (0099) adds a heuristic for goal-directed search.

**Talk track:** *"Dijkstra is GPS math for non-negative costs — priority queue grows the frontier by best distance."*

---

### 0099. A*

Best-first search with cost `g + h` (distance so far + heuristic estimate to goal). Optimal if heuristic is admissible (never overestimates).

Game pathfinding, map routing, puzzle search.

**Talk track:** *"A* is Dijkstra aimed at a goal with a heuristic — same family, less wasted exploration."*

---

### 0100. Consistent Hashing

Hash ring so adding/removing a node only remaps a fraction of keys — critical for caches and sharded stores. Virtual nodes smooth imbalance.

Without it, modulo hashing reshuffles almost everything on cluster size change.

**Talk track:** *"Consistent hashing keeps remapping small when servers come and go — essential for distributed caches."*

---

### 0101. Bloom Filter

Probabilistic set: can say "definitely not present" or "maybe present." Tiny memory, false positives possible, false negatives no (in standard form).

Used to skip disk lookups (Cassandra, browsers, CDNs). Can't delete easily unless counting variant.

**Talk track:** *"Bloom filters trade a bit of wrong 'yes' for huge memory savings on negative checks."*

---

### 0102. HyperLogLog

Probabilistic cardinality — estimate unique counts with tiny memory (Redis `PFCOUNT`). Error is a few percent typically.

Perfect for "how many unique users today?" at scale without storing every id.

**Talk track:** *"HLL estimates uniques without storing uniques — analytics at stream scale."*

---

### 0103. Skip List

Probabilistic layered linked lists for O(log n) search. Alternative to balanced trees; used in Redis sorted sets internally (with tweaks), LevelDB-ish ideas.

Easier concurrent variants than some trees historically.

**Talk track:** *"Skip lists get log n search with simpler structure than some trees — probabilistic balancing."*

---

### 0104. Bit Manipulation

Operate on bits: masks, shifts, XOR tricks, flags packed into integers. Permissions, compact sets, CPU flags, bloom bit arrays, lock-free algorithms.

Powers of two, clearing lowest set bit, parity — useful tools.

**Talk track:** *"Bits are packing and atomic-friendly state. Flag words and masks show up everywhere in systems."*
