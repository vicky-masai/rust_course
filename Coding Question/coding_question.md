# Rust Coding Questions — Top Product MNCs

Interview style: **1-hour coding**. Implement the core logic. Explain trade-offs.

**Daily practice (1 question per company):** see [`daily_company_rust_questions.md`](./daily_company_rust_questions.md) — Day 1→100 mapped to each company’s Rust business use case.

---

## Top 100 companies using Rust (ranked by est. avg US Rust salary)

**How ranking works**
- Ranked by estimated **average US base salary** for a mid–senior Rust engineer (2025–2026 market).
- Figures are **estimates** from public salary guides / job posts (Glassdoor, Levels-ish ranges, RustJobs, company postings) — not official company salary bands.
- **TC** (total comp) at big tech is often much higher after equity/bonus.
- Global / remote roles are often 10–30% lower than US onsite/hybrid.

**Market anchors (US)**
| Level | Typical base | Typical total comp |
| --- | --- | --- |
| Mid Rust | $140K–$175K | $160K–$220K |
| Senior Rust | $160K–$220K | $200K–$320K |
| Staff+ Rust | $200K–$260K+ | $280K–$450K+ |
| US average (all levels) | ~$159K base | — |

| Rank | Company | Rust used for | Est. avg base (US) | Est. avg TC (US) |
| ---: | --- | --- | ---: | ---: |
| 1 | OpenAI | Infra, performance services, safety-critical tooling | $210K | $380K |
| 2 | Anthropic | Systems / infra / safety tooling | $205K | $360K |
| 3 | Meta | Messaging/media hot paths, source control (Sapling), infra | $200K | $350K |
| 4 | Netflix | High-scale backend / performance components | $198K | $340K |
| 5 | Google | Android native (Rust), Fuchsia, infra | $195K | $335K |
| 6 | Apple | Systems / security / OS-adjacent components | $195K | $330K |
| 7 | Stripe | Payments infra, performance-critical services | $190K | $320K |
| 8 | Jump Trading | Low-latency trading systems | $190K | $350K |
| 9 | Jane Street | Systems tooling (select teams; mostly OCaml shop) | $190K | $350K |
| 10 | Citadel Securities | Trading / market-data systems | $188K | $340K |
| 11 | Solana Labs / Anza | Blockchain runtime, validators, networking | $185K | $300K |
| 12 | Coinbase | Crypto infra, wallets, secure services | $185K | $295K |
| 13 | Airbnb | Infra / performance services | $182K | $290K |
| 14 | Uber | Marketplace infra, performance services | $180K | $285K |
| 15 | LinkedIn | Infra / data-path components | $180K | $280K |
| 16 | Amazon / AWS | Firecracker, Bottlerocket, S3/Lambda/EC2 hot paths | $178K | $275K |
| 17 | Microsoft | Windows kernel, Azure, parsers, memory-safe rewrites | $178K | $275K |
| 18 | Snowflake | Database engine / cloud data platform components | $178K | $280K |
| 19 | Databricks | Data systems / performance components | $178K | $280K |
| 20 | Cloudflare | Pingora/Oxy proxy, FL2 edge, firewall, DNS, cache, quiche | $175K | $260K |
| 21 | Datadog | Agent, metrics/logs pipelines, observability hot paths | $175K | $265K |
| 22 | Discord | Real-time services, Read States, caches, media/NIFs | $175K | $255K |
| 23 | NVIDIA | Systems / driver-adjacent / tooling | $175K | $270K |
| 24 | Kraken | Exchange matching / crypto infra | $172K | $250K |
| 25 | Robinhood | Trading infra components | $172K | $255K |
| 26 | Block (Square / Cash App) | Payments / crypto infra | $170K | $250K |
| 27 | Twitch | Real-time / media infra | $170K | $245K |
| 28 | ByteDance / TikTok | Infra / media hot paths | $168K | $240K |
| 29 | Snap | Client/server performance components | $168K | $240K |
| 30 | Figma | Multiplayer sync engine, serialization hot path | $168K | $240K |
| 31 | Dropbox | Sync engine, hashing, Magic Pocket storage, Capture | $168K | $235K |
| 32 | Pinterest | Infra / storage hot paths | $168K | $235K |
| 33 | Notion | Sync / performance components | $165K | $230K |
| 34 | Slack (Salesforce) | Real-time / infra components | $165K | $235K |
| 35 | Salesforce | Platform / security / infra services | $165K | $230K |
| 36 | Adobe | Creative Cloud performance / native components | $165K | $230K |
| 37 | Crowdstrike | Endpoint agent, detection hot paths | $162K | $230K |
| 38 | Mysten Labs (Sui) | Sui blockchain runtime | $160K | $230K |
| 39 | Fastly | Edge compute, HTTP proxy, Wasmtime ecosystem | $162K | $225K |
| 40 | Palo Alto Networks | Security appliance / cloud security components | $162K | $225K |
| 41 | Aptos Labs | Blockchain infra | $158K | $225K |
| 42 | GitHub | Code search, performance services, npm stack | $160K | $225K |
| 43 | Autodesk | Native / compute components | $162K | $220K |
| 44 | Zscaler | Cloud security proxy / policy path | $160K | $220K |
| 45 | Okta | Auth / identity security components | $160K | $220K |
| 46 | Trail of Bits | Security engineering / auditors tooling | $160K | $220K |
| 47 | Vercel | Edge / build / performance tooling | $158K | $220K |
| 48 | Shopify | YJIT (Ruby JIT), performance infra | $158K | $220K |
| 49 | 1Password | Shared Rust core (crypto, sync, DB, credentials) | $160K | $215K |
| 50 | Roblox | Platform infra / safety components | $155K | $215K |
| 51 | Chainlink Labs | Oracle / secure infra | $155K | $215K |
| 52 | MongoDB | Database / sync / tooling components | $158K | $215K |
| 53 | Parity Technologies | Polkadot / Substrate stack | $155K | $210K |
| 54 | Fortanix | Confidential computing / SGX | $155K | $210K |
| 55 | Sentry | Ingest / processing pipelines | $155K | $210K |
| 56 | Elastic | Search / observability agents & components | $155K | $210K |
| 57 | Splunk (Cisco) | Data pipeline / agents | $155K | $210K |
| 58 | Riot Games | Game services / tooling | $155K | $210K |
| 59 | Zoom | Media / client performance components | $155K | $210K |
| 60 | npm (GitHub) | Registry performance services | $155K | $210K |
| 61 | Materialize | Streaming database engine | $155K | $210K |
| 62 | Neon | Serverless Postgres components | $155K | $210K |
| 63 | Turso | libSQL / edge database engine | $155K | $210K |
| 64 | Cisco | Networking OS / security components | $155K | $205K |
| 65 | Oracle | OCI control/data-plane services (growing Rust) | $155K | $205K |
| 66 | Docker | Runtime / tooling components | $155K | $205K |
| 67 | HashiCorp (IBM) | Infra tooling components | $155K | $205K |
| 68 | Intel | Systems software, tools, firmware-adjacent | $152K | $205K |
| 69 | IBM / Red Hat | Linux / cloud / security tooling | $152K | $200K |
| 70 | Fly.io | Edge orchestration, Firecracker-based isolation | $155K | $200K |
| 71 | Twilio | Communications infra | $152K | $200K |
| 72 | Atlassian | Platform performance services | $150K | $200K |
| 73 | Unity | Engine / editor native components | $150K | $200K |
| 74 | Near Protocol | Blockchain runtime | $150K | $200K |
| 75 | Dfinity | Internet Computer protocol | $150K | $200K |
| 76 | Qualcomm | Embedded / systems software | $150K | $200K |
| 77 | Arm | Tooling / systems software | $150K | $195K |
| 78 | Samsung | Android / systems components | $150K | $195K |
| 79 | Bytecode Alliance | Wasmtime, Cranelift | $150K | $195K |
| 80 | LaunchDarkly | Flag evaluation / edge components | $150K | $195K |
| 81 | NCC Group | Security research tooling | $150K | $195K |
| 82 | Wasmer | Wasm runtime | $148K | $190K |
| 83 | Huawei | Cloud / networking / systems | $148K | $190K |
| 84 | Protocol Labs | IPFS / Filecoin components | $148K | $190K |
| 85 | Embark Studios | Game engine / multiplayer | $145K | $190K |
| 86 | Mozilla | Browser/engine, Rust ecosystem projects | $145K | $185K |
| 87 | Brave | Browser / privacy components | $145K | $185K |
| 88 | Igalia | Browser / web platform | $145K | $185K |
| 89 | Helium (Nova Labs) | Wireless / crypto infra | $145K | $185K |
| 90 | PostHog | Product analytics pipelines | $145K | $185K |
| 91 | SurrealDB | Multi-model database engine | $145K | $185K |
| 92 | Toyota | Automotive embedded / safety software | $140K | $180K |
| 93 | JetBrains | IDE backends / tooling | $140K | $180K |
| 94 | Coursera | Secure code-execution sandbox components | $140K | $175K |
| 95 | Volvo Cars | Automotive software | $138K | $175K |
| 96 | Canonical | Ubuntu / systems tooling | $135K | $170K |
| 97 | Bosch | Embedded / automotive systems | $135K | $170K |
| 98 | Siemens | Industrial / embedded software | $135K | $170K |
| 99 | Renault / Ampere | Automotive software | $130K | $165K |
| 100 | Polars / DataFusion ecosystem | High-perf DataFrame / query engines in Rust | $150K | $200K |

