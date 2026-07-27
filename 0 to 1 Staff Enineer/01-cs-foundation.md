# LEVEL 01 — Computer Science Foundation

---

## Computer Architecture

### 0009. Binary & Hexadecimal

Computers only store bits: 0 and 1. Binary is the natural language of hardware. Hexadecimal (base 16) is how humans read binary without drowning in zeros and ones — one hex digit equals four bits, so a byte is two hex digits (`0xFF` = 255).

You need this to read memory dumps, network packets, color codes, bitmasks, permissions, and hash outputs. When someone says "the high bit is set" or "mask with `0x0F`," binary/hex is what they're talking about.

**Talk track:** *"Hex is a compact human view of binary. Debugging memory, protocols, and flags always comes back to bits."*

---

### 0010. CPU Architecture

The CPU fetches instructions, decodes them, and executes them. Modern CPUs have cores, caches, pipelines, and specialized units (ALU for math, SIMD for vector ops). Your program is not "running on the OS" in the abstract — the OS schedules threads onto CPU cores that execute machine instructions.

Architecture choices matter: x86_64 vs ARM (Apple Silicon, AWS Graviton) change performance, power, and binary compatibility. Staff engineers care because cost and latency often track CPU efficiency.

**Talk track:** *"Application performance is ultimately instruction throughput, memory stalls, and how well we use cores without fighting over shared resources."*

---

### 0011. Registers

Registers are tiny, ultra-fast storage *inside* the CPU. Values must usually be in registers to do math or call functions quickly. There are few of them, so compilers constantly move data between registers and RAM (spilling).

Calling conventions define which registers hold arguments, return values, and which the callee must preserve. This shows up in FFI, assembly reading, and understanding why "too many locals" can get slower.

**Talk track:** *"RAM is cheap and slow; registers are scarce and fast. Hot code lives or dies by register pressure and memory traffic."*

---

### 0012. Instruction Pipeline

CPUs don't finish one instruction before starting the next. They pipeline: fetch, decode, execute, memory, writeback — like an assembly line. Throughput rises when the pipe stays full.

Branches and cache misses stall the pipe. That's why branch prediction and cache-friendly code matter. A "simple" if-statement in a tight loop can dominate runtime if predicted wrong.

**Talk track:** *"Modern CPUs are pipelines. Performance is about keeping the pipe fed — fewer surprises, better locality, fewer dependencies."*

---

### 0013. CPU Cache (L1 / L2 / L3)

Between registers and RAM sit caches. L1 is smallest/fastest (per core), L2 bigger, L3 shared across cores (typically). Hitting L1 might be ~1ns; RAM can be ~100ns. That 100× gap is why "algorithms on paper" lose to "data layout in memory" in real systems.

Hot data that fits in cache flies. Data that jumps randomly through a huge array crawls — even if Big-O looks fine.

**Talk track:** *"Most 'slow code' is waiting on memory. Design for cache: contiguous data, tight working sets, avoid pointer chasing in hot paths."*

---

### 0014. Cache Lines

Caches move memory in blocks called cache lines (often 64 bytes), not single bytes. Touching one byte can pull 64. Two threads writing different variables on the *same* cache line cause false sharing (see 0025).

Struct layout, padding, and array-of-structs vs struct-of-arrays decisions often come down to cache lines.

**Talk track:** *"The unit of caching is the line, not the field. Pack hot fields together; keep heavily written fields on separate lines when shared across cores."*

---

### 0015. RAM

RAM (DRAM) is the machine's main working memory. It's volatile — power off, data gone. Capacity is large vs cache, latency is high vs cache. Your process's heap and stack live in virtual address space backed (eventually) by RAM pages.

OOM kills, swap thrashing, and memory bandwidth limits are RAM realities. In backend work, RAM often costs more than CPU at scale — so memory efficiency is money.

**Talk track:** *"RAM is the working set budget. Exceed it and you swap or die. Bandwidth and latency to RAM dominate many services."*

---

### 0016. Stack Memory

The stack holds function frames: return addresses, locals, spilled registers. Allocating on the stack is cheap (move a pointer). Stack size is limited (often a few MB per thread) — deep recursion or huge locals can overflow.

