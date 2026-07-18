# Shortest Path

## Interview Question

Given a weighted graph, find the shortest path from a source vertex to all other vertices.

## Interview Answer

The choice of algorithm depends on graph properties. **Dijkstra's Algorithm** is used for graphs with non-negative edge weights and runs in **O((V + E) log V)** using a min heap. **Bellman-Ford** handles negative weights in **O(VE)** time. **BFS** finds shortest paths in unweighted graphs in **O(V + E)**. **Floyd-Warshall** computes all-pairs shortest paths in **O(V³)**. For most backend applications with non-negative weights (network latencies, distances), Dijkstra's is the standard choice.

---

## Follow-up Questions & Answers

### Q1. Explain Dijkstra's Algorithm step by step.

**Interview Answer**

Dijkstra's Algorithm works as follows: **1)** Initialize distances to all vertices as infinity, except the source (distance 0). **2)** Insert the source into a min heap. **3)** Extract the vertex with minimum distance. **4)** For each neighbor, if the new distance (current + edge weight) is less than the stored distance, update it and push into the heap. **5)** Repeat steps 3-4 until the heap is empty. The key invariant is that once a vertex is extracted from the heap, its shortest distance is finalized. This is only valid with non-negative weights because negative edges could provide a shorter path to an already-finalized vertex.

---

### Q2. How would you implement Dijkstra's in Rust?

**Interview Answer**

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn dijkstra(graph: &Vec<Vec<(usize, i32)>>, src: usize) -> Vec<i32> {
    let n = graph.len();
    let mut dist = vec![i32::MAX; n];
    let mut heap = BinaryHeap::new();

    dist[src] = 0;
    heap.push(Reverse((0, src)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] { continue; }
        for &(v, w) in &graph[u] {
            let new_dist = d + w;
            if new_dist < dist[v] {
                dist[v] = new_dist;
                heap.push(Reverse((new_dist, v)));
            }
        }
    }
    dist
}
```

The `continue` when `d > dist[u]` is critical — it skips stale heap entries, keeping the algorithm efficient.

---

### Q3. What is the difference between Dijkstra's and Bellman-Ford?

**Interview Answer**

**Dijkstra's** is faster — O((V + E) log V) with a heap — but fails with negative edge weights because it finalizes vertices greedily. **Bellman-Ford** is slower — O(VE) — but handles negative weights and detects negative cycles (by checking if any distance can still be reduced after V-1 iterations). Bellman-Ford works by relaxing all edges V-1 times, which guarantees shortest paths even with negative weights (up to V-1 edges). Dijkstra's is preferred for non-negative graphs (most real-world cases), while Bellman-Ford is used for networks that might have negative costs (e.g., financial arbitrage detection).

---

### Q4. How do you reconstruct the actual shortest path, not just the distance?

**Interview Answer**

Maintain a `prev` array alongside the distance array. When you update `dist[v]` with a shorter path through `u`, set `prev[v] = u`. After the algorithm completes, trace back from the target to the source using the `prev` array, then reverse the path. In Rust:

```rust
let mut path = Vec::new();
let mut current = target;
while current != usize::MAX {
    path.push(current);
    current = prev[current];
}
path.reverse();
```

Initialize `prev` with `usize::MAX` (or a sentinel value) to indicate no predecessor. The path reconstruction adds O(V) time and space but provides the complete route, not just the distance.

---

### Q5. What are real-world applications of shortest path algorithms in backend systems?

**Interview Answer**

Shortest path algorithms are used extensively: **Routing** — OSPF and IS-IS protocols use Dijkstra's to compute internet routing tables. **GPS/Maps** — Google Maps uses hierarchical Dijkstra's with contraction hierarchies for real-time routing. **CDN edge selection** — finding the nearest/fastest server to a user. **Network packet routing** — finding optimal paths through network topologies. **Social networks** — degrees of separation (BFS for unweighted). **Microservice mesh** — finding optimal request paths in service meshes (Istio). In Rust backend services, you might use shortest path for load balancing across service endpoints or finding optimal shard routing in distributed databases.

---

### Q6. How does Dijkstra's perform with different graph densities?

**Interview Answer**

For **sparse graphs** (E ≈ V), Dijkstra's with a binary heap runs in O(V log V). For **dense graphs** (E ≈ V²), the heap operations dominate at O(V² log V), which is worse than a simple array-based implementation at O(V²). For dense graphs, use a **Fibonacci heap** for O(V log V + E) theoretical time, or a simple array (no heap) for O(V²) practical time. In Rust, `BinaryHeap` is a binary heap, so for dense graphs, consider replacing it with a sorted `Vec` or `BTreeSet`. Most real-world backend graphs (service topologies, road networks) are sparse, making the binary heap approach ideal.

---

### Q7. How do you handle very large graphs that don't fit in memory?

**Interview Answer**

For graphs too large for a single machine's memory, use **external memory algorithms**. Dijkstra's can be adapted to work with disk-based priority queues — load frontier nodes into memory, process them, and write updated distances back to disk. **Graph partitioning** distributes the graph across machines, with each machine running Dijkstra's on its partition and coordinating at boundaries. **Hierarchical approaches** (like Contraction Hierarchies for road networks) precompute shortcuts to speed up queries. **BFS-based** approaches with bounded depth work well for social networks. Systems like **Pregel** (Google) and **GraphX** (Apache Spark) implement distributed shortest path using vertex-centric computation.

---

### Q8. Can you use Dijkstra's with integer weights for better performance?

**Interview Answer**

Yes, when edge weights are small integers, **Dial's Algorithm** (a variant of Dijkstra's) uses an array of buckets instead of a heap. Bucket `i` contains vertices with distance `i`. This gives O(V + E + W) time where W is the maximum weight. For non-negative integer weights bounded by a constant C, this is O(V + E). In Rust, you can implement this with a `Vec<VecDeque<usize>>` as the bucket array. This is significantly faster than a binary heap for small integer weights because it avoids log-factor overhead and has better cache locality. It's commonly used in network routing where link costs are small integers.

---

### Q9. What is the A* algorithm and how does it relate to Dijkstra's?

**Interview Answer**

A* is Dijkstra's with a **heuristic function** that estimates the distance to the goal, prioritizing vertices likely to lead to shorter paths. The priority becomes `f(v) = g(v) + h(v)` where `g(v)` is the known cost from source and `h(v)` is the heuristic estimate to target. If `h(v)` is admissible (never overestimates), A* finds the optimal path. For `h(v) = 0`, A* reduces to Dijkstra's. A* is used in game pathfinding and GPS routing. The heuristic dramatically reduces the search space — A* might explore only a fraction of vertices that Dijkstra's would visit. However, A* is single-source-single-target, while Dijkstra's computes all shortest paths from a source.

---

### Q10. When would you choose BFS over Dijkstra's for shortest path?

**Interview Answer**

Use **BFS** when the graph is **unweighted** (all edges have equal weight) — BFS finds shortest paths in O(V + E) without any priority queue overhead. BFS is simpler, has better cache locality, and uses less memory. Use **Dijkstra's** when edges have **different non-negative weights**. A common pattern is to use BFS for initial exploration (finding reachable nodes) and Dijkstra's for weighted optimization. In backend systems, BFS is used for unweighted social network traversals (degrees of separation), while Dijkstra's is used for weighted network routing. BFS also naturally computes shortest paths in layers, which is useful for breadth-first analysis of graph structure.
