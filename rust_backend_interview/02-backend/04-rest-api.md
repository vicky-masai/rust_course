# REST API

## Interview Question

REST API.

## Interview Answer

"A REST API is a stateless HTTP service using resources and standard methods like GET, POST, PUT, PATCH, and DELETE."

---

## Follow-up Questions & Answers

### Q1. What does stateless mean in the context of REST APIs?

**Interview Answer**

Stateless means the server does not store any client session data between requests. Each request must contain all the information needed to process it, such as authentication tokens or query parameters. This makes horizontal scaling easier because any server instance can handle any request without relying on session affinity.

---

### Q2. How do you design REST API routes in Axum?

**Interview Answer**

I use Axum's `Router` with method-specific handlers like `.get()`, `.post()`, `.put()`, `.delete()`. Routes are organized hierarchically with `.nest()` for grouping, for example `/api/v1/users` maps to a users router. I extract path parameters, query parameters, and JSON bodies using Axum's type-safe extractors like `Path`, `Query`, and `Json`.

---

### Q3. What is the role of HTTP status codes in a well-designed REST API?

**Interview Answer**

Status codes communicate the result of a request semantically. I use `200` for success, `201` for resource creation, `204` for deletion, `400` for validation errors, `401` for unauthenticated, `403` for unauthorized, `404` for not found, and `500` for internal server errors. Proper status codes help frontend developers and API consumers handle responses correctly without reading the body.

---

### Q4. How do you version your REST APIs?

**Interview Answer**

I prefix routes with a version number like `/api/v1/users` so breaking changes can be introduced in `/api/v2/` without disrupting existing clients. The version is part of the URL path rather than a header because it's simpler to test and debug. I maintain backward compatibility for at least one version before deprecating old endpoints.

---

### Q5. What is HATEOAS and do you use it in your projects?

**Interview Answer**

HATEOAS stands for Hypermedia As The Engine Of Application State, meaning responses include links to related resources so clients navigate the API dynamically. In practice, I rarely implement full HATEOAS because it adds complexity that most frontend teams don't consume. I focus on consistent response formats, proper pagination links, and clear resource naming instead.

---

### Q6. How do you handle validation in Axum REST APIs?

**Interview Answer**

I use `serde` with derive macros like `Validate` from the `validator` crate on request body structs. Failed validation returns a `400 Bad Request` with descriptive error messages. I also validate path and query parameters in the extractor implementation so invalid requests are caught before reaching business logic.

---

### Q7. What is content negotiation and why does it matter?

**Interview Answer**

Content negotiation lets the client specify the response format using the `Accept` header, typically JSON or XML. In Axum I default to JSON since REST APIs almost always use `application/json`. Supporting multiple formats adds complexity but can be useful when integrating with legacy systems that require XML responses.

---
