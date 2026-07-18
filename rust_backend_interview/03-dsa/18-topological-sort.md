# Topological Sort

## Interview Question

What is Topological Sort and when would you use it in backend systems?

## Interview Answer

Topological Sort linearizes a **Directed Acyclic Graph (DAG)** such that for every directed edge (u, v), vertex u comes before vertex v in the ordering. It only works on DAGs — if the graph has a cycle, no valid topological order exists. Two algorithms exist: **Kahn's Algorithm** (BFS-based) — repeatedly remove nodes with in-degree 0. **DFS-based** — perform DFS and add nodes to the front of the result list in post-order. Both run in **O(V + E)** time. Topological Sort is essential for: task scheduling, dependency resolution, build systems, course prerequisites, and compilation order.

**Time Complexity**: O(V + E)
**Space Complexity**: O(V)

---

## Follow-up Questions & Answers

### Q1. What is the difference between Kahn's Algorithm and DFS-based Topological Sort?

**Interview Answer**

**Kahn's Algorithm** (BFS): Compute in-degrees, enqueue all nodes with in-degree 0, process them by decrementing neighbors' in-degrees. If a neighbor's in-degree becomes 0, enqueue it. If processed count < V, a cycle exists. Produces topological order naturally and detects cycles. **DFS-based**: Perform DFS, add each node to the front of the result list after all its descendants are visited (post-order). Reverse the result. Cycle detection requires tracking Gray/Black states. Kahn's is preferred because it naturally detects cycles and produces a valid order without post-processing. Both produce valid topological orders, but the specific order may differ.

---

### Q2. How would you implement Kahn's Algorithm in Rust?

**Interview Answer**

```rust
use std::collections::{VecDeque, HashMap};

fn topological_sort(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut in_degree = vec![0; n];

    for &(u, v) in edges {
        graph.entry(u).or_default().push(v);
        in_degree[v] += 1;
    }

    let mut queue: VecDeque<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
    let mut order = Vec::with_capacity(n);

    while let Some(node) = queue.pop_front() {
        order.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                in_degree[neighbor] -= 1;
                if in_degree[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    if order.len() == n { Some(order) } else { None } // None if cycle exists
}
```

---

### Q3. How is Topological Sort used in build systems?

**Interview Answer**

Build systems like **Make**, **Bazel**, **Gradle**, and **Cargo** use Topological Sort to determine compilation order. Each target is a node; dependencies are directed edges. Topological Sort ensures a target is built only after all its dependencies. If a cycle exists (A depends on B, B depends on A), the build system reports an error. Cargo detects circular crate dependencies at compile time. Parallel build systems (Bazel, Ninja) use Topological Sort to identify which targets can be built simultaneously (nodes with in-degree 0 at each step). This maximizes parallelism while respecting dependency constraints.

---

### Q4. How does Topological Sort relate to course scheduling problems?

**Interview Answer**

The classic "Course Schedule" problem: given n courses and prerequisite pairs, determine if all courses can be finished (no cycle) and return a valid order. This is Topological Sort on a DAG where courses are nodes and prerequisites are edges. If the graph has a cycle, some courses have circular prerequisites and can't all be completed. For "Course Schedule II," return the actual topological order. Variations include: returning the minimum number of semesters (longest path in DAG), or finding the earliest time each course can be started (weighted Topological Sort with course durations).

---

### Q5. How do you detect a cycle using Topological Sort?

**Interview Answer**

**Kahn's Algorithm** naturally detects cycles: if the processed node count is less than the total number of nodes, the remaining nodes form one or more cycles. These are nodes that never reach in-degree 0 because they're part of cyclic dependencies. **DFS-based approach**: track node states (White/Unvisited, Gray/In-Stack, Black/Finished). If DFS encounters a Gray node, a back edge exists → cycle detected. After DFS, the Gray nodes form cycles. For backend systems, cycle detection in dependency graphs is critical — circular dependencies between microservices, packages, or database tables must be resolved.

---

### Q6. What are all possible topological orderings and how to count them?

**Interview Answer**

A DAG can have multiple valid topological orderings. For example, if A→C and B→C, both [A, B, C] and [B, A, C] are valid. To count all orderings: use dynamic programming with bitmask DP. State: `dp[mask]` = number of ways to order the set of nodes in `mask`. Transition: for each node with in-degree 0 within the subset, add it to the ordering. Time: O(2^n × n), Space: O(2^n). For small graphs (n ≤ 20), this is feasible. For large graphs, enumerate a few orderings using randomized Topological Sort (randomly pick from available nodes at each step). In practice, most systems only need one valid ordering.

---

### Q7. How is Topological Sort used in data pipeline orchestration?

**Interview Answer**

Data pipeline tools like **Apache Airflow**, **Dagster**, and **Prefect** model tasks as DAGs. Topological Sort determines execution order. Tasks with no dependencies execute first. Parallel tasks (independent nodes) execute concurrently. If task B depends on task A, B waits for A to complete. Cycle detection prevents invalid pipeline definitions. In Rust backend systems, task schedulers use Topological Sort for: job scheduling (processing steps with dependencies), event processing (ensuring events are processed in causal order), and migration systems (database migration ordering). The `petgraph` crate provides efficient Topological Sort implementation.

---

### Q8. What is a Lexicographically Smallest Topological Sort?

**Interview Answer**

To find the lexicographically smallest topological order, use Kahn's Algorithm with a **min-heap** (priority queue) instead of a regular queue. At each step, always pick the smallest available node (in-degree 0). This produces the lexicographically smallest valid ordering. Time complexity: O(V log V + E) due to heap operations. Useful for: generating canonical orderings, competitive programming, and systems where you need deterministic, reproducible execution order. In Rust, use `BinaryHeap` with `Reverse` wrapper for a min-heap: `BinaryHeap<Reverse<usize>>`.

---

### Q9. How does Topological Sort handle parallel execution?

**Interview Answer**

Topological Sort naturally identifies parallelism: at any step, all nodes with in-degree 0 can execute simultaneously. This is the basis for **parallel task scheduling**. The algorithm proceeds in "levels" — Level 0 nodes have no dependencies, Level 1 nodes depend only on Level 0, etc. The minimum time to complete all tasks equals the longest path in the DAG (critical path). With unlimited parallelism, execution time equals the critical path length. With limited parallelism, use a greedy scheduler that assigns available tasks to free workers. In Rust, `tokio::spawn` handles parallel task execution while Topological Sort determines ordering.

---

### Q10. What is the relationship between Topological Sort and Dynamic Programming on DAGs?

**Interview Answer**

Topological Sort is the foundation for DP on DAGs. Once nodes are in topological order, process them sequentially — each node's DP value depends only on previously processed nodes (its predecessors). Classic examples: **Longest path in DAG**: `dp[v] = max(dp[v], dp[u] + w)` for all incoming edges (u, v). **Critical path method** (project scheduling): earliest start times propagate forward through Topological Sort. **Number of paths from source**: `paths[v] = sum(paths[u])` for all predecessors. The Topological Sort ensures we process nodes in dependency order, making each DP transition valid. Time: O(V + E) for all DAG DP problems.
