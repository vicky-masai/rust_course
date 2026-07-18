# SQL Injection Prevention

## Interview Question

How do you prevent SQL injection in a Rust backend using SQLx?

## Interview Answer

SQL injection is prevented by using parameterized queries that separate SQL logic from data. SQLx enforces this at compile time — query! and query_as! macros validate SQL against the database schema and refuse to compile if raw string interpolation is used. Always use $1, $2 placeholders instead of format!() or string concatenation. SQLx also provides query_scalar!, query_file!, and execute! macros that all enforce parameterization. The key rule is: never construct SQL strings from user input. SQLx's compile-time verification is a powerful safety net that catches injection vulnerabilities before deployment.

---

## Follow-up Questions & Answers

### Q1. How does SQLx prevent SQL injection at compile time?

**Interview Answer**

SQLx's query! macro connects to a test database at compile time, parses the SQL, and verifies that the query is valid and the parameter types match the database schema. If you try to interpolate user input directly into the SQL string, the macro fails to compile because it cannot find a matching prepared statement. This forces you to use parameterized queries: sqlx::query!("SELECT * FROM users WHERE email = $1", email). The compile-time check catches injection vulnerabilities that would otherwise only be found during testing or in production.

---

### Q2. What is the difference between parameterized queries and stored procedures for injection prevention?

**Interview Answer**

Parameterized queries prevent injection by sending SQL structure and data separately to the database — the database parses the SQL first, then binds parameters. Stored procedures can also prevent injection if they use parameterized inputs, but they introduce complexity, versioning challenges, and are harder to test and deploy. Parameterized queries are the preferred approach because they are simple, testable, and work with SQLx's compile-time verification. Stored procedures may be used for complex business logic but should not be relied upon as the primary injection defense.

---

### Q3. How does SQL injection work and what damage can it cause?

**Interview Answer**

SQL injection occurs when user input is concatenated into SQL queries, allowing attackers to modify the query structure. An attacker could: extract sensitive data (passwords, personal information), modify or delete data (DROP TABLE), escalate privileges (admin access), execute system commands (via xp_cmdshell), or establish persistent backdoors. For example, inputting `' OR '1'='1` in a login form without parameterization changes the query to match all users. SQLx prevents this entirely by treating user input as data, never as SQL code.

---

### Q4. What are ORM-based injection risks and does SQLx have them?

**Interview Answer**

ORMs can introduce injection if they support raw query construction. SQLx is not an ORM — it is a query builder that enforces parameterization through its macro system. However, sqlx::raw_sql() and format!() bypass this safety. If you must use dynamic queries, use sqlx::QueryBuilder which properly parameterizes dynamic parts. The risk with any database library is when developers bypass the safe API for convenience. The rule is: never use string interpolation in SQL queries, and review code for raw_sql usage during code review.

---

### Q5. How do you handle dynamic SQL queries safely in Rust?

**Interview Answer**

For dynamic queries with variable conditions, use sqlx::QueryBuilder which accumulates query parts and parameters safely. Example: let mut builder = QueryBuilder::new("SELECT * FROM users WHERE 1=1"); if let Some(name) = &name { builder.push(" AND name ILIKE ").push_bind(format!("%{}%", name)); } builder.build_query_as::<User>().fetch_all(&pool).await?; This ensures dynamic parts are properly parameterized. For complex dynamic queries, consider using a dedicated query builder library like sea-query, which generates parameterized SQL safely.

---

### Q6. What is second-order SQL injection and how does it differ from first-order?

**Interview Answer**

First-order injection occurs when user input is used in a query immediately. Second-order injection occurs when malicious input is stored in the database and later used in a query without re-parameterization. For example, a user registers with a username containing SQL injection code. The registration query uses parameters (safe), but an admin panel displays the username using raw SQL (unsafe). SQLx's compile-time checks protect against both types because the query! macro enforces parameterization at every query site, regardless of whether the data is from user input or the database.

---

### Q7. How do you audit your codebase for SQL injection vulnerabilities?

**Interview Answer**

Search for dangerous patterns: format!() used in SQL strings, string concatenation with SQL, raw_sql() usage, and any place where user input flows into SQL. Use cargo audit for dependency vulnerabilities. In CI, add a grep check for format!("SELECT/INSERT/UPDATE/DELETE patterns. Code review should specifically check every new query for proper parameterization. SQLx's compile-time checking catches most issues, but manual review is still needed for raw_sql usage. Additionally, use static analysis tools like clippy with custom lints to flag string interpolation in SQL contexts.

---

### Q8. What is the impact of SQL injection on a production Rust backend?

**Interview Answer**

SQL injection in production can lead to: complete data breach (all user data exposed), data destruction (tables dropped or corrupted), privilege escalation (attacker gains admin access), regulatory violations (GDPR, HIPAA fines), reputational damage, and legal liability. The OWASP Top 10 consistently ranks injection as a critical risk. For a Rust backend using SQLx with parameterized queries, SQL injection is prevented at compile time. The risk is near-zero if you exclusively use query! macros and avoid raw SQL construction.

---

### Q9. How do you handle SQL injection in legacy codebases during migration?

**Interview Answer**

Audit all SQL queries for injection risks: grep for format!, string interpolation in SQL, and raw query construction. Replace each with parameterized queries using SQLx's query! macro. For queries that cannot be parameterized (dynamic column names, table names), use whitelists and validate against known valid values — never use user input for identifiers. Test each replacement thoroughly. Run the migrated queries against a test database with SQLx's compile-time checking. Deploy incrementally with feature flags to catch regressions quickly.

---

### Q10. What are prepared statements and how do they prevent injection?

**Interview Answer**

Prepared statements separate SQL structure from data. The SQL is sent to the database first, which parses and compiles it into an execution plan. Parameters are sent separately and bound to placeholders ($1, $2). The database treats parameters strictly as data, never as SQL code, regardless of their content. SQLx uses PostgreSQL's extended query protocol, which implements prepared statements. This is why parameterized queries prevent injection — even if a parameter contains SQL keywords or special characters, they are never executed as code. The database engine handles all escaping and type checking.
