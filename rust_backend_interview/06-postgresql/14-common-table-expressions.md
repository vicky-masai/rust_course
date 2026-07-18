# Common Table Expressions

## Interview Question

What are Common Table Expressions (CTEs) in PostgreSQL?

## Interview Answer

A CTE (Common Table Expression) is a named temporary result set defined within a SQL statement using the WITH clause. It improves readability by breaking complex queries into logical named blocks, similar to variables in programming. PostgreSQL supports both non-recursive CTEs (for organizing queries) and recursive CTEs (for traversing hierarchical data like trees and graphs). CTEs are materialized by default in PostgreSQL (result is computed once), but PostgreSQL 12+ inlines non-recursive CTEs when referenced only once (optimization fence removal). Use CTEs for readability, recursive traversal, and breaking complex logic into manageable pieces.

---

## Follow-up Questions & Answers

### Q1. What is the difference between a CTE and a subquery?

**Interview Answer**

Both produce temporary result sets, but CTEs are named, defined before the main query, and can be referenced multiple times. Subqueries are inline and anonymous. CTEs improve readability for complex queries and can be referenced multiple times without duplication. PostgreSQL 12+ may inline single-reference CTEs (removing the materialization overhead), but multi-reference CTEs are still materialized. For one-time use, the performance is identical. For multiple references, CTEs avoid re-computation.

```sql
-- Subquery (hard to read with complex logic)
SELECT * FROM (
    SELECT customer_id, SUM(total) AS spent
    FROM orders WHERE created_at > '2025-01-01'
    GROUP BY customer_id
) sub
WHERE sub.spent > 1000;

-- CTE (readable and reusable)
WITH customer_spending AS (
    SELECT customer_id, SUM(total) AS spent
    FROM orders WHERE created_at > '2025-01-01'
    GROUP BY customer_id
)
SELECT c.name, cs.spent
FROM customer_spending cs
JOIN customers c ON cs.customer_id = c.id
WHERE cs.spent > 1000;
```

---

### Q2. What is a recursive CTE and when would you use one?

**Interview Answer**

A recursive CTE references itself, enabling traversal of hierarchical (tree) or graph data. It has two parts: the base case (anchor member) that provides initial rows, and the recursive member that joins with the CTE result until no more rows are produced. Use cases: organizational hierarchies, threaded comments, file systems, bill of materials, and graph traversal. The recursion stops when the recursive member returns no rows or a CYCLE clause is hit (PostgreSQL 14+).

```sql
-- Organizational hierarchy
WITH RECURSIVE org_chart AS (
    -- Base case: top-level managers
    SELECT id, name, manager_id, 1 AS level
    FROM employees WHERE manager_id IS NULL

    UNION ALL

    -- Recursive case: find reports
    SELECT e.id, e.name, e.manager_id, oc.level + 1
    FROM employees e
    JOIN org_chart oc ON e.manager_id = oc.id
)
SELECT id, name, level, REPEAT('  ', level - 1) || name AS tree
FROM org_chart
ORDER BY level, name;

-- Threaded comments
WITH RECURSIVE comment_tree AS (
    SELECT id, content, parent_id, 0 AS depth
    FROM comments WHERE parent_id IS NULL

    UNION ALL

    SELECT c.id, c.content, c.parent_id, ct.depth + 1
    FROM comments c
    JOIN comment_tree ct ON c.parent_id = ct.id
)
SELECT depth, REPEAT('  ', depth) || content AS thread
FROM comment_tree;
```

---

### Q3. What is the MATERIALIZED option for CTEs?

**Interview Answer**

In PostgreSQL, CTEs are materialized by default (computed once and stored in memory). Starting with PostgreSQL 12, the optimizer inlines single-use non-recursive CTEs automatically. You can force materialization with MATERIALIZED or prevent it with NOT MATERIALIZED. MATERIALIZED is useful when a CTE is referenced multiple times and you want to avoid re-computation. NOT MATERIALIZED allows the optimizer to flatten the CTE into the main query, which may be faster for simple CTEs.

```sql
-- Force materialization (computed once, used twice)
WITH MATERIALIZED stats AS (
    SELECT customer_id, SUM(total) AS total_spent
    FROM orders GROUP BY customer_id
)
SELECT c.name, s.total_spent
FROM customers c JOIN stats s ON c.id = s.customer_id
WHERE s.total_spent > 1000
UNION ALL
SELECT 'Top spender', MAX(total_spent) FROM stats;

-- Force inlining (optimizer may be faster)
WITH NOT MATERIALIZED recent_orders AS (
    SELECT * FROM orders WHERE created_at > NOW() - INTERVAL '7 days'
)
SELECT * FROM recent_orders WHERE total > 100;
```

