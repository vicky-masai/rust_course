# Quick Question Bank (Staff SWE · Rust · Zscaler)

Use for follow-ups and bar-raiser rounds. Answers live in topic folders — this is an index only.

## Rust

1. Move vs copy vs clone — cost on a hot path?
2. Why might `async fn` in a trait historically be painful?
3. `select!` cancellation hazard example
4. When `Arc<Mutex<T>>` is a smell
5. How do you structure `unsafe` modules?
6. `cargo` workspace strategy for large products
7. Debugging a deadlock in async code
8. Memory allocator / jemalloc musings — when it matters
9. Zeroize secrets in memory — why/when
10. WASM for policy plugins — pros/cons

## Systems / design

1. Control plane vs data plane responsibilities
2. Policy cache invalidation
3. Global anycast POP design trade-offs
4. Logging without killing p99
5. Canary for security policy changes
6. Multi-region data residency
7. Idempotent provisioning APIs
8. Backpressure from SIEM export
9. Feature flag danger on authz path
10. Cost-aware architecture choices

## Distributed systems

1. Quorum reads/writes mental model
2. Outbox pattern
3. Consumer lag playbook
4. Poison pill
5. Fencing tokens
6. Sticky sessions vs stateless
7. Retry amplification
8. Chaos testing philosophy
9. SLO vs SLA vs error budget
10. Hot partition

## Security / net

1. TLS termination vs passthrough
2. Certificate rotation at scale
3. SSRF from customer connector
4. Cross-tenant IDOR
5. PII in logs
6. Supply chain (`cargo audit`, SBOM)
7. mTLS + authz
8. DNS security relevance
9. Rate limit bypass patterns
10. Insider threat controls

## Leadership

1. RFC that failed — why?
2. Raising the hiring bar
3. Deprecating a beloved bad system
4. Partnering with PM on “no”
5. Growing an L4 → L5 engineer
6. Ambiguity for two quarters — how you created clarity
7. Operational load too high — what you changed
8. Cross-geo collaboration failure
9. When you were wrong publicly
10. Defining “done” for a platform bet

## Behavioral

1. Sev-1 story
2. Missed deadline with integrity
3. Difficult peer
4. Customer escalation
5. Unpopular but correct security call
