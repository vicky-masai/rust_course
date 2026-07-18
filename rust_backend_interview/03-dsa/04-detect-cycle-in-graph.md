# Detect Cycle in Graph

## Interview Question

Given a directed graph, determine whether it contains a cycle.

## Interview Answer

There are two primary approaches: **DFS with coloring** and **Union-Find**. In DFS, we mark nodes as White (unvisited), Gray (in progress), and Black (completed). If we encounter a Gray node during DFS traversal, a back edge exists, indicating a cycle. Union-Find (Disjoint Set Union) works for undirected graphs by tracking parent relationships — if adding an edge connects two already-connected nodes, a cycle exists. For directed graphs, DFS with three-color marking is the standard approach with **O(V + E)** time and **O(V)** space.

---

## Follow-up Questions & Answers

### Q1. What is the difference between cycle detection in directed vs undirected graphs?

**Interview Answer**

In **undirected graphs**, a cycle exists if DFS encounters a visited node that is not the parent (excluding the immediate back-edge). Union-Find is the most efficient approach — O(V + E) with near-constant amortized operations. In **directed graphs**, a cycle exists if DFS encounters a node currently on the recursion stack (Gray node). The key difference is that directed graphs can have cycles that don't involve the immediate parent, so the parent-tracking approach from undirected graphs doesn't work. The three-color DFS (White/Gray/Black) handles this correctly by distinguishing between nodes on the current path vs. nodes fully explored.

---

### Q2. How would you implement cycle detection in Rust using DFS?

**Interview Answer**

```rust
enum Color { White, Gray, Black }

fn has_cycle(graph: &Vec<Vec<usize>>) -> bool {
    let n = graph.len();
    let mut color = vec![Color::White; n];

    fn dfs(u: usize, graph: &Vec<Vec<usize>>, color: &mut Vec<Color>) -> bool {
        color[u] = Color::Gray;
        for &v in &graph[u] {
            match color[v] {
                Color::Gray => return true,
                Color::White => if dfs(v, graph, color) { return true; },
                Color::Black => {}
            }
        }
        color[u] = Color::Black;
        false
    }

    for u in 0..n {
        if matches!(color[u], Color::White) && dfs(u, graph, &mut color) {
            return true;
        }
    }
    false
}
```

The `Gray` state represents nodes currently in the recursion stack. Finding a Gray node means we've found a back edge, which confirms a cycle.

---

### Q3. How does Union-Find work for cycle detection in undirected graphs?

**Interview Answer**

Union-Find maintains a parent array where each node points to its set representative. For each edge (u, v), we find the roots of both u and v. If they share the same root, adding this edge creates a cycle. If they have different roots, we union them by making one root point to the other. Path compression and union by rank keep operations near O(1) amortized. The total time is **O(V + E · α(V))** where α is the inverse Ackermann function (effectively constant). This approach does NOT work for directed graphs because direction matters — edges in a directed graph can create cycles even when the underlying undirected graph is a tree.

---

### Q4. What is the time and space complexity of each approach?

**Interview Answer**

**DFS (Directed Graphs)**: Time O(V + E), Space O(V) for the color array and recursion stack. Worst-case recursion depth is O(V) for a linear chain. **Union-Find (Undirected Graphs)**: Time O(V + E · α(V)) ≈ O(V + E), Space O(V) for parent and rank arrays. **BFS with in-degree (Directed Graphs, Kahn's Algorithm)**: Time O(V + E), Space O(V). If the processed node count < V, a cycle exists. All three approaches are linear, but DFS is simplest for directed graphs while Union-Find is most natural for undirected graphs.

---

### Q5. How do you detect the actual cycle (not just whether one exists)?

**Interview Answer**

To find the actual cycle, modify DFS to track the recursion stack as a path. When you encounter a Gray node, you've found the cycle — backtrack from the current node to the Gray node using the path. In Rust, maintain a `Vec<usize>` as the current path and a `HashMap<usize, usize>` mapping node to its path index. When you find a Gray node `v` at path index `i`, the cycle is `path[i..]`. For Union-Find, you need additional bookkeeping — store edge lists per set and reconstruct the cycle after detection. Kahn's algorithm can also identify cycles: nodes remaining after topological sort are part of cycles.

---

### Q6. What are real-world applications of cycle detection in backend systems?

**Interview Answer**

Cycle detection is critical in: **Dependency resolution** — package managers (Cargo, npm) must detect circular dependencies before installation. **Deadlock detection** — operating systems and databases model resource allocation as a wait-for graph and detect cycles to identify deadlocks. **Task scheduling** — build systems (Make, Bazel) detect circular task dependencies. **Database migrations** — detecting circular foreign key references. **Distributed systems** — detecting circular dependencies in service call chains (microservices A→B→C→A). In Rust, Cargo explicitly detects and rejects circular crate dependencies at compile time.

---

### Q7. Can you use BFS instead of DFS for cycle detection?

**Interview Answer**

Yes, using **Kahn's Algorithm** (topological sort via BFS). Compute in-degrees of all nodes, enqueue nodes with in-degree 0, and process them by removing their outgoing edges. If the processed count equals V, there's no cycle. If processed count < V, the remaining nodes form cycles. This is O(V + E) time and O(V) space. The advantage of Kahn's over DFS is that it naturally identifies all nodes involved in cycles and produces a topological order when no cycle exists. For directed graphs in Rust, this is often preferred because it provides more information (topological order + cycle identification) than simple DFS.

---

### Q8. How does Rust's ownership model affect cycle detection implementations?

**Interview Answer**

Rust's ownership model makes graph representation tricky because cycles violate single ownership. Using `Vec<Vec<usize>>` (adjacency list with indices) avoids this entirely — nodes are identified by `usize` indices, not references, so no ownership issues arise. If using reference-counted nodes (`Rc<RefCell<Node>>`), cycles cause memory leaks because reference counts never reach zero. Rust's borrow checker prevents creating cyclic references at compile time with `Rc`. For cycle detection specifically, the index-based adjacency list approach is preferred in Rust because it's simple, safe, and avoids all ownership complications. The `petgraph` crate provides a full graph library with cycle detection built in.

---

### Q9. How does cycle detection relate to Strongly Connected Components (SCC)?

**Interview Answer**

A directed graph has a cycle if and only if it has a **non-trivial SCC** (an SCC with more than one node, or a self-loop). SCC algorithms (Kosaraju's, Tarjan's) find all cycles in O(V + E) time. If any SCC has size > 1 or any node has a self-loop, the graph has a cycle. SCCs provide more information than simple cycle detection — they identify the complete cyclic structure. For example, in a dependency graph, each non-trivial SCC represents a group of mutually dependent components that must be refactored together. Tarjan's algorithm finds SCCs in a single DFS pass.

---

### Q10. What about cycle detection in very large graphs (billions of edges)?

**Interview Answer**

For very large graphs, DFS may cause stack overflow due to deep recursion. Use **iterative DFS with an explicit stack** to avoid this. For distributed graphs (too large for one machine), use **BFS with in-degree counting** (Kahn's) since it parallelizes well — compute in-degrees in a map-reduce pass, then iteratively process zero-in-degree nodes. **Graph partitioning** across machines with edge cuts allows cycle detection in parallel. For streaming graphs (edges arriving continuously), maintain a dynamic Union-Find with edge additions and detect cycles in real-time. Systems like **Neo4j** and **Amazon Neptune** implement distributed cycle detection using these techniques.