### Rank notes (read before using numbers)
1. **Crypto / trading** often pays highest cash; equity is volatile.
2. **FAANG-tier TC** swings hard by level, location (SF/SEA/NYC), and stock price.
3. **1Password** public posting example: Senior Rust ~**$153K–$214K** US base.
4. Companies with **deep Rust in core product** (Cloudflare, Discord, Dropbox, Fastly, 1Password, AWS Firecracker) are often better interview prep targets than companies with only a small Rust island.
5. Re-check offers on Levels.fyi / team blind / current job posts — bands move every year.

### Best prep targets by domain
| Domain | Practice companies | Question themes in this file |
| --- | --- | --- |
| Edge / proxy | Cloudflare, Fastly | Q1–18 |
| Real-time | Discord, Figma | Q19–30 |
| Sync / files | Dropbox | Q31–40 |
| Cloud systems | AWS, Microsoft, Google | Q41–55 |
| Observability | Datadog, Sentry | Q56–65 |
| Security / crypto product | 1Password, Crowdstrike, Zscaler | Q66–74 |

---

## Cloudflare / Fastly style (edge, proxy, HTTP)

### 1. HTTP Reverse Proxy
Build a proxy that forwards HTTP requests to an upstream and returns the response.
- Listen for incoming requests
- Forward method, path, headers, body
- Apply upstream timeout
- Return 502/504 on upstream failure
- Log status and latency

