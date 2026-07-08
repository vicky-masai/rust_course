# Module 07 — Tokio Deep Dive

Tokio is the async runtime that powers most of production Rust. This module covers not just the API (`spawn`, `select!`, channels) but the machinery underneath: the scheduler, work stealing, and cooperative scheduling.

**Prerequisites:** Module 06 (Async Rust).

## Topics (one per day)

| Day | Topic | Folder |
|-----|-------|--------|
| 73 | The Tokio runtime | `01-tokio-runtime/` |
| 74 | Scheduler internals | `02-scheduler-internals/` |
| 75 | Work stealing | `03-work-stealing/` |
| 76 | Cooperative scheduling | `04-cooperative-scheduling/` |
| 77 | spawn | `05-spawn/` |
| 78 | spawn_blocking | `06-spawn-blocking/` |
| 79 | JoinHandle | `07-joinhandle/` |
| 80 | JoinSet | `08-joinset/` |
| 81 | select! | `09-select/` |
| 82 | timeout | `10-timeout/` |
| 83 | Semaphore | `11-semaphore/` |
| 84 | Notify | `12-notify/` |
| 85 | Barrier | `13-barrier/` |
| 86 | Tokio MPSC channels | `14-mpsc-channels/` |
| 87 | Broadcast channels | `15-broadcast-channels/` |
| 88 | Watch channels | `16-watch-channels/` |
