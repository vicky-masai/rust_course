# Kadane's Algorithm

## Interview Question

What is Kadane's Algorithm and when would you use it?

## Interview Answer

Kadane's Algorithm finds the **maximum sum contiguous subarray** in O(n) time. It maintains a running sum of the current subarray. If the running sum becomes negative, reset it to 0 (starting a new subarray). Track the maximum sum seen. The key insight: a negative-running prefix can never contribute to a future maximum — discard it. Extensions handle: "maximum subarray with at least k elements" (prefix sums + deque), "circular subarray" (total sum - minimum subarray), and "product instead of sum" (track min product too). Used in: stock trading (max profit), image processing (brightest region), signal processing (strongest signal segment).

**Time Complexity**: O(n)
**Space Complexity**: O(1)

---

## Follow-up Questions & Answers

### Q1. How would you implement Kadane's Algorithm in Rust?

**Interview Answer**

```rust
// Basic: maximum subarray sum
fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];

    for &val in &arr[1..] {
        current_sum = val.max(current_sum + val);
        max_sum = max_sum.max(current_sum);
    }
    max_sum
}

// With subarray indices
fn max_subarray_with_indices(arr: &[i32]) -> (i32, usize, usize) {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];
    let mut start = 0;
    let mut end = 0;
    let mut temp_start = 0;

    for i in 1..arr.len() {
        if arr[i] > current_sum + arr[i] {
            current_sum = arr[i];
            temp_start = i;
        } else {
            current_sum += arr[i];
        }
        if current_sum > max_sum {
            max_sum = current_sum;
            start = temp_start;
            end = i;
        }
    }
    (max_sum, start, end)
}
```

Note: the `val.max(current_sum + val)` line decides whether to extend the current subarray or start fresh at `val`.

---

### Q2. How do you handle the "all negative numbers" case?

**Interview Answer**

If all numbers are negative, the maximum subarray is the single largest (least negative) element. The basic Kadane's handles this correctly if initialized with `arr[0]` instead of 0. A common mistake is initializing `current_sum = 0`, which gives an empty subarray sum of 0 — wrong if all elements are negative. The fix: initialize both `max_sum` and `current_sum` with `arr[0]`, then iterate from index 1. If the problem allows an empty subarray (sum = 0), initialize with 0 and use `max(0, current_sum + val)`. Clarify with the interviewer whether empty subarrays are allowed.

---

### Q3. How is Kadane's used in stock trading (maximum profit)?

**Interview Answer**

The "best time to buy and sell stock" problem: given prices[i] = price on day i, find maximum profit from one buy-sell transaction. Transform: compute price differences `diff[i] = prices[i+1] - prices[i]`. The maximum profit is the maximum subarray sum of `diff`. Kadane's gives the answer in O(n). If all diffs are negative (prices always dropping), the answer is 0 (don't trade). Extensions: "at most k transactions" requires dynamic programming with O(nk) time. "Cooling period" between transactions adds state tracking. In Rust backend systems, similar patterns apply to: detecting the best time window for resource scaling, finding peak request periods.

---

### Q4. How do you find the maximum product subarray?

**Interview Answer**

Maximum product is trickier than maximum sum because negative × negative = positive. Track both `max_prod` and `min_prod` at each position. When encountering a negative number, swap max and min (since negative × min becomes the new max candidate). Reset at zero (product becomes 0). Time: O(n), Space: O(1).

```rust
fn max_product(arr: &[i32]) -> i32 {
    let mut max_prod = arr[0];
    let mut min_prod = arr[0];
    let mut result = arr[0];

    for &val in &arr[1..] {
        let candidates = [val, max_prod * val, min_prod * val];
        max_prod = *candidates.iter().max().unwrap();
        min_prod = *candidates.iter().min().unwrap();
        result = result.max(max_prod);
    }
    result
}
```

The key insight: the minimum product (potentially very negative) can become the maximum when multiplied by a negative number.

---

### Q5. What is the circular maximum subarray sum?

**Interview Answer**

In a circular array, the maximum subarray can wrap around. Two cases: (1) The maximum subarray is non-circular — use standard Kadane's. (2) The maximum subarray wraps around — this is equivalent to `total_sum - minimum_subarray_sum` (the complement of the minimum subarray). Answer = max(case1, case2), unless all elements are negative (then case2 would incorrectly give 0).