### 2. TCP Byte-Forwarding Proxy
For each client TCP connection, dial upstream and copy bytes both ways.
- Handle many clients concurrently
- If one side closes, close the other
- Do not crash the accept loop on one bad connection
- Support graceful shutdown

### 3. Length-Prefixed Protocol Parser
Messages are `[u32 big-endian length][payload]`.
- Decode from a TCP byte stream
- Handle partial reads
- Reject frames over max size
- Encode messages the same way
- Never panic on bad input

### 4. Per-IP Rate Limiter (DDoS-style)
Protect an endpoint from one IP sending too many requests.
- Allow N requests per window per IP
- Return “limited” when over quota
- Memory must not grow forever (expire old IPs)
- Safe under concurrent requests

### 5. Token Bucket Rate Limiter
Allow steady rate R with burst B.
- `allow(key) -> bool`
- Refill tokens over time
- Concurrent-safe
- Unit test burst then throttle

### 6. Edge LRU Cache with TTL
Cache GET responses at the edge.
- Key = URL (and method)
- Evict least recently used when full
- Expire entries after TTL
- Do not cache error responses
- Thread-safe get/put

### 7. Cache Stampede Protection
When many requests miss the same key, fetch upstream only once.
- First miss starts the fetch
- Other waiters share the same result
- After done, store in cache
- Errors are shared too (or retry policy documented)

