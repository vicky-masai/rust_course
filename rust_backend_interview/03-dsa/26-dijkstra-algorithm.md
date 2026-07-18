# Dijkstra's Algorithm

## Interview Question

Explain Dijkstra's Algorithm and its applications in backend systems.

## Interview Answer

Dijkstra's Algorithm finds the **shortest path** from a single source vertex to all other vertices in a weighted graph with **non-negative edge weights**. It uses a greedy approach with a priority queue (min-heap): repeatedly extract the vertex with the smallest known distance, relax its neighbors, and update their distances if a shorter path is found. The key invariant: once a vertex is extracted from the heap, its shortest distance is finalized (valid only with non-negative weights). Using a binary heap, it runs in **O((V + E) log V)** time. With a Fibonacci heap, the theoretical time is O(V log V + E).

**Time Complexity**: O((V + E) log V) with binary heap
**Space Complexity**: O(V + E)

---

## Follow-up Questions & Answers

### Q1. How would you implement Dijkstra's in Rust?

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
            let nd = d + w;
            if nd < dist[v] {
                dist[v] = nd;
                heap.push(Reverse((nd, v)));
            }
        }
    }
    dist
}

// With path reconstruction
fn dijkstra_with_path(graph: &Vec<Vec<(usize, i32)>>, src: usize, target: usize)
    -> Option<(i32, Vec<usize>)>
{
    let n = graph.len();
    let mut dist = vec![i32::MAX; n];
    let mut prev = vec![usize::MAX; n];
    let mut heap = BinaryHeap::new();

    dist[src] = 0;
    heap.push(Reverse((0, src)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if u == target { break; }
        if d > dist[u] { continue; }
        for &(v, w) in &graph[u] {
            let nd = d + w;
            if nd < dist[v] {
                dist[v] = nd;
                prev[v] = u;
                heap.push(Reverse((nd, v)));
            }
        }
    }

    if dist[target] == i32::MAX { return None; }

    let mut path = Vec::new();
    let mut cur = target;
    while cur != usize::MAX {
        path.push(cur);
        cur = prev[cur];
    }
    path.reverse();
    Some((dist[target], path))
}
```

The `if d > dist[u] { continue; }` line is critical — it skips stale heap entries without it, the algorithm would still be correct but much slower.

---

### Q2. What is the difference between Dijkstra's and A*?

**Interview Answer**

**Dijkstra's** explores vertices in order of distance from the source — it computes shortest paths to ALL vertices. **A*** adds a heuristic `h(v)` that estimates the distance from v to the target, prioritizing `f(v) = g(v) + h(v)`. A* is focused — it explores only vertices likely to be on the shortest path to the target. With an admissible heuristic (never overestimates), A* finds the optimal path. With `h(v) = 0`, A* reduces to Dijkstra's. A* is faster for single-source-single-target queries. Dijkstra's is necessary when you need shortest paths to all vertices. In GPS navigation, A* with Euclidean distance heuristic is standard.

---

### Q3. What happens when Dijkstra's encounters negative edge weights?

**Interview Answer**

Dijkstra's **fails** with negative edges because it finalizes vertices greedily. A negative edge can create a shorter path to an already-finalized vertex, violating the invariant. Example: A→B (cost 1), A→C (cost 5), C→B (cost -10). Dijkstra's finalizes B with distance 1, but the path A→C→B has distance -5. Solutions: **Bellman-Ford** handles negative weights in O(VE) time. **SPFA** (Shortest Path Faster Algorithm) is an optimized Bellman-Ford. **Johnson's Algorithm** reweights edges using Bellman-Ford results, then runs Dijkstra's. For graphs guaranteed to have non-negative weights (most real-world cases), Dijkstra's is preferred.

---

### Q4. How does Dijkstra's work with adjacency list vs adjacency matrix?

**Interview Answer**

**Adjacency list**: Each vertex stores a list of (neighbor, weight) pairs. Dijkstra's iterates over the adjacency list — total iteration across all vertices is O(E). Binary heap operations: O((V + E) log V). Best for **sparse** graphs (E << V²). **Adjacency matrix**: Each vertex has an array of V entries (weight or ∞). Finding neighbors requires scanning V entries per vertex — O(V²) total. No heap needed — use a simple array for O(V²) total. Best for **dense** graphs (E ≈ V²). In Rust, use `Vec<Vec<(usize, i32)>>` for adjacency lists and `Vec<Vec<i32>>` for matrices. Backend systems typically use adjacency lists (service topologies are sparse).

---

### Q5. How is Dijkstra's used in network routing protocols?

**Interview Answer**

**OSPF** (Open Shortest Path First) and **IS-IS** are intra-domain routing protocols that use Dijkstra's to compute shortest paths. Each router has a complete topology map (link-state database). When a link changes, the router floods the update and recomputes Dijkstra's. The result is a forwarding table mapping destinations to next-hop routers. **BGP** uses a different approach (path vector) for inter-domain routing. In software-defined networking (SDN), the controller runs Dijkstra's to compute optimal paths and installs flow rules. In Rust backend systems, you'd use Dijkstra's for: service mesh routing (finding optimal request path), database shard routing, and CDN edge selection.

---

### Q6. What is the difference between Dijkstra's and BFS for shortest path?

**Interview Answer**

**BFS** finds shortest paths in **unweighted** graphs in O(V + E) using a queue. **Dijkstra's** finds shortest paths in **weighted** graphs (non-negative) in O((V + E) log V) using a priority queue. BFS is a special case of Dijkstra's where all edge weights are 1. BFS is simpler, faster, and uses less memory (queue vs heap). Use BFS when: edges are unweighted or all weights are equal. Use Dijkstra's when: edges have different non-negative weights. A common mistake: using BFS for weighted graphs — BFS assumes each edge contributes equally to the path cost, which is wrong for weighted graphs.

---

### Q7. How do you optimize Dijkstra's for large-scale graphs?

**Interview Answer**

Several optimizations: **Fibonacci heap**: O(V log V + E) theoretical time, better for dense graphs. **Dial's algorithm**: For integer weights bounded by C, use buckets instead of a heap — O(V + E + C). **Bidirectional Dijkstra's**: Run from source and target simultaneously, meeting in the middle. Reduces search space by ~50%. **Landmark-based**: Precompute distances to landmarks, use triangle inequality for pruning. **Contraction hierarchies**: Precompute shortcut edges for highway-like graphs (used by Google Maps). In Rust, `BinaryHeap` is efficient for most cases. For very large graphs, consider graph partitioning (distribute across machines) or hierarchical approaches (precompute for different levels of detail).

---

### Q8. Can Dijkstra's detect negative cycles?

**Interview Answer**

Dijkstra's **cannot** detect negative cycles — it doesn't even work correctly with negative edges. To detect negative cycles: use **Bellman-Ford** — after V-1 relaxations, if any edge can still be relaxed, a negative cycle exists. Or use **Johnson's Algorithm** — run Bellman-Ford first to reweight edges (making them non-negative), then run Dijkstra's. The reweighting ensures no negative edges, but the reweighting itself reveals negative cycles (if Bellman-Ford detects one). In backend systems, negative cycles can represent: arbitrage opportunities in financial systems (buy low, sell high through a cycle of currencies), or bugs in cost-based routing.

---

### Q9. How would you implement Dijkstra's for a real-world road network?

**Interview Answer**

For road networks (millions of nodes, billions of edges): **Contraction Hierarchies**: Precompute "shortcut" edges for important nodes (highway junctions). Query time drops from minutes to microseconds. **ALT** (A*, Landmarks, Triangle inequality): Precompute distances to ~16 landmark nodes. Use triangle inequality for lower bounds in A* heuristic. **Customizable Route Planning**: Separate the graph into cells; precompute intra-cell paths, customize inter-cell paths at query time. In Rust, represent the road network as adjacency lists with `Vec<Vec<(usize, f64)>>` where weights are travel times. Use `petgraph` crate for graph operations. Google Maps uses a combination of these techniques to answer routing queries in milliseconds.

---

### Q10. What is the relationship between Dijkstra's and minimum spanning trees?

**Interview Answer**

Dijkstra's shortest path tree and MST (Minimum Spanning Tree) are **different** structures. **Shortest path tree**: minimizes the sum of edge weights from source to each vertex. **MST**: minimizes the total weight of all edges in the tree connecting all vertices. A shortest path tree is not necessarily an MST, and vice versa. However, there's a connection: **Prim's MST algorithm** is structurally identical to Dijkstra's — both extract the minimum from a heap and relax neighbors. Prim's key is edge weight; Dijkstra's key is cumulative distance. Use Dijkstra's for shortest paths, Prim's for MST. Kruskal's is an alternative MST algorithm using Union-Find.
