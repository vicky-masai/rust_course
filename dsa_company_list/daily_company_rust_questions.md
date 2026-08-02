# Daily Rust Practice — 1 Question per Top 100 Company

**How to use:** Solve **1 company per day** (~45–60 min).  
Implement in Rust. Add tests. Focus on correctness, concurrency, and edge cases.

| Day | Do this |
| --- | --- |
| 0–5 min | Read company context + requirements |
| 5–15 min | Design types / APIs |
| 15–50 min | Code + tests |
| 50–60 min | Complexity, failure modes, what you’d ship in prod |

---

## Day 1 — OpenAI
**Business:** AI platform infra / high-throughput services  
**Solve:** Request batcher for model inference jobs  
- Accept many small jobs  
- Batch by max size **or** max wait time  
- Process batch, return per-job results  
- Bound memory; cancel in-flight on shutdown  

## Day 2 — Anthropic
**Business:** AI safety / systems tooling  
**Solve:** Prompt/input size & depth guard  
- Reject input over max bytes  
- Reject nesting/depth over limit  
- Stream-parse without loading unbounded data  
- Clear error codes for each reject reason  

## Day 3 — Meta
**Business:** Messaging / media / source control hot paths  
**Solve:** Thread-safe read-state cache  
- `get(user, thread) -> last_read`  
- `set(user, thread, offset)`  
- LRU capacity eviction  
- Correct under concurrent updates (no lost writes)  

## Day 4 — Netflix
**Business:** Streaming at scale  
**Solve:** Adaptive concurrency limiter  
- Track in-flight requests  
- Raise/lower limit with AIMD on success/latency/errors  
- Reject or queue when over limit  
- Expose current limit metric  

## Day 5 — Google
**Business:** Android / systems / infra  
**Solve:** Safe binder-style message parser  
- Length-prefixed messages  
- Max size + max field count  
- Partial-read safe  
- No panic on malicious input  

## Day 6 — Apple
**Business:** OS / security components  
**Solve:** Keychain-like secret store API (in-memory)  
- Store secret by account  
- Constant-time compare on unlock  
- Zeroize on delete/drop  
- Never log secret bytes  

## Day 7 — Stripe
**Business:** Payments  
**Solve:** Idempotent charge API  
- Client sends `Idempotency-Key`  
- Same key returns same result  
- Concurrent duplicates run once  
- TTL expiry for old keys  

## Day 8 — Jump Trading
**Business:** Low-latency trading  
**Solve:** Lock-free-ish SPSC ring buffer for market ticks  
- Fixed capacity  
- Push/pop in O(1)  
- Full/empty handling  
- No data races (Atomics or unsafe documented carefully)  

## Day 9 — Jane Street
**Business:** Trading systems tooling  
**Solve:** Order book price-level map  
- Add/cancel orders  
- Best bid/ask query  
- Aggregate size at a price  
- Fast updates  

## Day 10 — Citadel Securities
**Business:** Market data  
**Solve:** Sliding-window VWAP  
- Ingest trades `(price, size, ts)`  
- Query VWAP over last W seconds  
- Evict old trades efficiently  
- Concurrent ingest + query  

## Day 11 — Solana Labs / Anza
**Business:** Blockchain validator / networking  
**Solve:** Transaction signature dedupe set  
- Insert tx id/hash  
- Reject duplicates  
- Bound memory with TTL or capacity  
- High ingest rate  

## Day 12 — Coinbase
**Business:** Crypto exchange / wallets  
**Solve:** Withdrawal rate limiter + allowlist  
- Per-user daily withdraw limit  
- Destination address allowlist check  
- Atomic debit of remaining quota  
- Reject over-limit safely  

## Day 13 — Airbnb
**Business:** Marketplace infra  
**Solve:** Calendar inventory lock  
- Reserve dates for a listing  
- Detect overlapping bookings  
- Rollback on failure  
- Concurrent reservation attempts don’t double-book  

## Day 14 — Uber
**Business:** Marketplace matching  
**Solve:** Nearest-driver finder  
- Upsert driver locations  
- Query k nearest to a point (simple grid or KD-lite OK)  
- Remove offline drivers  
- Concurrent updates  

## Day 15 — LinkedIn
**Business:** Feed / graph data paths  
**Solve:** Fan-out feed writer  
- On new post, enqueue fan-out to follower inboxes  
- Bound concurrency  
- Retry failed inbox writes  
- Deduplicate post delivery  

