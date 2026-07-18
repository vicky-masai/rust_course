# Sliding Window

## Interview Question

Explain the sliding window technique and its applications in backend systems.

## Interview Answer

The sliding window technique maintains a window (subarray/substring) over data and slides it to solve problems involving contiguous sequences. There are two types: **Fixed-size window** — the window has a predetermined size `k`, used for problems like "maximum sum of k consecutive elements." **Variable-size window** — the window expands and contracts based on a condition, used for "longest substring with at most k distinct characters." The key insight is avoiding redundant computation: instead of recalculating from scratch for each window position, we add the new element and remove the old element. This reduces O(n × k) brute force to **O(n)** time.

**Time Complexity**: O(n) for most sliding window problems
**Space Complexity**: O(k) or O(1) depending on the problem

---

## Follow-up Questions & Answers

### Q1. What are the two types of sliding window problems?

**Interview Answer**

**Fixed Window**: The window size is given (e.g., k). Slide the window across the array, computing some metric at each position. Examples: maximum/minimum sum subarray of size k, maximum of all subarrays of size k, average of all subarrays of size k. **Variable Window**: The window size changes dynamically based on a constraint. Two pointers (left and right) expand/contract the window. Examples: longest substring with at most k distinct characters, minimum window substring, smallest subarray with sum >= target. Variable window problems require careful handling of the expansion and contraction logic.

---

### Q2. How would you implement a sliding window in Rust?

**Interview Answer**

```rust
// Fixed window: maximum sum subarray of size k
fn max_sum_subarray(arr: &[i32], k: usize) -> i32 {
    let mut window_sum: i32 = arr[..k].iter().sum();
    let mut max_sum = window_sum;
    for i in k..arr.len() {
        window_sum += arr[i] - arr[i - k];
        max_sum = max_sum.max(window_sum);
    }
    max_sum
}

// Variable window: longest substring with at most k distinct chars
fn longest_substring(s: &str, k: usize) -> usize {
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    let mut left = 0;
    let mut max_len = 0;
    let chars: Vec<char> = s.chars().collect();

    for right in 0..chars.len() {
        *freq.entry(chars[right]).or_insert(0) += 1;
        while freq.len() > k {
            let count = freq.get_mut(&chars[left]).unwrap();
            *count -= 1;
            if *count == 0 { freq.remove(&chars[left]); }
            left += 1;
        }
        max_len = max_len.max(right - left + 1);
    }
    max_len
}
```

---

### Q3. How is sliding window used in network packet processing?

**Interview Answer**

**Rate limiting**: A sliding window counter tracks requests in the last N seconds. Unlike fixed windows (which allow bursts at window boundaries), sliding windows provide smooth rate limiting. TCP uses **sliding window protocol** for flow control — the receiver advertises a window size indicating how many bytes it can buffer, and the sender limits unacknowledged bytes to this window. **Intrusion detection**: Sliding windows over log data detect patterns like "5 failed logins in 60 seconds." In Rust, `tokio::time::interval` combined with a `VecDeque<Instant>` implements a sliding window rate limiter efficiently.

---

### Q4. What is the connection between sliding window and two pointers?

**Interview Answer**

Sliding window is a specialized form of the two-pointer technique where both pointers move in the same direction (left to right). The left pointer marks the window start, the right pointer marks the window end. In the variable window variant, the right pointer always advances (expanding the window), while the left pointer advances only when the constraint is violated (contracting the window). This ensures each element is visited at most twice (once by each pointer), giving O(n) time. Two pointers in opposite directions (e.g., for sorted array pair sum) is a different technique.

---

### Q5. How do you handle the "maximum of all subarrays of size k" problem efficiently?

**Interview Answer**

