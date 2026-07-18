# Why use sqlx instead of Diesel?

## Interview Question

Why use sqlx instead of Diesel?

## Interview Answer

"sqlx supports compile-time SQL validation, async execution, raw SQL flexibility, and integrates well with Axum."

---

## Follow-up Questions & Answers

### Q1. What does compile-time SQL validation mean in sqlx?

**Interview Answer**

sqlx checks your SQL queries against a real or mocked database at compile time using macros like `query!` and `query_as!`. If the query has syntax errors, missing columns, or type mismatches, the build fails immediately. This catches bugs early without running the application, which is a significant productivity gain over runtime-only validation.

---

### Q2. Can you use Diesel with async Rust?

**Interview Answer**

Diesel is primarily synchronous and uses blocking I/O by default. There are community forks and adapters like `diesel-async` that add async support, but they are less mature than sqlx's native async implementation. sqlx was designed from the ground up for async, making it a more natural fit for Axum and Tokio-based applications.

---

### Q3. What are the tradeoffs of using raw SQL with sqlx versus an ORM like Diesel?

**Interview Answer**

Raw SQL gives you full control over queries, makes performance tuning easier, and avoids ORM abstractions that can generate inefficient queries. Diesel's ORM provides type-safe schema definitions and migration tools but adds complexity and can obscure what SQL runs. For backend APIs with performance-critical queries, sqlx's raw SQL approach is often preferred.

---

### Q4. How does sqlx handle database migrations?

**Interview Answer**

sqlx provides `sqlx migrate` CLI for creating and running migrations, stored in a `migrations/` directory. Migrations are versioned and can be applied at startup with `sqlx::migrate!().run(&pool)`. Diesel has a more mature migration system, but sqlx's approach is simpler and sufficient for most projects.

---

### Q5. When would you choose Diesel over sqlx?

**Interview Answer**

Diesel is better when you need a full ORM with schema-first design, compile-time checked model definitions, and complex relationship mappings. It's suitable for applications with intricate domain models where type safety across the entire data layer is critical. sqlx is preferred for APIs that prioritize raw SQL control, minimal abstractions, and async-first design.

---

### Q6. How does sqlx handle connection pooling compared to Diesel?

**Interview Answer**

sqlx has a built-in async-native connection pool via `PgPool` that integrates seamlessly with Tokio. Diesel requires r2d2 or another pool adapter, which adds configuration complexity. sqlx's pool is simpler to set up and tune for async Axum applications.

---

### Q7. What is the `query!` macro doing differently from `query()`?

**Interview Answer**

`query!` validates SQL at compile time against a database connection specified by `DATABASE_URL`, ensuring column names and types match. `query()` does no compile-time checking and is used when validation isn't possible, like with dynamic queries. Use `query!` whenever possible for safety, and fall back to `query()` only for dynamic SQL.

---

### Q8. How do you structure a large sqlx project for maintainability?

**Interview Answer**

Organize code into modules for queries, models, repository layer, and migrations. Use `sqlx::query_as!` with dedicated model structs to keep query results type-safe. Separate database logic from Axum handlers using a service or repository pattern, and keep migrations version-controlled in the `migrations/` directory.
