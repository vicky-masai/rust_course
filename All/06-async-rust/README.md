# Module 06 — Async Rust

Async lets one thread juggle thousands of tasks. But in Rust, `async/await` is not magic — it compiles down to state machines you can understand piece by piece: `Future`, `Poll`, `Waker`, `Context`. This module opens the black box.

**Prerequisites:** Modules 01–05.

## Topics (one per day)

| Day | Topic | Folder |
|-----|-------|--------|
| 64 | The Future trait | `01-future-trait/` |
| 65 | Poll | `02-poll/` |
| 66 | Waker | `03-waker/` |
| 67 | Context | `04-context/` |
| 68 | Async/await internals | `05-async-await-internals/` |
| 69 | Async state machines | `06-async-state-machines/` |
| 70 | Async cancellation | `07-async-cancellation/` |
| 71 | Async streams | `08-async-streams/` |
| 72 | Backpressure | `09-backpressure/` |