## Day 16 — Amazon / AWS
**Business:** Firecracker / Lambda / storage  
**Solve:** Mini microVM slot manager  
- Allocate/free slots with memory budget  
- Enforce max concurrent VMs  
- Reclaim crashed slots  
- Thread-safe inventory  

## Day 17 — Microsoft
**Business:** Windows / Azure memory safety  
**Solve:** UTF-8 / path validator  
- Validate UTF-8 without panic  
- Normalize path and block `..` escape  
- Reject overlong / invalid sequences  
- Fuzz-friendly API  

## Day 18 — Snowflake
**Business:** Cloud data warehouse  
**Solve:** Columnar batch encoder  
- Encode a column of ints with RLE or dictionary  
- Decode back exactly  
- Report compression ratio  
- Stream large columns in chunks  

## Day 19 — Databricks
**Business:** Data processing  
**Solve:** Partitioned shuffle buffer  
- Route records by key hash to N partitions  
- Spill when partition buffer exceeds limit  
- Merge spilled runs  
- Bound RAM  

## Day 20 — Cloudflare
**Business:** Edge proxy / firewall / DNS / cache  
**Solve:** HTTP reverse proxy + per-IP rate limit + LRU cache  
- Forward GET to origin  
- Rate limit clients  
- Cache successful responses with TTL  
- Timeouts + graceful shutdown  

## Day 21 — Datadog
**Business:** Observability agent  
**Solve:** Metrics aggregator  
- `inc(metric, tags)` and `observe(metric, value)`  
- Aggregate by time window  
- Export snapshot  
- Cardinality guard (drop if too many tag combos)  

## Day 22 — Discord
**Business:** Real-time chat / presence  
**Solve:** Guild presence service  
- User online/offline with heartbeat  
- Query online members  
- Fan-out presence updates to subscribers  
- Expire stale heartbeats  

## Day 23 — NVIDIA
**Business:** GPU systems tooling  
**Solve:** Job queue for GPU workers  
- Submit jobs with memory requirement  
- Schedule only if free GPU memory enough  
- Preempt/cancel support  
- Fairness between clients (simple)  

## Day 24 — Kraken
**Business:** Crypto exchange  
**Solve:** Matching engine (price-time priority)  
- Limit buy/sell orders  
- Match when prices cross  
- Partial fills  
- Deterministic order  

## Day 25 — Robinhood
**Business:** Retail trading  
**Solve:** Order throttle + market-hours gate  
- Allow orders only in session window  
- Per-user order rate limit  
- Reject with clear reasons  
- Clock injectable for tests  

## Day 26 — Block (Square / Cash App)
**Business:** Payments / bitcoin  
**Solve:** Ledger transfer with double-entry  
- Debit A / credit B atomically  
- Reject insufficient funds  
- Idempotent transfer id  
- Audit log of entries  

## Day 27 — Twitch
**Business:** Live streaming  
**Solve:** Chat rate limiter + slow-mode  
- Per-channel message rate  
- Slow-mode: min seconds between user messages  
- Drop/reject over limit  
- Concurrent chatters  

## Day 28 — ByteDance / TikTok
**Business:** Short video infra  
**Solve:** Chunked video upload assembler  
- Accept numbered chunks  
- Assemble final object  
- Verify checksum  
- Reject missing/duplicate parts  

## Day 29 — Snap
**Business:** Ephemeral messaging  
**Solve:** Snap TTL store  
- Put media metadata with expiry  
- Get only if not expired  
- Background purge of expired  
- One-time view flag (delete after read)  

## Day 30 — Figma
**Business:** Multiplayer design sync  
**Solve:** OT/CRDT-lite document ops  
- Apply ops `(retain, insert, delete)` to a text doc  
- Transform concurrent ops (simple text OT)  
- Detect / reject invalid ops  
- Deterministic result for same op order  

## Day 31 — Dropbox
**Business:** File sync  
**Solve:** Sync planner  
- Local vs remote file lists `(path, hash, size)`  
- Output upload/download/delete/skip plan  
- Detect same-hash rename (stretch)  
- No data loss in plan  

## Day 32 — Pinterest
**Business:** Image / storage hot paths  
**Solve:** Content-hash blob store  
- `put(bytes) -> hash`  
- Dedup identical content  
- `get(hash)`  
- Concurrent put/get  

## Day 33 — Notion
**Business:** Collaborative docs  
**Solve:** Block tree apply  
- Document = tree of blocks  
- Ops: insert/update/delete block  
- Concurrent edits on different blocks OK  
- Conflict on same block: last-writer-wins or version check  