---

### Q4. How do CTEs interact with window functions?

**Interview Answer**

CTEs and window functions work together: compute window functions in a CTE, then filter on the results in the main query. This is cleaner than using window functions in subqueries. You cannot use window functions directly in a WHERE clause because they are evaluated after WHERE, but a CTE lets you compute them first and then filter.

```sql
-- Window function in CTE, filter in main query
WITH ranked_orders AS (
    SELECT id, customer_id, total,
           ROW_NUMBER() OVER (PARTITION BY customer_id ORDER BY created_at DESC) AS rn
    FROM orders
)
SELECT ro.id, ro.total, c.name
FROM ranked_orders ro
JOIN customers c ON ro.customer_id = c.id
WHERE ro.rn = 1;  -- Most recent order per customer

-- Running total
WITH running_totals AS (
    SELECT id, total, created_at,
           SUM(total) OVER (ORDER BY created_at) AS running_sum
    FROM orders
)
SELECT * FROM running_totals WHERE running_sum > 10000;
```

---

### Q5. How do you use CTEs with SQLx in Rust?

**Interview Answer**

CTEs are standard SQL and work seamlessly with SQLx. Write the CTE in the SQL string and use `query_as!` or `query!` to fetch results. SQLx's compile-time verification checks the CTE against your database schema. For recursive CTEs, the query returns all results at once (PostgreSQL doesn't support streaming recursive CTE results).

```rust
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct CategorySummary {
    category_name: String,
    product_count: i64,
    total_revenue: rust_decimal::Decimal,
}

async fn get_category_summary(pool: &sqlx::PgPool) -> Result<Vec<CategorySummary>> {
    sqlx::query_as!(CategorySummary,
        "WITH category_stats AS (
            SELECT c.name AS category_name,
                   COUNT(p.id) AS product_count,
                   COALESCE(SUM(oi.quantity * oi.price), 0) AS total_revenue
            FROM categories c
            LEFT JOIN products p ON c.id = p.category_id
            LEFT JOIN order_items oi ON p.id = oi.product_id
            GROUP BY c.name
        )
        SELECT category_name, product_count, total_revenue
        FROM category_stats
        WHERE total_revenue > 0
        ORDER BY total_revenue DESC"
    )
    .fetch_all(pool)
    .await
}
```

---

### Q6. What are the performance implications of CTEs?

**Interview Answer**

By default, PostgreSQL materializes CTEs, storing the result in memory. This is beneficial when the CTE is referenced multiple times (avoids re-computation) but wasteful when referenced once (adds overhead vs inlining). PostgreSQL 12+ automatically inlines single-use non-recursive CTEs. Multi-CTE chains can cause the optimizer to make poor choices because it treats each CTE as an opaque materialized result. For performance-critical queries, check EXPLAIN ANALYZE to verify the CTE is being optimized as expected. Use NOT MATERIALIZED to force inlining when the CTE is simple.

```sql
-- CTE may prevent join optimization
WITH active_users AS (
    SELECT * FROM users WHERE active = true
)
SELECT au.*, o.total
FROM active_users au
JOIN orders o ON au.id = o.user_id;
-- PostgreSQL may materialize active_users before joining with orders

-- NOT MATERIALIZED allows optimizer to push predicates
WITH NOT MATERIALIZED active_users AS (
    SELECT * FROM users WHERE active = true
)
SELECT au.*, o.total
FROM active_users au
JOIN orders o ON au.id = o.user_id
WHERE o.total > 100;
-- Now the WHERE o.total > 100 can be pushed down
```

---

### Q7. What are recursive CTE limitations in PostgreSQL?

**Interview Answer**

Recursive CTEs have several limitations: (1) Maximum recursion depth is controlled by `max_recursive_iterations` (default: unlimited, but use CYCLE clause to prevent infinite loops); (2) The recursive member cannot contain aggregation, window functions, or set operations (UNION ALL with the recursive term); (3) The result must be finite — infinite recursion consumes memory until OOM; (4) Performance degrades with depth because each level is a separate query execution. Use the CYCLE clause (PostgreSQL 14+) to detect cycles in graph traversal. For deep hierarchies, consider application-level iteration or ltree extension for path-based queries.

