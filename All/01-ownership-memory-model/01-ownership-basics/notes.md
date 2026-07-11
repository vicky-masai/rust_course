# Day 1 — Ownership Basics

> Module 01 — Ownership & Memory Model
> Format: every question answered the way a senior engineer would answer their CTO — direct, precise, no filler.

---

## Q: What is ownership in Rust?

Ownership is Rust's compile-time system for memory management. Three rules: every value has exactly one owner, there can only be one owner at a time, and when the owner goes out of scope the value is destroyed. The compiler enforces all three rules during compilation, which means memory safety costs us nothing at runtime — no garbage collector, no reference counting by default, no manual `free()`.

## Q: Why did the Rust team design it this way? What problem does it solve?

Every language before Rust picked one of two trade-offs. C and C++ give you manual memory management: fast, but the source of the majority of critical security vulnerabilities in the industry — use-after-free, double free, buffer issues. Microsoft and Google both attribute roughly 70% of their serious security bugs to memory safety. Java, Go, and Python use a garbage collector: safe, but you pay with CPU cycles, memory overhead, and unpredictable pause times, which matters for latency-sensitive services.

Ownership is the third option: the compiler proves at build time exactly when each value can be freed and inserts the cleanup automatically. We get C-level performance and memory safety at the same time. That's the core business case for Rust.

## Q: What does "a move" mean in practice?

Assignment and function calls transfer ownership rather than copying data. If I assign a `String` from variable `a` to variable `b`, `b` becomes the new owner and the compiler invalidates `a` — any later use of `a` is a compile error (`E0382: borrow of moved value`). Nothing happens at runtime; the invalidation is purely a compile-time rule.

This is the opposite default from C++, where assignment silently deep-copies. Rust makes the cheap operation (move) the default and forces the expensive operation (clone) to be explicit in the code. Performance costs are visible in review.

## Q: If moves invalidate the source, how does any code get written? Do we clone everywhere?

No. Three tools cover every case:

1. **Borrowing (`&T`, `&mut T`)** — the common case. A function that only needs to read or temporarily modify a value takes a reference and ownership never changes hands. Covered in depth on Day 3.
2. **Returning ownership** — a function can take a value in and hand it back in its return value. Useful for builder-style APIs.
3. **`.clone()`** — an explicit deep copy when we genuinely need two independent copies. It allocates, so it shows up in profiling and code review — which is the point.

A team new to Rust typically over-clones for the first month. That's fine; correctness first, then remove clones as the borrow checker becomes intuitive.

## Q: Why do integers behave differently? `let b = a;` leaves both usable.

Types that are small and live entirely on the stack — `i32`, `bool`, `char`, `f64`, tuples of them — implement the `Copy` trait. For them, assignment is a bitwise copy, both variables stay valid, and there is no ownership transfer. The rule of thumb: if duplicating the value is as cheap as copying a pointer, Rust copies; if the value owns heap resources, Rust moves. `Copy` vs `Clone` gets a full day (Day 5).

## Q: When exactly is memory freed?

At the end of the owner's scope — the nearest closing brace — the compiler inserts a call to the value's destructor (`drop`). Deterministic, immediate, and exactly once. Not "eventually" like a GC, and not "whenever the developer remembered" like C. This determinism is also why Rust handles non-memory resources well: files, sockets, and locks release at scope end using the same mechanism (RAII).

## Q: What's the runtime cost of all this?

Zero. Ownership checking happens entirely in the compiler. A move compiles to at most a small stack copy (often optimized away entirely). This is the concrete meaning of "zero-cost abstraction": the safety checks leave no trace in the compiled binary.

## Q: What's the risk or downside I should know about?

Learning curve. Engineers fight the borrow checker for the first few weeks because it rejects patterns that are legal in other languages — sharing mutable state freely, self-referential structures, certain graph designs. The patterns that replace them are well documented, but budget onboarding time. The payoff is that entire bug classes — use-after-free, double free, data races — are unrepresentable in safe Rust, so they never reach production.

---

## Key takeaways

- One value, one owner; assignment and function calls **move** ownership.
- Freeing is **deterministic**: at the owner's closing brace, inserted by the compiler, exactly once.
- Moves are silent because they're cheap; **clones are explicit** because they're not.
- `Copy` types (integers, bools, chars) are duplicated instead of moved.
- All of it is compile-time: **zero runtime cost**.

## Practice

Answer the questions in [assignment.md](assignment.md) from memory, out loud or in writing, before moving to Day 2.