In Rust, stack vs heap is visible in ownership and types. Large objects usually go on the heap (`Box`, `Vec`).

**Talk track:** *"Stack is fast, scoped, and limited. Per-thread stacks also mean 10k threads can burn gigabytes just on stack reservations."*

---

### 0017. Heap Memory

The heap is for dynamic lifetimes — data that outlives a stack frame or has unknown size. Allocators (`malloc`, jemalloc, etc.) find free blocks, split/coalesce, and can fragment.

Heap allocation is slower than stack and can contend under threads. High-performance Rust often reduces allocations in hot paths.

**Talk track:** *"Heap buys flexibility; you pay with latency, fragmentation, and allocator contention. Profile allocations before guessing."*

---

### 0018. Virtual Memory

Processes don't see physical RAM addresses directly. They see virtual addresses. The OS + MMU map virtual pages to physical frames. This isolation stops one process from scribbling on another's memory.

Virtual memory enables: process isolation, memory-mapped files, demand paging, and the illusion of large address spaces.

**Talk track:** *"Every pointer your program holds is virtual. The hardware walks page tables to find real RAM — or triggers a page fault."*

---

### 0019. Paging

Memory is split into pages (commonly 4KB; huge pages exist). Page tables map virtual → physical. A page fault happens when the mapping isn't present — OS may load from disk (swap/file) or kill the process (segfault / OOM paths).

TLB (translation lookaside buffer) caches recent translations. TLB misses hurt. Huge pages reduce TLB pressure for big working sets (DBs, VMs).

**Talk track:** *"Paging gives isolation and overcommit flexibility, but page faults and TLB misses are real latency."*

---

### 0020. Memory Alignment

CPUs prefer (or require) data at addresses divisible by their size — e.g., a `u64` at a multiple of 8. Misaligned access can be slower or fault on some architectures.

Compilers insert padding so fields align. This affects struct size and cache behavior.

**Talk track:** *"Alignment is a hardware contract. Ignoring it costs performance or correctness on strict platforms."*

---

### 0021. Memory Padding

To satisfy alignment, compilers insert unused bytes between struct fields. A struct with a `u8` then a `u64` isn't 9 bytes — it may be 16. Field order can shrink structs (`u8, u8, u64` vs interleaved poorly).

In networked protocols and disk formats, you control layout explicitly (`#[repr(C)]`, packed structs) and must understand padding.

**Talk track:** *"Padding is the silent struct bloat. Reorder fields; measure size with `std::mem::size_of`."*

---

### 0022. Branch Prediction

When the CPU sees an `if`, it guesses which way you'll go and speculatively executes. Correct guess = pipeline stays full. Wrong guess = flush and restart — expensive in tight loops.

Patterns help predictors (always-true checks). Random branches hurt. This is why sorting data before a branchy loop can mysteriously speed things up.

**Talk track:** *"Branches aren't free. Predictable control flow is a performance feature."*

---

### 0023. SIMD

Single Instruction Multiple Data — one instruction operates on a vector of values (4 floats at once, etc.). Used in codecs, crypto, JSON parsers, databases, ML, image processing.

Compilers auto-vectorize sometimes; explicit SIMD (intrinsics / crates) when you need guaranteed speed. Data must be laid out for vector loads.

**Talk track:** *"SIMD turns loops over numbers into batch ops. Layout and alignment matter as much as the instruction."*

---

### 0024. NUMA

Non-Uniform Memory Access: on multi-socket machines, each CPU has local RAM that's faster than remote RAM. Cross-socket memory access adds latency.

Databases and high-throughput services pin threads and allocate memory local to the socket. Blindly using all cores without NUMA awareness can leave performance on the table.

**Talk track:** *"On big boxes, 'which socket owns this memory?' is a real question. Locality isn't just cache — it's NUMA."*

---

### 0025. False Sharing

Two threads write different variables that sit on the same cache line. Hardware keeps cache lines coherent — each write invalidates the other's copy. They thrash the line even though logically they don't share data.

