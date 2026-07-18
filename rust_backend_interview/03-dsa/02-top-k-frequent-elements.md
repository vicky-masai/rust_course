# Top K Frequent Elements

## Interview Question

Given an integer array `nums` and an integer `k`, return the `k` most frequent elements. The answer can be in any order.

## Interview Answer

The optimal approach uses a **HashMap** to count element frequencies, then a **Min Heap (Priority Queue)** of size `k` to track the top k elements. First, iterate through the array to build a frequency map. Then, iterate through the frequency map entries, maintaining a min heap of size k — if the heap exceeds size k, remove the minimum. The heap stores `(frequency, element)` pairs, so the root is always the least frequent among the top k. This runs in **O(n log k)** time with **O(n)** space for the frequency map.

---

## Follow-up Questions & Answers

### Q1. Why use a Min Heap instead of a Max Heap for this problem?

**Interview Answer**

A **Min Heap** of size k is used because we want to efficiently find and remove the element with the **lowest** frequency among the current top k candidates. When a new element's frequency exceeds the heap's root (minimum), we pop the minimum and push the new element. If we used a Max Heap, we would need to store all n elements and then extract the top k, resulting in **O(n log n)** time. The Min Heap approach limits heap operations to **O(log k)** per element, giving us the better **O(n log k)** overall complexity.

---

### Q2. How would you implement this in Rust using the `binary_heap` crate?

**Interview Answer**

Rust's `std::collections::BinaryHeap` is a Max Heap by default. To create a Min Heap, you wrap elements in `std::cmp::Reverse`. Here's the approach:

```rust
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq = HashMap::new();
    for &n in &nums {
        *freq.entry(n).or_insert(0) += 1;
    }

    let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
    for (&num, &count) in &freq {
        heap.push(Reverse((count, num)));
        if heap.len() > k as usize {
            heap.pop();
        }
    }

    heap.into_iter().map(|Reverse((_, num))| num).collect()
}
```

The `Reverse` wrapper inverts the ordering so that `BinaryHeap` behaves as a min heap.

---

### Q3. What is the time and space complexity of this approach?

**Interview Answer**

**Time Complexity**: O(n log k) — building the frequency map is O(n), and we perform at most n heap insertions/removals, each costing O(log k). **Space Complexity**: O(n + k) — O(n) for the HashMap storing frequency counts, and O(k) for the min heap. If we use bucket sort instead of a heap (see Q5), the time complexity improves to **O(n)** with **O(n)** space, since bucket sort runs in linear time when the range of frequencies is bounded by n.

---

### Q4. What edge cases should you consider?

**Interview Answer**

Several edge cases must be handled: **k equals the number of unique elements** — return all unique elements. **All elements are the same** — return that single element. **k is 0** — return an empty vector. **Multiple elements with the same frequency** — any of them can be returned since the problem says "any order." **Negative numbers** — they should be counted and returned normally. **Single element array with k=1** — return that element. Always validate that k is positive and does not exceed the number of unique elements.

---

### Q5. Can you solve this in O(n) time using Bucket Sort?

**Interview Answer**

Yes. Since the maximum frequency of any element is n (the array length), we can create n+1 buckets where bucket `i` holds elements that appear exactly `i` times. First, build the frequency HashMap in O(n). Then, iterate through the map and place each element into its frequency bucket. Finally, iterate the buckets from highest frequency downward, collecting elements until we have k. This avoids any sorting or heap operations entirely.

```rust
fn top_k_frequent_bucket_sort(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq = HashMap::new();
    for &n in &nums { *freq.entry(n).or_insert(0) += 1; }

    let mut buckets: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];
    for (&num, &count) in &freq {
        buckets[count as usize].push(num);
    }

    let mut result = Vec::new();
    for bucket in buckets.iter().rev() {
        for &num in bucket {
            result.push(num);
            if result.len() == k as usize { return result; }
        }
    }
    result
}
```

---

### Q6. How does this relate to backend systems like Redis or search engines?

**Interview Answer**

Top K Frequent Elements is directly applicable to **Redis** `SORTED SET` operations where you rank items by score. **Search engines** use this pattern for term frequency analysis — finding the k most common terms in a document (TF-IDF preprocessing). **Log aggregation systems** (like ELK stack) use it to identify the most frequent error codes or IP addresses. **Rate limiting** implementations track the most frequent requests. In Rust backend services, you might use this for identifying the k most popular API endpoints from access logs, or the k most active users in a real-time analytics pipeline.

---

### Q7. What if k is very large, say close to n?

**Interview Answer**

When k approaches n, the Min Heap approach degrades toward O(n log n) since the heap size is nearly n. In this case, the **bucket sort** approach (O(n)) is significantly better. Alternatively, a **Quickselect** algorithm (average O(n)) can partition the frequency array to find the k-th largest frequency, then collect all elements with frequency >= that threshold. Quickselect has O(n) average time but O(n²) worst case, though randomized pivot selection makes worst case extremely unlikely. For k close to n, even a simple sort of all (element, frequency) pairs in O(n log n) is competitive.

---

### Q8. How would you handle this problem in a streaming/distributed context?

**Interview Answer**

In streaming systems, you cannot store all elements in memory. Use the **Count-Min Sketch** or **Space-Saving Algorithm** to approximate frequencies in O(1) per element with bounded memory. For distributed systems, use **MapReduce** — the map phase counts local frequencies per partition, and the reduce phase merges counts and selects top k. Systems like **Apache Flink** and **Kafka Streams** use windowed counting with approximate algorithms. In Rust, you could implement a streaming version using the `count-sketch` crate or a custom probabilistic data structure for memory-efficient frequency estimation.

---

### Q9. What is the difference between Top K Frequent and Top K Largest elements?

**Interview Answer**

**Top K Frequent** requires counting occurrences first (HashMap) then selecting by frequency. **Top K Largest** simply selects the k largest values, which can be solved with a Min Heap of size k or Quickselect without any frequency counting. The key difference is that frequency requires a preprocessing step (O(n) counting), while "largest" is a direct comparison. Top K Frequent needs O(n) extra space for the frequency map, while Top K Largest can be done in O(k) space with a heap. Both can be solved with Quickselect for O(n) average time.

---

### Q10. Can you use Quickselect for Top K Frequent, and what are its trade-offs?

**Interview Answer**

Yes. After building the frequency map, create a list of unique elements and use Quickselect to partition around the k-th largest frequency. Elements on the right side of the partition are your answer. Quickselect runs in **O(n) average** time but **O(n²) worst case**. The Min Heap approach is **O(n log k)** worst case, which is more predictable. Quickselect modifies the input array (in-place partitioning), while the heap approach is non-destructive. In practice, Quickselect is faster for large n and small k, but the heap approach is simpler to implement and has guaranteed O(n log k) performance.
