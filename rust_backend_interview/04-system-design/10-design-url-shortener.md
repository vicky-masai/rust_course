# Design URL Shortener

## Interview Question

How would you design a high-performance URL shortener using Rust?

## Interview Answer

I would design a URL shortener with an Axum API that accepts long URLs, generates a unique short code using Base62 encoding, stores the mapping in PostgreSQL, and caches hot URLs in Redis for fast redirects. The system uses a snowflake-like ID generator to produce unique 64-bit IDs, which are then Base62-encoded to create 7-character short URLs (supporting 3.5 trillion unique URLs). Redis handles the read-heavy redirect path — a `GET /{code}` request checks Redis first (sub-millisecond), falls back to PostgreSQL on miss, and returns a 301 redirect. The system also includes rate limiting to prevent abuse, click analytics tracked via Kafka events, and a CDN for caching popular redirect responses. The estimated throughput is 10,000 redirects/second per Axum instance.

---

## Follow-up Questions & Answers

### Q1. How do you generate unique short codes?

**Interview Answer**

I use a Snowflake-inspired ID generator that produces 64-bit IDs with a timestamp prefix, ensuring monotonicity and uniqueness without coordination between instances. The 64-bit ID is then Base62-encoded (a-z, A-Z, 0-9) into a 7-character string. For example, ID `8475937281024` becomes `Kj3nPx2`. In Rust, I implement the generator as a `Mutex<SnowflakeIdGenerator>` shared across Axum handlers via `Extension`. Each instance has a unique worker ID (assigned at startup from a Redis INCR), preventing ID collisions across instances. The alternative of using PostgreSQL `SERIAL` creates a bottleneck since every short URL requires a database write. The Snowflake approach generates IDs locally without any database round-trip.

---

### Q2. How do you handle the redirect path for maximum performance?

**Interview Answer**

The redirect path (`GET /{code}`) is optimized for speed since it's the most critical and highest-traffic endpoint. I check Redis first using `GET url:{code}`, which returns in under 1ms. On a cache miss, I query PostgreSQL using a prepared statement and populate Redis with a 1-hour TTL before returning. The Axum handler returns a 301 (permanent redirect) for SEO-friendly URLs or 302 (temporary redirect) if tracking is needed. I implement connection pooling via `sqlx::PgPool` with `max_connections(20)` to handle database queries efficiently. For popular URLs (top 1%), I implement an in-process `moka` cache that eliminates Redis calls entirely, achieving sub-microsecond redirect latency. The entire redirect path adds under 2ms of server-side latency.

---

### Q3. How do you prevent short code collisions?

**Interview Answer**

With a Snowflake ID generator, collisions are impossible as long as worker IDs are unique across instances. However, I add a safety check: after Base62-encoding the ID, I check Redis for the key using `SET NX` (set-if-not-exists) before writing to PostgreSQL. If `SET NX` returns false (key exists), it means a collision occurred (extremely unlikely with Snowflake IDs), and I regenerate with a new ID. The PostgreSQL table has a unique index on the short code, providing a final safety net. I also implement a "custom alias" feature where users can choose their own short code — these are checked against Redis and PostgreSQL before acceptance to prevent duplicates.

---

### Q4. How do you handle URL expiry and cleanup?

**Interview Answer**

I add an `expires_at` column to the PostgreSQL `urls` table, defaulting to NULL (never expires) or set by the user. For expired URLs, I implement two cleanup strategies: (1) A background Tokio task runs every hour, scanning for expired URLs using `SELECT id FROM urls WHERE expires_at < NOW() AND deleted = false`, soft-deletes them (sets `deleted = true`), and removes them from Redis. (2) At read time, the redirect handler checks the expiry and returns 410 Gone for expired URLs, avoiding stale redirects. For Redis cleanup, expired keys are automatically evicted by Redis's TTL mechanism. I use TimescaleDB's continuous aggregation to track URL expiry statistics and optimize the cleanup query with an index on `expires_at`.

---

### Q5. How do you implement click analytics?

**Interview Answer**