### 8. HTTP Request Parser (no framework)
Parse raw HTTP/1.1 request bytes.
- Method, path, version
- Headers map
- Cap header size and count
- Reject malformed requests with clear errors

### 9. Header Allowlist / Strip Middleware
Before forwarding to origin, clean hop-by-hop / dangerous headers.
- Remove `Connection`, `Transfer-Encoding`, etc.
- Optionally strip hop-by-hop listed in Connection
- Keep required forwarding headers (`X-Forwarded-For`)
- Do not let client override internal trust headers

### 10. Least-Connections Load Balancer
Pick the backend with fewest active requests.
- Track in-flight count per backend
- Skip unhealthy backends
- Increment/decrement safely on start/finish
- Round-robin tie-break

### 11. Active Health Checker
Mark backends up/down using probes.
- Periodically dial/HTTP-check each backend
- After N failures → unhealthy
- After N successes → healthy
- Load balancer must ignore unhealthy nodes

### 12. Connection Pool
Reuse TCP/HTTP connections to origin.
- Max pool size
- Checkout / return
- Drop idle connections after timeout
- Replace dead connections

### 13. Sliding Window Rate Limit
More accurate than fixed windows.
- Max N requests in the last W seconds per key
- Efficient under high QPS
- Concurrent-safe

### 14. Circuit Breaker for Origin
Stop calling a failing origin.
- Open after repeated failures
- Fail fast while open
- Half-open probe after cooldown
- Close on success

### 15. SSRF-Safe URL Fetch
User gives a URL; server may fetch it only if safe.
- Allow only http/https
- Block localhost, private, link-local, cloud metadata IPs
- Clear deny reasons
- Document redirect policy

### 16. Streaming Response Proxy
Proxy a large upstream body without buffering everything in RAM.
- Stream chunks to client
- Bound memory
- Abort read if client disconnects

### 17. IP / CIDR Allow-Deny List
Decide if a client IP is allowed.
- Support exact IP and CIDR
- Deny overrides allow (document precedence)
- Fast lookup
- Invalid rules rejected at load time

### 18. Route Prefix Matcher
Match `/api/v1/users/123` to the best route rule.
- Insert prefix routes
- Longest prefix match
- Return matched route id / handler key

---

## Discord style (real-time, concurrency, caches)

### 19. Thread-Safe LRU Cache
Hot-path cache for presence / read-state style data.
- O(1) get/put
- Capacity eviction
- Safe for many concurrent readers/writers

### 20. Concurrent Hash Map (Sharded)
Reduce lock contention under heavy updates.
- N shards, each with own lock
- get / insert / remove by key
- Show it works under parallel updates

### 21. Singleflight Coalescer
Many clients ask for the same key at once → one lookup.
- Coalesce in-flight work by key
- Share Ok/Err with waiters
- Clear in-flight entry after completion

### 22. Fan-out Messaging
One event must be delivered to N subscribers.
- Publish to a topic/room
- Each subscriber gets the event
- Slow subscriber policy: buffer bound / disconnect / drop (pick one)
- Remove disconnected subscribers

### 23. WebSocket Chat Room
- Clients join a room
- Broadcast messages to room members
- Remove on disconnect
- Limit message size
- Many rooms supported

### 24. Presence / Online Set
Track who is online.
- User connect → online
- Disconnect / heartbeat timeout → offline
- Query: is online? list online in a guild/room
- Concurrent updates safe

### 25. Sorted Set by Score
Like a leaderboard or ordered member list.
- Add member with score
- Update score
- Range query by rank or score
- Concurrent-safe enough for interview (Mutex OK if justified)

