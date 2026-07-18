# KMP String Matching

## Interview Question

What is the KMP (Knuth-Morris-Pratt) algorithm and when would you use it?

## Interview Answer

KMP is a linear-time string matching algorithm that finds occurrences of a pattern `P` within a text `T`. It preprocesses the pattern to build a **failure function** (partial match table / LPS array) that determines how much of the pattern can be skipped after a mismatch. This avoids re-checking characters that have already been matched. Time: **O(n + m)** where n = text length, m = pattern length. Without KMP, naive string matching is O(n × m). KMP is used in: text editors (find/replace), DNA sequence matching, log searching, intrusion detection (pattern scanning), and network packet inspection.

**Time Complexity**: O(n + m)
**Space Complexity**: O(m) for the failure function

---

## Follow-up Questions & Answers

### Q1. How does the KMP failure function (LPS array) work?

**Interview Answer**

The LPS (Longest Proper Prefix which is also Suffix) array stores for each position `i` in the pattern, the length of the longest proper prefix of `P[0..=i]` that is also a suffix. For pattern "ABABAC": LPS = [0, 0, 1, 2, 3, 0]. When a mismatch occurs at position `i`, instead of restarting from the beginning of the pattern, we jump to position `LPS[i-1]`. This works because the first `LPS[i-1]` characters of the pattern already match the text (they were the suffix of the matched portion). The LPS array is built in O(m) time using a two-pointer technique.

---

### Q2. How would you implement KMP in Rust?

**Interview Answer**

```rust
fn build_lps(pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let mut lps = vec![0; m];
    let mut len = 0;
    let mut i = 1;

    while i < m {
        if pattern[i] == pattern[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len > 0 {
            len = lps[len - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }
    lps
}

fn kmp_search(text: &[u8], pattern: &[u8]) -> Vec<usize> {
    let (n, m) = (text.len(), pattern.len());
    if m == 0 || m > n { return vec![]; }

    let lps = build_lps(pattern);
    let mut matches = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < n {
        if text[i] == pattern[j] {
            i += 1;
            j += 1;
            if j == m {
                matches.push(i - j);
                j = lps[j - 1];
            }
        } else if j > 0 {
            j = lps[j - 1];
        } else {
            i += 1;
        }
    }
    matches
}
```

---

### Q3. What is the difference between KMP and Boyer-Moore?

**Interview Answer**

**KMP**: O(n + m) worst case. Processes text left-to-right. Uses pattern preprocessing (LPS array). Best for: small alphabets, patterns with many repeated prefixes, and when worst-case guarantees matter. **Boyer-Moore**: O(n × m) worst case but O(n/m) best case. Processes text right-to-left. Uses bad character and good suffix heuristics. Best for: large alphabets (English text), long patterns, and practical performance. Boyer-Moore is generally faster in practice because it skips large portions of text. KMP is preferred when: worst-case linear time is required, the alphabet is small (DNA), or the pattern has structure.

---

### Q4. How is KMP used in network intrusion detection?

**Interview Answer**

Intrusion Detection Systems (IDS) like Snort scan network packets for malicious patterns (signatures). KMP enables fast pattern matching across packet payloads. With thousands of signatures, **Aho-Corasick** (multi-pattern KMP) is used — it builds a single automaton from all patterns and scans the text once, finding all matches in O(n + m + z) time. For single-pattern scenarios (matching one known attack signature), KMP is sufficient. In Rust, the `aho-corasick` crate provides SIMD-accelerated multi-pattern matching. Network appliances process gigabits per second — KMP's linear time is essential for line-rate inspection.

---

### Q5. How does KMP handle patterns with many repeated characters?

**Interview Answer**

