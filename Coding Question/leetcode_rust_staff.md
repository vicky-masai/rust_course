# LeetCode in Rust — Staff Interview Track

Solve these **in Rust** on LeetCode (language: Rust).  
Goal: clear **Senior / Staff** coding rounds at Cloudflare, Discord, AWS, Meta, Google, Microsoft, etc.

**How to use**
1. Open the link → choose **Rust**
2. Timebox: Easy 20m / Medium 35m / Hard 45–50m
3. After AC: rewrite once cleanly (ownership, fewer clones, clear complexity)
4. For Medium/Hard: write 2–3 of your own tests mentally (empty, overflow, duplicates)

**Staff bar (not just “got AC”)**
- Explain time/space before coding
- Handle edges without panic
- Name invariants (what is always true)
- Mention what changes at production scale (memory, concurrency)

---

## Phase 0 — Warm-up (Days 1–7)

| # | Problem | Diff | Link | Pattern |
| ---: | --- | --- | --- | --- |
| 1 | Two Sum | E | https://leetcode.com/problems/two-sum/ | HashMap |
| 2 | Valid Parentheses | E | https://leetcode.com/problems/valid-parentheses/ | Stack |
| 3 | Merge Two Sorted Lists | E | https://leetcode.com/problems/merge-two-sorted-lists/ | Linked list |
| 4 | Best Time to Buy and Sell Stock | E | https://leetcode.com/problems/best-time-to-buy-and-sell-stock/ | Kadane-lite |
| 5 | Valid Anagram | E | https://leetcode.com/problems/valid-anagram/ | Counting |
| 6 | Binary Search | E | https://leetcode.com/problems/binary-search/ | Binary search |
| 7 | Invert Binary Tree | E | https://leetcode.com/problems/invert-binary-tree/ | DFS |

---

## Phase 1 — Arrays / Strings / Windows (Days 8–20)

| # | Problem | Diff | Link | Pattern |
| ---: | --- | --- | --- | --- |
| 8 | Contains Duplicate | E | https://leetcode.com/problems/contains-duplicate/ | HashSet |
| 9 | Product of Array Except Self | M | https://leetcode.com/problems/product-of-array-except-self/ | Prefix |
| 10 | Maximum Subarray | M | https://leetcode.com/problems/maximum-subarray/ | Kadane |
| 11 | Maximum Product Subarray | M | https://leetcode.com/problems/maximum-product-subarray/ | DP |
| 12 | Find Minimum in Rotated Sorted Array | M | https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/ | Binary search |
| 13 | Search in Rotated Sorted Array | M | https://leetcode.com/problems/search-in-rotated-sorted-array/ | Binary search |
| 14 | 3Sum | M | https://leetcode.com/problems/3sum/ | Two pointers |
| 15 | Container With Most Water | M | https://leetcode.com/problems/container-with-most-water/ | Two pointers |
| 16 | Longest Substring Without Repeating Characters | M | https://leetcode.com/problems/longest-substring-without-repeating-characters/ | Sliding window |
| 17 | Longest Repeating Character Replacement | M | https://leetcode.com/problems/longest-repeating-character-replacement/ | Sliding window |
| 18 | Minimum Window Substring | H | https://leetcode.com/problems/minimum-window-substring/ | Sliding window |
| 19 | Sliding Window Maximum | H | https://leetcode.com/problems/sliding-window-maximum/ | Monotonic deque |
| 20 | Trapping Rain Water | H | https://leetcode.com/problems/trapping-rain-water/ | Two pointers / stack |

---

## Phase 2 — Linked List / Stack / Heap (Days 21–32)

