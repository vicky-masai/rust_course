# Assignment — Ownership Basics

Answer these from memory — out loud, as if in an interview. If you can't answer one cleanly, reread that section of [notes.md](notes.md) and try again. Don't move to Day 2 until you can answer all of them without hesitation.

## Warm-up

1. State the three rules of ownership.
2. Why doesn't Rust need a garbage collector?
3. What is the compile error code and message you get when you use a moved value?

## Core

4. `let a = String::from("hi"); let b = a;` — describe what happened to `a` and `b`, and what would happen if you tried to print `a`.
5. The same two lines, but with `let a = 5;` — why does printing `a` still work?
6. You pass a `String` into a function and need to use it afterwards. Give three different ways to make that compile, and say when you'd pick each.
7. What is the difference between a move and a clone — in terms of what gets copied and what it costs?
8. When exactly is a value's memory freed? Be precise about *where* in the code it happens.

## Interview-level

9. Explain to a non-Rust engineer why "assignment moves by default, clone is explicit" is a deliberate design choice. What does C++ do instead, and what problem does that cause?
10. "Rust's move semantics have zero runtime cost." Defend or refute this statement precisely — what happens to the old variable's memory at runtime after a move?
11. Ownership eliminates double frees at compile time. Walk through the reasoning: what exactly would go wrong if Rust allowed two owners of one `String`?
12. Your team is coming to Rust from Go. What ownership-related friction should you warn them about in the first month, and what's your argument for why it's worth it?
