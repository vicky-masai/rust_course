# Two-Pointer Technique

## Interview Question

What is the Two-Pointer technique and what problems does it solve?

## Interview Answer

The Two-Pointer technique uses two indices (pointers) that traverse a data structure — typically an array or string — to solve problems in **O(n)** time instead of the O(n²) brute force. There are three main variants: **Opposite ends**: pointers start at both ends and move inward (e.g., palindrome check, two-sum in sorted array). **Same direction**: both pointers move in the same direction, with the fast pointer leading (e.g., remove duplicates, partition array). **Sliding window**: a special case where the left pointer marks window start, right marks window end (covered separately). The key insight: by leveraging sorted order or structural properties, we eliminate unnecessary comparisons.

**Time Complexity**: O(n) — each element visited at most once
**Space Complexity**: O(1)

---

## Follow-up Questions & Answers

### Q1. How would you implement the two-sum sorted array problem in Rust?

**Interview Answer**

Given a sorted array and a target sum, find two numbers that add up to the target. Brute force: O(n²). Two-pointer: O(n).

```rust
fn two_sum_sorted(arr: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let sum = arr[left] + arr[right];
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    None
}
```

The sorted property allows us to move pointers intelligently: if sum is too small, move left forward (larger value); if too large, move right backward (smaller value). Each pointer moves at most n times → O(n).

---

### Q2. What is the difference between two-pointer and sliding window?

**Interview Answer**

**Two-pointer (opposite ends)**: Pointers move toward each other from opposite ends of the array. Used for: palindrome validation, two-sum in sorted array, container-with-most-water. **Two-pointer (same direction)**: Both move left-to-right, with different speeds or purposes. Used for: removing duplicates, partitioning arrays, merging sorted arrays. **Sliding window**: A special same-direction case where the window expands and contracts based on a constraint. Used for: longest substring, minimum window subarray. The distinction: two-pointer problems typically have O(1) space and involve comparing elements at two positions. Sliding window problems maintain a window state (sum, frequency map, etc.).

---

### Q3. How is two-pointer used for container-with-most-water?

**Interview Answer**

Given an array of heights, find two lines that together with the x-axis form a container holding the most water. Two-pointer: start with the widest container (left=0, right=n-1). The area is `min(height[left], height[right]) × (right - left)`. Move the pointer pointing to the shorter line inward (moving the taller one can never increase area since width decreases). Track the maximum area seen. O(n) time, O(1) space.

```rust
fn max_area(height: &[i32]) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max_area = 0;

    while left < right {
        let area = height[left].min(height[right]) * (right - left) as i32;
        max_area = max_area.max(area);
        if height[left] < height[right] { left += 1; }
        else { right -= 1; }
    }
    max_area
}
```

This works because the area is limited by the shorter line — moving the taller line inward only decreases width without increasing height.

---

### Q4. How do you use two-pointer to remove duplicates from a sorted array?

**Interview Answer**

Given a sorted array, remove duplicates in-place and return the new length. Use two pointers: a slow pointer (write position) and a fast pointer (reader). The slow pointer tracks where the next unique element should be written. The fast pointer scans through the array. When it finds a new element (different from arr[slow]), increment slow and write the element there. O(n) time, O(1) space.

```rust
fn remove_duplicates(arr: &mut Vec<i32>) -> usize {
    if arr.is_empty() { return 0; }
    let mut slow = 0;
    for fast in 1..arr.len() {
        if arr[fast] != arr[slow] {
            slow += 1;
            arr[slow] = arr[fast];
        }
    }
    slow + 1
}
```

The sorted property guarantees that duplicates are adjacent, making the comparison `arr[fast] != arr[slow]` sufficient.

---

### Q5. How is two-pointer used for three-sum?

**Interview Answer**

Find all unique triplets that sum to zero. Sort the array (O(n log n)). For each element `arr[i]`, use two-pointer on the remaining subarray to find pairs that sum to `-arr[i]`. Skip duplicates to avoid duplicate triplets. Time: O(n²), Space: O(1) excluding output.

