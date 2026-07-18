# Database Replication in PostgreSQL

## Interview Question

What are the types of replication in PostgreSQL and when would you use each?

## Interview Answer

PostgreSQL supports two main replication types: Streaming Replication (physical replication) and Logical Replication. Streaming Replication sends WAL (Write-Ahead Log) records from the primary to standby servers byte-by-byte, creating an exact physical copy — used for high availability and read scaling. Logical Replication replicates individual changes at the logical level (row-by-row), allowing selective table replication and different schema versions between publisher and subscriber — used for data migration, selective sync, and cross-version replication. Both rely on WAL shipping but operate at different levels. Streaming is simpler and faster; logical is more flexible.

---

## Follow-up Questions & Answers

### Q1. What is streaming replication and how does it work?

**Interview Answer**

Streaming replication copies WAL records from the primary server to one or more standby servers in near-real-time. The standby continuously receives WAL, writes it to disk, and applies it to maintain an identical copy of the primary. The primary runs a walsender process; the standby runs a walreceiver process. Replication can be synchronous (primary waits for standby acknowledgment before committing) or asynchronous (primary commits immediately, standby may lag). Streaming is configured via `primary_conninfo` in `recovery.conf` (PostgreSQL 12+: `standby.signal` + `primary_conninfo` in postgresql.conf). The standby is read-only and can serve read queries to distribute load.

```sql
-- Primary server configuration
-- postgresql.conf
wal_level = replica
max_wal_senders = 3
wal_keep_size = 1GB
synchronous_standby_names = ''  -- Async; set name for sync

-- pg_hba.conf (allow replication connections)
-- host replication replicator standby_ip/32 md5

-- Create replication user
CREATE ROLE replicator WITH REPLICATION LOGIN PASSWORD 'password';

-- Standby server setup (PostgreSQL 12+)
-- 1. Base backup from primary
pg_basebackup -h primary_host -D /var/lib/postgresql/data -U replicator -P

-- 2. Create standby.signal file
touch /var/lib/postgresql/data/standby.signal

-- 3. Add to postgresql.conf on standby
primary_conninfo = 'host=primary_host port=5432 user=replicator password=password'
```

---

### Q2. What is logical replication and how does it differ from streaming?

**Interview Answer**

Logical replication decodes WAL records into logical changes (INSERT, UPDATE, DELETE) and sends them to subscribers. Unlike streaming (which replicates the entire database physically), logical replication lets you choose which tables to replicate, replicate across different PostgreSQL versions, and have different schemas on publisher and subscriber. It uses a publisher-subscriber model with publications (what to replicate) and subscriptions (what to receive). Logical replication is ideal for selective data sync, schema migration, and multi-master patterns. It requires `wal_level = logical` on the publisher and the pgoutput or similar plugin for decoding.

```sql
-- Publisher (primary) configuration
-- postgresql.conf
wal_level = logical
max_replication_slots = 4
max_wal_senders = 4

-- Create publication
CREATE PUBLICATION my_pub FOR TABLE users, orders;

-- Or publish all tables
CREATE PUBLICATION all_tables_pub FOR ALL TABLES;

-- Subscriber setup
-- Create subscription
CREATE SUBSCRIPTION my_sub
    CONNECTION 'host=publisher_host dbname=mydb user=replicator password=password'
    PUBLICATION my_pub;

-- Check subscription status
SELECT * FROM pg_stat_subscription;

-- Add a new table to existing publication
ALTER PUBLICATION my_pub ADD TABLE products;

-- Refresh subscription schema
ALTER SUBSCRIPTION my_sub REFRESH PUBLICATION;
```

---

### Q3. What is synchronous versus asynchronous replication?

**Interview Answer**

Asynchronous replication (default) — the primary commits transactions without waiting for the standby to confirm receipt. This is faster but means committed data may be lost if the primary crashes before the standby receives the WAL. Synchronous replication — the primary waits for at least one standby to confirm it has received and flushed the WAL to disk before acknowledging the commit. This guarantees zero data loss (RPO = 0) but increases commit latency. Configure with `synchronous_standby_names` on the primary. You can use a hybrid approach: critical tables use synchronous replication, non-critical use asynchronous.

```sql
-- Asynchronous (default, no data loss guarantee)
-- Primary commits immediately
synchronous_standby_names = ''

-- Synchronous (zero data loss)
-- Primary waits for standby confirmation
synchronous_standby_names = 'FIRST 1 (standby1, standby2)'
-- FIRST 1: wait for at least 1 standby
-- ANY 2: wait for any 2 standbys

-- Hybrid: synchronous for critical data only
-- Use separate replication slots for critical vs non-critical tables
synchronous_standby_names = 'FIRST 1 (critical_standby)'

-- Monitor replication lag
SELECT client_addr, state, sent_lsn, write_lsn, replay_lsn,
       replay_lag
FROM pg_stat_replication;
```

---

### Q4. What is replication lag and how do you monitor it?