Every redirect triggers a Kafka event containing the short code, timestamp, IP address (for geo-lookup), user agent, and referrer. A Kafka consumer aggregates these events in real time: total clicks per URL, clicks per hour, geographic distribution (using a MaxMind GeoIP2 lookup in Rust), and device breakdown. Aggregated data is stored in TimescaleDB for time-series queries and Redis for real-time dashboards. The analytics API (Axum) queries TimescaleDB with `time_bucket` functions to provide hourly/daily/weekly click reports. I implement sampling for very popular URLs (>1M clicks) — only 10% of clicks are recorded to reduce storage costs while maintaining statistical accuracy.

---

### Q6. How do you handle abuse and spam?

**Interview Answer**

I implement multi-layer abuse prevention: rate limiting (100 creates/hour per API key, 10 for unauthenticated users), URL scanning via a VirusTotal API integration to detect phishing and malware, and a blocklist of known malicious domains stored in Redis. The Axum middleware checks the long URL against the blocklist before creating the short URL. I also implement a "report" API endpoint where users can flag malicious short URLs, which are immediately disabled and queued for review. For spam campaigns, I detect patterns like a single IP creating hundreds of short URLs pointing to the same domain and auto-block the IP. All abuse prevention runs in the Axum middleware pipeline, keeping business logic clean.

---

### Q7. How do you implement custom short URLs?

**Interview Answer**

Custom URLs (e.g., `short.ly/my-brand`) are handled by a separate API endpoint `POST /api/v1/custom` that accepts the desired alias and the long URL. The alias is validated: 3-20 characters, alphanumeric and hyphens only, not a reserved word (like "api", "admin", "health"), and not already taken. The check uses `SET NX` in Redis for atomicity, and if the alias is available, it's written to PostgreSQL with a unique constraint. Custom URLs bypass the Snowflake ID generator entirely. I implement a "claim" feature where verified businesses can reserve brand-related aliases by verifying domain ownership via DNS TXT records, preventing others from squatting on their brand.

---

### Q8. How do you design the database schema?

**Interview Answer**

The `urls` table has columns: `id` (BIGSERIAL primary key), `short_code` (VARCHAR(7) UNIQUE NOT NULL), `long_url` (TEXT NOT NULL), `user_id` (BIGINT FK), `created_at` (TIMESTAMPTZ), `expires_at` (TIMESTAMPTZ), `click_count` (BIGINT DEFAULT 0), and `deleted` (BOOLEAN DEFAULT false). Indexes include: a unique B-tree index on `short_code` for O(1) lookups, a composite index on `(user_id, created_at)` for user URL listings, and a partial index on `expires_at WHERE expires_at IS NOT NULL` for cleanup queries. I partition the table by `created_at` using PostgreSQL declarative partitioning (monthly partitions) to keep indexes small and queries fast as the table grows to billions of rows.

---

### Q9. How do you implement analytics for URL creators?

**Interview Answer**

The analytics API provides URL creators with real-time and historical click data. The Axum endpoint `GET /api/v1/urls/{code}/analytics` queries TimescaleDB for time-series data, returning click counts per hour, geographic breakdown (top 10 countries), device types (mobile/desktop/tablet), and referrer sources. I use Redis to cache the most recent 24 hours of data, since creators often refresh their analytics page. For export, I implement a CSV download endpoint that generates the data asynchronously via Kafka and stores the result in S3 with a pre-signed URL. The analytics dashboard uses these APIs to display charts with `Chart.js` on the frontend, updated via Server-Sent Events for real-time updates.

---

### Q10. How do you handle scaling to billions of URLs?

**Interview Answer**

At billion-URL scale, I partition the `urls` table by the first character of the short_code using PostgreSQL table partitioning, distributing data across 62 partitions (a-z, A-Z, 0-9). Redis Cluster with 12 shards handles the hot cache — I estimate 100GB of Redis memory for the top 10 million most-clicked URLs. The Axum API scales to 30+ instances behind an ALB, each handling 10,000+ redirects per second. For the write path (URL creation), I use Kafka as a write-ahead buffer: the API publishes creation events to Kafka, and a consumer batch-inserts into PostgreSQL every 5 seconds, reducing database write load by 90%. Read replicas handle analytics queries, keeping the primary focused on redirects and creations.