### 26. Bounded Channel Pipeline
Ingest → process → push to clients.
- Bounded queues between stages
- Apply backpressure when full
- Drop or block policy documented
- Graceful drain on shutdown

### 27. Atomic Counters at Scale
Guild/member counters updated by many tasks.
- inc/dec/get with atomics
- Correct under high contention
- Test with many parallel tasks

### 28. Deduplicate Events in a Time Window
Same event id should not be processed twice within W seconds.
- Insert/check API
- Evict old ids
- Bound memory
- Concurrent-safe

### 29. Priority Notification Queue
Urgent events jump ahead of normal ones.
- Push with priority
- Pop highest priority first
- Multiple producers/consumers

### 30. Graceful Shutdown of Real-Time Server
- Stop accepting new connections
- Close or drain existing sockets
- Finish in-flight fan-out with timeout
- Exit cleanly

---

## Dropbox style (files, sync, hashing)

### 31. Concurrent File Hasher
Hash thousands of files (e.g. SHA-256).
- Limit concurrency
- Stream file contents (no full read into RAM for large files)
- Return path → hash map
- Collect per-file errors

### 32. File Chunker
Split a large file into fixed-size chunks for upload/sync.
- Chunk size configurable
- Last chunk may be smaller
- Compute per-chunk checksum
- Do not load entire file at once

### 33. Content-Defined Chunking (CDC) Lite
Split file where content hash matches a pattern (sync-friendly).
- Rolling hash (Rabin-lite or simple window hash)
- Emit chunk boundaries
- Stable-ish boundaries when a middle byte changes
- Stream input

### 34. Local Sync Planner
Given local file list and remote file list (path + hash).
- Decide: upload, download, delete, skip
- Detect renames if same hash different path (stretch)
- Output a plan list

### 35. Parallel Uploader with Retry
Upload many chunks/files.
- Max concurrent uploads
- Retry with exponential backoff + jitter
- Progress: completed / failed / remaining
- Cancel support

### 36. Dedup Store by Hash
Store blobs keyed by content hash.
- `put(bytes) -> hash`
- Same content → same hash, store once
- `get(hash) -> bytes`
- Thread-safe

### 37. Sparse / Partial File Download
Download only missing byte ranges.
- Track which ranges are present
- Request missing ranges
- Write ranges into the correct file offsets
- Finalize when complete

### 38. Directory Walker with Bounded Concurrency
Walk a large directory tree and process each file.
- Do not spawn unlimited tasks
- Skip permission errors without dying
- Return summary counts

### 39. Atomic File Replace
Write a temp file then replace the target safely.
- Write temp in same directory
- fsync (conceptually / call if available)
- Rename over target
- Readers never see half-written file

### 40. Streaming Zip / Archive Reader Lite
Read local file entries from an archive stream without full extract to RAM.
- List entry names/sizes
- Extract one entry by name to disk
- Bound memory

---

## AWS / Microsoft / Google style (systems, runtime, isolation)

### 41. Worker Pool with Bounded Queue
- Fixed workers
- Bounded job queue
- Reject or block when full
- Graceful shutdown drains jobs
- Return job results/errors

### 42. Timeout Wrapper
Run async work with a deadline.
- Success → return value
- Timeout → cancel work, return error
- Do not leak the background task

### 43. Cancellation Token Job
Long job checks cancel between steps.
- Cooperative cancel
- Returns Cancelled vs Completed
- Cleans up partial work

### 44. Exponential Backoff Retry
- Max attempts
- Exponential wait + jitter
- Non-retryable errors stop immediately
- Return last error

### 45. Semaphore Bounded Concurrency
Run N tasks with at most K in flight.
- Collect all outcomes
- Support cancel remaining

### 46. Process/Task Supervisor Lite
Restart a failing child task with backoff.
- Start task
- On panic/error, restart until max
- Reset backoff after healthy period
- Shutdown stops restarts

