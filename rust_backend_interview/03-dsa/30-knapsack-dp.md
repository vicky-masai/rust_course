# Knapsack / Dynamic Programming Partitioning

## Interview Question

What is the Knapsack Problem and how does Dynamic Programming solve it?

## Interview Answer

The 0/1 Knapsack Problem: given `n` items, each with a weight and value, and a knapsack with capacity `W`, maximize the total value of items that fit. Each item is either taken (1) or not (0). The DP solution: `dp[i][w]` = maximum value using items 1..i with capacity w. Recurrence: `dp[i][w] = max(dp[i-1][w], dp[i-1][w-weight[i]] + value[i])` if `weight[i] ≤ w`. Time: **O(n × W)**, Space: O(n × W) reducible to O(W). This is a **pseudo-polynomial** algorithm — efficient when W is small, exponential when W is large (e.g., coin values in cents). Variants: unbounded knapsack, subset sum, partition problem.

**Time Complexity**: O(n × W)
**Space Complexity**: O(W) optimized

---

## Follow-up Questions & Answers

### Q1. How would you implement 0/1 Knapsack in Rust?

**Interview Answer**

```rust
fn knapsack(weights: &[i32], values: &[i32], capacity: i32) -> i32 {
    let n = weights.len();
    let w = capacity as usize;
    let mut dp = vec![vec![0; w + 1]; n + 1];

    for i in 1..=n {
        for cap in 0..=w {
            dp[i][cap] = dp[i - 1][cap]; // don't take item i
            if weights[i - 1] <= cap as i32 {
                dp[i][cap] = dp[i][cap].max(
                    dp[i - 1][cap - weights[i - 1] as usize] + values[i - 1]
                );
            }
        }
    }
    dp[n][w]
}

// Space-optimized version (O(W) space)
fn knapsack_optimized(weights: &[i32], values: &[i32], capacity: i32) -> i32 {
    let w = capacity as usize;
    let mut dp = vec![0; w + 1];

    for i in 0..weights.len() {
        // iterate backwards to avoid using same item twice
        for cap in (weights[i] as usize..=w).rev() {
            dp[cap] = dp[cap].max(dp[cap - weights[i] as usize] + values[i]);
        }
    }
    dp[w]
}
```

The backward iteration in the optimized version ensures each item is used at most once.

---

### Q2. What is the difference between 0/1 Knapsack and Unbounded Knapsack?

**Interview Answer**

**0/1 Knapsack**: Each item can be used at most once. Iterate capacities in reverse order to prevent reuse. **Unbounded Knapsack**: Each item can be used unlimited times. Iterate capacities in forward order to allow reuse. The recurrence differs only in which indices you reference: 0/1 uses `dp[i-1]`, unbounded uses `dp[i]`. Real-world examples: 0/1 — selecting projects with limited budget. Unbounded — making change with unlimited coins. Subset Sum is 0/1 Knapsack where value = weight and the goal is to reach exactly capacity W.

---

### Q3. How is Knapsack used in resource allocation in backend systems?

**Interview Answer**

**Budget allocation**: Choose which features/projects to fund given a budget constraint — each has a cost (weight) and expected ROI (value). **Server provisioning**: Select which instances to spin up given a cost budget — each instance type has a cost and performance value. **Package selection**: Choose which microservices to deploy given memory constraints. **Feature flags**: Select which features to enable in a deployment given performance budget. In Rust backend systems, the knapsack pattern applies to: selecting which API rate limits to enforce, choosing cache strategies (memory vs latency tradeoff), and optimizing deployment configurations.

---

### Q4. What is the Partition Problem and how does it relate to Knapsack?

**Interview Answer**

The Partition Problem asks: can an array be split into two subsets with equal sum? This is a special case of Subset Sum (which is a special case of Knapsack): can we find a subset with sum = total_sum / 2? If total_sum is odd, partitioning is impossible. Otherwise, solve with 0/1 Knapsack where weights = values = array elements, capacity = total_sum / 2. Time: O(n × total_sum/2). Used in: load balancing (distributing tasks equally across servers), memory management (splitting allocations evenly), and fair division (splitting assets between parties).

---

### Q5. How do you reconstruct the actual items selected in Knapsack?

**Interview Answer**

After filling the DP table, backtrack from `dp[n][W]`: if `dp[n][W] != dp[n-1][W]`, item n was selected. Otherwise, it wasn't. Move to the previous row and repeat. In Rust:

```rust
fn knapsack_items(weights: &[i32], values: &[i32], capacity: i32) -> Vec<usize> {
    let n = weights.len();
    let w = capacity as usize;
    let mut dp = vec![vec![0; w + 1]; n + 1];

    for i in 1..=n {
        for cap in 0..=w {
            dp[i][cap] = dp[i - 1][cap];
            if weights[i - 1] <= cap as i32 {
                dp[i][cap] = dp[i][cap].max(
                    dp[i - 1][cap - weights[i - 1] as usize] + values[i - 1]
                );
            }
        }
    }

    let mut items = Vec::new();
    let mut cap = w;
    for i in (1..=n).rev() {
        if dp[i][cap] != dp[i - 1][cap] {
            items.push(i - 1);
            cap -= weights[i - 1] as usize;
        }
    }
    items.reverse();
    items
}
```

---

### Q6. What is the "Meet in the Middle" technique for large Knapsack?

**Interview Answer**

When n is moderate (e.g., 40) but W is very large, O(n × W) is too slow. "Meet in the Middle" splits items into two halves. Generate all 2^(n/2) subsets for each half with their (weight, value) pairs. Sort one half by weight. For each subset in the first half, binary search the second half for the best complement that fits in the remaining capacity. Time: O(2^(n/2) × n/2), Space: O(2^(n/2)). This solves Knapsack in O(2^(n/2) × n) instead of O(2^n) brute force or O(n × W) DP. Useful when W is up to 10^18 but n ≤ 40.

---

### Q7. How is the Fractional Knapsack different from 0/1 Knapsack?

**Interview Answer**

**0/1 Knapsack**: Items are indivisible — take all or none. Requires DP, O(n × W). **Fractional Knapsack**: Items can be fractionally taken (e.g., take 30% of item A). Solved greedily: sort by value/weight ratio, take items in decreasing order until full. O(n log n). The greedy approach works because fractional items have no "all-or-nothing" constraint. Real-world: 0/1 — choosing which projects to fund. Fractional — allocating CPU time or bandwidth (can split proportionally). The fractional version is a linear programming problem; the 0/1 version is an integer programming problem.

---

### Q8. What is the Coin Change problem and its DP formulation?

**Interview Answer**

Given coin denominations and a target amount, find the minimum number of coins to make the amount. This is an Unbounded Knapsack variant where: weight = value = coin denomination, goal = minimum items to reach sum = target. `dp[i]` = minimum coins to make amount `i`. Recurrence: `dp[i] = min(dp[i - coin] + 1)` for each coin where `coin ≤ i`. Time: O(amount × num_coins), Space: O(amount).

```rust
fn coin_change(coins: &[i32], amount: i32) -> Option<i32> {
    let amount = amount as usize;
    let mut dp = vec![i32::MAX; amount + 1];
    dp[0] = 0;

    for i in 1..=amount {
        for &coin in coins {
            if coin as usize <= i && dp[i - coin as usize] != i32::MAX {
                dp[i] = dp[i].min(dp[i - coin as usize] + 1);
            }
        }
    }

    if dp[amount] == i32::MAX { None } else { Some(dp[amount]) }
}
```

Used in: currency systems, making change at vending machines, and resource bundling.

---

### Q9. How do you optimize DP space for Knapsack?

**Interview Answer**

The 2D DP table `dp[i][w]` only depends on the previous row `dp[i-1][...]`. This enables rolling array optimization: use two 1D arrays (current and previous row) or one array with reverse iteration. Space: O(n × W) → O(W). Further optimization: if weights are small integers, use bit manipulation to pack multiple boolean states into a single integer. For the partition problem (boolean Knapsack), use bitset operations: `dp |= dp << weight[i]`. In Rust, `BitVec` or `u64` bitmasks enable this. For n = 1000 and W = 1000, the bitset approach runs in O(n × W / 64) — 64× faster.

---

### Q10. What are the real-world applications of DP partitioning?

**Interview Answer**

**Task scheduling**: Distribute tasks across servers to minimize completion time (partition into k subsets with minimum maximum sum — multiprocessor scheduling). **Memory allocation**: Split memory blocks between processes to minimize fragmentation. **Load balancing**: Distribute requests across backend servers equally. **Network packet fragmentation**: Split large packets into MTU-sized chunks. **Database query optimization**: Partition tables for parallel query execution. **Container orchestration**: Assign pods to nodes with resource constraints (this is a multi-dimensional Knapsack). In Rust backend systems, DP partitioning applies to: sharding strategies (distribute data evenly), connection pooling (allocate connections to services), and deployment planning (fit services into available nodes).
