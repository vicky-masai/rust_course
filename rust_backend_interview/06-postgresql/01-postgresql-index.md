# PostgreSQL Index

## Interview Question

What is a PostgreSQL index and when should you use one?

## Interview Answer

A PostgreSQL index is a data structure that improves the speed of data retrieval operations on a table at the cost of additional storage and slower writes. Indexes work by creating a separate lookup structure — most commonly a B-tree — that allows the database engine to find rows without scanning every row in the table. PostgreSQL supports several index types including B-tree (default, good for equality and range), Hash (equality only), GIN (for composite types like JSONB, arrays, full-text search), and GiST (for geometric and geographic data). You should create indexes on columns that appear frequently in WHERE clauses, JOIN conditions, and ORDER BY clauses, but avoid over-indexing since each index slows down INSERT, UPDATE, and DELETE operations.

---

## Follow-up Questions & Answers

### Q1. What is the difference between a B-tree index and a Hash index in PostgreSQL?

**Interview Answer**

A B-tree index supports equality comparisons, range queries (`<`, `>`, `<=`, `>=`, `BETWEEN`), and `IS NULL` checks. It is the default index type and works well for most queries. A Hash index only supports equality comparisons (`=`) and is slightly faster for pure equality lookups, but it cannot handle range scans. Historically, Hash indexes were not WAL-logged in PostgreSQL versions before 10, making them unsafe for replication. Since PostgreSQL 10, Hash indexes are WAL-logged and crash-safe, but B-tree remains the preferred choice for general use because of its versatility.

```sql
-- B-tree (default)
CREATE INDEX idx_users_email ON users USING btree (email);

-- Hash (equality only)
CREATE INDEX idx_users_status ON users USING hash (status);
```

---

### Q2. What is a GIN index and when would you use it?

**Interview Answer**

GIN stands for Generalized Inverted Index. It is designed for data types that contain multiple values per row, such as arrays, JSONB columns, full-text search vectors, and hstore. A GIN index maps each element to the row(s) that contain it, making it ideal for queries that search for containment (`@>`), existence (`?`, `?|`, `?&`), or full-text search (`@@`). You would use a GIN index on a JSONB column when you frequently query for specific keys or values, or on a tsvector column for full-text search. GIN indexes can be slower to build but are extremely fast at read time.

```sql
-- JSONB GIN index
CREATE INDEX idx_metadata ON products USING gin (metadata);

-- Query that benefits
SELECT * FROM products WHERE metadata @> '{"color": "red"}';

-- Full-text search GIN index
CREATE INDEX idx_search ON articles USING gin (to_tsvector('english', content));
SELECT * FROM articles WHERE to_tsvector('english', content) @@ plainto_tsquery('english', 'postgresql tuning');
```

---

### Q3. What is a partial index and why is it useful?

**Interview Answer**

A partial index is an index built over a subset of a table's rows, defined by a WHERE clause in the CREATE INDEX statement. It is useful when only a fraction of rows match a condition, because the index is smaller, faster to scan, and cheaper to maintain. For example, if you frequently query for active orders but only 5% of orders are active, a partial index on `status = 'active'` is far more efficient than a full index on the status column. PostgreSQL will automatically use the partial index when a query's WHERE clause matches or is a subset of the index's WHERE clause.

```sql
-- Only indexes active orders (5% of total)
CREATE INDEX idx_active_orders ON orders (created_at) WHERE status = 'active';

-- PostgreSQL uses this index automatically
SELECT * FROM orders WHERE status = 'active' AND created_at > '2025-01-01';

-- Also useful for NULL filtering
CREATE INDEX idx_unverified_users ON users (email) WHERE email_verified_at IS NULL;
```

---

### Q4. What is an expression index and when would you use one?

**Interview Answer**

An expression index (also called a functional index) is an index on the result of a function or expression applied to one or more columns. It is useful when you frequently filter or sort by a computed value rather than the raw column. For example, if you often query users by lowercase email, an expression index on `LOWER(email)` avoids repeated function calls during query execution. Without the expression index, PostgreSQL cannot use a standard index on the email column for case-insensitive lookups.

```sql
-- Expression index on lowercase email
CREATE INDEX idx_users_lower_email ON users (LOWER(email));

-- Query that benefits
SELECT * FROM users WHERE LOWER(email) = 'alice@example.com';

-- Expression index on date truncation
CREATE INDEX idx_orders_month ON orders (DATE_TRUNC('month', created_at));
SELECT * FROM orders WHERE DATE_TRUNC('month', created_at) = '2025-07-01';
```

---

### Q5. What is a covering index (index-only scan) and how does it work?

**Interview Answer**

A covering index includes all the columns a query needs, so PostgreSQL can satisfy the query entirely from the index without ever touching the table heap. This is called an index-only scan and is significantly faster because it avoids random I/O to the main table. In PostgreSQL, you can use the `INCLUDE` clause to add payload columns to a B-tree index that are not part of the search key but are stored in the leaf pages. The visibility map must also show that all tuples on a page are visible to all transactions for an index-only scan to work.

```sql
-- Covering index with INCLUDE
CREATE INDEX idx_orders_covering ON orders (customer_id) INCLUDE (total, status);

-- This query is answered entirely from the index
SELECT total, status FROM orders WHERE customer_id = 42;

-- Check if index-only scan is used
EXPLAIN ANALYZE SELECT total, status FROM orders WHERE customer_id = 42;
-- Output: Index Only Scan using idx_orders_covering on orders
```

---

### Q6. What is the cost of maintaining indexes and how do you decide which indexes to create?

**Interview Answer**