Fix: pad/align so hot mutable fields sit on separate cache lines (`crossbeam` CachePadded, align attributes).

**Talk track:** *"False sharing is invisible contention. Symptoms look like 'scaling stops adding cores' with low lock times."*

---

### 0026. Cache Locality

Spatial locality: nearby addresses touched soon (arrays win over linked lists). Temporal locality: same address reused soon (loop over hot data).

Good locality = cache hits. Pointer-heavy graphs, huge objects, and random maps hurt locality. This is why contiguous `Vec` often beats clever pointer structures in practice.

**Talk track:** *"Big-O assumes uniform memory. Real machines reward locality. Design data for how it's scanned."*

---

### 0027. Memory Barriers

On multi-core CPUs, reads/writes can be reordered for speed. A memory barrier (fence) tells the CPU/compiler: don't reorder memory ops across this point. Atomics with orderings (`Acquire`, `Release`, `SeqCst`) insert the needed constraints.

Without barriers, lock-free code sees impossible-looking states. This is deep concurrency — most app code should use mutexes/channels and leave barriers to atomics experts.

**Talk track:** *"Barriers restore a sane happens-before story across cores. Wrong ordering = heisenbugs."*

---

## Operating Systems

### 0028. Linux Architecture

Linux is a monolithic kernel: process scheduling, virtual memory, filesystems, networking live in kernel space. User programs make syscalls to ask the kernel for work. Everything is often modeled as files (devices, procfs, sysfs).

For backend engineers, Linux *is* the production machine. Containers still share a kernel. Understanding Linux beats memorizing cloud YAML alone.

**Talk track:** *"User space runs your app; kernel space owns CPU, memory, devices, and the network stack. Syscalls are the boundary."*

---

### 0029. Process

A process is an isolated program instance: its own virtual address space, file descriptors, credentials. Isolation is the safety boundary. Creating a process is heavier than creating a thread (new address space).

In production: one service instance ≈ one or more processes. Crash of a process shouldn't corrupt another's memory.

**Talk track:** *"Processes isolate memory and privileges. They're the unit of crash isolation and deployment."*

---

### 0030. Thread

A thread is a schedulable execution path inside a process. Threads share the same address space — easy data sharing, easy data races. Each thread has its own stack.

Too many OS threads → high memory and context-switch cost. That's why async runtimes multiplex many tasks on fewer threads.

**Talk track:** *"Threads share memory; that's power and danger. Use shared state carefully or prefer message passing."*

---

### 0031. Scheduler

The kernel scheduler decides which runnable thread gets a CPU core and for how long. Policies (CFS on Linux) try to be fair while favoring interactive latency.

Your thread can be runnable but not running — waiting for a core. Priority, CPU affinity, and blocking syscalls all interact with scheduling.

**Talk track:** *"You don't control the CPU continuously. The scheduler slices time. Blocking one thread frees the core for others."*

---

### 0032. Context Switching

When the CPU switches threads/processes, it saves registers and loads another set — a context switch. Too many switches waste CPU on bookkeeping instead of work.

Causes: too many threads, frequent blocking, lock contention wakeups, bad sleep patterns. Measure with `perf`, `pidstat`, or runtime metrics.

**Talk track:** *"Context switches aren't free. If CPUs are busy but app throughput is low, check scheduling and contention."*

---

### 0033. Synchronization

When threads share mutable state, you need synchronization: mutexes, rwlocks, atomics, condition variables. Without it: data races and undefined behavior (in languages that allow it).

Synchronization serializes access — correctness first, then reduce contention. Prefer immutable data and channels when you can.

**Talk track:** *"Shared mutability needs a protocol. Locks are the simple protocol; lock-free is the expert protocol."*

---

### 0034. System Calls

A syscall is how user space asks the kernel to do privileged work: read a socket, allocate memory, open a file, sleep. Crossing into kernel has cost (mode switch).

High-performance servers minimize syscalls or batch them (`io_uring`, `readv`/`writev`). `strace` shows the syscall story of a process.

**Talk track:** *"Every I/O path goes through syscalls. Knowing which ones you issue is half of systems debugging."*

---

### 0035. File Systems

