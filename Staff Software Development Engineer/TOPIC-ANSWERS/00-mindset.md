# LEVEL 00 — Engineering Mindset

These eight questions are how a staff engineer thinks. Ask them for every topic you study. If you can answer them out loud, you can talk about the topic with anyone — juniors, peers, or interviewers.

---

### 0001. Why does this exist?

Every tool, pattern, or system exists because someone had a pain. Databases exist because files alone don't handle concurrent writes and queries well. Kubernetes exists because running containers by hand doesn't scale. When you learn something new, don't start with "how do I use it?" Start with "what pain made people invent this?"

In conversation: *"This exists because X kept breaking / getting expensive / getting unsafe at scale."* That one sentence shows you think like a designer, not a tutorial follower.

---

### 0002. What problem does it solve?

Name the problem in plain words. Redis solves "we need the same data in memory, very fast, shared across many servers." Raft solves "several machines must agree on one truth even when some crash." If you can't name the problem in one sentence, you don't understand the tool yet.

Staff tip: the same tool can solve different problems in different companies. Always ask *which* problem your team is using it for — not the marketing list of every feature.

---

### 0003. How does it work internally?

Surface knowledge is "I call this API." Depth is "when I call this, here's what happens under the hood." Example: HTTP isn't magic — TCP carries bytes, TLS encrypts them, HTTP parses request lines and headers. Redis isn't "fast storage" — it's a single-threaded event loop over in-memory data structures with optional persistence.

When you explain internals, walk the path of one request or one write. People trust engineers who can follow the path.

---

### 0004. What are the tradeoffs?

Nothing is free. Strong consistency costs latency. Caching speeds reads but risks stale data. Microservices give team independence but add network failure and ops complexity. Staff engineers speak in tradeoffs: *"We chose A over B because we valued X more than Y."*

If someone asks "is X better than Y?" the staff answer is almost never yes/no. It's "better for what constraint?"

---

### 0005. What happens when it fails?

Production systems fail. Disks fill. Networks partition. Processes OOM. The staff question is: when this piece dies, what does the user see, what do we lose, and how do we recover? Idempotent APIs, retries with backoff, dead-letter queues, circuit breakers — these exist because failure is normal.

Practice saying: *"If Redis goes down, we degrade to DB with higher latency; we don't take the whole site down."* That is reliability thinking.

---

### 0006. How would Amazon / Uber / Stripe / OpenAI build this?

This forces scale imagination. A todo app and Uber's trip system are different animals. Big companies add: multi-region, strong observability, careful consistency models, blast-radius limits, and clear ownership. You don't copy FAANG blindly — you borrow the *questions* they ask: blast radius, cost, latency SLOs, abuse, compliance.

Good talk track: *"At our scale we don't need their full design, but the principle of isolating failure domains still applies."*

---

### 0007. How would I debug it in production?

Theory without debugging is fragile. Know: logs, metrics, traces, `strace`/`perf`, flamegraphs, DB `EXPLAIN`, packet captures. For any topic, ask: "If this misbehaves at 3am, what do I look at first?"

Staff engineers reduce MTTR (mean time to recovery) by knowing the symptoms → hypothesis → signal chain. Always connect a concept to a concrete debug path.

---

### 0008. How would I scale it to 100M users?

Scaling isn't "add more servers." It's: where is the bottleneck (CPU, memory, disk, network, lock contention)? What can be cached, sharded, async, or approximated? What consistency can you relax? What becomes a cost problem before it's a tech problem?

Talk like this: *"Reads are 95% of traffic, so cache + read replicas first. Writes need partitioning and careful idempotency. Cross-region is a consistency and latency project, not a checkbox."*