| # | Problem | Diff | Link | Pattern |
| ---: | --- | --- | --- | --- |
| 21 | Reverse Linked List | E | https://leetcode.com/problems/reverse-linked-list/ | Pointers |
| 22 | Linked List Cycle | E | https://leetcode.com/problems/linked-list-cycle/ | Floyd |
| 23 | Merge k Sorted Lists | H | https://leetcode.com/problems/merge-k-sorted-lists/ | Heap merge |
| 24 | Reorder List | M | https://leetcode.com/problems/reorder-list/ | Split + reverse |
| 25 | Remove Nth Node From End of List | M | https://leetcode.com/problems/remove-nth-node-from-end-of-list/ | Two pointers |
| 26 | Min Stack | M | https://leetcode.com/problems/min-stack/ | Design |
| 27 | Evaluate Reverse Polish Notation | M | https://leetcode.com/problems/evaluate-reverse-polish-notation/ | Stack |
| 28 | Daily Temperatures | M | https://leetcode.com/problems/daily-temperatures/ | Monotonic stack |
| 29 | Largest Rectangle in Histogram | H | https://leetcode.com/problems/largest-rectangle-in-histogram/ | Monotonic stack |
| 30 | Kth Largest Element in an Array | M | https://leetcode.com/problems/kth-largest-element-in-an-array/ | Heap / Quickselect |
| 31 | Top K Frequent Elements | M | https://leetcode.com/problems/top-k-frequent-elements/ | Heap / bucket |
| 32 | Find Median from Data Stream | H | https://leetcode.com/problems/find-median-from-data-stream/ | Two heaps |

---

## Phase 3 — Trees (Days 33–44)

| # | Problem | Diff | Link | Pattern |
| ---: | --- | --- | --- | --- |
| 33 | Maximum Depth of Binary Tree | E | https://leetcode.com/problems/maximum-depth-of-binary-tree/ | DFS |
| 34 | Same Tree | E | https://leetcode.com/problems/same-tree/ | DFS |
| 35 | Subtree of Another Tree | E | https://leetcode.com/problems/subtree-of-another-tree/ | DFS |
| 36 | Lowest Common Ancestor of a BST | M | https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/ | BST |
| 37 | Lowest Common Ancestor of a Binary Tree | M | https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/ | DFS |
| 38 | Binary Tree Level Order Traversal | M | https://leetcode.com/problems/binary-tree-level-order-traversal/ | BFS |
| 39 | Binary Tree Right Side View | M | https://leetcode.com/problems/binary-tree-right-side-view/ | BFS |
| 40 | Validate Binary Search Tree | M | https://leetcode.com/problems/validate-binary-search-tree/ | Bounds DFS |
| 41 | Kth Smallest Element in a BST | M | https://leetcode.com/problems/kth-smallest-element-in-a-bst/ | Inorder |
| 42 | Construct Binary Tree from Preorder and Inorder Traversal | M | https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/ | Divide & conquer |
| 43 | Binary Tree Maximum Path Sum | H | https://leetcode.com/problems/binary-tree-maximum-path-sum/ | Tree DP |
| 44 | Serialize and Deserialize Binary Tree | H | https://leetcode.com/problems/serialize-and-deserialize-binary-tree/ | Design / BFS |

---

## Phase 4 — Graphs (Days 45–60) — **Staff heavy**

| # | Problem | Diff | Link | Pattern |
| ---: | --- | --- | --- | --- |
| 45 | Number of Islands | M | https://leetcode.com/problems/number-of-islands/ | DFS/BFS |
| 46 | Clone Graph | M | https://leetcode.com/problems/clone-graph/ | Graph copy |
| 47 | Pacific Atlantic Water Flow | M | https://leetcode.com/problems/pacific-atlantic-water-flow/ | Multi-source DFS |
| 48 | Course Schedule | M | https://leetcode.com/problems/course-schedule/ | Cycle / topo |
| 49 | Course Schedule II | M | https://leetcode.com/problems/course-schedule-ii/ | Topo sort |
| 50 | Graph Valid Tree | M | https://leetcode.com/problems/graph-valid-tree/ | Union-Find / DFS |
| 51 | Number of Connected Components in an Undirected Graph | M | https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/ | Union-Find |
| 52 | Redundant Connection | M | https://leetcode.com/problems/redundant-connection/ | Union-Find |
| 53 | Word Ladder | H | https://leetcode.com/problems/word-ladder/ | BFS |
| 54 | Word Ladder II | H | https://leetcode.com/problems/word-ladder-ii/ | BFS + path |
| 55 | Network Delay Time | M | https://leetcode.com/problems/network-delay-time/ | Dijkstra |
| 56 | Cheapest Flights Within K Stops | M | https://leetcode.com/problems/cheapest-flights-within-k-stops/ | Bellman / BFS |
| 57 | Min Cost to Connect All Points | M | https://leetcode.com/problems/min-cost-to-connect-all-points/ | MST |
| 58 | Critical Connections in a Network | H | https://leetcode.com/problems/critical-connections-in-a-network/ | Bridges (Tarjan) |
| 59 | Alien Dictionary | H | https://leetcode.com/problems/alien-dictionary/ | Topo + unique |
| 60 | Shortest Path in Binary Matrix | M | https://leetcode.com/problems/shortest-path-in-binary-matrix/ | 0-1 BFS |