KMP excels with repetitive patterns. For pattern "AAAAAB" with LPS = [0, 1, 2, 3, 4, 0], a mismatch at position 5 skips back to position 4 (not 0), avoiding 4 redundant comparisons. For text "AAAAAAAAAAB", naive matching would be O(11 × 6) = 66 operations, while KMP is O(11 + 6) = 17 operations. The LPS array captures the pattern's internal structure. For extremely repetitive patterns (like DNA with long runs of the same base), KMP's advantage is significant. The LPS array for "AAAAA..." is [0, 1, 2, ..., m-1], enabling maximum skip on each mismatch.

---

### Q6. Can KMP be used for multiple pattern matching?

**Interview Answer**

KMP alone handles one pattern at a time. For multiple patterns: run KMP separately for each pattern (O(k × (n + m)) total) or use **Aho-Corasick** (O(n + m + z) total). Aho-Corasick builds a trie of all patterns with failure links (similar to KMP's LPS but generalized to a tree). It processes the text in a single pass, following trie transitions and failure links. Used in: virus scanners (matching thousands of signatures), spam filters (matching keyword lists), and search engines (multi-keyword search). The `aho-corasick` crate in Rust handles this efficiently with SIMD acceleration.

---

### Q7. What is the relationship between KMP and finite automata?

**Interview Answer**

KMP's failure function is essentially a **deterministic finite automaton (DFA)** for pattern matching. The LPS array defines state transitions: state `i` represents having matched the first `i` characters. On a match, transition to state `i+1`. On a mismatch, follow the failure link (like an DFA's error transition). The full DFA would have O(m × |Σ|) transitions (Σ = alphabet), but KMP's LPS achieves the same effect in O(m) space by computing transitions on-the-fly. This connection is theoretical — KMP is more practical than explicitly building the full DFA. The Aho-Corasick algorithm makes this automaton connection explicit for multiple patterns.

---

### Q8. How does KMP perform with Unicode/multi-byte strings?

**Interview Answer**

KMP works on any sequence of comparable elements — not just ASCII bytes. For Unicode strings, compare characters (`char` in Rust) instead of bytes. The LPS array is built on character comparisons. For UTF-8 encoded strings, you can either: (1) decode to `Vec<char>` first, then run KMP on chars — O(n) decode + O(n + m) matching. (2) Run KMP on raw bytes — works for ASCII but may produce false matches at UTF-8 boundaries. The correct approach is (1). In Rust: `text.chars().collect::<Vec<_>>()` and `pattern.chars().collect::<Vec<_>>()`, then apply KMP. Performance is slightly worse due to char decoding, but correctness is guaranteed.

---

### Q9. What is the Z algorithm and how does it compare to KMP?

**Interview Answer**

The Z algorithm computes the Z-array: `Z[i]` = length of the longest substring starting at position `i` that matches a prefix of the string. For pattern matching, concatenate `P + $ + T` and compute Z-array. Any `Z[i] >= m` (pattern length) indicates a match at position `i - m - 1` in T. Z-algorithm is conceptually simpler than KMP and also runs in O(n + m). The Z-array and LPS array are inverses of each other — you can derive one from the other. Z-algorithm is preferred for: suffix array construction, and problems where "matching prefix from position i" is the natural formulation. KMP is preferred for streaming pattern matching.

---

### Q10. How is KMP optimized in production implementations?

**Interview Answer**

**SIMD acceleration**: Process multiple characters per instruction (SSE2/AVX2 for 16/32 bytes at once). The `memchr` and `aho-corasick` crates in Rust use SIMD. **Bitap algorithm**: KMP variant using bit-parallelism for patterns up to 64 characters — processes one character using bitwise operations on a 64-bit word. **Shift-AND/Shift-OR**: Similar bit-parallel approach with simpler logic. **Two-way algorithm**: Combines KMP's ideas with Crochemore-Perrin's technique for better cache performance. **Pre-filtering**: Use a fast check (hash of first few characters) before full KMP matching. In Rust, `regex` crate uses automata-based matching that incorporates KMP ideas. For simple cases, `str::find()` uses optimized byte-scanning.
