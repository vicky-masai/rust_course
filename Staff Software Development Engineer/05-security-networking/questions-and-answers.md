# Round: Security & Networking — Questions & Expected Answers

**Duration:** 45m  
**Critical for Zscaler.** Staff candidates must show security judgment, not only feature delivery.

---

## Networking fundamentals

### Q1. Walk through a TLS 1.3 handshake at a high level. Where are the CPU costs?

**Strong answer**
- ClientHello/ServerHello, key share, certificate verify, finished; 1-RTT (0-RTT caveats).
- Costs: crypto (handshake > bulk AES-GCM), cert verify, OCSP/CRL if any, session tickets for resumption.
- Resumption / session tickets / ticket key rotation importance at proxy scale.

---

### Q2. TCP keepalive vs application heartbeat vs idle timeouts in a proxy.

**Strong answer**
- Different layers detect different failures; NAT middleboxes; need coordinated timeouts; half-open connections; resource leaks from idle sessions.

---

### Q3. HTTP/2 / HTTP/3 (QUIC) implications for a proxy.

**Strong answer**
- Multiplexing, head-of-line blocking differences; stream limits; DoS via many streams; connection coalescing; UDP/QUIC operational challenges (middleboxes).

---

### Q4. DNS: why does it matter for security products?

**Strong answer**
- Resolution path determines destination; DNS tunneling; spoofing; DoH/DoT; caching poison risks; customer DNS policy.

---

## Security engineering

### Q5. Threat model a multi-tenant SaaS control-plane API.

**Strong answer** (STRIDE-ish without needing the acronym)
- Authn/authz bugs → cross-tenant access
- IDOR on resource IDs
- Token theft / session fixation
- SSRF from connectors
- Supply chain / dependency compromise
- Insider admin abuse → break-glass + audit
- Log injection / PII leakage

---

### Q6. Fail open vs fail closed for a security gateway when policy service is unavailable.

**Strong answer**
- Product/risk decision: enterprise security often **fail closed** for sensitive apps; availability products may degrade.
- Mitigations: cached last-known-good policy with TTL + alerting; local compiled policy; tiered mode.
- Staff articulates business risk, not only engineering preference.

---

### Q7. How do you prevent cross-tenant data leakage in logs and metrics?

**Strong answer**
- Tenant ID mandatory on events; access-controlled query; careful cardinality in metrics (avoid raw user IDs as labels); redaction; separate encryption keys optional for high assurance.

---

### Q8. Secrets management in Kubernetes for a Rust service.

**Strong answer**
- Not long-lived secrets in env if avoidable; KMS/Vault/cloud secret manager; short-lived credentials; rotation; memory handling (don’t log secrets); file permissions; mTLS between services.

---

### Q9. Secure coding in Rust: what do you still worry about?

**Strong answer**
- Logic bugs, authz bugs, TOCTOU, panics as DoS, `unsafe`, dependency vulns, timing side channels in crypto, XSS/injection at boundaries, protobuf/JSON bomb DoS.

**Key point:** Memory safety ≠ application security.

---

### Q10. mTLS service-to-service: design pitfalls.

**Strong answer**
- Cert rotation; SPIFFE/SPIRE or mesh; clock skew; identity vs authorization (cert proves identity, still need authz); revocation hard → short TTL.

---

## Cryptography judgment (conceptual)

### Q11. Should you implement your own crypto protocol?

**Strong answer**
- Almost never; use vetted libraries (`rustls`, established AEAD); high-level misuse resistance; review by specialists for novel protocols.

---

### Q12. Password/token storage vs API keys at rest.

**Strong answer**
- Hash passwords (argon2/bcrypt); API keys hashed at rest; show-once; scopes; rotation; anomaly detection.

---

## Zero Trust concepts

### Q13. How is ZTNA different from VPN from an architecture/security perspective?

**Strong answer**
- App-level access vs network-level; reduced lateral movement; continuous verification; device posture; outbound connectors; identity-centric policy.

---

### Q14. Device posture in access decisions — failure modes?

**Strong answer**
- Stale posture; agent compromise; false positives blocking business; privacy; caching TTLs; fallback policies.

---

## Incident-style probes

### Q15. Suspicious spike in 5xx + customer reports of intermittent auth failures. First 30 minutes?

**Strong answer**
- Declare incident severity; dashboards (auth, IdP latency, cert expiry, deploy markers); blast radius by tenant/region; consider rollback; communications; preserve forensics.

---

## Pass / fail snapshot

| Hire | Threat models + networking + fail-open/closed judgment |
| No-hire | “Security team handles that”; cannot discuss TLS or tenancy |