```sql
-- CYCLE clause to prevent infinite loops (PostgreSQL 14+)
WITH RECURSIVE graph_traversal AS (
    SELECT id, name, ARRAY[id] AS path
    FROM nodes WHERE id = 1

    UNION ALL

    SELECT n.id, n.name, gt.path || n.id
    FROM nodes n
    JOIN graph_traversal gt ON n.parent_id = gt.id
)
SELECT * FROM graph_traversal;

-- With cycle detection
WITH RECURSIVE graph_traversal AS (
    SELECT id, name, ARRAY[id] AS path
    FROM nodes WHERE id = 1

    UNION ALL

    SELECT n.id, n.name, gt.path || n.id
    FROM nodes n
    JOIN graph_traversal gt ON n.parent_id = gt.id
)
SELECT * FROM graph_traversal
-- PostgreSQL automatically stops when a cycle is detected
```

---

### Q8. Can CTEs be used for data modification (INSERT/UPDATE/DELETE)?

**Interview Answer**

Yes, PostgreSQL supports modifying data in CTEs using WITH ... RETURNING. The CTE performs the modification and returns the affected rows, which the main query can reference. This is useful for batch operations where you need to modify rows and use the modified data in a subsequent query. The CTE modification and main query execute in a single statement, ensuring atomicity.

```sql
-- Delete old orders and archive them
WITH deleted AS (
    DELETE FROM orders
    WHERE created_at < '2024-01-01'
    RETURNING *
)
INSERT INTO orders_archive
SELECT * FROM deleted;

-- Update and return results
WITH updated AS (
    UPDATE users SET last_login = NOW()
    WHERE id = ANY($1)
    RETURNING id, name, last_login
)
SELECT * FROM updated;

-- Conditional update with CTE
WITH to_update AS (
    SELECT id FROM products WHERE stock < 10
),
updated AS (
    UPDATE products SET stock = stock + 100
    WHERE id IN (SELECT id FROM to_update)
    RETURNING id, name, stock
)
SELECT * FROM updated;
```

---

### Q9. What is the ltree extension and how does it compare to recursive CTEs?

**Interview Answer**

The ltree extension provides a data type for representing labels in a tree structure (paths like 'A.B.C'). It supports efficient queries using ltree operators (@>, <@, ~, ?) with GiST/GIN indexes, which is much faster than recursive CTEs for tree traversal. Use ltree when you need fast ancestor/descendant queries on static or slowly-changing hierarchies. Recursive CTEs are better for dynamic traversal, cycle detection, and computed depth. ltree stores the full path, so updates to parent paths require updating all descendants.

```sql
-- Enable ltree extension
CREATE EXTENSION IF NOT EXISTS ltree;

CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100),
    path ltree
);

INSERT INTO categories (name, path) VALUES
('Electronics', 'Electronics'),
('Phones', 'Electronics.Phones'),
('iPhones', 'Electronics.Phones.iPhones');

CREATE INDEX idx_categories_path ON categories USING gist (path);

-- Find all descendants (fast with GiST index)
SELECT * FROM categories WHERE path <@ 'Electronics.Phones';

-- Find all ancestors
SELECT * FROM categories WHERE path @> 'Electronics.Phones.iPhones';
```

---

### Q10. What are window functions and how do they differ from GROUP BY?

**Interview Answer**

Window functions perform calculations across a set of rows related to the current row without collapsing them (unlike GROUP BY which reduces rows). They use OVER (PARTITION BY ... ORDER BY ...) to define the window frame. Common window functions: ROW_NUMBER(), RANK(), DENSE_RANK(), SUM() OVER, AVG() OVER, LAG(), LEAD(). Window functions are evaluated after WHERE and GROUP BY but can appear in SELECT and ORDER BY. Use them for running totals, ranking, moving averages, and gap-filling.

```sql
-- Ranking per group
SELECT name, department,
       RANK() OVER (PARTITION BY department ORDER BY salary DESC) AS dept_rank
FROM employees;

-- Running total
SELECT id, total, created_at,
       SUM(total) OVER (ORDER BY created_at) AS running_total
FROM orders;

-- Moving average
SELECT date, revenue,
       AVG(revenue) OVER (ORDER BY date ROWS BETWEEN 6 PRECEDING AND CURRENT ROW) AS moving_avg_7d
FROM daily_revenue;

-- LAG/LEAD for comparisons
SELECT month, revenue,
       revenue - LAG(revenue) OVER (ORDER BY month) AS change_from_previous
FROM monthly_revenue;
```
