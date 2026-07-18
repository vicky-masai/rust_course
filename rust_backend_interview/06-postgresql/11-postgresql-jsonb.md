# PostgreSQL JSONB

## Interview Question

What is JSONB in PostgreSQL and when should you use it?

## Interview Answer

JSONB is PostgreSQL's binary JSON data type that stores JSON documents in a decomposed binary format, enabling efficient indexing and querying. Unlike the text-based JSON type, JSONB supports GIN indexes for fast containment and existence checks, and provides operators for deep querying (`->`, `->>`, `@>`, `?`, `#>`). JSONB is ideal for semi-structured data like API responses, user preferences, feature flags, and event payloads where the schema varies or evolves. Use JSONB when you need to store flexible key-value data without altering the table schema, but prefer relational columns for data you frequently query, filter, or join on. JSONB is not a replacement for proper normalization — it's a tool for schema flexibility.

---

## Follow-up Questions & Answers

### Q1. What is the difference between JSON and JSONB in PostgreSQL?

**Interview Answer**

The JSON type stores raw text input and validates it's valid JSON, but doesn't parse it — every access requires re-parsing. JSONB stores a decomposed binary representation that is slightly slower to insert (parsing overhead) but much faster to query and index. JSONB preserves the semantic meaning of JSON values (e.g., duplicate keys are removed, numeric values are stored efficiently). JSONB supports GIN indexes and containment operators (`@>`), while JSON does not. In practice, always use JSONB unless you need to preserve exact whitespace or duplicate key ordering. JSONB columns can be indexed, queried efficiently, and even have CHECK constraints on extracted values.

```sql
-- JSON type (text-based, no indexing)
CREATE TABLE events_json (id SERIAL, data JSON);
-- Cannot create GIN index, slow queries

-- JSONB type (binary, indexable)
CREATE TABLE events_jsonb (id SERIAL, data JSONB);
CREATE INDEX idx_events_data ON events_jsonb USING gin (data);

-- JSONB supports efficient queries
SELECT * FROM events_jsonb WHERE data @> '{"type": "click"}';
SELECT * FROM events_jsonb WHERE data->>'user_id' = '42';
SELECT * FROM events_jsonb WHERE data ? 'timestamp';

-- JSON does not support GIN indexes
-- This fails: CREATE INDEX ON events_json USING gin (data);
```

---

### Q2. How do you query JSONB data using operators?

**Interview Answer**

PostgreSQL provides several JSONB operators: `->` returns a JSONB value by key/index, `->>` returns text by key/index, `#>` returns JSONB by path, `#>>` returns text by path, `@>` checks containment (left contains right), `<@` checks containment (right contains left), `?` checks key existence, `?|` checks any key existence, `?&` checks all keys existence, `||` concatenates two JSONB values, `-` removes a key, `#-` removes by path. Use `->>` when comparing with text values, `->` when chaining JSONB operations, and `@>` for GIN-indexed containment queries. The choice of operator determines whether an index can be used.

```sql
-- Create test data
INSERT INTO events (data) VALUES
('{"type": "click", "user_id": 42, "page": "/home", "metadata": {"browser": "chrome"}}');

-- Extract by key (returns JSONB)
SELECT data->'type' FROM events;    -- "click"
SELECT data->'type' FROM events;    -- "click"

-- Extract by key (returns text)
SELECT data->>'type' FROM events;   -- click
SELECT data->>'user_id' FROM events; -- 42

-- Nested access
SELECT data#>>'{metadata,browser}' FROM events;  -- chrome

-- Containment check (GIN-indexable)
SELECT * FROM events WHERE data @> '{"type": "click"}';

-- Key existence (GIN-indexable)
SELECT * FROM events WHERE data ? 'user_id';

-- Multiple key existence
SELECT * FROM events WHERE data ?& ARRAY['type', 'user_id'];

-- Path-based query
SELECT * FROM events WHERE data @> '{"metadata": {"browser": "chrome"}}';
```

---

### Q3. How do you create GIN indexes on JSONB columns for different query patterns?

**Interview Answer**