## Day 34 — Slack
**Business:** Real-time messaging  
**Solve:** Channel message fan-out  
- Post message to channel  
- Deliver to all connected members  
- Persist last N messages for backlog  
- Handle disconnect/reconnect  

## Day 35 — Salesforce
**Business:** CRM platform / security  
**Solve:** Object ACL checker  
- Users, roles, sharing rules  
- `can(user, read|write, record)`  
- Default deny  
- Unit tests for allow/deny matrix  

## Day 36 — Adobe
**Business:** Creative asset processing  
**Solve:** Parallel image/job processor  
- Worker pool with bounded queue  
- Process files with retries  
- Progress + cancel  
- Failure summary  

## Day 37 — Crowdstrike
**Business:** Endpoint detection  
**Solve:** Indicator matcher  
- Load IOC set (hashes/domains/IPs)  
- Stream events and match  
- Report which IOC matched  
- High throughput, low false-miss  

## Day 38 — Mysten Labs (Sui)
**Business:** Object-centric blockchain  
**Solve:** Object ownership lock  
- Objects owned by addresses  
- Transfer object with version check  
- Reject stale version (optimistic concurrency)  
- Concurrent transfers don’t corrupt owner  

## Day 39 — Fastly
**Business:** Edge HTTP / Wasm  
**Solve:** Edge cache proxy  
- Cache GET by URL+Vary  
- TTL + LRU  
- Singleflight on miss  
- Bypass on POST/errors  

## Day 40 — Palo Alto Networks
**Business:** Firewall / threat prevention  
**Solve:** Packet policy engine  
- Rules: src/dst CIDR, port, action allow/deny  
- First-match evaluation  
- Default deny  
- Explain which rule matched  

## Day 41 — Aptos Labs
**Business:** Blockchain  
**Solve:** Mempool with priority fee  
- Insert txs with gas price  
- Pop highest fee first  
- Deduplicate by hash  
- Capacity eviction of lowest fee  

## Day 42 — GitHub
**Business:** Code search / git infra  
**Solve:** Substring / trigram search index (lite)  
- Index documents  
- Query terms  
- Return matching doc ids  
- Bound memory  

## Day 43 — Autodesk
**Business:** CAD / compute  
**Solve:** Geometry job scheduler  
- Jobs with CPU/memory cost  
- Schedule under resource caps  
- Cancel long jobs  
- Queue fairness  

## Day 44 — Zscaler
**Business:** Cloud security proxy  
**Solve:** HTTPS CONNECT proxy with policy  
- Parse CONNECT host  
- Allow/deny by domain/category list  
- Tunnel bytes if allowed  
- Audit log decisions  

## Day 45 — Okta
**Business:** Identity  
**Solve:** Session token manager  
- Issue opaque session id  
- Store hash + expiry  
- Validate / revoke  
- Sliding expiration optional  

## Day 46 — Trail of Bits
**Business:** Security tooling  
**Solve:** Taint-lite dataflow helper  
- Mark sources/sinks  
- Track if tainted value reaches sink  
- Simple AST/IR of assignments  
- Report violation paths  

## Day 47 — Vercel
**Business:** Edge / deployments  
**Solve:** Deployment promotion lock  
- Only one active deploy per project  
- Blue/green switch atomic  
- Rollback to previous  
- Concurrent promote attempts safe  

## Day 48 — Shopify
**Business:** Commerce / YJIT performance  
**Solve:** Flash-sale inventory counter  
- Atomic stock decrement  
- Reject when zero  
- Idempotent checkout attempt id  
- Metrics for sold-out rate  

## Day 49 — 1Password
**Business:** Password manager crypto core  
**Solve:** Vault item seal/open  
- Encrypt item with key (crate OK)  
- Decrypt and verify integrity  
- Zeroize keys on drop  
- Wrong key / tamper fails  

## Day 50 — Roblox
**Business:** Platform safety  
**Solve:** Chat filter pipeline  
- Rate limit messages  
- Blocklist words  
- Queue for async moderation  
- Drop spam patterns (repeated text)  

## Day 51 — Chainlink Labs
**Business:** Oracles  
**Solve:** Multi-source price aggregator  
- Ingest prices from N oracles  
- Median (or trimmed mean)  
- Quorum: require k-of-n fresh updates  
- Stale data rejected  

## Day 52 — MongoDB
**Business:** Database  
**Solve:** WiredTiger-lite page cache  
- Get/put pages by id  
- LRU eviction  
- Pin pages while in use  
- Concurrent readers  

