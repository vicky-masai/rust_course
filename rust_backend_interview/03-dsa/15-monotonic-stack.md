# Monotonic Stack

## Interview Question

What is a monotonic stack and what problems does it solve?

## Interview Answer

A monotonic stack is a stack data structure where elements are maintained in either strictly increasing or strictly decreasing order from bottom to top. It efficiently solves the **"next greater/smaller element"** class of problems. When a new element arrives, elements that violate the monotonic property are popped — these popped elements have found their answer (the new element is their next greater/smaller). Each element is pushed and popped at most once, giving **O(n)** time for processing the entire array. Common variants: monotonic decreasing stack (for next greater element), monotonic increasing stack (for next smaller element).

**Time Complexity**: O(n) — each element pushed/popped at most once
**Space Complexity**: O(n) for the stack

---

## Follow-up Questions & Answers

### Q1. How does a monotonic stack solve "next greater element"?

**Interview Answer**

Traverse the array from left to right. Maintain a decreasing stack (top is smallest). For each element `arr[i]`, pop all stack elements smaller than `arr[i]` — each popped element's next greater element is `arr[i]`. Push `arr[i]` onto the stack. After traversal, remaining elements have no greater element (their answer is -1). The stack stores indices, not values, to track positions. This is O(n) because each element enters and leaves the stack at most once.

```rust
fn next_greater_element(arr: &[i32]) -> Vec<i32> {
    let n = arr.len();
    let mut result = vec![-1; n];
    let mut stack: Vec<usize> = Vec::new(); // indices

    for i in 0..n {
        while let Some(&top) = stack.last() {
            if arr[top] < arr[i] {
                result[top] = arr[i];
                stack.pop();
            } else { break; }
        }
        stack.push(i);
    }
    result
}
```

---

### Q2. What is the difference between monotonic stack and monotonic queue?

**Interview Answer**

A **monotonic stack** processes elements once and answers "next greater/smaller" queries — it only looks forward (or backward in a single pass). A **monotonic queue** (deque) maintains a sliding window's minimum/maximum and supports adding/removing from both ends. The stack is used for problems where each element needs to find the next element that satisfies a condition. The queue is used for problems requiring a window's extreme value (e.g., maximum of all subarrays of size k). Stack: O(n) for one-pass problems. Queue: O(n) for sliding window extremes. Both are O(1) amortized per element.

---

### Q3. How is monotonic stack used in "daily temperatures" and "stock span" problems?

**Interview Answer**

**Daily Temperatures** (LeetCode 739): For each day, find how many days until a warmer temperature. Use a monotonic decreasing stack of indices. When a warmer day arrives, pop stack elements and compute the difference in indices. **Stock Span** (LeetCode 901): For each day, find how many consecutive days the stock price was <= today's price. Use a monotonic decreasing stack storing (price, span). When a new price arrives, pop all smaller prices and accumulate their spans. Both problems are classic monotonic stack applications. The key insight: the stack stores elements waiting for their answer, and each new element resolves pending queries.

---

### Q4. How would you use monotonic stack for "largest rectangle in histogram"?

**Interview Answer**

For each bar, find the first smaller bar to the left and right — these define the bar's maximum-width rectangle. Use two passes: one left-to-right monotonic increasing stack to find left boundaries, one right-to-left to find right boundaries. For each bar popped from the stack, its width extends from the current stack top to the popped position. The area is `height × width`. This runs in O(n) time. This problem is a classic application of monotonic stack where the answer isn't directly "next greater" but uses the same underlying mechanism.

```rust
fn largest_rectangle(heights: &[i32]) -> i32 {
    let mut stack: Vec<usize> = Vec::new();
    let mut max_area = 0;
    let n = heights.len();

    for i in 0..=n {
        let h = if i < n { heights[i] } else { 0 };
        while let Some(&top) = stack.last() {
            if heights[top] <= h { break; }
            stack.pop();
            let width = if let Some(&prev) = stack.last() { i - prev - 1 } else { i };
            max_area = max_area.max(heights[top] * width as i32);
        }
        stack.push(i);
    }
    max_area
}
```

---

### Q5. How is monotonic stack used in "trapping rain water"?

**Interview Answer**

The trapped water at position `i` is `min(max_left, max_right) - height[i]`. A monotonic decreasing stack stores indices of bars in decreasing height. When a bar taller than the stack top arrives, pop the top and compute water trapped above it. The water width is the distance between the current index and the new stack top. The height difference is `min(heights[current], heights[stack_top]) - heights[popped]`. This O(n) approach avoids precomputing left_max and right_max arrays. The stack essentially tracks "valleys" where water can accumulate.

---

### Q6. What real-world systems use monotonic stack?

**Interview Answer**

**CPU scheduling**: Finding the next higher-priority process in O(1) using monotonic structures. **Load balancers**: Tracking consecutive health check failures — the monotonic stack identifies when a server's failure streak is broken. **Time-series databases**: Detecting price/temperature thresholds — "when was the last time temperature exceeded X?" **Financial systems**: Candlestick pattern detection in trading — finding support/resistance levels. **Web servers**: Rate limiting — tracking request timestamps to find when the rate drops below threshold. In Rust, monotonic stacks are used in parsing (e.g., matching brackets in code parsers) and data pipeline processing.

---

### Q7. Can monotonic stack handle "circular" arrays?

**Interview Answer**

Yes, for problems like "next greater element in circular array" (LeetCode 503). Process the array twice (indices 0 to 2n-1, using `i % n` for actual values) with a monotonic stack. The first pass finds next greater elements for straightforward cases. The second pass handles wrap-around cases. The stack stores indices, and we check if the index is still within bounds (or use a visited set to avoid processing the same index twice). Alternatively, process the array once and push remaining elements through again. Time complexity remains O(n) since each element is processed at most twice.

---

### Q8. How do you choose between monotonic increasing vs decreasing stack?

**Interview Answer**

**Decreasing stack** (bottom to top): Used for finding **next greater element**. The stack maintains elements in decreasing order, so when a larger element arrives, it "resolves" smaller elements. **Increasing stack** (bottom to top): Used for finding **next smaller element**. The stack maintains elements in increasing order. **Decision rule**: If the problem asks for "next greater" → decreasing stack. If "next smaller" → increasing stack. If the problem involves finding boundaries (like largest rectangle), use the opposite: increasing stack for left boundaries, decreasing for right boundaries. Always think about what the stack "waits for."

---

### Q9. How does monotonic stack relate to bracket matching and parsing?

**Interview Answer**

Bracket matching is a special case of monotonic stack where the ordering condition is matching pairs rather than numeric comparison. A stack tracks open brackets. When a closing bracket arrives, check if it matches the stack top. While not strictly "monotonic" in the numeric sense, the principle is the same: elements on the stack are waiting for a resolving element. In expression parsing (infix to postfix), a monotonic operator stack maintains precedence order. The Shunting-yard algorithm uses a stack that keeps operators in decreasing precedence order — operators with lower precedence are popped first. In Rust, `Vec<char>` as a stack handles bracket matching naturally.

---

### Q10. What is the time complexity proof that monotonic stack is O(n)?

**Interview Answer**

Each element is pushed onto the stack exactly once. Each element is popped from the stack at most once (once popped, it never returns). Therefore, the total number of push and pop operations across the entire algorithm is at most 2n. Each push/pop is O(1). The traversal of the array is O(n). Total time: O(n) + O(2n) = **O(n)**. This is an amortized analysis — individual operations may take O(n) in the worst case (when the stack grows to size n), but the amortized cost per element is O(1). This tight bound makes monotonic stack one of the most efficient techniques for element-boundary problems.