PostgreSQL supports two GIN operator classes for JSONB: `jsonb_ops` (default) — supports `@>`, `?`, `?|`, `?&` operators, indexes every key and value; and `jsonb_path_ops` — supports only `@>` containment, but produces smaller indexes and is faster for containment-only workloads. Choose `jsonb_ops` if you use existence checks (`?` operators), choose `jsonb_path_ops` if you only use containment (`@>`). For queries on specific keys, a B-tree expression index on `data->>'key'` is often faster than GIN because it's more selective. Composite GIN indexes aren't supported, but you can create multiple expression indexes for frequently queried keys.

```sql
-- Default GIN index (supports @>, ?, ?|, ?&)
CREATE INDEX idx_events_gin ON events USING gin (data);
-- Supports: data @> '{"type": "click"}'
-- Supports: data ? 'user_id'

-- Path-only GIN (smaller, faster for @>)
CREATE INDEX idx_events_gin_path ON events USING gin (data jsonb_path_ops);
-- Supports: data @> '{"type": "click"}'
-- Does NOT support: data ? 'user_id'

-- B-tree expression index (best for specific key access)
CREATE INDEX idx_events_type ON events ((data->>'type'));
-- Supports: WHERE data->>'type' = 'click'

-- B-tree for numeric extraction
CREATE INDEX idx_events_user_id ON events ((data->>'user_id')::int);
-- Supports: WHERE (data->>'user_id')::int = 42

-- Partial GIN index (most efficient for specific patterns)
CREATE INDEX idx_events_clicks ON events USING gin (data)
    WHERE data->>'type' = 'click';
-- Only indexes click events, much smaller
```

---

### Q4. How do you update specific keys in a JSONB column?

**Interview Answer**

PostgreSQL provides the `jsonb_set()` function to update specific keys within a JSONB column, preserving other keys. Use the `-` operator to remove keys, `||` to merge JSONB objects, and `#-` to remove by path. For appending to arrays, use `jsonb_array_append()` or `jsonb_array_elements()`. When updating frequently accessed keys, consider whether those keys should be promoted to regular columns for better performance. JSONB updates create new row versions (MVCC), so frequent updates to the same JSONB column cause table bloat — monitor with pgstattuple.

```sql
-- Set a specific key
UPDATE events
SET data = jsonb_set(data, '{type}', '"pageview"')
WHERE id = 1;

-- Set nested key
UPDATE events
SET data = jsonb_set(data, '{metadata, browser}', '"firefox"')
WHERE id = 1;

-- Remove a key
UPDATE events
SET data = data - 'metadata'
WHERE id = 1;

-- Merge two JSONB objects
UPDATE events
SET data = data || '{"timestamp": "2025-07-15T10:00:00Z"}'
WHERE id = 1;

-- Add to array
UPDATE events
SET data = jsonb_set(data, '{tags}', data->'tags' || '"new_tag"')
WHERE id = 1;

-- Remove by path
UPDATE events
SET data = data #- '{metadata,browser}'
WHERE id = 1;
```

---

### Q5. How do you use JSONB with SQLx in Rust?

**Interview Answer**

SQLx supports JSONB through the `sqlx::types::Json` wrapper type, which serializes/deserializes Rust structs to/from JSONB using serde. Define your Rust struct with `#[derive(Serialize, Deserialize)]`, wrap it in `sqlx::types::Json<T>`, and use it in queries with `sqlx::types::Json<T>` as the bind type. SQLx will automatically serialize to JSONB for inserts and deserialize from JSONB for selects. For raw JSONB queries, use `sqlx::query_scalar!` with `data->>'key'` syntax. The `Json<T>` type handles all conversion transparently.

```rust
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

#[derive(Debug, Serialize, Deserialize)]
struct EventMetadata {
    browser: String,
    os: String,
    screen_width: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    id: i64,
    event_type: String,
    metadata: EventMetadata,
}

// Insert with JSONB
async fn create_event(pool: &PgPool, event: &Event) -> Result<()> {
    sqlx::query!(
        "INSERT INTO events (event_type, data) VALUES ($1, $2)",
        event.event_type,
        Json(&event.metadata) as Json<EventMetadata>
    )
    .execute(pool)
    .await?;
    Ok(())
}

// Query JSONB fields
async fn get_events_by_browser(pool: &PgPool, browser: &str) -> Result<Vec<(i64, String)>> {
    sqlx::query_as!(
        (i64, String),
        "SELECT id, data->>'event_type' AS event_type
         FROM events
         WHERE data @> $1",
        serde_json::json!({"metadata": {"browser": browser}})
    )
    .fetch_all(pool)
    .await
}

// Deserialize JSONB column into Rust struct
async fn get_event(pool: &PgPool, id: i64) -> Result<Option<Event>> {
    let row = sqlx::query!(
        "SELECT id, event_type, data FROM events WHERE id = $1", id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Event {
        id: r.id,
        event_type: r.event_type,
        metadata: serde_json::from_value(r.data).unwrap(),
    }))
}
```

