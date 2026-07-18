# SQL JOIN Types

## Interview Question

What are the different types of SQL JOINs in PostgreSQL?

## Interview Answer

PostgreSQL supports several JOIN types that determine how rows from two or more tables are combined. INNER JOIN returns only rows that have matching values in both tables. LEFT (OUTER) JOIN returns all rows from the left table and matched rows from the right table, with NULLs for non-matching right rows. RIGHT (OUTER) JOIN is the mirror of LEFT JOIN. FULL (OUTER) JOIN returns all rows from both tables, with NULLs where there is no match. CROSS JOIN produces the Cartesian product of both tables (every combination of rows). LATERAL JOIN allows a subquery in the FROM clause to reference columns from preceding tables, enabling correlated subqueries in the FROM clause.

---

## Follow-up Questions & Answers

### Q1. What is the difference between INNER JOIN and WHERE clause filtering?

**Interview Answer**

An INNER JOIN filters rows at the join stage, combining rows from both tables where the join condition is true. A WHERE clause filters rows after the join has been computed. In practice, for simple equality conditions, the query planner often produces identical execution plans for both approaches. However, there is a semantic difference with OUTER JOINs: adding a WHERE condition after a LEFT JOIN can effectively convert it to an INNER JOIN. For example, `LEFT JOIN orders ON ... WHERE orders.id IS NOT NULL` filters out non-matching rows, defeating the purpose of the LEFT JOIN.

```sql
-- These are equivalent for INNER JOIN
SELECT u.name, o.total
FROM users u INNER JOIN orders o ON u.id = o.user_id;

SELECT u.name, o.total
FROM users u, orders o WHERE u.id = o.user_id;

-- WARNING: This converts LEFT JOIN to INNER JOIN
SELECT u.name, o.total
FROM users u LEFT JOIN orders o ON u.id = o.user_id
WHERE o.total > 100;  -- Filters out NULLs from non-matching rows
```

---

### Q2. What is a LATERAL JOIN and when would you use one?

**Interview Answer**

A LATERAL JOIN allows the right-hand side of a JOIN to reference columns from the left-hand side, making it a correlated subquery in the FROM clause. This is essential when you need to compute a dependent result set for each row of the outer query. A common use case is fetching the top-N items per group — for example, the 3 most recent orders per customer. Without LATERAL, you would need complex window functions or multiple queries. LATERAL works with all JOIN types (INNER, LEFT, CROSS).

```sql
-- Top 3 orders per customer using LATERAL
SELECT c.id, c.name, o.order_date, o.total
FROM customers c
LEFT JOIN LATERAL (
    SELECT order_date, total
    FROM orders o
    WHERE o.customer_id = c.id
    ORDER BY order_date DESC
    LIMIT 3
) o ON true;

-- LATERAL with generate_series for pivot-like results
SELECT d.month, COALESCE(s.amount, 0) AS revenue
FROM generate_series('2025-01-01'::date, '2025-12-01'::date, '1 month') AS d(month)
LEFT JOIN LATERAL (
    SELECT SUM(total) AS amount
    FROM orders o
    WHERE DATE_TRUNC('month', o.created_at) = d.month
) s ON true;
```

---

### Q3. What is the difference between LEFT JOIN and NOT EXISTS for finding non-matching rows?

**Interview Answer**

Both approaches find rows in one table that have no match in another, but they behave differently with duplicates. LEFT JOIN + WHERE right_table.id IS NULL returns one row per left table row even if there are duplicate matches. NOT EXISTS returns one row per left table row regardless of duplicates on the right side. If the right table has multiple matches, LEFT JOIN produces multiple rows, and the NULL check still works but may be less intuitive. NOT EXISTS is generally preferred for readability and is often more efficient because PostgreSQL can stop scanning as soon as it finds the first match. Both approaches leverage indexes on the join column.

```sql
-- Customers who have NEVER placed an order
-- Method 1: LEFT JOIN
SELECT c.id, c.name
FROM customers c
LEFT JOIN orders o ON c.id = o.customer_id
WHERE o.id IS NULL;

-- Method 2: NOT EXISTS (often preferred)
SELECT c.id, c.name
FROM customers c
WHERE NOT EXISTS (
    SELECT 1 FROM orders o WHERE o.customer_id = c.id
);

-- Method 3: NOT IN (be careful with NULLs)
SELECT c.id, c.name
FROM customers c
WHERE c.id NOT IN (SELECT customer_id FROM orders);
-- NOTE: NOT IN fails if subquery returns NULLs
```

---

### Q4. What is a CROSS JOIN and when is it useful?

**Interview Answer**

A CROSS JOIN produces the Cartesian product of two tables — every row from the first table is combined with every row from the second table. If table A has 100 rows and table B has 50 rows, the result has 5,000 rows. While often considered dangerous due to result set explosion, CROSS JOIN is genuinely useful for generating combinations like date × product reports, test data generation, or calendar tables. PostgreSQL supports both the explicit `CROSS JOIN` syntax and the implicit comma syntax (`FROM A, B`). Always ensure you have a WHERE clause or use it intentionally to avoid accidental full Cartesian products.