```rust
fn three_sum(arr: &[i32]) -> Vec<Vec<i32>> {
    let mut arr = arr.to_vec();
    arr.sort();
    let mut result = Vec::new();

    for i in 0..arr.len() - 2 {
        if i > 0 && arr[i] == arr[i - 1] { continue; }
        let (mut left, mut right) = (i + 1, arr.len() - 1);
        while left < right {
            let sum = arr[i] + arr[left] + arr[right];
            if sum == 0 {
                result.push(vec![arr[i], arr[left], arr[right]]);
                while left < right && arr[left] == arr[left + 1] { left += 1; }
                while left < right && arr[right] == arr[right - 1] { right -= 1; }
                left += 1;
                right -= 1;
            } else if sum < 0 { left += 1; }
            else { right -= 1; }
        }
    }
    result
}
```

---

### Q6. What is the fast-slow pointer (Floyd's cycle detection)?

**Interview Answer**

A variant of two-pointer where one pointer moves twice as fast as the other. Used for: **Cycle detection** in linked lists — if fast and slow meet, a cycle exists. **Finding the cycle start** — after meeting, reset one pointer to head and move both at the same speed; where they meet is the cycle start. **Finding the middle of a linked list** — when fast reaches the end, slow is at the middle. **Happy number detection** — the sequence of digit-square-sums either reaches 1 or enters a cycle. In Rust, this pattern works with indices into arrays (simulating linked lists) since Rust's ownership model makes pointer-based linked lists tricky.

---

### Q7. How do you choose which pointer to move in a two-pointer problem?

**Interview Answer**

The decision depends on the problem's goal: **Maximize area/distance**: move the pointer at the smaller value (container-with-most-water). **Find a specific sum**: move the pointer that brings the sum closer to target (two-sum sorted). **Remove duplicates**: advance the fast pointer always, advance the slow pointer only on new elements. **Partition**: move pointers based on a predicate (e.g., move left past elements satisfying condition, move right before elements not satisfying). The general rule: choose the move that makes progress toward the goal while maintaining the invariant. If stuck, think about what invariant the pointers maintain.

---

### Q8. Can two-pointer be used on unsorted arrays?

**Interview Answer**

Standard two-pointer requires sorted data (for two-sum, container problems). However, some two-pointer variants work on unsorted arrays: **Partitioning** (Dutch National Flag — sort 0s, 1s, 2s in one pass), **Remove duplicates** (requires sorted), **Fast-slow cycle detection** (works on any linked list). For unsorted arrays needing two-pointer-like behavior: sort first (O(n log n)) then apply two-pointer (O(n)), total O(n log n) — still better than O(n²) brute force. For problems where sorting isn't allowed, use HashMap (O(n) time, O(n) space) instead.

---

### Q9. How is two-pointer used in string problems?

**Interview Answer**

**Palindrome check**: left and right pointers at string ends, moving inward comparing characters. O(n) time. **Valid palindrome with deletion**: skip at most one mismatch and check the rest. **Reverse words in a string**: reverse the entire string, then reverse each word individually. **String compression**: read pointer scans, write pointer writes compressed output. In Rust, `s.chars().collect::<Vec<_>>()` enables index-based access for two-pointer on strings. For UTF-8 strings, be careful with byte vs character indices — use `.char_indices()` for correct multi-byte handling.

---

### Q10. What are the common mistakes when implementing two-pointer?

**Interview Answer**

**Off-by-one errors**: Using `<=` instead of `<` for the while condition, or incorrect initial values (e.g., `right = arr.len()` instead of `arr.len() - 1`). **Not handling edge cases**: Empty array, single element, all elements the same. **Incorrect pointer movement**: Moving both pointers simultaneously when only one should move. **Losing elements**: Skipping elements that could form valid pairs. **Integer overflow**: When computing area or sum with large values (use i64). In Rust, `arr.len() - 1` panics on empty arrays — always check `!arr.is_empty()` first. Use `.saturating_sub(1)` for safety.
