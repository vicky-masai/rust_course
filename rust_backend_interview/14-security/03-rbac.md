# Role-Based Access Control (RBAC)

## Interview Question

Explain how you implement RBAC in a production Rust backend with Axum.

## Interview Answer

RBAC (Role-Based Access Control) assigns permissions to roles, and users receive one or more roles. In a Rust backend, I define a roles table and a permissions table in PostgreSQL with a many-to-many relationship. Each route declares required permissions, and an Axum extractor checks the user's roles (from JWT claims or a database lookup) against the route's required permission. Roles like "admin", "editor", and "viewer" aggregate multiple permissions (e.g., "user:read", "user:write", "user:delete"). This model is auditable, scalable, and easy to understand — new permissions are added to roles, not scattered across handler code. The RBAC service layer is separate from the HTTP layer, allowing reuse across different interfaces.

---

## Follow-up Questions & Answers

### Q1. What is the difference between RBAC and ABAC?

**Interview Answer**

RBAC assigns permissions based on roles — a user with the "admin" role gets all admin permissions regardless of context. ABAC (Attribute-Based Access Control) evaluates policies based on attributes of the user, resource, action, and environment (e.g., "allow if user.department == resource.department AND time is during business hours"). RBAC is simpler to implement, audit, and explain to stakeholders. ABAC provides fine-grained, context-aware access control but is significantly more complex. For most Rust backends, RBAC is sufficient; ABAC is needed for complex multi-tenant or compliance-heavy systems.

---

### Q2. How do you model RBAC in a PostgreSQL database?

**Interview Answer**

Create four tables: users, roles, permissions, and junction tables user_roles and role_permissions. The schema: users(id, email, ...), roles(id, name, description), permissions(id, name, resource, action), user_roles(user_id, role_id), role_permissions(role_id, permission_id). Query a user's permissions with: SELECT p.name FROM permissions p JOIN role_permissions rp ON p.id = rp.permission_id JOIN user_roles ur ON rp.role_id = ur.role_id WHERE ur.user_id = $1. This normalized design allows flexible role-permission assignments without data duplication.

---

### Q3. How do you implement RBAC checking in Axum middleware?

**Interview Answer**

Create an Axum extractor or tower middleware that extracts the user's roles from the authenticated request context (set by authentication middleware). Define a RequirePermission middleware that takes the required permission as a parameter and checks if the user's roles include it. Apply it to route groups: Router::new().route("/users", get(list_users).layer(RequirePermission::new("user:read"))). The middleware checks the user's permissions against the route's requirement, returning 403 Forbidden if unauthorized. For complex checks, use a dedicated AuthorizationService that can query role hierarchies and resource ownership.

---

### Q4. What is the difference between RBAC1, RBAC2, and RBAC3?

**Interview Answer**

RBAC0 is the base model with users, roles, and permissions. RBAC1 adds role hierarchy — roles can inherit permissions from other roles (e.g., "super_admin" inherits all "admin" permissions). RBAC2 adds separation of duties constraints — a user cannot have conflicting roles simultaneously (e.g., cannot be both "approver" and "requester" for the same workflow). RBAC3 combines RBAC1 and RBAC2, providing both hierarchy and constraints. In Rust backends, RBAC0 is most common; RBAC1 is useful when you have many overlapping roles; RBAC2 is needed for compliance requirements.

---

### Q5. How do you handle role hierarchies in Rust?

**Interview Answer**

Model role hierarchies with a parent_role_id column in the roles table. When checking permissions, recursively resolve inherited roles using a CTE (Common Table Expression) in SQL. Example: WITH RECURSIVE role_hierarchy AS (SELECT role_id FROM user_roles WHERE user_id = $1 UNION SELECT rp.parent_role_id FROM role_permissions rp JOIN role_hierarchy rh ON rp.role_id = rh.role_id) SELECT DISTINCT p.name FROM permissions p JOIN role_permissions rp ON p.id = rp.permission_id JOIN role_hierarchy rh ON rp.role_id = rh.role_id. Cache the resolved permissions to avoid repeated recursive queries.

---

### Q6. What are common RBAC anti-patterns and how do you avoid them?

**Interview Answer**

Common anti-patterns include: role explosion (creating a new role for every minor variation — use permission inheritance instead), permission creep (granting excessive permissions — audit regularly), hardcoded roles (embedding role checks in handlers — use declarative middleware), and mixing authentication with authorization (checking permissions in auth middleware — keep them separate). Avoid these by: using fine-grained permissions with few roles, implementing automated permission audits, using Axum's layer system for declarative authorization, and maintaining clear separation between authn and authz.

---

### Q7. How do you handle multi-tenant RBAC in Rust?

**Interview Answer**

In multi-tenant RBAC, roles and permissions are scoped to tenants. Add a tenant_id to the roles and permissions tables, and filter all queries by tenant_id. A user might be "admin" in tenant A but "viewer" in tenant B. The JWT should include the tenant context, and all authorization checks must include the tenant scope. Example: SELECT p.name FROM permissions p JOIN role_permissions rp ON p.id = rp.permission_id JOIN user_roles ur ON rp.role_id = ur.role_id WHERE ur.user_id = $1 AND ur.tenant_id = $2. Use Row-Level Security in PostgreSQL for database-level tenant isolation.

---

### Q8. How do you migrate from a flat permission model to RBAC?

**Interview Answer**

Audit existing permissions by listing all actions and who can perform them. Group related permissions into logical roles (e.g., all user management permissions become the "user_manager" role). Create the role and permission tables, map existing users to new roles, and deploy the new authorization middleware alongside the old one. Use a feature flag to switch between old and new logic. Run both systems in parallel during migration, comparing authorization decisions for discrepancies. Once validated, remove the old authorization code. The key is incremental migration with validation at each step.

---

### Q9. How do you test RBAC in a Rust backend?

**Interview Answer**

Write integration tests that verify authorization for each route and role combination. Use test fixtures to create users with different roles, send requests to protected endpoints, and assert correct 200/403 responses. Test edge cases: user with no roles, user with expired tokens, user accessing another tenant's resources. Use cargo test with a test database and sqlx::migrate! for schema setup. For the authorization service layer, write unit tests that verify permission resolution, role hierarchy, and separation of duties constraints. Automate these tests in CI to catch authorization regressions.

---

### Q10. How do you audit RBAC in production for compliance?

**Interview Answer**

Maintain an audit log of every authorization decision with user ID, roles, requested resource, action, and outcome. Use structured logging with the tracing crate to capture this at the authorization middleware level. Export logs to a centralized system (ELK, Datadog) for analysis. Create dashboards showing: denied access attempts (potential attacks), users with excessive permissions, roles with unused permissions, and permission changes over time. Periodic access reviews verify that role assignments are still appropriate. For compliance frameworks like SOC 2, maintain evidence of authorization controls and regular audits.
