# Module 02 — Lifetimes

Lifetimes are how Rust proves, at compile time, that every reference points to data that still exists. They look scary (`<'a>`), but they describe something simple: "this reference must not outlive the thing it points to."

**Prerequisites:** Module 01 (Ownership & Memory Model).

## Topics (one per day)

| Day | Topic | Folder |
|-----|-------|--------|
| 15 | Lifetime elision | `01-lifetime-elision/` |
| 16 | Explicit lifetimes | `02-explicit-lifetimes/` |
| 17 | Lifetime bounds | `03-lifetime-bounds/` |
| 18 | The 'static lifetime | `04-static-lifetime/` |
| 19 | Higher-Ranked Trait Bounds (HRTB) | `05-hrtb/` |
| 20 | Generic lifetime parameters | `06-generic-lifetime-parameters/` |
| 21 | Lifetime variance | `07-lifetime-variance/` |
| 22 | Lifetime subtyping | `08-lifetime-subtyping/` |
| 23 | Async lifetime challenges | `09-async-lifetime-challenges/` |
