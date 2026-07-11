# Day 2 — Ownership Internals (stack, heap, and who frees what)

> Module 01 — Ownership & Memory Model
> Format: every question answered the way a senior engineer would answer their CTO — direct, precise, no filler.

---

## Q: Yesterday you said assignment "moves" a value. What actually happens in memory?

A move copies only the small, fixed-size stack portion of the value and invalidates the source variable. For a `String`, that stack portion is three machine words — pointer to the heap buffer, length, and capacity — 24 bytes on a 64-bit machine. The heap data itself is never touched. Moving a `String` holding 1 GB of text costs exactly the same as moving one holding 5 bytes: a 24-byte copy.

## Q: Walk me through the memory layout of a String.

Two parts:

```text
 STACK (24 bytes on 64-bit)          HEAP
 ┌──────────┬────────┬──────────┐   ┌───┬───┬───┬───┬───┐
 │ ptr ─────┼────────┼──────────┼─► │ h │ e │ l │ l │ o │
 │ len: 5   │        │ cap: 5   │   └───┴───┴───┴───┴───┘
 └──────────┴────────┴──────────┘
```

- **Stack part:** the handle — pointer, length (bytes in use), capacity (bytes allocated). Fixed size, known at compile time.
- **Heap part:** the actual character data. Variable size, allocated at runtime.

`Vec<T>` has the identical layout. This handle-plus-buffer pattern is how Rust represents nearly every growable structure.

## Q: Why does the stack/heap split exist at all?

The stack is fast — allocation is just moving a pointer — but it requires sizes known at compile time and values die in strict LIFO order with their function frame. The heap handles everything else: data that grows, shrinks, or must outlive the function that created it. The cost is that something must track when each heap allocation can be freed. In C that's the programmer's job; in Java it's the GC's job; in Rust it's the ownership system's job, resolved at compile time.

## Q: So why must the old variable be invalidated after a move?

Because after copying the stack part, two handles point at the same heap buffer. Both owners would run their destructor at scope end, and the buffer would be freed twice — a double free, which is undefined behavior and a classic security vulnerability in C. Rust's rule "invalidate the source, exactly one owner survives" guarantees exactly one free per allocation, proven at compile time. This single rule is the entire reason move semantics exist.

## Q: Is anything written to memory when a variable is invalidated?

No. The old variable's bytes are untouched — no zeroing, no flag. The compiler simply refuses to compile any code that uses the variable after the move. It's bookkeeping in the type checker, not an operation in the binary. That's why the claim "move checking has zero runtime cost" is literally true.

## Q: If moves are so cheap, what's the expensive operation I should watch for in review?

`.clone()` on heap-owning types. Clone allocates a new buffer and copies all the data — that's the real deep copy. Rust's design makes this visible: a silent `=` is always cheap, and anything expensive requires the word `clone` in the diff. When reviewing Rust code for performance, grep for `clone()` in hot paths; each one is a candidate for borrowing instead.

## Q: When is the heap buffer actually freed?

At the closing brace of the final owner's scope. The compiler inserts a destructor call (`drop`) there. Three properties matter:

1. **Deterministic** — we know at compile time exactly which line frees it, unlike a GC.
2. **Immediate** — memory pressure doesn't build up waiting for a collection cycle.
3. **Exactly once** — guaranteed by single ownership.

Note it's the end of the *scope* (nearest `}`), not the end of the function. An inner block `{ ... }` frees its values at its own closing brace. You can also free early with `std::mem::drop(value)` — which is nothing magical, just a function that takes ownership and lets the value die inside it.

## Q: How does this compare to what C++ does with move semantics?

C++11 added moves as an opt-in (`std::move`, rvalue references), but the moved-from object stays alive in a "valid but unspecified state" — the language can't stop you from using it, so it must remain destructible, and misuse is a runtime bug. Rust made moves the default and enforcement compile-time: using a moved-from variable isn't a bug you catch in code review, it's code that doesn't build. Same idea, but Rust closes the safety hole.

## Q: Bottom line for the team?

- Stop fearing moves — they're a 24-byte copy regardless of data size.
- Fear (or at least review) clones — they're the real allocations.
- Trust scope-based cleanup — it's deterministic and it extends beyond memory to files, sockets, and locks (RAII).

---

## Key takeaways

- A `String`/`Vec` is a **stack handle (ptr, len, cap)** pointing at a **heap buffer**.
- A **move copies the 24-byte handle only**; the heap data never moves.
- Source invalidation exists to make **double frees impossible**; it's pure compile-time bookkeeping.
- The buffer is freed **exactly once, at the final owner's closing brace**.
- `=` is always cheap; **`clone()` is the explicit, expensive copy**.

## Practice

Answer the questions in [assignment.md](assignment.md) from memory before moving to Day 3.