## Day 53 — Parity Technologies
**Business:** Substrate / Polkadot  
**Solve:** Extrinsic pool  
- Validate basic tx fields  
- Priority by tip/fee  
- Ban invalid senders temporarily  
- Remove included txs  

## Day 54 — Fortanix
**Business:** Confidential computing  
**Solve:** Sealed blob with attested key id  
- Seal plaintext under key_id  
- Open only with matching key_id  
- Audit access attempts  
- Zeroize plaintext buffers  

## Day 55 — Sentry
**Business:** Error tracking  
**Solve:** Event fingerprint coalescer  
- Group events by fingerprint  
- Count occurrences  
- Sample high-volume groups  
- Flush aggregates on interval  

## Day 56 — Elastic
**Business:** Search / agents  
**Solve:** Bulk indexer buffer  
- Buffer docs  
- Flush by count/size/time  
- Retry failed bulk with backoff  
- Backpressure when full  

## Day 57 — Splunk
**Business:** Log pipelines  
**Solve:** Multiline log joiner  
- Join stack traces into one event  
- Timeout flush partial events  
- Bound pending joins  
- Parse timestamp  

## Day 58 — Riot Games
**Business:** Game services  
**Solve:** Matchmaker queue  
- Players enqueue with MMR  
- Form teams within MMR delta  
- Timeout expands delta  
- Concurrent queue ops  

## Day 59 — Zoom
**Business:** Video meetings  
**Solve:** SFU subscription manager  
- Participants publish/subscribe streams  
- Limit max subscriptions  
- Remove on leave  
- Fan-out frame/metadata to subscribers (simulated)  

## Day 60 — npm (GitHub)
**Business:** Package registry  
**Solve:** Package tarball integrity checker  
- Store package metadata + sha  
- Verify upload sha  
- Deduplicate identical tarball content  
- Rate limit publishes per user  

## Day 61 — Materialize
**Business:** Streaming SQL  
**Solve:** Incremental count view  
- Ingest insert/delete rows  
- Maintain `COUNT(*) GROUP BY key`  
- Emit only changed groups  
- Correct under late updates  

## Day 62 — Neon
**Business:** Serverless Postgres  
**Solve:** Page server range fetch  
- Store pages by (timeline, page_no)  
- Fetch contiguous ranges efficiently  
- Cache hot pages LRU  
- Concurrent get/put  

## Day 63 — Turso
**Business:** Edge SQLite  
**Solve:** Embedded KV with WAL  
- put/get/delete  
- Append WAL entries  
- Checkpoint WAL into main store  
- Recover after crash (replay WAL)  

## Day 64 — Cisco
**Business:** Networking OS  
**Solve:** FIB / route table  
- Insert CIDR → next hop  
- Longest-prefix match lookup  
- Delete route  
- Efficient enough for many prefixes  

## Day 65 — Oracle
**Business:** OCI cloud control plane  
**Solve:** Desired-state reconciler  
- Diff desired vs observed instances  
- Create/update/delete to match  
- Retry transient errors  
- Idempotent apply  

## Day 66 — Docker
**Business:** Containers  
**Solve:** Image layer store  
- Layers keyed by digest  
- Reference count mounts  
- GC unreferenced layers  
- Concurrent pull/delete safe  

## Day 67 — HashiCorp
**Business:** Infra tooling  
**Solve:** Distributed lock (in-memory/Raft-lite OK)  
- Acquire with TTL  
- Renew / release  
- Only owner releases  
- Leader failover story documented  

## Day 68 — Intel
**Business:** Systems software  
**Solve:** Memory pool allocator  
- Fixed-size block pool  
- alloc/free O(1)  
- Detect double-free  
- Report utilization  

## Day 69 — IBM / Red Hat
**Business:** Linux / cloud  
**Solve:** cgroup-like quota enforcer  
- Track CPU/memory usage tokens  
- Admit/reject work under quota  
- Replenish over time  
- Per-tenant isolation  

## Day 70 — Fly.io
**Business:** Edge VMs  
**Solve:** Region placement scheduler  
- Place app in closest region with capacity  
- Soft/hard capacity limits  
- Rebalance when region overloaded  
- Concurrent allocate/free  

## Day 71 — Twilio
**Business:** Communications  
**Solve:** SMS send queue with delivery receipts  
- Enqueue messages  
- Workers send with concurrency limit  
- Update status: queued/sent/failed  
- Retry + DLQ  