File systems organize bytes on disk: directories, inodes, metadata, journaling. Different FS (ext4, XFS, btrfs) trade performance, features, and recovery.

For backends: fsync durability, directory entry costs, and "write then rename" atomic replace patterns matter for safe persistence.

**Talk track:** *"A file write isn't durable until the FS and disk say so. Understand fsync or accept data loss on crash."*

---

### 0036. mmap()

Memory-map a file into your address space — access file bytes like an array; kernel pages data in/out. Used by databases, language loaders, high-perf parsers.

Risks: page faults on access (latency spikes), tricky consistency with concurrent writers, error handling harder than read()/write().

**Talk track:** *"mmap trades explicit I/O for demand paging. Great for large read-mostly files; be careful with durability and faults."*

---

### 0037. epoll

Linux API to wait efficiently on many file descriptors (sockets). Instead of checking thousands one-by-one, you sleep until something is ready. Foundation of event-driven servers (nginx, many runtimes).

Edge vs level triggering changes how you drain sockets. epoll is the classic C10k solution on Linux.

**Talk track:** *"epoll lets one thread manage thousands of connections without spinning. It's readiness notification."*

---

### 0038. io_uring

Modern Linux async I/O interface: submit a queue of operations, reap completions from another queue — fewer syscalls, can do network + disk, even linked ops. High-performance databases and proxies adopt it.

Harder API than epoll; still maturing operationally. Staff engineers watch it as the next I/O bottleneck breaker.

**Talk track:** *"io_uring batches and completes I/O with less kernel round-tripping. It's the scalability bet after epoll."*

---

### 0039. Signals

Signals are asynchronous notifications to a process (`SIGTERM`, `SIGINT`, `SIGKILL`, `SIGSEGV`). Used for graceful shutdown, job control, and crash reporting.

Async-signal-safe constraints make signal handlers treacherous — keep them tiny or use signalfd/self-pipe patterns. K8s sends SIGTERM before SIGKILL.

**Talk track:** *"SIGTERM means shut down cleanly; SIGKILL means die now. Design graceful drain on TERM."*

---

### 0040. IPC

Inter-Process Communication: pipes, Unix sockets, shared memory, message queues, gRPC across processes. Processes don't share memory by default (unless you set that up), so IPC is how they collaborate.

Choose by latency, safety, and operability. Shared memory is fastest and hardest; sockets are simpler and network-transparent.

**Talk track:** *"IPC is the API between processes. Prefer clear protocols over ad-hoc shared memory unless profiling forces you."*

---

## Networking

### 0041. OSI Model

A teaching layered model: Physical → Data Link → Network → Transport → Session → Presentation → Application. Real life is messier (TCP/IP stack), but layers help you locate bugs: is it DNS, TCP, TLS, or HTTP?

Staff use: when debugging "it doesn't connect," walk layers upward instead of randomly restarting pods.

**Talk track:** *"Layer the problem. Don't debug HTTP JSON when TCP never connected."*

---

### 0042. TCP

Reliable, ordered, connection-oriented byte stream. Handshakes, sequence numbers, ACKs, retransmits, congestion control. Great for most APIs. Head-of-line blocking inside a connection; latency sensitive to round trips.

Tuning: buffers, keepalive, Nagle/`TCP_NODELAY`, connection pools. TCP failures look like timeouts and resets.

**Talk track:** *"TCP gives reliability and ordering on a single connection. You still design retries and idempotency above it."*

---

### 0043. UDP

Datagrams, no connection, no delivery guarantee, no ordering. Low overhead — used in DNS, QUIC/HTTP3 underpinnings, gaming, VoIP, custom protocols.

You build reliability yourself if you need it. Firewalling and NAT behavior differ from TCP.

**Talk track:** *"UDP is raw speed and simplicity. Reliability is your job if you need it."*

---

### 0044. DNS

Maps names to addresses (and more: MX, TXT, SRV). Clients cache; TTLs matter. Failures: wrong record, stale cache, slow resolvers — often look like "the app is down."

Service discovery in cloud often still involves DNS. Debug with `dig`/`nslookup`, check TTL and split-horizon DNS.

