# Database Normalization

## Interview Question

What is database normalization and what are the normal forms?

## Interview Answer

Database normalization is the process of organizing a relational database to reduce data redundancy and improve data integrity. The goal is to ensure each piece of data is stored in one place, minimizing update, insertion, and deletion anomalies. The normal forms are: First Normal Form (1NF) with atomic values and no repeating groups; Second Normal Form (2NF) requiring full dependency on the primary key; Third Normal Form (3NF) eliminating transitive dependencies; and Boyce-Codd Normal Form (BCNF) where every determinant is a candidate key. Most production databases aim for 3NF or BCNF, then selectively denormalize for read performance.

---

## Follow-up Questions & Answers

### Q1. What is First Normal Form (1NF)?

**Interview Answer**

1NF requires every column to contain only atomic values with no repeating groups. The most common violation is comma-separated values in a single column. Fix it by creating a junction table for many-to-many relationships or using PostgreSQL arrays for simple cases.

```sql
-- Violation: tags = 'rust,postgresql'
CREATE TABLE posts_violation (id SERIAL PRIMARY KEY, tags TEXT);

-- Fix: junction table
CREATE TABLE posts (id SERIAL PRIMARY KEY, title VARCHAR(255));
CREATE TABLE tags (id SERIAL PRIMARY KEY, name VARCHAR(100) UNIQUE);
CREATE TABLE post_tags (
    post_id INT REFERENCES posts(id),
    tag_id INT REFERENCES tags(id),
    PRIMARY KEY (post_id, tag_id)
);

-- Alternative: PostgreSQL array
CREATE TABLE posts_array (id SERIAL PRIMARY KEY, tags TEXT[]);
SELECT * FROM posts_array WHERE 'rust' = ANY(tags);
```

---

### Q2. What is Second Normal Form (2NF)?

**Interview Answer**

2NF applies to tables with composite primary keys. Every non-key column must depend on the entire composite key, not just part of it. Partial dependencies mean data belongs in a separate table. With single-column primary keys (the common case), 2NF is automatically satisfied.

```sql
-- 2NF violation: student_name depends only on student_id
CREATE TABLE enrollments (
    student_id INT, course_id INT,
    student_name VARCHAR(100),  -- Partial dependency!
    grade CHAR(1),
    PRIMARY KEY (student_id, course_id)
);

-- Fix
CREATE TABLE students (id SERIAL PRIMARY KEY, name VARCHAR(100));
CREATE TABLE courses (id SERIAL PRIMARY KEY, name VARCHAR(100));
CREATE TABLE enrollments (
    student_id INT REFERENCES students(id),
    course_id INT REFERENCES courses(id),
    grade CHAR(1),
    PRIMARY KEY (student_id, course_id)
);
```

---

### Q3. What is Third Normal Form (3NF) and BCNF?

**Interview Answer**

