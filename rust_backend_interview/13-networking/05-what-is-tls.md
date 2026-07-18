# What is TLS?

## Interview Question

Explain the TLS handshake process and how it secures communication in a Rust backend.

## Interview Answer

TLS (Transport Layer Security) provides encrypted, authenticated communication over an untrusted network. The TLS 1.3 handshake works as follows: the client sends a ClientHello with supported cipher suites and key share, the server responds with ServerHello, its certificate, and a key share, and both derive session keys from the shared secret using Diffie-Hellman. The entire handshake completes in one round-trip (1-RTT) in TLS 1.3, down from two in TLS 1.2. In a Rust backend, TLS is typically implemented using rustls with certificates managed by Let's Encrypt or a corporate CA, and Nginx or the application itself handles termination. TLS ensures confidentiality (encryption), integrity (HMAC), and authentication (certificates).

---

## Follow-up Questions & Answers

### Q1. What is the difference between TLS 1.2 and TLS 1.3?

**Interview Answer**

TLS 1.3 reduces the handshake from 2 round-trips to 1 by having the client send key shares in the initial ClientHello. It removes insecure algorithms (RSA key exchange, CBC mode, RC4, SHA-1) and mandates forward secrecy through ephemeral Diffie-Hellman. It adds 0-RTT resumption for faster reconnections. TLS 1.3 also encrypts more of the handshake, including the certificate in some cases, improving privacy. The result is faster handshakes, stronger security, and reduced attack surface. All modern browsers and libraries support TLS 1.3, and it should be the default for new Rust backends.

---

### Q2. What are X.509 certificates and how does certificate validation work?

**Interview Answer**

X.509 certificates bind a public key to an identity (domain name) through a digital signature from a Certificate Authority (CA). A certificate contains the subject name, public key, validity period, issuer, and a signature from the issuer's private key. During TLS handshake, the client validates the server's certificate by: checking the chain of trust up to a trusted root CA, verifying the certificate is not expired, confirming the certificate's CN or SAN matches the hostname, and checking for revocation via OCSP or CRL. In Rust, rustls handles certificate validation automatically using the webpki-roots crate for trusted CAs.

---

### Q3. How do you implement TLS in a Rust Axum server using rustls?

**Interview Answer**

Using rustls, you load the server certificate and private key from PEM files, create a ServerConfig with rustls::ServerConfig::builder(), and configure it with the certificate chain and private key. You then wrap the TcpListener with TlsAcceptor from rustls to accept TLS connections. Alternatively, using axum-server with axum_server::bind_rustls, you can serve directly with TLS. In production, Nginx typically handles TLS termination and forwards plain HTTP to Axum, simplifying the application code. Let's Encrypt certificates can be automatically renewed using the rcgen or acme-rs crates.

---

### Q4. What is Perfect Forward Secrecy (PFS) and why is it important?

**Interview Answer**

Perfect Forward Secrecy ensures that even if a server's private key is compromised in the future, past session traffic cannot be decrypted. PFS is achieved through ephemeral key exchange — each session uses a new Diffie-Hellman key pair that is discarded after use. Without PFS, an attacker who records encrypted traffic and later steals the server's private key could decrypt all recorded sessions. TLS 1.3 mandates PFS by requiring ephemeral Diffie-Hellman for all connections. In rustls, PFS is enabled by default through the key exchange configuration.

---

### Q5. What is certificate pinning and when should you use it?

**Interview Answer**

Certificate pinning associates a specific certificate or public key with a server, rather than trusting any certificate signed by a trusted CA. If an attacker compromises a CA or performs a MITM attack with a valid certificate, pinning prevents acceptance of the fraudulent certificate. It is commonly used in mobile apps connecting to specific APIs. However, pinning is risky — if the pinned certificate expires or rotates without updating the client, connectivity breaks. In Rust backends, pinning is less common for server-to-server communication because certificate rotation is controllable. It is more relevant for client applications connecting to your API.

---

### Q6. How does OCSP stapling improve TLS performance?

**Interview Answer**

Without OCSP stapling, the client must contact the CA's OCSP responder to check if a certificate has been revoked, adding latency to the TLS handshake. With OCSP stapling, the server periodically queries the OCSP responder and "staples" the signed response to the TLS handshake, so the client does not need to make a separate request. This improves privacy (the CA does not know which sites you visit) and performance (eliminates an extra network round-trip). In Rust with Nginx, OCSP stapling is enabled by adding ssl_stapling on; to the Nginx configuration. For self-managed TLS in Rust, you can use the ocsp-stapling feature in some TLS libraries.

---

### Q7. What are the security implications of TLS misconfiguration?

**Interview Answer**

Common TLS misconfigurations include supporting deprecated protocols (SSL 3.0, TLS 1.0, TLS 1.1), weak cipher suites (RC4, DES, 3DES), missing HSTS headers (allowing downgrade attacks), and not validating client certificates when required. These misconfigurations can lead to eavesdropping, data tampering, and man-in-the-middle attacks. In Rust backends, using rustls with default secure settings avoids most misconfigurations. Tools like ssllabs.com test TLS configuration and report vulnerabilities. The principle of least privilege applies — only enable protocols and ciphers you actually need.

---

### Q8. How does TLS termination at a reverse proxy work?

**Interview Answer**

When Nginx terminates TLS, it handles the entire TLS handshake with the client, decrypts the traffic, and forwards plain HTTP to the backend server. This centralizes certificate management — you only manage certificates in one place. The backend communicates over a trusted internal network (or uses mTLS for internal encryption). Benefits include reduced CPU load on application servers (TLS operations are expensive), simplified certificate rotation, and the ability to add security headers at the proxy layer. In production, this is the standard architecture for Rust backends behind Nginx.

---

### Q9. What is mutual TLS (mTLS) and when is it used?

**Interview Answer**

Mutual TLS requires both the client and server to present certificates during the handshake. The server validates the client's certificate, and the client validates the server's certificate. mTLS is used for service-to-service authentication in microservices architectures, zero-trust networks, and API client authentication. In Rust, rustls supports mTLS by configuring ClientConfig with client certificates and ServerConfig with a trusted CA for client certificates. mTLS provides strong authentication without passwords or tokens but adds complexity to certificate management and rotation.

---

### Q10. How do you manage TLS certificate rotation in a production Rust backend?

**Interview Answer**

Certificate rotation involves replacing the TLS certificate before it expires without downtime. With Nginx, you update certificate files and run nginx -s reload, which gracefully reopens sockets with the new certificate. For application-level TLS in Rust, you can use dynamic configuration by watching certificate files and recreating the TlsAcceptor when they change. Let's Encrypt provides automatic rotation through ACME challenges — tools like certbot or acme-rs handle the renewal process. For internal services with mTLS, certificate rotation is managed through a certificate authority (like Vault's PKI backend) that issues short-lived certificates automatically.