### 47. Memory Cap / Quota Guard
Track allocated bytes for a request/tenant.
- charge / release
- Reject when over quota
- Never go negative
- Concurrent-safe

### 48. Arena / Request Scratch Allocator
Allocate many short-lived objects for one request, free all at once.
- Allocate from arena
- Reset arena after request
- Returned references must not escape (lifetime design)

### 49. Safe Parser with Size Limits
Parse untrusted bytes (JSON/custom).
- Max bytes
- Max depth
- Max elements
- Error instead of huge allocations

### 50. Lock Ordering Fix
Code with two locks deadlocks under concurrency.
- Reproduce
- Explain
- Fix ordering or redesign
- Test that no longer hangs

### 51. Offload Blocking Work
From async code, run blocking disk/DNS work safely.
- Do not block the async runtime worker
- Timeout the blocking call
- Surface overload errors

### 52. Ring Buffer
Fixed-size queue for high-throughput events.
- Push/pop O(1)
- Full policy: error or overwrite
- Correct wrap-around
- Optional SPSC lock-free stretch

### 53. Timer Wheel / Delayed Work
Schedule work to run after delay.
- Schedule at time T
- Pop/run due items
- Cancel pending item
- Efficient for many timers

### 54. Idempotent Apply
Apply config/action only once per id.
- Remember applied ids (TTL or permanent for exercise)
- Duplicate apply returns previous result
- Concurrent duplicates do not double-apply

### 55. Mini Reconcile Loop
Desired state vs observed state.
- Diff
- Apply missing/changed
- Delete extras (if in scope)
- Retry transient failures
- Stop when matched

---

## Datadog / Meta style (pipelines, agents, telemetry)

### 56. Metrics Counter + Histogram
- `inc(name, labels)`
- `observe(name, value)`
- Snapshot/export in-memory
- Concurrent updates

### 57. Log Line Pipeline
- Multiple producers push lines
- Filter by level
- Batch by size or time
- Write batches to sink
- Bound memory under overload

### 58. Agent Event Batcher
Ingest events fast, export in batches.
- Add event
- Flush at N events or every T ms
- On sink failure, retry or spill policy
- Shutdown flushes remaining

### 59. Sampling Filter
Keep only 1% (or hash-based) of events but always keep errors.
- Deterministic sampling by id optional
- Always keep severity >= error
- Count sampled vs dropped

### 60. Top-K Heavy Hitters
From a stream of keys (endpoints, customers), find top K.
- One-pass / bounded memory approach
- Return key + count
- Handle ties

### 61. Streaming Aggregator
Aggregate counts by key from a huge file/stream.
- Do not load all lines into RAM
- Bound map size or flush periodically
- Ignore bad lines

### 62. Multi-Stage Async Pipeline
ingest → validate → enrich → export
- Bounded channels
- Invalid events to DLQ with reason
- Ordered shutdown

### 63. Dead Letter Queue
Failed events go to DLQ after retries.
- Retry with backoff
- DLQ stores payload + error
- Reprocess API (in-memory OK)

### 64. Cardinality Guard
Reject/label-drop metrics when unique label combinations explode.
- Track cardinality per metric
- Over limit → reject or drop labels
- Concurrent-safe

### 65. Clock-Safe Window Counter
Count events in 1-minute windows.
- Advance windows with time
- Do not break if time jumps slightly
- Query current window count

---

## 1Password / Microsoft security-product style

### 66. Constant-Time Compare
Compare secrets without timing leaks.
- Equal-length byte compare
- Use in API key check
- Tests for match/mismatch

### 67. Secure Token Generate + Store Hash
- Generate high-entropy token
- Store only hash
- Verify token
- Revoke token
- Never log raw token

### 68. Secret Wrapper that Zeroizes on Drop
- Hold secret bytes
- Wipe on drop
- No Debug leak of secret
- Use for password/key

### 69. HMAC Request Auth
- Canonical string: method + path + timestamp + body hash
- Sign with HMAC
- Verify with skew window
- Reject bad/expired signatures