```sql
-- Generate a report of every product × every month
SELECT p.product_name, d.month
FROM products p
CROSS JOIN generate_series('2025-01-01'::date, '2025-12-01'::date, '1 month') AS d(month);

-- Generate all date × store combinations for a heatmap
SELECT s.store_name, d.day
FROM stores s
CROSS JOIN generate_series('2025-07-01'::date, '2025-07-31'::date, '1 day') AS d(day);

-- Test data generation
INSERT INTO test_cases (user_id, scenario)
SELECT u.id, s.scenario_name
FROM users u
CROSS JOIN (VALUES ('login'), ('checkout'), ('logout')) AS s(scenario_name);
```

---

### Q5. How does SQLx handle JOINs in Rust and what patterns should you follow?

**Interview Answer**

SQLx supports JOINs by allowing you to write raw SQL JOINs in your queries and deserialize the results into Rust structs. The recommended pattern is to use `sqlx::query_as!` with a custom struct that represents the joined result. For one-to-many relationships, you should use the `sqlx::query!` macro with manual aggregation or fetch related data in separate queries (the "select N+1 but batch it" pattern). SQLx does not have a built-in ORM-style eager loading system, so you design your queries to fetch exactly what you need. Use `#[sqlx(rename_all = "camelCase")]` to handle PostgreSQL snake_case naming conventions.

```rust
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct OrderWithCustomer {
    order_id: i64,
    total: rust_decimal::Decimal,
    customer_name: String,
}

// Single JOIN query
let orders = sqlx::query_as::<_, OrderWithCustomer>(
    "SELECT o.id AS order_id, o.total, c.name AS customer_name
     FROM orders o
     INNER JOIN customers c ON o.customer_id = c.id
     WHERE o.created_at > $1
     ORDER BY o.created_at DESC"
)
    .bind(start_date)
    .fetch_all(&pool)
    .await?;
```

---

### Q6. What is an anti-join and how is it implemented in PostgreSQL?

**Interview Answer**

An anti-join returns rows from one table that have no matching rows in another table. PostgreSQL does not have a dedicated ANTI JOIN keyword, but the optimizer implements it using several patterns. The most common are LEFT JOIN ... WHERE right.id IS NULL, NOT EXISTS, and NOT IN (with NULL-safety caveats). The query planner recognizes these patterns and can use a hash anti-join or a nested loop anti-join depending on table sizes and available indexes. Hash anti-join builds a hash table of the right table and probes it for each left row, which is efficient for large tables. Ensure the anti-join column is indexed for optimal performance.

```sql
-- Anti-join: users who never verified their email
-- Using NOT EXISTS (optimizer may convert to hash anti-join)
EXPLAIN ANALYZE
SELECT u.id, u.name
FROM users u
WHERE NOT EXISTS (
    SELECT 1 FROM email_verifications ev WHERE ev.user_id = u.id
);

-- Using LEFT JOIN ... IS NULL
EXPLAIN ANALYZE
SELECT u.id, u.name
FROM users u
LEFT JOIN email_verifications ev ON u.id = ev.user_id
WHERE ev.id IS NULL;

-- Using EXCEPT (set-based anti-join)
SELECT user_id FROM users EXCEPT SELECT user_id FROM banned_users;
```

---

### Q7. What are the performance implications of using FULL OUTER JOIN?

**Interview Answer**

FULL OUTER JOIN is the most expensive join type because PostgreSQL must match rows from both tables and produce NULL-filled rows for unmatched rows on both sides. It cannot use hash joins as efficiently as INNER or LEFT JOIN in some cases, and the result set can be significantly larger than either input table. FULL OUTER JOIN is rare in practice — most use cases can be rewritten as a UNION of two LEFT JOINs, which the optimizer may handle more efficiently. If you must use FULL OUTER JOIN, ensure both join columns are indexed and consider materializing large intermediate results with CTEs.

```sql
-- FULL OUTER JOIN: compare two data sources
SELECT COALESCE(a.id, b.id) AS record_id,
       a.source_a_value,
       b.source_b_value
FROM source_a a
FULL OUTER JOIN source_b b ON a.id = b.id
WHERE a.id IS NULL OR b.id IS NULL;  -- Find mismatches

-- Equivalent rewrite using UNION (often faster)
SELECT id, source_a_value, NULL AS source_b_value FROM source_a
WHERE id NOT IN (SELECT id FROM source_b)
UNION
SELECT id, NULL AS source_a_value, source_b_value FROM source_b
WHERE id NOT IN (SELECT id FROM source_a);
```

---

### Q8. What is a self-join and when would you use one?

**Interview Answer**

A self-join is a join where a table is joined with itself, using table aliases to distinguish the two instances. Self-joins are essential for hierarchical data like organizational charts (employee → manager), threaded comments (comment → parent_comment), and adjacency list trees. The join condition references the same table twice with different aliases. Performance depends on proper indexing of the foreign key column. For deep hierarchical queries (trees with many levels), consider using recursive CTEs instead of self-joins, as they can traverse arbitrary depth without multiple join operations.

```sql
-- Organizational hierarchy: find each employee's manager
SELECT e.name AS employee, m.name AS manager
FROM employees e
LEFT JOIN employees m ON e.manager_id = m.id;

-- Find employees who earn more than their manager
SELECT e.name, e.salary AS emp_salary, m.salary AS mgr_salary
FROM employees e
INNER JOIN employees m ON e.manager_id = m.id
WHERE e.salary > m.salary;

-- Threaded comments: find top-level comments and their replies
SELECT parent.content AS parent_comment, child.content AS reply
FROM comments parent
LEFT JOIN comments child ON child.parent_id = parent.id
WHERE parent.parent_id IS NULL;
```