**Interview Answer**

Replication lag is the delay between a transaction committing on the primary and being applied on the standby. In streaming replication, lag is measured in bytes (WAL difference) and time. High lag means the standby is behind — reads from the standby may be stale. Causes: network latency, slow standby hardware, long transactions on primary blocking WAL replay. Monitor with `pg_stat_replication` (primary side) and `pg_stat_wal_receiver` (standby side). Set up alerts when lag exceeds thresholds (e.g., 10MB or 30 seconds).

```sql
-- On primary: check replication status
SELECT client_addr, application_name, state,
       sent_lsn, write_lsn, replay_lsn,
       pg_wal_lsn_diff(sent_lsn, replay_lsn) AS replay_lag_bytes,
       replay_lag
FROM pg_stat_replication;

-- On standby: check receiver status
SELECT status, receive_start_lsn, received_lsn,
       last_msg_send_time, last_msg_receipt_time,
       pg_wal_lsn_diff(receive_start_lsn, received_lsn) AS bytes_received
FROM pg_stat_wal_receiver;

-- Monitor lag over time (set up alerting)
SELECT client_addr,
       pg_size_pretty(pg_wal_lsn_diff(sent_lsn, replay_lsn)) AS lag,
       replay_lag
FROM pg_stat_replication;

-- Alert if lag > 100MB or > 30 seconds
```

---

### Q5. How do you handle failover in a streaming replication setup?

**Interview Answer**

Failover promotes a standby to primary when the primary fails. Steps: (1) Detect primary failure (use pg_monitor, Patroni, or pg_auto_failover); (2) Ensure the standby is caught up (check replay_lag); (3) Promote the standby (pg_promote() or touch promotion file); (4) Redirect application connections to the new primary; (5) Reconfigure old primary as standby when it recovers. Tools like Patroni, pg_auto_failover, and repmgr automate this process. Manual failover is risky — always use automated failover in production. After failover, the old primary must be reconfigured as a new standby of the new primary.

```sql
-- Manual failover (NOT recommended for production)
-- 1. On standby: promote to primary
SELECT pg_promote();

-- Or (PostgreSQL 12+):
-- pg_ctl promote -D /var/lib/postgresql/data

-- 2. Redirect connections (update DNS, load balancer, or connection string)

-- 3. When old primary recovers, reconfigure as standby
-- On old primary:
-- Create standby.signal
-- Set primary_conninfo in postgresql.conf
-- Start PostgreSQL

-- Automated failover with Patroni (recommended)
-- patroni.yml
-- bootstrap:
--   dcs:
--     ttl: 30
--     loop_wait: 10
--     retry_timeout: 10
--     maximum_lag_on_failover: 1048576
```

---

### Q6. What are replication slots and why are they important?

**Interview Answer**

Replication slots track the position of a replica in the WAL stream, preventing the primary from deleting WAL that the standby hasn't received yet. Without slots, the primary may recycle WAL files before the standby catches up, causing the standby to disconnect and require a full re-sync. Slots are essential for logical replication (subscribers must have a slot) and for slow standbys in streaming replication. Monitor slot usage — unused slots cause WAL accumulation and disk fill. Drop slots for disconnected replicas. Each slot uses `max_replication_slots` on the primary.

```sql
-- Create replication slot (for streaming replication)
SELECT pg_create_physical_replication_slot('standby1_slot');

-- Create replication slot (for logical replication)
SELECT pg_create_logical_replication_slot('sub1_slot', 'pgoutput');

-- On standby: use the slot
primary_conninfo = 'host=primary user=replicator slot_name=standby1_slot'

-- Monitor slot usage
SELECT slot_name, slot_type, active,
       pg_wal_lsn_diff(pg_current_wal_lsn(), restart_lsn) AS retained_bytes
FROM pg_replication_slots;

-- Drop unused slots (prevents WAL bloat)
SELECT pg_drop_replication_slot('old_slot');

-- Check WAL retention due to slots
SELECT slot_name,
       pg_size_pretty(pg_wal_lsn_diff(pg_current_wal_lsn(), restart_lsn)) AS wal_retained
FROM pg_replication_slots WHERE NOT active;
```

---

### Q7. How do you set up logical replication for selective table sync?

**Interview Answer**

Logical replication lets you replicate only specific tables between publisher and subscriber. This is useful for: syncing a subset of data to a reporting database, migrating specific tables to a new cluster, or maintaining a denormalized read replica. Configure by creating a publication with specific tables on the publisher and a subscription on the subscriber. The subscriber can have different indexes, constraints, and even schema — only the replicated data matches. Initial data sync happens automatically; subsequent changes stream via logical decoding.