Every index must be updated whenever its underlying table is modified (INSERT, UPDATE, DELETE), which adds write amplification. Each index increases storage, slows down writes, and must be vacuumed. To decide which indexes to create, analyze slow queries using `EXPLAIN ANALYZE` and look for sequential scans on large tables. Focus on columns used in WHERE, JOIN ON, and ORDER BY clauses. Check `pg_stat_user_indexes` to find unused indexes (`idx_scan = 0`) and consider dropping them. A good rule of thumb is to index columns that appear in high-selectivity queries — if a query returns fewer than 5-10% of rows, an index helps.

```sql
-- Find unused indexes
SELECT schemaname, relname, idx_scan, idx_tup_read
FROM pg_stat_user_indexes
WHERE idx_scan = 0 AND schemaname = 'public'
ORDER BY pg_relation_size(relid) DESC;

-- Find missing indexes on foreign keys
SELECT c.relname AS table_name, a.attname AS column_name
FROM pg_constraint con
JOIN pg_class c ON con.conrelid = c.oid
JOIN pg_attribute a ON a.attrelid = c.oid AND a.attnum = ANY(con.conkey)
WHERE con.contype = 'f'
  AND NOT EXISTS (
    SELECT 1 FROM pg_index i
    WHERE i.indrelid = con.conrelid AND a.attnum = ANY(i.indkey)
  );
```

---

### Q7. How does SQLx work with PostgreSQL indexes in Rust?

**Interview Answer**

SQLx does not manage indexes directly — indexes are a database-level concern, not an application-level one. However, SQLx provides `sqlx::query!` and `sqlx::query_as!` macros that compile-time verify your queries against the database schema, which includes indexes. You create and manage indexes through raw SQL migrations that SQLx can execute via `sqlx::migrate!()`. When designing your data access layer, you should ensure that your queries are index-friendly by using parameterized queries that match your index key patterns. SQLx's connection pool also benefits from efficient indexing because faster queries free up connections sooner.

```rust
// Migration to create an index
// migrations/20250701000000_add_user_email_index.sql
// CREATE INDEX idx_users_email ON users (email);

// Rust code — query benefits from the index
let user = sqlx::query_as!(
    User,
    "SELECT id, name, email FROM users WHERE email = $1",
    email
)
.fetch_optional(&pool)
.await?;
```

---

### Q8. What is the difference between a unique index and a primary key constraint in PostgreSQL?

**Interview Answer**

A primary key constraint creates a unique, NOT NULL index on the constrained columns and enforces entity integrity — no duplicate values and no NULLs. A unique index enforces uniqueness but allows a single NULL value per column (multiple NULLs are allowed for composite unique indexes since NULL != NULL in SQL). In PostgreSQL, a PRIMARY KEY constraint automatically creates a unique B-tree index named after the constraint. You can have only one primary key per table, but multiple unique indexes. Both are useful for different purposes: primary keys for row identity, unique indexes for business rules like "each user has a unique email."

```sql
-- Primary key (implicit unique index)
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    CONSTRAINT users_email_unique UNIQUE (email)
);

-- The UNIQUE constraint creates an index: users_email_unique
-- This is functionally equivalent to:
CREATE UNIQUE INDEX idx_users_email ON users (email);
```

---

### Q9. How do you monitor index usage and performance in PostgreSQL?

**Interview Answer**

PostgreSQL provides several system views for monitoring index usage. `pg_stat_user_indexes` shows per-index statistics including scan count, rows read, and size. `pg_stat_user_tables` shows whether sequential scans are happening more often than index scans for a given table. You can use `EXPLAIN ANALYZE` to see whether a query uses an index and how many rows are processed. The `pg_indexes` view provides index definitions. Regular monitoring helps identify unused indexes (candidates for removal), missing indexes (candidates for creation), and bloated indexes that need REINDEX or VACUUM.

```sql
-- Index usage statistics
SELECT indexrelname, idx_scan, idx_tup_read, idx_tup_fetch,
       pg_size_pretty(pg_relation_size(indexrelid)) AS index_size
FROM pg_stat_user_indexes
WHERE schemaname = 'public'
ORDER BY idx_scan ASC;

-- Table scan ratio (high seq_scan = needs index)
SELECT relname, seq_scan, idx_scan,
       CASE WHEN (seq_scan + idx_scan) > 0
            THEN round(100.0 * idx_scan / (seq_scan + idx_scan), 2)
            ELSE 0 END AS index_usage_pct
FROM pg_stat_user_tables
WHERE schemaname = 'public'
ORDER BY seq_scan DESC;
```

---

### Q10. What is an GiST index and what are its use cases?

**Interview Answer**

GiST stands for Generalized Search Tree. It is an index infrastructure that supports many different data types and query operators, including geometric/spatial data (boxes, circles, polygons), full-text search, range types, and network address types. Unlike B-tree, GiST is lossy — it stores summary information about the indexed data rather than exact values, so PostgreSQL may need to recheck conditions after fetching a candidate row. GiST is essential for PostGIS spatial queries, range type containment and overlap queries, and nearest-neighbor searches. For JSONB containment queries, GIN is preferred over GiST, but GiST is better for range overlap queries.

```sql
-- Spatial index using GiST
CREATE INDEX idx_locations ON places USING gist (location);

-- Range type GiST index
CREATE INDEX idx_event_dates ON events USING gist (during);
SELECT * FROM events WHERE during && '[2025-07-01, 2025-07-31)'::tsrange;

-- Nearest-neighbor search with GiST
SELECT * FROM places
ORDER BY location <-> ST_Point(-73.9857, 40.7484)
LIMIT 5;
```
