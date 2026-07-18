# Docker Networking

## Interview Question

How does Docker networking work and what are the different network drivers?

## Interview Answer

Docker networking allows containers to communicate with each other, the host, and external services. Docker provides several network drivers: bridge (default, for single-host communication), host (shares the host's network stack), overlay (multi-host communication in Swarm), macvlan (assigns MAC addresses), and none (disables networking). Each container gets its own network namespace with virtual Ethernet pairs connecting it to the Docker bridge. For Rust backends, understanding networking is critical for connecting to databases, caching layers, and exposing HTTP APIs.

---

## Follow-up Questions & Answers

### Q1. What is the default bridge network and what are its limitations?

**Interview Answer**

The default bridge network (`docker0`) automatically connects all containers that don't specify a network. Containers on the default bridge can communicate via IP addresses but cannot resolve each other by container name — DNS is only available on user-defined bridge networks. It also applies the same iptables rules to all containers without fine-grained control. Always create user-defined bridge networks for production Rust deployments to get DNS-based service discovery.

---

### Q2. How do you create a user-defined bridge network and why is it preferred?

**Interview Answer**

Create one with `docker network create my-network` and attach containers with `--network my-network`. User-defined bridges provide automatic DNS resolution so a Rust backend can connect to a PostgreSQL container using the hostname `db` instead of a dynamic IP. They also offer better network isolation and can be connected/disconnected from running containers. This is the standard approach for multi-container applications like a Rust API with a database.

---

### Q3. What is the `host` network driver and when would you use it?

**Interview Answer**

The `host` network driver removes network isolation entirely — the container shares the host's IP address and ports directly. Use it when raw network performance matters since it eliminates the overhead of Docker's virtual networking. It's common in high-throughput Rust applications where latency is critical. The tradeoff is that port conflicts can occur, and you lose the ability to run multiple instances on different ports without external load balancing.

---

### Q4. How does port mapping work with `-p` and `-P` flags?

**Interview Answer**

The `-p` flag maps a specific host port to a container port: `-p 8080:8080` routes host port 8080 to container port 8080. The `-P` flag maps all `EXPOSE` ports to random high-numbered host ports. Port mapping only works on bridge networks, not host networking. For a Rust web server inside a container, you'd use `-p 8080:8080` to make it accessible on the host. You can also bind to a specific interface: `-p 127.0.0.1:8080:8080` restricts access to localhost.

---

### Q5. What is the `overlay` network driver and when is it used?

**Interview Answer**

Overlay networks enable container-to-container communication across multiple Docker hosts in a Swarm or Kubernetes cluster. They use VXLAN tunneling to create a virtual network spanning physical machines. For distributed Rust microservices, overlay networks let services on different nodes communicate as if they're on the same local network. They're not relevant for single-host deployments but become essential in clustered production environments.

---

### Q6. How do you connect an existing container to a new network?

**Interview Answer**

Use `docker network connect <network-name> <container-name>` to attach a running container to an additional network. A container can belong to multiple networks simultaneously, enabling it to communicate with different groups of services. For example, a Rust API server might connect to both a `frontend` network for receiving requests and a `backend` network for database access. Use `docker network disconnect` to remove a network attachment.

---

### Q7. What is the `none` network driver and what are its use cases?

**Interview Answer**

The `none` driver completely disables networking for a container — it gets only a loopback interface. Use it for batch processing containers that don't need network access, such as a Rust job that reads from and writes to mounted volumes. It's also useful for security-sensitive workloads that must be completely network-isolated. Containers on `none` can still communicate with the host via loopback if explicitly configured.

---

### Q8. How does Docker resolve DNS between containers?

**Interview Answer**

On user-defined bridge and overlay networks, Docker runs an embedded DNS server at 127.0.0.11 that resolves container names and network aliases. When a Rust backend connects to a PostgreSQL container on the same bridge network, it can use `db` as the hostname. This DNS resolution only works within the same user-defined network — containers on the default bridge or different networks cannot resolve each other by name. Custom DNS aliases can be added with `--network-alias`.

---

### Q9. What is the `macvlan` network driver?

**Interview Answer**

`macvlan` assigns a real MAC address to each container, making it appear as a physical device on the network. The container gets an IP address from the physical network's DHCP server or a static configuration. Use it when containers must be directly addressable on the LAN without port mapping. It's useful for legacy Rust applications that expect to be directly on the network, but it can cause issues with WiFi networks and requires careful network planning.

---

### Q10. How would you troubleshoot Docker network connectivity issues?

**Interview Answer**

Start with `docker network ls` to verify network existence and `docker network inspect <network>` to see connected containers and their IPs. Use `docker exec` to run `ping`, `curl`, or `nslookup` inside containers to test connectivity. Check iptables rules with `iptables -L` and verify port mappings with `docker port <container>`. For Rust applications, use the `RUST_LOG=trace` environment variable to log connection attempts. Common issues include mismatched network names, firewall rules, and DNS resolution failures between different networks.