**Talk track:** *"DNS is critical path infrastructure. Cache and TTL mistakes cause long-lived wrong routing."*

---

### 0045. TLS

Transport Layer Security encrypts and authenticates the connection (HTTPS). Handshake negotiates keys/certs; then symmetric crypto for data. Certificate authenticity chains to trusted CAs (or private PKI).

mTLS (0324) is mutual auth. TLS termination at load balancers is common. Version and cipher policy are security baselines.

**Talk track:** *"TLS protects data in transit and proves who you're talking to — if you verify certificates properly."*

---

### 0046. HTTP/1.1

Texty request/response protocol: methods, headers, body. Keep-alive reuses TCP connections. One request at a time per connection without pipelining tricks — browsers open multiple connections.

Still ubiquitous. Understanding status codes, headers (`Content-Length`, `Transfer-Encoding`, `Connection`) is daily backend work.

**Talk track:** *"HTTP/1.1 is simple and universal. Connection limits and head-of-line blocking pushed us to HTTP/2."*

---

### 0047. HTTP/2

Binary framing, multiplexing many streams on one TCP connection, header compression (HPACK). Fixes HTTP/1.1's connection sprawl. Still suffers TCP head-of-line blocking at packet layer.

Used widely behind TLS (`h2`). gRPC typically runs on HTTP/2.

**Talk track:** *"HTTP/2 multiplexes streams over one connection. Better for many small requests; TCP loss still stalls all streams."*

---

### 0048. HTTP/3

HTTP over QUIC (usually UDP). Independent streams so packet loss on one doesn't block others the same way. Faster connection setup (TLS integrated).

Adoption growing (CDNs, browsers). Operationally different: UDP allowlists, different debugging tools.

**Talk track:** *"HTTP/3 moves HTTP onto QUIC to reduce head-of-line blocking and handshake latency."*

---

### 0049. QUIC

Google-born transport over UDP: crypto by default, multiplexed streams, connection migration (network change). Basis for HTTP/3.

Think of it as "TCP + TLS lessons learned, redesigned." Complexity is high; libraries matter.

**Talk track:** *"QUIC redesigns reliable transport for modern networks — encrypted, multiplexed, mobile-friendly."*

---

### 0050. WebSocket

Upgrade from HTTP to a persistent bidirectional message channel over a single TCP connection. Good for live updates, chat, collaborative editing, game ticks.

Ops concerns: sticky sessions, idle timeouts, fanout scale, backpressure when clients are slow.

**Talk track:** *"WebSockets are long-lived bidirectional pipes. Great for push; plan for connection count and failure reconnect."*

---

### 0051. SSE

Server-Sent Events: server pushes a text event stream over HTTP; client receives one way. Simpler than WebSocket when you only need server→client.

Auto-reconnect is built into browsers. Works through many proxies more easily than WS sometimes. Not for binary or client→server heavy chat.

**Talk track:** *"SSE is one-way server push with less complexity than WebSockets when that fits the product."*

---

### 0052. gRPC

RPC framework over HTTP/2 (commonly) with Protobuf contracts. Strong typing, streaming RPCs, codegen clients. Popular for service-to-service.

Tradeoffs: browser support needs grpc-web, human debugging harder than JSON REST, versioning discipline required.

**Talk track:** *"gRPC is contract-first service RPC — fast and typed, with streaming, at the cost of tooling universality vs REST/JSON."*

---

### 0053. Reverse Proxy

Sits in front of app servers: TLS termination, routing, compression, auth gates, buffering. nginx, Envoy, HAProxy, cloud LBs.

Apps shouldn't all invent these concerns. A proxy centralizes cross-cutting edge behavior.

**Talk track:** *"A reverse proxy is the front door — security, routing, and shielding origins from the open internet."*

---

### 0054. Load Balancing

Spread traffic across instances: round-robin, least connections, latency-based, consistent hashing. L4 (TCP) vs L7 (HTTP) decide how smart routing can be.

Health checks + removing bad nodes matter more than the algorithm. Sticky sessions are a smell if overused — prefer stateless apps.

