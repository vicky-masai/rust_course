# Assignment — Ownership Internals

Answer these from memory — out loud, as if in an interview. If you can't answer one cleanly, reread that section of [notes.md](notes.md) and try again.

## Warm-up

1. Draw (on paper) the memory layout of `let s = String::from("hello");` — label the stack part, the heap part, and the three fields of the handle.
2. How many bytes is the stack part of a `String` on a 64-bit machine, and what are its three fields?
3. Which other standard type has exactly the same layout as `String`?

## Core

4. `let b = a;` where `a` is a `String` holding 1 GB of text. Exactly which bytes get copied? What happens to the heap buffer?
5. Why must Rust invalidate `a` after that move? Name the specific bug class this prevents and explain how it would occur with two owners.
6. Does invalidating a moved-from variable write anything to memory at runtime? Justify your answer.
7. When and where is a heap buffer freed? Be precise: function end or scope end? Who inserts the free?
8. What does `std::mem::drop(value)` actually do? Why does it need no special language support?

## Interview-level

9. "Moving a large String is expensive, so I clone it once and pass references everywhere." Find the two errors in this sentence.
10. Compare Rust moves with C++ `std::move`: what state is the moved-from object in, in each language, and which class of bug does Rust eliminate that C++ cannot?
11. Your service has a latency SLA of 5 ms p99. Explain to your CTO why Rust's scope-based deallocation helps meet it where a garbage-collected language might struggle.
12. A junior engineer asks: "If the stack is faster, why not put everything on the stack?" Give the two hard constraints that make the heap necessary.
13. You're reviewing a PR with `data.clone()` inside a request handler's hot loop. What questions do you ask, and what alternatives do you suggest?
