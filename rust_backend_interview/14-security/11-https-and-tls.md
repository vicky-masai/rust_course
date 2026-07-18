# HTTPS and TLS

## Interview Question

How do you implement HTTPS and TLS in a production Rust backend and what certificate management strategies do you use?

## Interview Answer

HTTPS ensures encrypted, authenticated communication between clients and your backend. In production, TLS termination typically happens at Nginx or the load balancer, with certificates managed by Let's Encrypt (free, automated) or a corporate CA. For application-level TLS in Rust, rustls provides a safe TLS implementation with automatic certificate loading and validation. Certificate management involves: obtaining certificates via ACME challenges, automating renewal before expiry, monitoring certificate expiry, and handling certificate revocation. The goal is zero-downtime certificate rotation — updating certificates without restarting the server. Let's Encrypt certificates expire every 90 days, making automation essential.

---

## Follow-up Questions & Answers

### Q1. How does Let's Encrypt certificate automation work?

**Interview Answer**

Let's Encrypt uses the ACME protocol to automate certificate issuance and renewal. The client proves domain ownership through HTTP-01 challenges (serving a file at a well-known URL) or DNS-01 challenges (creating a DNS TXT record). Certbot or acme.sh handles the challenge automatically. In Rust, the acme-rs crate implements the ACME protocol, allowing your application to obtain and renew certificates programmatically. For automated renewal, set up a cron job or systemd timer that runs certbot renew, which renews certificates expiring within 30 days. Nginx reloads after renewal to pick up new certificates.

---

### Q2. What is TLS termination and where should it happen?

**Interview Answer**

TLS termination decrypts HTTPS traffic into plain HTTP at a designated point. Common termination points: Nginx/reverse proxy (most common — centralizes TLS management), load balancer (cloud-managed — AWS ALB handles TLS), or the application itself (for end-to-end encryption). For Rust backends, Nginx termination is preferred because: it offloads CPU-intensive crypto operations from the application, simplifies certificate management (one place), and allows the application to communicate securely over the internal network. For high-security environments, use mTLS for internal communication between services.

---

### Q3. How do you implement TLS directly in a Rust Axum server?

**Interview Answer**

Using axum-server with rustls: load certificate chain and private key from PEM files, configure rustls::ServerConfig with the certificates, and bind with axum_server::bind_rustls(addr, config). For dynamic certificate loading, use rustls::server::ResolvesServerCert to provide certificates based on SNI (Server Name Indication). The rustls crate handles TLS 1.2/1.3, cipher suite selection, and certificate validation automatically. For production, prefer Nginx termination over application-level TLS to simplify management, but application-level TLS is useful for internal services or direct client connections.

---

### Q4. What are certificate chains and why are they important?

**Interview Answer**

A certificate chain links your server certificate to a trusted root CA through intermediate certificates. The chain is: root CA (trusted by browsers) → intermediate CA → your server certificate. Browsers validate the entire chain — if any link is missing or invalid, they show a security warning. When configuring TLS, you must provide the full certificate chain (your cert + intermediates), not just your cert. Missing intermediate certificates are a common cause of TLS errors. In Rust with rustls, provide the chain as a Vec<Certificate> ordered from leaf to root.

---

### Q5. How do you handle certificate pinning in a Rust client?

**Interview Answer**

Certificate pinning associates a specific certificate or public key with a server, preventing MITM attacks with fraudulent certificates. In Rust with reqwest, use ClientBuilder::add_root_certificate to pin specific CAs, or implement custom certificate verification using rustls::client::ServerCertVerifier. Pin a specific public key (SPKI hash) rather than the entire certificate to survive certificate rotation. The downside is that pinning requires client updates when certificates change. For server-to-server communication in controlled environments, pinning provides strong protection. For public-facing applications, it is risky due to rotation requirements.

---

### Q6. What is OCSP stapling and how do you enable it?

**Online Certificate Status Protocol (OCSP)** stapling lets the server fetch and staple the certificate's revocation status to the TLS handshake, avoiding client-side OCSP checks that add latency and privacy concerns. In Nginx, enable with ssl_stapling on; ssl_stapling_verify on; and ssl_trusted_certificate pointing to the full certificate chain. For Rust with rustls, OCSP stapling is not natively supported in the server API, so it is typically handled at the Nginx layer. OCSP stapling improves performance (eliminates client OCSP requests) and privacy (the CA cannot track which sites clients visit).

---

### Q7. How do you handle TLS certificate errors in production?

**Interview Answer**

Common TLS errors and fixes: certificate_expired (renew the certificate), unknown_ca (missing intermediate certificates in the chain), hostname_mismatch (certificate CN/SAN does not match the domain), and self_signed (untrusted certificate). In Rust, rustls returns TLS errors that should be logged and monitored. For clients, implement retry logic with backoff for transient TLS errors. Monitor certificate expiry with Prometheus metrics and alert 30 days before expiry. For internal services, use a private CA (Vault PKI) and distribute the CA certificate to all services.

---

### Q8. What is the difference between TLS 1.2 and 1.3 cipher suites?

**Interview Answer**

TLS 1.2 cipher suites specify four components: key exchange (RSA, DHE, ECDHE), authentication (RSA, ECDSA), bulk encryption (AES-128-GCM, AES-256-GCM), and MAC (SHA-256). TLS 1.3 simplified this by supporting only five cipher suites: TLS_AES_256_GCM_SHA384, TLS_CHACHA20_POLY1305_SHA256, TLS_AES_128_GCM_SHA256, and two others. TLS 1.3 mandates forward secrecy (ephemeral key exchange) and removes weak algorithms. In Rust with rustls, the default cipher suite selection is secure — you rarely need to configure cipher suites manually. Stick with defaults unless you have specific compliance requirements.

---

### Q9. How do you manage TLS certificates across multiple services?

**Interview Answer**

Centralize certificate management with: a shared certificate store (Vault PKI, Kubernetes cert-manager, or cloud ACM), automatic renewal through the ACME protocol, and distribution to services via secrets mounting. In Kubernetes, cert-manager automates Let's Encrypt certificate management with Certificate resources. For VMs, use a deployment tool (Ansible, Terraform) to distribute certificates. For Rust services behind Nginx, manage certificates at the Nginx level. Monitor all certificate expiry dates in a central dashboard. The goal is that no human manually handles certificates — the entire lifecycle is automated.

---

### Q10. How do you test TLS configuration for security?

**Interview Answer**

Use SSL Labs (ssllabs.com/ssltest) to scan your public TLS configuration and check for vulnerabilities (BEAST, POODLE, Heartbleed). Use testssl.sh for local testing: testssl.sh --all https://yourdomain.com. Verify: TLS 1.3 is preferred, strong cipher suites only, HSTS header present, OCSP stapling enabled, and certificate chain is complete. In Rust, use rustls's built-in validation to test client-side TLS. For internal services, test that mTLS is correctly configured. Automate TLS testing in CI to catch configuration regressions before deployment.