The brute force O(n × k) approach checks each subarray. The optimal approach uses a **monotonic deque** (double-ended queue) that maintains indices of elements in decreasing order. For each new element, remove all smaller elements from the back of the deque (they can't be the maximum). Remove the front element if it's outside the window. The front of the deque is always the maximum. This gives O(n) time since each element enters and leaves the deque at most once. This is a classic combination of sliding window + monotonic deque.

```rust
use std::collections::VecDeque;

fn max_sliding_window(arr: &[i32], k: usize) -> Vec<i32> {
    let mut deque: VecDeque<usize> = VecDeque::new();
    let mut result = Vec::new();

    for i in 0..arr.len() {
        while deque.back().map_or(false, |&j| arr[j] <= arr[i]) {
            deque.pop_back();
        }
        deque.push_back(i);
        if deque.front().map_or(false, |&j| j + k <= i) {
            deque.pop_front();
        }
        if i >= k - 1 {
            result.push(arr[*deque.front().unwrap()]);
        }
    }
    result
}
```

---

### Q6. What is the difference between sliding window and prefix sum?

**Interview Answer**

**Sliding window** works on contiguous subarrays and maintains a running metric by adding/removing elements at the boundaries. It's O(n) and O(1) or O(k) space. **Prefix sum** precomputes cumulative sums to answer range sum queries in O(1) after O(n) preprocessing. It works for non-contiguous queries and arbitrary ranges. Sliding window is better for problems where the window slides sequentially (maximum sum subarray of size k). Prefix sum is better for arbitrary range queries (sum of elements from index i to j). For "subarray sum equals k" problems, prefix sum + HashMap is the standard approach.

---

### Q7. How is sliding window used in string processing?

**Interview Answer**

**Longest substring without repeating characters**: Expand right pointer; when a duplicate is found, contract left until the duplicate is removed. O(n) time. **Minimum window substring**: Find the smallest substring of s containing all characters of t. Expand right until all characters are included, then contract left to minimize. O(n) time. **Longest substring with at most k distinct characters**: Expand until more than k distinct chars, then contract. These problems appear frequently in backend systems for: input validation, log pattern matching, and data stream processing. In Rust, `HashMap<char, usize>` or `Vec<usize>` (for ASCII) tracks character frequencies.

---

### Q8. Can sliding window be applied to 2D arrays?

**Interview Answer**

Yes. **2D sliding window** applies to problems like "maximum sum submatrix of size k×k." The approach: first compute prefix sums for 2D, then slide the k×k window across the matrix. Alternatively, fix the top and bottom rows, compress columns into a 1D array, and apply 1D sliding window. This reduces the 2D problem to a series of 1D problems. Time complexity is O(n² × m) for an n×m matrix with k×k window. Used in: image processing (convolution), matrix analysis, and grid-based game logic. In Rust, represent the 2D grid as `Vec<Vec<i32>>` or a flattened `Vec<i32>` with index arithmetic.

---

### Q9. How does the sliding window TCP protocol relate to the algorithmic technique?

**Interview Answer**

TCP's sliding window protocol controls flow between sender and receiver. The sender maintains a window of unacknowledged bytes. The receiver advertises a **receive window** (rwnd) indicating buffer space. The sender can transmit up to rwnd bytes before waiting for ACKs. This is conceptually similar to the algorithmic sliding window: the "window" slides forward as ACKs arrive. **Congestion window** (cwnd) adds sender-side flow control. The actual window is min(cwnd, rwnd). This prevents buffer overflow and network congestion. In backend systems, understanding TCP sliding windows is crucial for tuning connection throughput, buffer sizes, and performance optimization.

---

### Q10. What are common mistakes when implementing sliding window?

**Interview Answer**

**Off-by-one errors**: Incorrect window boundaries — using `<=` vs `<` for the right pointer. **Forgetting to update state when shrinking**: When the left pointer moves, the removed element's contribution must be properly subtracted. **Integer overflow**: When computing window sums, use appropriate integer types (i64 for sums of large arrays). **Not handling empty input**: Check for empty arrays/slices before processing. **Incorrect loop invariant**: The window must be valid after each iteration. In Rust, use `windows(k)` for fixed-size iteration, but be aware it returns slices which borrows the original. For variable windows, manual pointer management with `while` loops is more flexible.