## Day 72 — Atlassian
**Business:** Collaboration platforms  
**Solve:** Issue search inverted index  
- Index issues by tokens  
- Boolean AND query  
- Update/delete issue reindex  
- Bound memory  

## Day 73 — Unity
**Business:** Game engine  
**Solve:** Asset dependency resolver  
- Assets depend on other assets  
- Topological load order  
- Detect cycles  
- Parallel load independent assets  

## Day 74 — Near Protocol
**Business:** Blockchain  
**Solve:** Receipt queue between shards (lite)  
- Enqueue cross-shard receipts  
- Process in deterministic order  
- Retry failed execution  
- Cap queue size  

## Day 75 — Dfinity
**Business:** Internet Computer  
**Solve:** Canister call multiplexer  
- Many logical calls over one connection  
- Route responses by call id  
- Timeout unanswered calls  
- Cancel on disconnect  

## Day 76 — Qualcomm
**Business:** Embedded / mobile SoC  
**Solve:** Ring buffer logger for device events  
- Fixed memory  
- Overwrite oldest when full  
- Snapshot dump API  
- IRQ-safe design discussion + Mutex/atomic impl  

## Day 77 — Arm
**Business:** CPU tooling  
**Solve:** Instruction histogram  
- Stream opcodes  
- Count frequencies  
- Top-K hot opcodes  
- Bound memory  

## Day 78 — Samsung
**Business:** Android / device systems  
**Solve:** Binder-like permission gate  
- Calls require permissions  
- Grant/revoke runtime permissions  
- Deny by default  
- Audit denied calls  

## Day 79 — Bytecode Alliance
**Business:** Wasmtime / Cranelift  
**Solve:** Wasm-lite stack machine  
- Ops: push/add/call/halt (tiny ISA)  
- Trap on stack overflow/underflow  
- Fuel/gas meter limits execution  
- Deterministic traps  

## Day 80 — LaunchDarkly
**Business:** Feature flags  
**Solve:** Flag evaluator  
- Flags with rules on user attrs  
- Percentage rollouts by user-key hash  
- Default fallthrough  
- Explain why flag returned true/false  

## Day 81 — NCC Group
**Business:** Security research  
**Solve:** Protocol fuzzer harness target  
- Parser under test  
- Generate mutated inputs  
- Catch panics / hangs (timeout)  
- Save crashing inputs  

## Day 82 — Wasmer
**Business:** Wasm runtime  
**Solve:** Host function import table  
- Register host funcs by name  
- Call by name with typed args (enum values OK)  
- Trap on missing/mismatch  
- Fuel limit per call  

## Day 83 — Huawei
**Business:** Cloud networking  
**Solve:** Flow table for SDN-lite  
- Match packets on 5-tuple / CIDR  
- Actions: forward, drop, mirror  
- Age out idle flows  
- Concurrent lookups  

## Day 84 — Protocol Labs
**Business:** IPFS / Filecoin  
**Solve:** Content-addressed block store  
- Put block → CID/hash  
- Get by CID  
- Pin / unpin  
- GC unpinned blocks  

## Day 85 — Embark Studios
**Business:** Multiplayer games  
**Solve:** Client prediction buffer  
- Store last N inputs  
- Replay inputs on server correction  
- Interpolate remote entity positions (simple)  
- Bound buffer size  

## Day 86 — Mozilla
**Business:** Browser engine  
**Solve:** URL parser + origin checker  
- Parse scheme/host/port/path  
- Compute origin  
- Same-origin check  
- Reject illegal URLs  

## Day 87 — Brave
**Business:** Privacy browser  
**Solve:** Tracker blocklist matcher  
- Load domain blocklist  
- Check request URL host (incl. suffix match)  
- Efficient lookup (HashSet / radix)  
- Reload list without downtime  

## Day 88 — Igalia
**Business:** Web platform  
**Solve:** CSS color parser (subset)  
- Parse `#rgb`, `#rrggbb`, `rgb()`  
- Reject invalid  
- Output RGBA struct  
- No panic on junk input  

## Day 89 — Helium (Nova Labs)
**Business:** Decentralized wireless  
**Solve:** Packet router by device id  
- Register device → gateway route  
- Route uplink packets  
- Expire stale routes  
- Duplicate packet filter window  

## Day 90 — PostHog
**Business:** Product analytics  
**Solve:** Event ingestion pipeline  
- Validate event schema  
- Sample high-volume events  
- Batch write to sink  
- Backpressure + drop policy  

