# Authorization

## Interview Question

What is authorization and how do you implement it in a production Rust backend?

## Interview Answer

Authorization determines what an authenticated user is permitted to do. After authentication verifies identity, authorization enforces access control policies — checking whether the user has the required permissions for the requested action. In a production Rust backend, I implement authorization using RBAC (Role-Based Access Control) with Axum extractors and middleware. Each route declares required permissions, and the authorization layer checks the user's roles and permissions before allowing the request to proceed. This is enforced at multiple levels: route-level middleware for broad access control, resource-level checks for ownership verification, and field-level filtering for data exposure. Authorization logic is centralized in a service layer rather than scattered across handlers.

---

## Follow-up Questions & Answers

### Q1. What is the difference between RBAC, ABAC, and ACL?

**Interview Answer**

RBAC (Role-Based Access Control) assigns permissions to roles, and users receive roles. ABAC (Attribute-Based Access Control) evaluates policies based on attributes of the user, resource, action, and environment (e.g., "allow if user.department == resource.department"). ACL (Access Control Lists) directly map users or groups to permissions on specific resources. RBAC is simplest and most common for web applications. ABAC is more flexible but complex to implement and audit. ACLs are common at the OS/file system level. In Rust backends, RBAC is typically sufficient, with ABAC for complex multi-tenant systems.

---

### Q2. How do you implement authorization checks in Axum?

**Interview Answer**

Implement authorization using Axum extractors and middleware. Create a PermissionChecker extractor that reads the user's roles from the JWT claims (set by authentication middleware) and checks if they include the required permission. Use tower::Layer to create reusable authorization middleware that wraps route groups. For resource-level authorization, pass the resource owner to an AuthorizationService that checks ownership. Example: if (user.id != resource.owner_id && !user.has_permission("admin")) return 403. The key is to make authorization checks declarative at the route level rather than imperative in every handler.

---

### Q3. What is the principle of least privilege and how does it apply to backend authorization?

**Interview Answer**

The principle of least privilege states that users and services should have only the minimum permissions needed to perform their functions. In a Rust backend, this means: a read-only API user should not have write permissions, an admin endpoint should require admin role, a user should only access their own data unless they have a specific permission, and database connections should use credentials with minimal required privileges. Implement this by defining granular permissions (not just "admin" vs "user"), using default-deny policies, and regularly auditing permission assignments.

---

### Q4. How do you handle authorization for resource ownership?

**Interview Answer**

Resource ownership ensures users can only access their own resources. In a Rust backend, when a user requests a resource, verify that user.id matches the resource's owner_id before returning data. With SQLx, add a WHERE user_id = $1 clause to all queries, so the database enforces ownership at the query level. For shared resources, check both ownership and explicit sharing permissions. Example: a user's posts are accessible only by the owner, but shared posts are accessible by anyone with the share link. Ownership checks should be in the service layer, not the handler, to ensure consistency across all access paths.

---

### Q5. What is the difference between authentication middleware and authorization middleware?

**Interview Answer**

Authentication middleware runs first, validates the JWT or session, and extracts user identity into the request extensions (request.extensions().insert(user)). It rejects unauthenticated requests with 401 Unauthorized. Authorization middleware runs after authentication, checks the user's permissions against the route's requirements, and rejects unauthorized requests with 403 Forbidden. In Axum, they are separate tower layers with different responsibilities. Authentication is concerned with identity; authorization is concerned with permissions. Mixing them creates code that is harder to audit and maintain.

---

### Q6. How do you prevent Insecure Direct Object References (IDOR)?

**Interview Answer**

IDOR occurs when a user can access resources by manipulating IDs in the URL (e.g., /api/users/123 to /api/users/456). Prevention requires verifying that the authenticated user has access to the requested resource, not just that the resource exists. Always add an ownership or permission check: SELECT * FROM posts WHERE id = $1 AND user_id = $2. Never return data based solely on the resource ID without verifying the user's authorization. Use UUIDs instead of sequential IDs to make enumeration harder, but do not rely on UUIDs as a security measure — always enforce authorization checks.

---

### Q7. How do you handle cross-service authorization in microservices?

**Interview Answer**

Cross-service authorization uses service-to-service authentication (mTLS or API keys) and token forwarding. The API gateway authenticates the user and forwards a signed token or header with user identity to downstream services. Each service validates the token's signature using a shared public key. For fine-grained cross-service permissions, use a centralized policy engine (like Open Policy Agent) or carry permissions in the JWT claims. In Rust microservices, each service has middleware that validates incoming tokens and extracts user context. Trust between services is established through mutual TLS, not re-authentication on every hop.

---

### Q8. What is the difference between deny-by-default and allow-by-default authorization?

**Interview Answer**

Deny-by-default means all access is denied unless explicitly permitted — you must add permissions for every allowed action. Allow-by-default means all access is permitted unless explicitly denied — you must add restrictions for every blocked action. Deny-by-default is the secure approach because new routes or features start locked down, and you must consciously grant access. Allow-by-default is dangerous because forgotten restrictions leave resources exposed. In Rust backends, implement deny-by-default by requiring explicit permission annotations on every route and rejecting any request without a matching permission.

---

### Q9. How do you audit and log authorization decisions?

**Interview Answer**

Log every authorization decision (allowed and denied) with the user ID, requested resource, action, and outcome. Use structured logging (tracing crate in Rust) to enable querying and alerting. Denied attempts should generate warning-level logs that can trigger alerts for potential attacks. Audit logs should be immutable and stored in a separate system (ELK, CloudWatch) for compliance. Example: tracing::info!(user_id, resource, action, "authorized") or tracing::warn!(user_id, resource, action, "denied"). Regular audits of permission assignments ensure no excessive privileges have accumulated.

---

### Q10. How do you handle authorization during database migrations or feature flags?

**Interview Answer**

During database migrations that change permission structures, implement a migration strategy that handles both old and new permission models during the transition period. Use feature flags to toggle between authorization logic versions. In Rust, use environment variables or a configuration service to control which authorization path is active. Test both authorization paths in staging. For migrations that add new permissions, default new users to deny until permissions are explicitly granted. Always have a rollback plan — if the new authorization logic has bugs, you can revert to the previous version quickly.