**Talk track:** *"Load balancing is traffic + health. Algorithm is secondary to correct health signals and even capacity."*

---

## Linux Tools

### 0055. top

Live view of processes: CPU%, memory, load average. First glance in an incident: is the box saturated? which PID?

Load average needs context (CPU count). `top` is the flashlight, not the root-cause microscope.

**Talk track:** *"Start with top to see if we're CPU-bound, memory-bound, or idle while the app still fails (then look elsewhere)."*

---

### 0056. htop

Friendlier interactive `top`: trees, search, nicer UI. Same purpose — live process resource view.

Use it for human navigation; use scripts/`ps` for automation.

---

### 0057. ps

Snapshot of processes (`ps aux`, `ps -ef`). Good for scripts and "what's running right now?" with filters.

Combine with `grep`, or better `pgrep`. Shows states: R running, S sleep, D uninterruptible wait (often disk), Z zombie.

**Talk track:** *"ps is the portable process inventory. Learn process states — D state often means storage pain."*

---

### 0058. vmstat

Reports processes, memory, swap, I/O, CPU. Watch `si`/`so` (swap in/out) — swapping kills latency. `wa` = CPU waiting on I/O.

Quick health of memory pressure and I/O wait.

**Talk track:** *"vmstat tells you if you're swapping or stuck in iowait — classic production tells."*

---

### 0059. iostat

Disk I/O stats: throughput, utilization, await times. When APIs are slow and CPU is idle, check disks.

`await` high + `%util` high → storage bottleneck. Cloud disks have IOPS/throughput caps.

**Talk track:** *"iostat separates 'app is slow' from 'disk is slow'."*

---

### 0060. sar

Historical system activity reporter (sysstat). Look back in time: CPU, memory, network at 2:15am when the page fired.

Install/enable data collection before you need it — staff move.

**Talk track:** *"sar is time-travel for host metrics when the incident already passed."*

---

### 0061. lsof

List open files/sockets by process. "Which process holds this port?" `lsof -i :8080`. "Who has this file open?" after deploy/delete issues.

Also finds leaked file descriptors.

**Talk track:** *"lsof maps processes to files and ports — essential for 'address already in use' and FD leaks."*

---

### 0062. ss

Modern socket statistics (`ss -lptn`). Replaces much of `netstat`. See listening ports, connection states, which process owns them.

Check TIME_WAIT piles, ESTABLISHED counts, listen queues.

**Talk track:** *"ss is how you inspect sockets on Linux today — listeners, states, and owning processes."*

---

### 0063. tcpdump

Packet capture. See what's actually on the wire: SYN storms, TLS handshakes failing, wrong IPs, retransmits.

Powerful and sensitive (privacy/security). Filter tightly. Pair with Wireshark for deep analysis.

**Talk track:** *"When logs lie, tcpdump shows the truth of packets — use carefully and with filters."*

---

### 0064. strace

Trace syscalls of a process. See exact `open`/`read`/`futex`/`connect` behavior. Amazing for "why is it hung?"

Heavy overhead — don't leave on in hot prod without care. Attach to one stuck worker.

**Talk track:** *"strace answers 'what is this process asking the kernel right now?'"*

---

### 0065. perf

Linux profiling: CPU flame graphs, cache misses, scheduler latency. Sample-based — find where CPU time actually goes.

Staff performance work: measure with `perf`, don't guess. Needs symbols for readable stacks.

**Talk track:** *"perf turns 'it's slow' into 'this function and these stalls burn the CPU'."*

---

### 0066. journalctl

Query systemd's journal — unit logs with time filters. `journalctl -u my.service -b` for since boot.

Central place for service stdout/stderr when under systemd.

**Talk track:** *"journalctl is the host log lens for systemd services — filter by unit and time."*

---

### 0067. systemd

Init system and service manager: units, dependencies, restarts, cgroups integration, timers. How most Linux services are supervised.

Know unit files, restart policies, and environment injection. Containers often hide this — bare metal/VMs don't.

**Talk track:** *"systemd owns service lifecycle on modern Linux. Restart policy and dependencies are production behavior."*
