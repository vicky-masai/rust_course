# Monotonic Queue / Deque

## Interview Question

What is a Monotonic Queue and what problems does it solve?

## Interview Answer**

A Monotonic Queue is a double-ended queue (deque) that maintains elements in strictly increasing or decreasing order. It efficiently solves the **sliding window maximum/minimum** problem in O(n) time — far better than the O(n × k) brute force. When a new element arrives, remove all elements from the back that violate the monotonic order. The front of the deque always holds the window's maximum (or minimum). Each element is pushed and popped at most once, giving amortized O(1) per element. Used for: sliding window extremes, stock span problems, and "next greater element in a window."

**Time Complexity**: O(n) for processing all elements
**Space Complexity**: O(k) where k is the window size

---

## Follow-up Questions & Answers

### Q1. How would you implement a Monotonic Queue in Rust?

**Interview Answer**

```rust
use std::collections::VecDeque;

fn max_sliding_window(arr: &[i32], k: usize) -> Vec<i32> {
    let mut deque: VecDeque<usize> = VecDeque::new(); // stores indices
    let mut result = Vec::new();

    for i in 0..arr.len() {
        // Remove indices outside the window
        while deque.front().map_or(false, |&j| j + k <= i) {
            deque.pop_front();
        }

        // Remove elements smaller than current from back
        while deque.back().map_or(false, |&j| arr[j] < arr[i]) {
            deque.pop_back();
        }

        deque.push_back(i);

        // Window is full, record the maximum
        if i >= k - 1 {
            result.push(arr[*deque.front().unwrap()]);
        }
    }
    result
}

// For minimum sliding window, flip the comparison:
// while deque.back().map_or(false, |&j| arr[j] > arr[i]) { deque.pop_back(); }
```

The deque stores **indices**, not values, so we can check if the front index is outside the current window.

---

### Q2. What is the difference between Monotonic Queue and Monotonic Stack?

**Interview Answer**

**Monotonic Stack**: Processes elements once, answers "next greater/smaller" queries. Elements are pushed and popped permanently — once popped, they're done. O(n) for one-pass problems. **Monotonic Queue**: Maintains a sliding window's minimum/maximum. Elements can be removed from both ends — the front is evicted when it leaves the window, and the back is evicted when a larger/smaller element arrives. O(n) for sliding window extremes. **Key difference**: Stack is for static problems (one pass). Queue is for dynamic window problems (elements enter and leave). Both use the same monotonic principle — maintain sorted order to enable O(1) extreme queries.

---

### Q3. How is Monotonic Queue used in the "shortest subarray with sum >= k" problem?

**Interview Answer**

This problem requires prefix sums + monotonic deque. Compute prefix sums `P[i]`. For each position `i`, find the smallest `j < i` such that `P[i] - P[j] >= k` (i.e., `P[j] <= P[i] - k`). Maintain a deque of prefix sum indices in increasing order. For each `i`: (1) remove from front all `j` where `P[i] - P[j] >= k` — these are valid subarrays, track the minimum length. (2) remove from back all `j` where `P[j] >= P[i]` — they can't be better starting points. (3) push `i` to the back. Time: O(n). This handles negative numbers (which break sliding window).

---

### Q4. How would you implement "jump game" using Monotonic Queue?

**Interview Answer**

The "Jump Game" asks: given an array where `arr[i]` is the maximum jump length from position `i`, can you reach the end? A monotonic deque can solve "minimum jumps to reach the end" (BFS with deque). Process positions in order. For each position, add unvisited positions reachable from it to the deque. Use a deque instead of a queue for 0-1 BFS when edges have weight 0 or 1. For the standard jump game, a greedy approach (track the farthest reachable position) is simpler and O(n). The monotonic deque variant is useful when jump costs vary (weighted jumps).

---

### Q5. How is Monotonic Queue used in the "sliding window median" problem?

**Interview Answer**

Maintain two deques: one for the lower half (max at front) and one for the upper half (min at front). The median is the front of the larger deque (or average of both fronts). When the window slides: (1) remove the element leaving the window from its deque, (2) add the new element to the appropriate deque, (3) rebalance if one deque is too large. Finding and removing an arbitrary element from a deque is O(k) in the worst case — use a balanced BST or two `BTreeSet`s for O(log k). For integer values, use a `VecDeque` with lazy deletion (mark elements as invalid).

---

### Q6. What is lazy deletion in Monotonic Queue?

**Interview Answer**

When a Monotonic Queue removes elements from the front (leaving the window), sometimes direct removal isn't possible (e.g., the element to remove isn't at the front). Lazy deletion: instead of removing the element immediately, mark it as invalid. When the front element is invalid, pop it. Maintain a counter or a separate HashMap tracking how many of each element need to be lazily deleted. This avoids the O(k) cost of searching and removing from the middle of a deque. Time remains amortized O(1) per element. In Rust, use `HashMap<usize, usize>` mapping value → pending deletions, and clean up during `pop_front`.

---

### Q7. Can Monotonic Queue handle multiple sliding windows simultaneously?

**Interview Answer**

Yes. For problems requiring multiple concurrent windows (e.g., "for each position, find maximum in windows of different sizes"), maintain separate deques per window. For example, finding maximum in windows of size 3 and size 5 simultaneously requires two independent monotonic deques. For batch processing of many windows, a segment tree or sparse table is more efficient (O(1) per query after O(n log n) preprocessing). In backend systems, multiple monitoring windows (1-minute, 5-minute, 15-minute averages) each need their own monotonic deque for real-time computation.

---

### Q8. What is the relationship between Monotonic Queue and the "sliding window median" problem?

**Interview Answer**

The sliding window median requires maintaining a data structure that supports: insert, delete, and find median — all in O(log k) time. Two heaps (max-heap for lower half, min-heap for upper half) achieve this, but deletion from a heap is O(k). A monotonic deque helps for maintaining extremes but doesn't directly solve median (median isn't an extreme). Instead, use two `BTreeSet`s (balanced BSTs) in Rust — they support O(log k) insert, delete, and finding the kth element. The monotonic deque approach works for "sliding window maximum" but not for "sliding window median."

---

### Q9. How does Monotonic Queue perform vs other approaches for sliding window extremes?

**Interview Answer**

| Approach | Time | Space | Preprocessing |
|----------|------|-------|---------------|
| Brute force | O(n × k) | O(1) | None |
| Multiset/BST | O(n × log k) | O(k) | None |
| Sparse table | O(n × log k) | O(n × log k) | O(n × log k) |
| **Monotonic deque** | **O(n)** | **O(k)** | **None** |

Monotonic deque is optimal: O(n) time, O(k) space, no preprocessing. Sparse table is better for static arrays with many queries (O(1) per query after preprocessing). Multiset is simpler but slower. For most backend systems, the monotonic deque is the best choice for streaming sliding window extremes.

---

### Q10. What are real-world applications of Monotonic Queue?

**Interview Answer**

**Load monitoring**: Track maximum requests per second over a sliding window. **Network bandwidth**: Compute maximum throughput over the last N packets. **Trading**: Find the maximum stock price in the last K minutes. **Database query optimization**: Estimate cardinality over sliding time windows. **Rate limiting**: Track the maximum burst size in a sliding window. **Video streaming**: Buffer management — find the maximum buffer level over recent frames. In Rust, a monotonic deque combined with `tokio::time::interval` enables real-time monitoring dashboards that efficiently compute rolling maximums without storing all data points.