> Some premium problems (#50, #51, #59) need LeetCode Premium. Alternatives (free):  
> - Graph Valid Tree → https://leetcode.com/problems/redundant-connection/  
> - Connected Components → https://leetcode.com/problems/number-of-provinces/  
> - Alien Dictionary → https://leetcode.com/problems/course-schedule-ii/ + custom letter-order twist

---

## Phase 5 — DP (Days 61–75) — **Staff heavy**

| # | Problem | Diff | Link | Pattern |
| ---: | --- | --- | --- | --- |
| 61 | Climbing Stairs | E | https://leetcode.com/problems/climbing-stairs/ | 1D DP |
| 62 | House Robber | M | https://leetcode.com/problems/house-robber/ | 1D DP |
| 63 | House Robber II | M | https://leetcode.com/problems/house-robber-ii/ | Circular DP |
| 64 | Longest Increasing Subsequence | M | https://leetcode.com/problems/longest-increasing-subsequence/ | Patience / DP |
| 65 | Coin Change | M | https://leetcode.com/problems/coin-change/ | Unbounded knapsack |
| 66 | Word Break | M | https://leetcode.com/problems/word-break/ | String DP |
| 67 | Combination Sum | M | https://leetcode.com/problems/combination-sum/ | Backtrack |
| 68 | House Robber → Decode Ways | M | https://leetcode.com/problems/decode-ways/ | String DP |
| 69 | Unique Paths | M | https://leetcode.com/problems/unique-paths/ | Grid DP |
| 70 | Jump Game | M | https://leetcode.com/problems/jump-game/ | Greedy/DP |
| 71 | Jump Game II | M | https://leetcode.com/problems/jump-game-ii/ | Greedy |
| 72 | Longest Common Subsequence | M | https://leetcode.com/problems/longest-common-subsequence/ | 2D DP |
| 73 | Edit Distance | M | https://leetcode.com/problems/edit-distance/ | 2D DP |
| 74 | Maximal Square | M | https://leetcode.com/problems/maximal-square/ | Grid DP |
| 75 | Burst Balloons | H | https://leetcode.com/problems/burst-balloons/ | Interval DP |

---

## Phase 6 — Design problems (Days 76–88) — **maps to Rust systems rounds**

| # | Problem | Diff | Link | Why Staff / Rust orgs ask this |
| ---: | --- | --- | --- | --- |
| 76 | LRU Cache | M | https://leetcode.com/problems/lru-cache/ | Discord/CDN/edge caches |
| 77 | LFU Cache | H | https://leetcode.com/problems/lfu-cache/ | Cache policy trade-offs |
| 78 | Design HashMap | E | https://leetcode.com/problems/design-hashmap/ | Internals thinking |
| 79 | Implement Trie (Prefix Tree) | M | https://leetcode.com/problems/implement-trie-prefix-tree/ | Routing / autocomplete |
| 80 | Design Add and Search Words Data Structure | M | https://leetcode.com/problems/design-add-and-search-words-data-structure/ | Wildcard trie |
| 81 | Time Based Key-Value Store | M | https://leetcode.com/problems/time-based-key-value-store/ | Binary search versions |
| 82 | Design Twitter | M | https://leetcode.com/problems/design-twitter/ | Feed / fan-out lite |
| 83 | Design Underground System | M | https://leetcode.com/problems/design-underground-system/ | Aggregations |
| 84 | Snapshot Array | M | https://leetcode.com/problems/snapshot-array/ | Versioned state |
| 85 | Encode and Decode TinyURL | M | https://leetcode.com/problems/encode-and-decode-tinyurl/ | Hash / biject |
| 86 | Flatten Nested List Iterator | M | https://leetcode.com/problems/flatten-nested-list-iterator/ | Lazy iterators (Rust-y) |
| 87 | Design a Stack With Increment Operation | M | https://leetcode.com/problems/design-a-stack-with-increment-operation/ | Lazy updates |
| 88 | Design Browser History | M | https://leetcode.com/problems/design-browser-history/ | Two stacks / arrays |

---

## Phase 7 — Hard “Staff filter” set (Days 89–110)

| # | Problem | Diff | Link | Signal |
| ---: | --- | --- | --- | --- |
| 89 | Merge Intervals | M | https://leetcode.com/problems/merge-intervals/ | Intervals |
| 90 | Insert Interval | M | https://leetcode.com/problems/insert-interval/ | Intervals |
| 91 | Non-overlapping Intervals | M | https://leetcode.com/problems/non-overlapping-intervals/ | Greedy |
| 92 | Meeting Rooms II | M | https://leetcode.com/problems/meeting-rooms-ii/ | Heap (Premium) → alt: https://leetcode.com/problems/car-pooling/ |
| 93 | Task Scheduler | M | https://leetcode.com/problems/task-scheduler/ | Greedy counts |
| 94 | Least Interval → Hand of Straights | M | https://leetcode.com/problems/hand-of-straights/ | Greedy map |
| 95 | Subarray Sum Equals K | M | https://leetcode.com/problems/subarray-sum-equals-k/ | Prefix + HashMap |
| 96 | Continuous Subarray Sum | M | https://leetcode.com/problems/continuous-subarray-sum/ | Prefix mod |
| 97 | Longest Consecutive Sequence | M | https://leetcode.com/problems/longest-consecutive-sequence/ | HashSet O(n) |
| 98 | Word Search II | H | https://leetcode.com/problems/word-search-ii/ | Trie + DFS |
| 99 | N-Queens | H | https://leetcode.com/problems/n-queens/ | Backtracking |
| 100 | Sudoku Solver | H | https://leetcode.com/problems/sudoku-solver/ | Backtracking |
| 101 | Regular Expression Matching | H | https://leetcode.com/problems/regular-expression-matching/ | DP |
| 102 | Wildcard Matching | H | https://leetcode.com/problems/wildcard-matching/ | DP |
| 103 | Basic Calculator | H | https://leetcode.com/problems/basic-calculator/ | Parser stack |
| 104 | Basic Calculator II | M | https://leetcode.com/problems/basic-calculator-ii/ | Parser |
| 105 | The Skyline Problem | H | https://leetcode.com/problems/the-skyline-problem/ | Sweep line |
| 106 | Swim in Rising Water | H | https://leetcode.com/problems/swim-in-rising-water/ | Binary search + DFS / heap |
| 107 | Bus Routes | H | https://leetcode.com/problems/bus-routes/ | BFS modeling |
| 108 | Parallel Courses III | H | https://leetcode.com/problems/parallel-courses-iii/ | Topo + DP |
| 109 | Find Celebrity | M | https://leetcode.com/problems/find-the-celebrity/ | Premium → alt: https://leetcode.com/problems/find-the-town-judge/ |
| 110 | First Missing Positive | H | https://leetcode.com/problems/first-missing-positive/ | In-place indexing |

---

## Phase 8 — Company-flavored LeetCode (map to your Rust targets)

| Company flavor | Solve these | Links |
| --- | --- | --- |
| **Cloudflare / Fastly** (proxy, limits, cache) | LRU Cache, LFU Cache, Sliding Window Maximum, Design HashMap | https://leetcode.com/problems/lru-cache/ · https://leetcode.com/problems/lfu-cache/ · https://leetcode.com/problems/sliding-window-maximum/ |
| **Discord** (real-time, presence) | Design Twitter, Time Based KV, Top K Frequent, Course Schedule | https://leetcode.com/problems/design-twitter/ · https://leetcode.com/problems/time-based-key-value-store/ · https://leetcode.com/problems/top-k-frequent-elements/ |
| **Dropbox** (sync, files) | Merge k Sorted Lists, Find Duplicate File in System, Longest Consecutive | https://leetcode.com/problems/merge-k-sorted-lists/ · https://leetcode.com/problems/find-duplicate-file-in-system/ · https://leetcode.com/problems/longest-consecutive-sequence/ |
| **AWS** (scheduling, resources) | Task Scheduler, Meeting Rooms II / Car Pooling, Parallel Courses III | https://leetcode.com/problems/task-scheduler/ · https://leetcode.com/problems/car-pooling/ · https://leetcode.com/problems/parallel-courses-iii/ |
| **Google** | Word Ladder, Employee Free Time(premium)/Insert Interval, Median Stream, Skyline | https://leetcode.com/problems/word-ladder/ · https://leetcode.com/problems/insert-interval/ · https://leetcode.com/problems/find-median-from-data-stream/ · https://leetcode.com/problems/the-skyline-problem/ |
| **Meta** | Account Merge, Cut Off Trees(premium)/Bus Routes, Binary Tree Max Path, Valid BST | https://leetcode.com/problems/accounts-merge/ · https://leetcode.com/problems/bus-routes/ · https://leetcode.com/problems/binary-tree-maximum-path-sum/ |
| **Microsoft** | LRU Cache, Serialize Tree, Course Schedule, Edit Distance | https://leetcode.com/problems/lru-cache/ · https://leetcode.com/problems/serialize-and-deserialize-binary-tree/ · https://leetcode.com/problems/course-schedule/ · https://leetcode.com/problems/edit-distance/ |
| **Datadog** | Top K Frequent, Subarray Sum Equals K, Merge Intervals, Network Delay | https://leetcode.com/problems/top-k-frequent-elements/ · https://leetcode.com/problems/subarray-sum-equals-k/ · https://leetcode.com/problems/merge-intervals/ · https://leetcode.com/problems/network-delay-time/ |
| **1Password / Security** | Basic Calculator, Regex Matching, Implement Trie, First Missing Positive | https://leetcode.com/problems/basic-calculator/ · https://leetcode.com/problems/regular-expression-matching/ · https://leetcode.com/problems/implement-trie-prefix-tree/ |
| **Stripe / Payments** | Design Underground System, Snapshot Array, Continuous Subarray Sum | https://leetcode.com/problems/design-underground-system/ · https://leetcode.com/problems/snapshot-array/ · https://leetcode.com/problems/continuous-subarray-sum/ |
| **Trading / Crypto** (Jump, Coinbase, Solana) | Best Time to Buy/Sell Stock III/IV, Max Profit Job Scheduling, Network Delay | https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/ · https://leetcode.com/problems/maximum-profit-in-job-scheduling/ · https://leetcode.com/problems/network-delay-time/ |

### Extra high-ROI links (do after Blind set)

| Problem | Link |
| --- | --- |
| Accounts Merge | https://leetcode.com/problems/accounts-merge/ |
| Number of Provinces | https://leetcode.com/problems/number-of-provinces/ |
| Rotting Oranges | https://leetcode.com/problems/rotting-oranges/ |
| Walls and Gates (Premium) → 01 Matrix | https://leetcode.com/problems/01-matrix/ |
| Find Duplicate File in System | https://leetcode.com/problems/find-duplicate-file-in-system/ |
| Maximum Profit in Job Scheduling | https://leetcode.com/problems/maximum-profit-in-job-scheduling/ |
| Best Time to Buy and Sell Stock III | https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/ |
| Best Time to Buy and Sell Stock IV | https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/ |
| Car Pooling | https://leetcode.com/problems/car-pooling/ |
| Find the Town Judge | https://leetcode.com/problems/find-the-town-judge/ |
| Prefix and Suffix Search | https://leetcode.com/problems/prefix-and-suffix-search/ |
| Design Hit Counter (Premium) → hit rate: use https://leetcode.com/problems/logger-rate-limiter/ if available, else implement yourself from daily Q5 | — |
| Logger Rate Limiter | https://leetcode.com/problems/logger-rate-limiter/ |
| Design Bounded Blocking Queue (Java concurrency) | https://leetcode.com/problems/design-bounded-blocking-queue/ — implement the same API in Rust with `Mutex` + `Condvar` locally |
| Print in Order | https://leetcode.com/problems/print-in-order/ — practice ordering; implement in Rust with channels/`Barrier` |
| Fizz Buzz Multithreaded | https://leetcode.com/problems/fizz-buzz-multithreaded/ — same idea in Rust |

---

## 12-week Staff plan (Rust + LeetCode)

| Week | Focus | Problems | Also do (from your repo) |
| ---: | --- | --- | --- |
| 1 | Warm-up + arrays | #1–14 | — |
| 2 | Windows / hard arrays | #15–20 | Cloudflare daily Q20 |
| 3 | Lists / heaps | #21–32 | Discord Q22 |
| 4 | Trees | #33–44 | — |
| 5 | Graphs I | #45–52 | Zscaler Q44 |
| 6 | Graphs II | #53–60 | AWS Q16 |
| 7 | DP I | #61–68 | — |
| 8 | DP II + hard | #69–75 | — |
| 9 | Design | #76–88 | LRU + rate limiter company Qs |
| 10 | Staff hard set | #89–100 | Dropbox Q31 |
| 11 | Staff hard set | #101–110 | 1Password Q49 |
| 12 | Company-tagged review | Phase 8 table | System design + daily company Qs |

**Cadence:** 1–2 LeetCode/day + 1 company systems question every 2 days.

---

## Rust tips on LeetCode

| Topic | Do this |
| --- | --- |
| Ownership | Prefer indexes over cloning nodes when possible |
| Linked lists | Use `Option<Box<ListNode>>`; practice take/replace |
| Graphs | `Vec<Vec<usize>>` + `Vec<bool>` visited |
| Heaps | `BinaryHeap` (max-heap); negate for min-heap |
| Strings | `&[u8]` often faster/cleaner than `chars()` |
| Hash maps | `HashMap` / `HashSet` from `std::collections` |
| Recursion | Watch stack depth on Hard DFS; convert to iterative if needed |
| Integers | Be careful with `i32` overflow; use `i64` when summing |

Template mindset before Submit:

```rust
impl Solution {
    pub fn solve(input: Vec<i32>) -> i32 {
        // 1) clarify invariants
        // 2) choose structure
        // 3) implement
        // 4) complexity comment in head: O(n) time / O(1) space
        0
    }
}
```

---

## Must-pass shortlist (if interview is in 2 weeks)

Do these **in order**, all in Rust:

1. https://leetcode.com/problems/lru-cache/
2. https://leetcode.com/problems/merge-k-sorted-lists/
3. https://leetcode.com/problems/course-schedule/
4. https://leetcode.com/problems/number-of-islands/
5. https://leetcode.com/problems/word-ladder/
6. https://leetcode.com/problems/network-delay-time/
7. https://leetcode.com/problems/coin-change/
8. https://leetcode.com/problems/longest-increasing-subsequence/
9. https://leetcode.com/problems/edit-distance/
10. https://leetcode.com/problems/binary-tree-maximum-path-sum/
11. https://leetcode.com/problems/serialize-and-deserialize-binary-tree/
12. https://leetcode.com/problems/minimum-window-substring/
13. https://leetcode.com/problems/sliding-window-maximum/
14. https://leetcode.com/problems/trapping-rain-water/
15. https://leetcode.com/problems/find-median-from-data-stream/
16. https://leetcode.com/problems/time-based-key-value-store/
17. https://leetcode.com/problems/accounts-merge/
18. https://leetcode.com/problems/critical-connections-in-a-network/
19. https://leetcode.com/problems/maximum-profit-in-job-scheduling/
20. https://leetcode.com/problems/first-missing-positive/

Plus locally (not only LeetCode): rate limiter, worker pool, TCP proxy — from `daily_company_rust_questions.md`.

---

## Progress tracker

Copy this into a note:

```text
[ ] Phase 0 warm-up (1–7)
[ ] Phase 1 arrays/windows (8–20)
[ ] Phase 2 list/stack/heap (21–32)
[ ] Phase 3 trees (33–44)
[ ] Phase 4 graphs (45–60)
[ ] Phase 5 DP (61–75)
[ ] Phase 6 design (76–88)
[ ] Phase 7 hard staff (89–110)
[ ] Phase 8 company-flavored
[ ] 2-week must-pass shortlist (20)
```

Related files:
- Company systems questions: [`daily_company_rust_questions.md`](./daily_company_rust_questions.md)
- Company salary rank + domains: [`coding_question.md`](./coding_question.md)