---

### Q6. When should you use JSONB columns versus separate relational columns?

**Interview Answer**

Use JSONB when: the schema is dynamic or varies between rows (e.g., different product attributes), the data is written once and read rarely, you need schema evolution without ALTER TABLE, or the data is semi-structured (API responses, webhooks). Use relational columns when: you frequently query/filter/join on the data, you need foreign key constraints, you need type safety and validation, the data is frequently updated, or you need to sort on specific fields. A common pattern is to store core fields as columns (id, name, created_at) and extended attributes as JSONB — giving you the best of both worlds. The `data->>'key'` extraction can be indexed with expression indexes for specific key access.

```sql
-- Good: hybrid approach
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,        -- Relational (frequently queried)
    price DECIMAL NOT NULL,             -- Relational (filtered, sorted)
    category_id INT REFERENCES categories(id), -- Relational (joined)
    attributes JSONB DEFAULT '{}'       -- JSONB (flexible, varies by product)
);

-- Index specific JSONB keys you query often
CREATE INDEX idx_products_brand ON products ((attributes->>'brand'));
CREATE INDEX idx_products_color ON products USING gin (attributes)
    WHERE attributes ? 'color';

-- Query both relational and JSONB
SELECT * FROM products
WHERE category_id = 5                    -- Relational index
  AND (attributes->>'brand') = 'Nike'   -- JSONB expression index
  AND price < 100;                       -- Relational index
```

---

### Q7. What are JSONB containment operators and how do they work with indexes?

**Interview Answer**

Containment operators check whether one JSONB value contains another. The `@>` operator checks if the left JSONB contains the right JSONB — this is the most important operator because it's GIN-indexable and fast. For example, `data @> '{"type": "click"}'` returns true if `data` has a key "type" with value "click". The `<@` operator is the reverse (left contained by right). Containment checks are recursive — `{"a": {"b": 1}} @> {"a": {"b": 1}}` is true. GIN indexes on JSONB support `@>`, `?`, `?|`, `?&` operators. The `jsonb_path_ops` GIN class only supports `@>` but produces smaller, faster indexes.

```sql
-- Create GIN index for containment queries
CREATE INDEX idx_events_data ON events USING gin (data);

-- Containment queries (all use the GIN index)
SELECT * FROM events WHERE data @> '{"type": "click"}';
SELECT * FROM events WHERE data @> '{"user_id": 42}';
SELECT * FROM events WHERE data @> '{"metadata": {"browser": "chrome"}}';

-- Nested containment
SELECT * FROM events WHERE data @> '{
    "type": "click",
    "metadata": {"browser": "chrome"}
}';

-- Existence checks (also use GIN index)
SELECT * FROM events WHERE data ? 'user_id';
SELECT * FROM events WHERE data ?| ARRAY['type', 'event_name'];

-- Combining containment with relational columns
SELECT * FROM events
WHERE data @> '{"type": "click"}'
  AND created_at > '2025-07-01'
  AND user_id = 42;
```

---

### Q8. What are the performance implications of using JSONB in PostgreSQL?

**Interview Answer**

JSONB has several performance considerations: (1) GIN indexes are slower to build and update than B-tree indexes, but provide fast read queries; (2) JSONB updates create new row versions, causing table bloat if done frequently — consider promoting frequently-updated JSONB keys to relational columns; (3) JSONB queries require type casting for numeric comparisons: `(data->>'price')::numeric > 100`; (4) Large JSONB documents (many KB) slow down scanning because entire documents must be decompressed; (5) JSONB is not compressed — storing 10KB of JSON costs 10KB of storage; (6) For analytics on JSONB data, consider materialized views with extracted columns. Profile with EXPLAIN ANALYZE to verify index usage.

