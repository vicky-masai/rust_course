# Horizontal vs Vertical Scaling

## Interview Question

Horizontal vs Vertical Scaling.

## Interview Answer

"Horizontal scaling adds more servers. Vertical scaling increases the resources of one server."

---

## Follow-up Questions & Answers

### Q1. When would you choose vertical over horizontal scaling?

**Interview Answer**

Vertical scaling is simpler when you need a quick performance boost without changing application architecture. It works well for databases or applications that are not designed for concurrency. However, there is a hardware ceiling, so I use vertical scaling as a short-term fix while planning horizontal scaling for long-term growth.

---

### Q2. What are the challenges of horizontal scaling for a stateful application?

**Interview Answer**

Stateful applications like WebSocket servers or session-based services need sticky sessions or shared state stores like Redis so any server can handle any request. Without this, user context is lost when requests hit different instances. I design backends to be stateless by storing session data in Redis, which makes horizontal scaling straightforward.

---

### Q3. How does horizontal scaling work with a Rust/Axum backend?

**Interview Answer**

I deploy multiple Axum instances behind a load balancer like NGINX or AWS ALB. Since Axum handlers are stateless by default with shared state in `Extension` or `State`, any instance can process any request. I use connection pooling per instance and deploy with Docker or Kubernetes to scale based on CPU or request count metrics.

---

### Q4. What is auto-scaling and how do you configure it?

**Interview Answer**

Auto-scaling automatically adds or removes instances based on metrics like CPU utilization, request count, or queue depth. On AWS I configure an auto-scaling group with minimum, maximum, and desired instance counts tied to CloudWatch alarms. For Kubernetes, I use a Horizontal Pod Autoscaler that scales pods based on custom or built-in metrics.

---

### Q5. What is database scaling and why is it often the bottleneck?

**Interview Answer**

While you can scale application servers horizontally, a single database instance eventually becomes the bottleneck for writes. I address this with read replicas for read-heavy workloads, connection pooling with PgBouncer, and sharding for extreme scale. For most projects, read replicas and caching with Redis handle the load without the complexity of sharding.

---

### Q6. How do you handle load balancing for an Axum backend?

**Interview Answer**

I use a reverse proxy like NGINX or Traefik in front of multiple Axum instances with round-robin or least-connections routing. For cloud deployments, AWS ALB distributes traffic and performs health checks to remove unhealthy instances. I configure Axum's `/health` endpoint so the load balancer knows which instances are ready to serve traffic.

---

### Q7. What are the cost implications of horizontal vs vertical scaling?

**Interview Answer**

Vertical scaling has predictable costs since you pay for one larger machine, but prices increase non-linearly at higher tiers. Horizontal scaling distributes load across cheaper machines, which can be more cost-effective, but adds operational costs for load balancers, service discovery, and monitoring. I usually start vertical and move to horizontal when the hardware ceiling or cost efficiency justifies it.

---

### Q8. How does caching interact with scaling strategies?

**Interview Answer**

Caching with Redis reduces database load, so each server instance can handle more requests before you need to scale. A shared Redis cluster means all instances read from the same cache, maintaining consistency. I combine caching with horizontal scaling to delay the need for read replicas and database sharding, which are more expensive and complex to manage.

---
