# Testing Database Queries in Rust

## Interview Question

How do you test database queries in Rust, and what strategies ensure query correctness and performance?

## Interview Answer

Use `sqlx` with its `#[sqlx::test]` macro that automatically manages test database connections and transactions. The macro creates a fresh database per test, runs migrations, and rolls back after the test completes. Write tests that insert data, run queries, and assert on results. For performance, use `EXPLAIN` to verify query plans and test with realistic data volumes. In-memory SQLite works for simple cases but PostgreSQL-specific features require a real database. Use testcontainers for consistent CI environments.

---

## Follow-up Questions & Answers

### Q1. How does `#[sqlx::test]` work?

**Interview Answer**

`#[sqlx::test]` creates a dedicated test database, runs your migrations, executes the test function, and drops the database afterward. Each test gets its own isolated database, preventing cross-test contamination. The test function receives a `Pool<Postgres>` (or other database type) parameter. Use `migrate!()` to run migrations within the test. This approach ensures tests are deterministic and don't leave behind state.

---

### Q2. How do you test database migrations in Rust?

**Interview Answer**

Run migrations up and down to verify they're reversible. Use `sqlx::migrate!()` in tests to ensure migrations execute without errors. Test that migrations don't lose data by inserting data, running a migration, and verifying the data is still accessible. Use a separate test database for migration tests. For complex migrations, test both the up and down paths. Include data migration tests that verify transformations are correct.

---

### Q3. How do you test query performance in Rust?

**Interview Answer**

Use `EXPLAIN ANALYZE` through sqlx to verify query execution plans. Test queries with realistic data volumes — a query that's fast with 10 rows may be slow with 1 million. Use `criterion` benchmarks to measure query latency. Test index effectiveness by running queries with and without specific indexes. Monitor query plans after schema changes to catch performance regressions. Use connection pool metrics to detect connection exhaustion.

---

### Q4. How do you handle database fixtures in tests?

**Interview Answer**

Create fixture functions that insert known data into the test database. Use factory patterns with builder APIs to construct test data: `UserFactory::new().with_email("test@example.com").create(&pool)`. Keep fixtures minimal — only insert data needed for the specific test. Use UUIDs for test data to avoid collisions. Clean up fixtures explicitly or rely on transaction rollback. For complex scenarios, use SQL scripts to seed test data.

---

### Q5. How do you test transactions and rollback behavior?

**Interview Answer**

Wrap test operations in database transactions to test rollback scenarios. Verify that failed operations don't leave partial data. Test nested transactions if your database supports savepoints. Use `sqlx::Transaction` to manually control commit/rollback. Test that connection pool timeouts and errors are handled gracefully. Verify transaction isolation levels by running concurrent operations and checking for race conditions.

---

### Q6. What is the difference between testing with SQLite vs. PostgreSQL in Rust?

**Interview Answer**

SQLite is in-memory and fast but doesn't support PostgreSQL-specific features like JSONB, arrays, full-text search, or LISTEN/NOTIFY. Use SQLite for simple CRUD tests where portability matters. Use PostgreSQL for tests that rely on Postgres-specific features. Testcontainers can spin up PostgreSQL containers for CI. sqlx supports both backends, but you may need conditional compilation for database-specific test code.

---

### Q7. How do you test database connection pooling?

**Interview Answer**

Configure the pool with small `max_connections` in tests to verify behavior under pool exhaustion. Use concurrent test tasks that all need connections to test pool queuing. Verify that connections are properly returned to the pool after use. Test pool timeout behavior by setting short acquire timeouts. Monitor pool metrics (active connections, idle connections, wait time) during tests. Use `sqlx::Pool::connect` with test-specific configuration.

---

### Q8. How do you test code that uses raw SQL queries?

**Interview Answer**

Use `sqlx::query!` and `sqlx::query_as!` which provide compile-time query verification against the database schema. For raw SQL strings with `sqlx::query()`, test that the query executes and returns expected results. Verify parameter binding by checking that the correct values are substituted. Test edge cases like SQL injection attempts with special characters in inputs. Always use parameterized queries and test that they're used correctly.

---

### Q9. How do you test database schema changes without breaking existing tests?

**Interview Answer**

Run all migrations before tests to ensure the schema is current. Use additive migrations (add columns, create tables) that don't break existing queries. Test that new columns have defaults or are nullable. Verify that existing queries still work after schema changes. Use feature flags to conditionally run tests for new schema features. Keep migration tests separate from functional tests to isolate schema concerns.

---

### Q10. How do you test database-intensive operations at scale?

**Interview Answer**

Use large dataset generators to create realistic test data (millions of rows). Test batch operations with realistic volumes to identify N+1 query problems. Measure query performance under concurrent load using `tokio::spawn` with multiple test tasks. Use `EXPLAIN ANALYZE` to verify index usage. Test connection pool behavior under sustained load. For very large datasets, use a dedicated test database rather than in-memory SQLite. Profile query performance with `sqlx` logging enabled.