### 70. Path Traversal Safe Open
Open user path only under a root directory.
- Block `..` escape
- Canonicalize safely
- Return error on escape attempts

### 71. RBAC Checker
`can(user, action, resource) -> bool`
- Roles → permissions
- Wildcards like `vault:*`
- Default deny
- Unit tests

### 72. Audit Log Hash Chain
- Append events
- Each event hashes previous
- Verify integrity
- Detect middle tampering

### 73. Password / Key Stretching Wrapper
Wrap a KDF call (API sketch OK).
- Salt + password → derived key
- Constant-time verify helper
- Never store plaintext password

### 74. Encrypted Blob Seal/Open
Using a crypto crate is OK.
- Seal plaintext with key → ciphertext
- Open ciphertext → plaintext
- Wrong key / tampered data fails
- Nonce/IV handled correctly

---

## Cross-company classics (asked everywhere)

### 75. Merge K Sorted Streams
Merge K sorted iterators into one sorted stream with a heap.
- Bounded memory
- Handle empty streams
- Correct order

### 76. Concurrent URL Downloader
Download N URLs with limit K, timeout, retries, per-URL result.

### 77. Concurrent Web Crawler
BFS crawl, dedupe URLs, max depth, concurrency limit, same-domain only.

### 78. Consistent Hashing Ring
Map keys to nodes with virtual nodes; add/remove node moves few keys.

### 79. Bloom Filter
`add` / `might_contain`; no false negatives; configurable false-positive trade-off.

### 80. Trie / Radix Prefix Search
Insert words/routes; longest prefix or autocomplete prefix query.

### 81. Sliding Window Deduper + Rate Limit Combo
Per-user: dedupe identical requests and rate limit remaining QPS.

### 82. Hedged Request
Primary call + backup after delay; first success wins; cancel loser.

### 83. Distributed Lock (Redis semantics)
SET NX PX + owner token; safe unlock only by owner; optional extend.

### 84. Pub/Sub Bus
subscribe/publish; bounded subscriber queues; unsubscribe cleanup.

### 85. Background Job System
Enqueue job id; workers run with concurrency limit; status query; retries.

### 86. Leader Election Lease
Only leader runs a periodic task; heartbeat renew; failover on expiry.

### 87. Outbox Pattern
Save event with state change; worker publishes; mark sent; retry failures.

### 88. Mini API Gateway
Route by path + auth check + rate limit + upstream timeout + request id logs.

### 89. Policy Engine
Ordered allow/deny rules on ip/user/path/method; default deny; explain match.

### 90. Service Registry
register / heartbeat / discover / expire dead instances.

### 91. WebSocket Frame Parser
Parse header/mask/payload length; unmask; reject bad frames.

### 92. DNS Label Parser
Parse DNS name from bytes with length caps; no panic/hang on bad input.

### 93. Multiplex Streams on One TCP Conn
stream id + frames; open/data/close; per-stream routing.

### 94. HTTP CONNECT Tunnel
Parse CONNECT; dial; 200; bidirectional raw tunnel; block bad targets.

### 95. Config Load + Validate + Hot Reload
Typed config; fail fast invalid; reload keeps last good; in-flight uses snapshot.

### 96. Request ID Propagation
Create/accept request id; attach to logs across spawned tasks; return in response.

### 97. Health / Ready Probes
Liveness cheap; readiness checks deps with timeouts; correct status codes.

### 98. Parallel CSV/NDJSON Parse
Stream large file; typed rows; skip bad lines; optional parallel batch process.

### 99. Progress + Checkpoint Batch Job
Process items with progress; cancel mid-way; resume from checkpoint.

### 100. End-to-End Hot Path (pick one and finish)
Choose **one** and implement working code with tests:
- A) Proxy + rate limit + LRU cache
- B) Chat room + presence + fan-out
- C) File chunk + hash + parallel upload plan
- D) Agent ingest → sample → batch → export

For A/B/C/D: happy path + timeout/cancel + at least 3 tests.