```rust
fn max_circular_subarray(arr: &[i32]) -> i32 {
    let total: i32 = arr.iter().sum();
    let max_kadane = max_subarray_sum(arr);

    // min subarray sum (flip signs, apply Kadane's, flip back)
    let min_kadane: i32 = arr.iter().map(|&x| -x).collect::<Vec<_>>().iter().copied()
        .fold(i32::MIN, |acc, x| x.max(acc + x));
    let min_sum = -min_kadane;

    if max_kadane > 0 {
        max_kadane.max(total - min_sum)
    } else {
        max_kadane // all negative
    }
}
```

Used in: circular buffer analysis, wrap-around log processing, periodic signal analysis.

---

### Q6. How do you solve "maximum subarray with at least k elements"?

**Interview Answer**

This requires combining Kadane's idea with prefix sums and a monotonic deque. Compute prefix sums `P[i] = arr[0] + ... + arr[i-1]`. For each position `i`, find `max(P[i] - min(P[j]))` where `j ≤ i - k`. Maintain a deque of candidate minimum prefix sums (monotonically increasing). For each `i ≥ k`, the front of the deque gives the minimum prefix sum before position `i`. Time: O(n), Space: O(n).

```rust
fn max_subarray_at_least_k(arr: &[i32], k: usize) -> i32 {
    let n = arr.len();
    let mut prefix = vec![0; n + 1];
    for i in 0..n { prefix[i + 1] = prefix[i] + arr[i]; }

    let mut deque = std::collections::VecDeque::new();
    let mut result = i32::MIN;

    for i in k..=n {
        while deque.back().map_or(false, |&j| prefix[j] >= prefix[i - k]) {
            deque.pop_back();
        }
        deque.push_back(i - k);

        if let Some(&j) = deque.front() {
            result = result.max(prefix[i] - prefix[j]);
        }
    }
    result
}
```

---

### Q7. How does Kadane's relate to Dynamic Programming?

**Interview Answer**

Kadane's is a DP algorithm with state `dp[i]` = maximum subarray sum ending at index i. Recurrence: `dp[i] = max(arr[i], dp[i-1] + arr[i])`. Base case: `dp[0] = arr[0]`. Space optimization: since `dp[i]` only depends on `dp[i-1]`, we only need one variable — reducing space from O(n) to O(1). This is the classic "rolling array" space optimization in DP. Kadane's demonstrates that not all DP problems require a table — when the recurrence depends only on the previous state, O(1) space suffices. Other examples: longest increasing subsequence (patience sorting), edit distance for single row.

---

### Q8. What is the maximum sum of k non-overlapping subarrays?

**Interview Answer**

This requires DP with two dimensions: `dp[i][j]` = max sum using j subarrays in arr[0..i]. Recurrence: `dp[i][j] = max(dp[i-1][j], max over t < i: dp[t][j-1] + sum(arr[t+1..i]))`. Time: O(n² × k) naive, O(n × k) with prefix sum optimization. For k=1, this reduces to Kadane's. For k=n/2 (every other element), this is the house robber problem. In Rust, use `Vec<Vec<i64>>` for the DP table, with `i64` to avoid overflow.

---

### Q9. How is Kadane's used in image processing?

**Interview Answer**

**Maximum sum submatrix**: For each pair of rows, compress columns into a 1D array (sum of elements between the two rows), then apply Kadane's to find the maximum subarray. This finds the brightest/densest rectangular region in O(n² × m) time. Used in: object detection (finding high-intensity regions), medical imaging (identifying tumors), and satellite imagery analysis (locating fire hotspots). **1D signal processing**: Finding the strongest signal segment in a time series. **Log analysis**: Finding the time window with the highest error rate.

---

### Q10. What are the edge cases to consider for Kadane's Algorithm?

**Interview Answer**

**Single element**: The maximum subarray is that element itself. **All negative**: The maximum is the least negative element (or 0 if empty subarray allowed). **All positive**: The maximum is the entire array. **Empty array**: Undefined — clarify with interviewer. **Integer overflow**: Use i64 if sums can exceed i32::MAX. **Zero-length subarray**: Some problems allow empty subarray (sum=0), others don't. In Rust, `arr.first()` returns `Option<&i32>` — handle the empty case explicitly. For production code, add assertions or return `Option<i32>` to signal invalid input.