```sql
-- Performance test
CREATE TABLE events (id SERIAL, data JSONB);
INSERT INTO events (data)
SELECT jsonb_build_object(
    'type', (ARRAY['click', 'view', 'scroll'])[floor(random()*3+1)],
    'user_id', floor(random()*10000),
    'page', '/' || md5(random()::text)
)
FROM generate_series(1, 1000000);

-- GIN index for containment
CREATE INDEX idx_events_gin ON events USING gin (data);

-- Test containment query performance
EXPLAIN ANALYZE
SELECT * FROM events WHERE data @> '{"type": "click"}';
-- GIN Index Scan: ~5ms for 333K matching rows

-- Expression index for specific key (often faster than GIN)
CREATE INDEX idx_events_type ON events ((data->>'type'));
EXPLAIN ANALYZE
SELECT * FROM events WHERE data->>'type' = 'click';
-- B-tree Index Scan: ~3ms (more selective)
```

---

### Q9. How do you validate JSONB data in PostgreSQL?

**Interview Answer**

PostgreSQL provides several mechanisms for JSONB validation: CHECK constraints with JSONB functions, custom functions for schema validation, and the `jsonb_typeof()` function for type checking. For basic validation, use CHECK constraints to ensure required keys exist and have correct types. For complex validation, create a PL/pgSQL function that validates the JSONB structure and call it from a CHECK constraint. JSONB does not enforce schema — any valid JSON is accepted. The `jsonb_pretty()` function helps debug malformed JSON. For API-style validation, consider using a Rust validation layer with serde before inserting into JSONB.

```sql
-- Basic validation with CHECK constraint
ALTER TABLE events ADD CONSTRAINT chk_event_type
    CHECK (data ? 'type' AND data->>'type' IN ('click', 'view', 'scroll'));

-- Numeric validation
ALTER TABLE events ADD CONSTRAINT chk_event_user_id
    CHECK (data ? 'user_id' AND (data->>'user_id')::int > 0);

-- Complex schema validation function
CREATE OR REPLACE FUNCTION validate_event(data JSONB)
RETURNS BOOLEAN AS $$
BEGIN
    -- Must have 'type' key
    IF NOT (data ? 'type') THEN RETURN FALSE; END IF;
    -- 'type' must be a string
    IF jsonb_typeof(data->'type') != 'string' THEN RETURN FALSE; END IF;
    -- Must have 'metadata' key if type is 'click'
    IF data->>'type' = 'click' AND NOT (data ? 'metadata') THEN RETURN FALSE; END IF;
    RETURN TRUE;
END;
$$ LANGUAGE plpgsql IMMUTABLE;

ALTER TABLE events ADD CONSTRAINT chk_event_schema
    CHECK (validate_event(data));
```

---

### Q10. What is JSONB aggregation and how do you combine multiple JSONB values?

**Interview Answer**

PostgreSQL provides `jsonb_object_agg()` to aggregate key-value pairs into a JSONB object, and `jsonb_agg()` to aggregate values into a JSONB array. These are useful for building JSONB responses directly in SQL without application-level transformation. For combining JSONB objects, use the `||` operator (merges right into left, overwriting duplicate keys). For merging without overwriting, use `jsonb_build_object()` with explicit key selection. These functions work with GROUP BY to create aggregated JSONB documents per group.

```sql
-- Aggregate rows into JSONB object
SELECT jsonb_object_agg(user_id, user_data) AS all_users
FROM (
    SELECT id AS user_id, jsonb_build_object('name', name, 'email', email) AS user_data
    FROM users
) sub;

-- Aggregate into JSONB array
SELECT jsonb_agg(jsonb_build_object('id', id, 'name', name)) AS users
FROM users;

-- Group by and aggregate
SELECT category_id,
       jsonb_agg(jsonb_build_object('name', name, 'price', price)) AS products
FROM products
GROUP BY category_id;

-- Merge JSONB objects
SELECT '{"a": 1}'::jsonb || '{"b": 2}'::jsonb;
-- Result: {"a": 1, "b": 2}

-- Merge with conflict (right overwrites left)
SELECT '{"a": 1, "b": 2}'::jsonb || '{"b": 3}'::jsonb;
-- Result: {"a": 1, "b": 3}
```