## Day 91 — SurrealDB
**Business:** Multi-model DB  
**Solve:** Document + graph edge store  
- Upsert document by id  
- Add edge relation  
- Query neighbors  
- Transactional apply of doc+edge (in-memory)  

## Day 92 — Toyota
**Business:** Automotive safety  
**Solve:** Sensor reading validator  
- Range checks, rate-of-change limits  
- Mark reading valid/invalid  
- Fail-safe default when invalid  
- Deterministic, no alloc in hot path (stretch)  

## Day 93 — JetBrains
**Business:** IDE tooling  
**Solve:** Incremental token index  
- Update index on text edit  
- Query symbol occurrences  
- Undo last edit  
- Fast enough for repeated edits  

## Day 94 — Coursera
**Business:** Secure code execution  
**Solve:** Submission sandbox policy  
- Limit CPU time, memory, output size  
- Kill on exceed  
- Capture stdout/stderr capped  
- Never execute untrusted path outside jail (path check)  

## Day 95 — Volvo Cars
**Business:** Automotive software  
**Solve:** CAN message demux  
- Parse id + payload  
- Route to handlers by id  
- Drop unknown ids  
- Rate-limit per id  

## Day 96 — Canonical
**Business:** Ubuntu / systems  
**Solve:** Package dependency resolver  
- Packages with Depends  
- Resolve install set  
- Detect conflicts/cycles  
- Pin versions (simple)  

## Day 97 — Bosch
**Business:** Embedded automotive  
**Solve:** Fixed-periodic task scheduler  
- Tasks with period + WCET  
- Schedule if CPU utilization ≤ 1 (RM/EDF lite)  
- Reject unschedulable set  
- Simulate timeline  

## Day 98 — Siemens
**Business:** Industrial systems  
**Solve:** OPC-lite tag store  
- Tags with types and timestamps  
- Read/write with quality flag  
- Subscription: notify on change  
- Concurrent read/write  

## Day 99 — Renault / Ampere
**Business:** Automotive software  
**Solve:** OTA update state machine  
- States: idle → downloading → verifying → applying → success/rollback  
- Checksum verify before apply  
- Abort/rollback on failure  
- No illegal transitions  

## Day 100 — Polars / DataFusion
**Business:** High-perf DataFrames / SQL  
**Solve:** Vectorized filter + aggregate  
- Columnar batches of ints  
- Filter predicate  
- Group-by sum  
- Avoid per-row allocations; process in batches  

---

## 30-day starter track (highest signal)

If you only have a month, do these days first:

| Order | Day | Company | Why |
| ---: | ---: | --- | --- |
| 1 | 20 | Cloudflare | proxy + limit + cache |
| 2 | 22 | Discord | real-time presence |
| 3 | 31 | Dropbox | sync planner |
| 4 | 16 | AWS | resource slot manager |
| 5 | 21 | Datadog | metrics pipeline |
| 6 | 49 | 1Password | seal/open secrets |
| 7 | 7 | Stripe | idempotency |
| 8 | 39 | Fastly | edge cache |
| 9 | 44 | Zscaler | policy proxy |
| 10 | 37 | Crowdstrike | IOC matcher |
| 11 | 3 | Meta | concurrent cache |
| 12 | 5 | Google | safe parser |
| 13 | 17 | Microsoft | path/UTF-8 safety |
| 14 | 24 | Kraken | matching engine |
| 15 | 30 | Figma | concurrent ops |
| 16 | 8 | Jump | ring buffer |
| 17 | 55 | Sentry | coalescing |
| 18 | 64 | Cisco | LPM routes |
| 19 | 79 | Bytecode Alliance | fuel VM |
| 20 | 100 | Polars/DataFusion | columnar agg |
| 21 | 12 | Coinbase | limits + allowlist |
| 22 | 45 | Okta | sessions |
| 23 | 61 | Materialize | incremental view |
| 24 | 66 | Docker | refcount GC |
| 25 | 80 | LaunchDarkly | flag eval |
| 26 | 84 | Protocol Labs | CAS store |
| 27 | 86 | Mozilla | URL/origin |
| 28 | 4 | Netflix | concurrency limit |
| 29 | 51 | Chainlink | median oracle |
| 30 | 65 | Oracle | reconciler |

---

## Daily checklist (copy per solution)

```text
[ ] Compiles
[ ] Happy-path test
[ ] Edge-case test (empty / overflow / timeout / cancel)
[ ] Concurrent test if shared state
[ ] Noted time + space complexity
[ ] Noted one production gap (metrics, persist, etc.)
```
