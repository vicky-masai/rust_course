# Trie (Prefix Tree)

## Interview Question

What is a Trie and when would you use it in a backend system?

## Interview Answer

A Trie is a tree-like data structure where each node represents a character, and paths from root to leaf represent strings. It supports **O(m)** insertion, lookup, and deletion where `m` is the string length — independent of the number of stored strings. Tries are ideal for **prefix-based operations**: autocomplete, spell checking, IP routing (longest prefix match), and dictionary lookups. In backend systems, Tries power search suggestions, URL routing in web frameworks, and IP lookup in networking equipment. A compressed Trie (Radix Tree / Patricia Trie) merges single-child nodes to reduce memory usage.

**Time Complexity**: O(m) for insert/search/delete (m = key length)
**Space Complexity**: O(ALPHABET_SIZE × m × n) worst case, much less with compression

---

## Follow-up Questions & Answers

### Q1. What is the difference between a Trie, Radix Tree, and Suffix Tree?

**Interview Answer**

A **Trie** stores one character per edge, which can waste space for strings with common prefixes. A **Radix Tree** (Patricia Trie) compresses chains of single-child nodes into single edges, storing multiple characters per edge. This reduces memory usage significantly. A **Suffix Tree** stores all suffixes of a string, enabling substring search in O(m) time after O(n) preprocessing. For backend systems, Radix Trees are most common — they're used in Linux kernel's memory management, Redis's Rax tree (used for Streams), and URL routing in HTTP frameworks like Axum and Actix.

---

### Q2. How would you implement a Trie in Rust?

**Interview Answer**

```rust
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie { root: TrieNode { children: HashMap::new(), is_end: false } }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(TrieNode {
                children: HashMap::new(),
                is_end: false,
            });
        }
        node.is_end = true;
    }

    fn search(&self, word: &str) -> bool {
        let node = self.find_node(word);
        node.map_or(false, |n| n.is_end)
    }

    fn starts_with(&self, prefix: &str) -> bool {
        self.find_node(prefix).is_some()
    }

    fn find_node(&self, prefix: &str) -> Option<&TrieNode> {
        let mut node = &self.root;
        for ch in prefix.chars() {
            node = node.children.get(&ch)?;
        }
        Some(node)
    }
}
```

Using `HashMap<char, TrieNode>` instead of an array allows Unicode support without wasting space on unused characters.

---

### Q3. How is a Trie used in URL routing for web frameworks?

**Interview Answer**

Web frameworks like Axum, Actix, and Rocket use Radix Trees for URL routing. Each route pattern (e.g., `/users/:id/posts`) becomes a path in the tree. Static segments are merged into single edges, and parameter segments create branching points. When a request arrives, the router traverses the tree character by character. This is faster than regex-based routing for large route tables because it avoids scanning all routes. The `matchit` crate (used by Axum) implements a Radix Tree router that handles static routes, parameterized routes (`:id`), and catch-all routes (`*path`). Lookup is O(url_length) regardless of the number of routes.

---

### Q4. How do you implement autocomplete using a Trie?

**Interview Answer**

Autocomplete works by finding the node for the given prefix, then collecting all words in the subtree rooted at that node. To optimize, store a count of words in each subtree to know how many completions exist without traversing. For ranked autocomplete, store a frequency counter at each end-of-word node and sort completions by frequency. For large-scale systems (Google search suggestions), the Trie is stored in memory across distributed servers, with the top-K suggestions precomputed at each node. In Rust, collect completions using DFS from the prefix node:

```rust
fn autocomplete(&self, prefix: &str) -> Vec<String> {
    let mut results = Vec::new();
    if let Some(node) = self.find_node(prefix) {
        self.dfs(node, prefix.to_string(), &mut results);
    }
    results
}

fn dfs(&self, node: &TrieNode, current: String, results: &mut Vec<String>) {
    if node.is_end { results.push(current.clone()); }
    for (ch, child) in &node.children {
        self.dfs(child, format!("{}{}", current, ch), results);
    }
}
```

---

### Q5. What is a Trie's memory overhead and how to reduce it?

**Interview Answer**