3NF eliminates transitive dependencies where a non-key column depends on another non-key column. BCNF is stricter: every determinant must be a candidate key. If customer_email depends on customer_id (not the order's primary key), it belongs in a customers table. In practice, aim for BCNF.

```sql
-- 3NF violation: customer_email depends on customer_id, not order id
CREATE TABLE orders_bad (
    id SERIAL PRIMARY KEY,
    customer_id INT,
    customer_email VARCHAR(255),  -- Transitive dependency!
    total DECIMAL
);

-- Fix
CREATE TABLE customers (id SERIAL PRIMARY KEY, email VARCHAR(255));
CREATE TABLE orders_good (
    id SERIAL PRIMARY KEY,
    customer_id INT REFERENCES customers(id),
    total DECIMAL
);
```

---

### Q4. When should you denormalize?

**Interview Answer**

Denormalize when read performance is critical and JOINs are the bottleneck: analytics dashboards, high-traffic APIs, and reporting. Store pre-computed aggregates, duplicate frequently-read columns, or create materialized views. Always measure with EXPLAIN ANALYZE first.

```sql
-- Denormalized: pre-computed order count on customers
ALTER TABLE customers ADD COLUMN order_count INT DEFAULT 0;

-- Keep in sync with trigger
CREATE OR REPLACE FUNCTION update_order_count() RETURNS TRIGGER AS $$
BEGIN
    UPDATE customers SET order_count = order_count + 1 WHERE id = NEW.customer_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_order_count AFTER INSERT ON orders
    FOR EACH ROW EXECUTE FUNCTION update_order_count();
```

---

### Q5. How does SQLx handle normalized schemas in Rust?

**Interview Answer**

SQLx works with normalized schemas through JOIN queries and `query_as!`. Define Rust structs for write models (normalized) and separate read models (denormalized JOIN results). This maps to the CQRS pattern.

```rust
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct OrderWithCustomer {
    order_id: i64,
    total: rust_decimal::Decimal,
    customer_name: String,
}

async fn get_order(pool: &sqlx::PgPool, id: i64) -> Result<OrderWithCustomer, sqlx::Error> {
    sqlx::query_as!(OrderWithCustomer,
        "SELECT o.id AS order_id, o.total, c.name AS customer_name
         FROM orders o INNER JOIN customers c ON o.customer_id = c.id
         WHERE o.id = $1", id
    ).fetch_one(pool).await
}
```

---

### Q6. What is the difference between normalization and denormalization?

**Interview Answer**

Normalization splits data into related tables, reducing redundancy and improving write integrity. Denormalization adds redundancy for read performance. Most production systems use both: normalized write model and denormalized read model (CQRS).

```sql
-- Normalized: JOIN required
SELECT p.name, c.name AS category
FROM products p JOIN categories c ON p.category_id = c.id;

-- Denormalized: no JOIN needed
SELECT name, category_name FROM products;
```

---

### Q7. What are common normalization mistakes?

**Interview Answer**

Mistakes include: storing comma-separated values, missing foreign keys, storing computed values, using natural keys instead of surrogate keys, over-normalizing (too many JOINs), and not using CHECK constraints. The sweet spot is usually 3NF.

```sql
-- Mistake: computed value stored
CREATE TABLE line_items_bad (quantity INT, price DECIMAL, total DECIMAL);

-- Fix: generated column
CREATE TABLE line_items_good (
    quantity INT, price DECIMAL,
    total DECIMAL GENERATED ALWAYS AS (quantity * price) STORED
);
```

---

### Q8. How does PostgreSQL enforce normalization?

**Interview Answer**

PostgreSQL enforces normalization through PRIMARY KEY, FOREIGN KEY, UNIQUE, CHECK, NOT NULL, and EXCLUDE constraints at the database level, ensuring data integrity regardless of application.

```sql
CREATE TABLE customers (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL
);

CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    customer_id INT NOT NULL REFERENCES customers(id) ON DELETE RESTRICT,
    total DECIMAL NOT NULL CHECK (total >= 0),
    status VARCHAR(20) CHECK (status IN ('pending', 'shipped', 'delivered'))
);
```

---

### Q9. What is the difference between 3NF and BCNF?

**Interview Answer**

In 3NF, a non-key column can determine another non-key column if the determinant is part of a candidate key. BCNF removes this exception: every determinant must be a candidate key. BCNF is stricter but most well-designed schemas naturally satisfy it.

```sql
-- BCNF violation: instructor_id determines office
CREATE TABLE student_courses (
    student_id INT, course_id INT,
    instructor_id INT,
    instructor_office VARCHAR(50),  -- Determined by instructor_id
    PRIMARY KEY (student_id, course_id)
);

-- Fix: extract instructor data
CREATE TABLE instructors (id SERIAL PRIMARY KEY, office VARCHAR(50));
CREATE TABLE student_courses_fixed (
    student_id INT, course_id INT,
    instructor_id INT REFERENCES instructors(id),
    PRIMARY KEY (student_id, course_id)
);
```

---

### Q10. When is denormalization the wrong choice?

**Interview Answer**

Denormalization is wrong when: write volume is high (duplicated data must be updated everywhere), consistency is critical (duplicated data can diverge), storage is constrained, or the performance problem is actually an indexing issue. Always check EXPLAIN ANALYZE first — most slow queries are solved by adding an index, not denormalizing.

```sql
-- Check if index solves the problem before denormalizing
EXPLAIN ANALYZE SELECT * FROM orders WHERE customer_id = 42;
-- If Seq Scan: add index first
CREATE INDEX idx_orders_customer ON orders (customer_id);
-- Only denormalize if index + proper query still isn't fast enough
```