```sql
-- Publisher: create publication for specific tables
CREATE PUBLICATION selective_pub FOR TABLE users, orders, products;

-- Subscriber: create subscription
CREATE SUBSCRIPTION selective_sub
    CONNECTION 'host=publisher_host dbname=mydb user=replicator password=password'
    PUBLICATION selective_pub;

-- Subscriber can have different schema
-- (e.g., additional columns, different indexes)
CREATE TABLE users_replica (
    id INT PRIMARY KEY,
    name VARCHAR(255),
    email VARCHAR(255),
    replica_updated_at TIMESTAMPTZ DEFAULT NOW()  -- Extra column
);

-- Add new table to existing publication
ALTER PUBLICATION selective_pub ADD TABLE categories;

-- Check replication status
SELECT srrelid::regclass, srsubid::regclass,
       nspname, relname
FROM pg_subscription_rel;

-- Monitor subscriber lag
SELECT * FROM pg_stat_subscription;
```

---

### Q8. How do you handle schema changes with logical replication?

**Interview Answer**

Logical replication requires manual schema synchronization — when you ALTER TABLE on the publisher, you must apply the same change on the subscriber before the replicated data arrives. The subscriber's schema must be compatible with the incoming data. If the subscriber's table has fewer columns, extra columns must have defaults. If the subscriber has more columns, they must have defaults or be nullable. Best practice: apply schema changes to the subscriber first, then the publisher. For complex migrations, pause replication, apply changes to both, then resume.

```sql
-- Publisher: add a column
ALTER TABLE users ADD COLUMN phone VARCHAR(20);

-- Subscriber: must add column BEFORE the data arrives
ALTER TABLE users ADD COLUMN phone VARCHAR(20);

-- Or add with a default if the column is NOT NULL
ALTER TABLE users ADD COLUMN phone VARCHAR(20) NOT NULL DEFAULT '';

-- If schema change fails on subscriber, replication stops
-- Check subscription status
SELECT * FROM pg_stat_subscription;

-- Resume after fixing schema
ALTER SUBSCRIPTION selective_sub REFRESH PUBLICATION;

-- For complex migrations: use pg_dump/pg_restore or migration tools
-- Pause replication -> migrate schema -> resume
```

---

### Q9. How do you use streaming replication with SQLx in Rust?

**Interview Answer**

With streaming replication, your application connects to the primary for writes and optionally to standbys for reads (read scaling). SQLx connects to whichever database URL you provide — to use read replicas, create separate connection pools for primary and standby. Route read queries to the standby pool and write queries to the primary pool. For logical replication, you subscribe to changes and process them in your Rust application using a logical replication client (like `tokio-postgres` with `pg_replication`).

```rust
use sqlx::PgPool;

struct DatabasePool {
    primary: PgPool,   // For writes
    standby: PgPool,   // For reads (replica)
}

impl DatabasePool {
    async fn new(primary_url: &str, standby_url: &str) -> Result<Self> {
        let primary = PgPool::connect(primary_url).await?;
        let standby = PgPool::connect(standby_url).await?;
        Ok(Self { primary, standby })
    }

    async fn create_order(&self, order: &NewOrder) -> Result<Order> {
        // Write to primary
        sqlx::query_as!(Order,
            "INSERT INTO orders (customer_id, total) VALUES ($1, $2) RETURNING *",
            order.customer_id, order.total
        )
        .fetch_one(&self.primary)
        .await
    }

    async fn get_orders(&self, customer_id: i64) -> Result<Vec<Order>> {
        // Read from standby (may lag slightly)
        sqlx::query_as!(Order,
            "SELECT * FROM orders WHERE customer_id = $1", customer_id
        )
        .fetch_all(&self.standby)
        .await
    }
}

// Usage
let db = DatabasePool::new(
    "postgres://primary:5432/mydb",
    "postgres://standby:5432/mydb"
).await?;
```

---

### Q10. What is cascading replication and when would you use it?

**Interview Answer**

Cascading replication is when a standby server acts as a WAL source for other standbys, forming a chain: Primary -> Standby1 -> Standby2 -> Standby3. This reduces the load on the primary because it only sends WAL to one standby, which relays it to others. Use cascading for geographic distribution (primary in US, standby in EU, second standby in Asia) or when you need many replicas but want to limit primary's `max_wal_senders`. The cascading standby must have `hot_standby = on` and `wal_receiver_status_interval > 0`. The primary only tracks the first-level standby; cascaded standbys don't appear in `pg_stat_replication`.

```sql
-- Primary: normal replication setup
-- postgresql.conf
wal_level = replica
max_wal_senders = 3

-- Standby 1 (receives from primary, sends to standby 2)
-- postgresql.conf
hot_standby = on
wal_receiver_status_interval = 10
primary_conninfo = 'host=primary user=replicator'

-- Standby 2 (receives from standby 1)
-- postgresql.conf
primary_conninfo = 'host=standby1 user=replicator'

-- Monitor cascade
-- On standby 1: check it's receiving from primary
SELECT * FROM pg_stat_wal_receiver;

-- On primary: check standby1 is connected
SELECT * FROM pg_stat_replication;
-- standby2 won't appear here (it's connected to standby1)
```