A naive Trie uses a `HashMap<char, TrieNode>` per node, which has significant overhead — HashMap itself is ~56 bytes plus the heap allocation for the map entries. For ASCII text, a `Vec<Option<Box<TrieNode>>>` of size 26 (or 128 for extended ASCII) is more compact. Further optimization: **Double-array Trie** stores nodes in two arrays (base and check) for cache-friendly access. **Ternary Search Trie** uses three pointers per node (less, equal, greater) for a balance between Trie and BST. In Rust, the `trie` and `radix_trie` crates provide optimized implementations. For production, consider memory-mapped Tries that can be shared across processes.

---

### Q6. How does a Trie compare to a HashMap for string lookups?

**Interview Answer**

A **HashMap** provides O(1) average-case lookup but needs to hash the entire key, uses O(n × key_length) memory (where n is number of entries), and doesn't support prefix operations. A **Trie** provides O(key_length) lookup, shares prefix storage (reducing memory for strings with common prefixes), and natively supports prefix queries (autocomplete, prefix matching). For **exact match only** with no prefix needs, HashMap is simpler and often faster in practice due to cache locality. For prefix operations, ranked search, or lexicographic ordering, Trie is superior. In backend systems, Tries are used when prefix matching is needed (IP routing, autocomplete), while HashMaps are used for exact key-value lookups.

---

### Q7. What are compressed Tries (Radix Trees) and why are they important?

**Interview Answer**

A compressed Trie merges chains of single-child nodes into a single edge with a multi-character label. For example, if only "server" and "service" are stored, an uncompressed Trie has 7 nodes for "server", while a compressed Trie shares the "serv" prefix and branches at "e" vs "ice". This reduces node count from O(total_characters) to O(number_of_strings × 2). Redis uses a Radix Tree (called Rax) internally for its Stream data structure. Linux's page cache uses a Radix Tree for page lookup. The `matchit` crate used by Axum implements a Radix Tree for URL routing. Memory savings can be 50-80% compared to uncompressed Tries.

---

### Q8. How would you persist a Trie to disk or database?

**Interview Answer**

Tries can be serialized using: **Pre-order traversal** — store each node as (character, is_end, child_count), then recursively store children. **Adjacency list** — store nodes as (node_id, char, is_end, parent_id). **Array-based** — use the double-array representation where nodes are stored in contiguous memory. For database storage, PostgreSQL's `ltree` extension provides prefix tree indexing using a compact binary format. For distributed systems, each level of the Trie can be sharded by prefix range. In Rust, use `bincode` or `serde` for serialization. For memory-mapped persistence, ensure the serialization format is endianness-independent and supports random access.

---

### Q9. What is a Aho-Corasick automaton and how does it relate to Tries?

**Interview Answer**

Aho-Corasick is a **multi-pattern string matching** algorithm built on top of a Trie. It constructs a finite automaton from a set of patterns, then processes the text in a single pass, finding all occurrences of all patterns simultaneously in **O(n + m + z)** time where n is text length, m is total pattern length, and z is number of matches. It adds "failure links" (similar to KMP's failure function) to the Trie to avoid backtracking. Used in: intrusion detection systems (matching multiple attack signatures), text editors (multi-pattern find), and log analysis (searching for multiple error patterns). The `aho-corasick` crate in Rust provides a high-performance implementation using SIMD acceleration.

---

### Q10. How are Tries used in system design for IP routing and DNS lookup?

**Interview Answer**

**IP Routing**: Routers use **PATRICIA Tries** (Radix Trees) for longest prefix matching. Each IP prefix (e.g., `192.168.0.0/16`) maps to a next-hop interface. When a packet arrives, the router traverses the Trie by IP bits, finding the longest matching prefix. This is done in hardware (TCAM) for line-rate forwarding. **DNS Lookup**: DNS resolvers cache domain names in a Trie-like structure where each label (com, google, www) forms a level. Reverse DNS (IP → hostname) uses a Trie keyed on reversed IP octets. In backend systems, route tables in service meshes (Istio, Linkerd) use Trie-based matching for traffic routing rules. The `cidr` crate in Rust handles CIDR prefix matching efficiently.
