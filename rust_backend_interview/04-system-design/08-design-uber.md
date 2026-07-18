# Design Uber

## Interview Question

How would you design a ride-sharing platform like Uber using Rust?

## Interview Answer

I would design Uber with microservices handling distinct domains: a driver location service using geospatial indexing for real-time location tracking, a matching service that pairs riders with nearby drivers, a trip service managing ride lifecycle, a pricing service calculating dynamic fares, and a payment service handling transactions. The driver location service stores positions in Redis with geospatial indexes (GEOADD) for fast nearby queries, updated every 4 seconds via WebSocket connections from driver apps. Kafka acts as the event backbone, decoupling location updates, trip events, and payment processing. PostgreSQL stores trip history, user profiles, and payment records. The matching algorithm runs in a dedicated Axum service that queries Redis for nearby drivers and applies scoring based on distance, driver rating, and estimated arrival time.

---

## Follow-up Questions & Answers

### Q1. How do you implement real-time driver location tracking?

**Interview Answer**

Each driver's app maintains a persistent WebSocket connection to a location service (Axum with `tokio-tungstenite`). The driver's GPS coordinates are sent every 4 seconds. The server updates Redis using `GEOADD driver_locations {longitude} {latitude} {driver_id}`, which stores the driver's position in a Redis geospatial index. Redis can then answer "find all drivers within 5km" queries using `GEORADIUS` in under 1ms. For persistence, location updates are also published to a Kafka topic, and a consumer writes them to a TimescaleDB table for historical analysis. The WebSocket server scales to handle 1 million concurrent driver connections by running on dedicated pods with generous file descriptor limits (`ulimit -n 100000`).

---

### Q2. How does the driver-rider matching algorithm work?

**Interview Answer**

When a rider requests a ride, the matching service queries Redis `GEORADIUS` for drivers within 5km, filtered by driver status (available, not on a trip). The service scores each candidate: distance contributes 40%, estimated arrival time contributes 30%, driver rating contributes 20%, and acceptance rate contributes 10%. The highest-scoring driver receives the ride request via their WebSocket connection. If the driver doesn't accept within 15 seconds, the request is offered to the next-best driver. I implement this in Rust as an async function that runs the scoring in parallel using `tokio::join!` on the Redis queries. The entire matching process completes in under 200ms, which is critical for user experience.

---

### Q3. How do you handle surge pricing?

**Interview Answer**

Surge pricing is calculated by the pricing service using a supply-demand model. Every 30 seconds, a background Tokio task counts available drivers and pending ride requests per geo-zone (using Redis `GEORADIUS` for drivers and a Kafka consumer count for requests). The surge multiplier is `max(1.0, request_count / (driver_count * 2.0))`, capped at 5.0x. The surge map is stored in Redis as a hash: `surge:zone:{zone_id}` with the current multiplier. When a rider requests a ride, the pricing service reads the surge multiplier and calculates the fare estimate. Surge zones are displayed on the rider's map as colored overlays. The pricing service recalculates every 30 seconds to respond to demand changes quickly.

---

### Q4. How do you implement trip lifecycle management?

**Interview Answer**

A trip progresses through states: requested → matched → accepted → in_progress → completed → paid. Each state transition is a Kafka event, and the trip service (Axum) processes them via a consumer. The trip state is stored in PostgreSQL with a state machine implemented in Rust using an enum: `enum TripStatus { Requested, Matched, Accepted, InProgress, Completed, Paid }`. Each transition is validated — you can't go from `Requested` to `Completed` directly. State transitions trigger side effects: `Accepted` sends a notification to the rider, `InProgress` starts the fare timer, `Completed` triggers the payment service. I use PostgreSQL transactions for state transitions to ensure atomicity, and I emit Kafka events for each transition so other services can react.

---

### Q5. How do you handle payment processing?

**Interview Answer**

The payment service integrates with Stripe for credit card processing. When a trip completes, the pricing service publishes a `trip_completed` event with the fare amount to Kafka. The payment consumer picks it up, creates a Stripe payment intent using the rider's saved payment method, and executes the charge. Payment results are stored in PostgreSQL with trip_id, amount, currency, status, and Stripe transaction ID. I implement idempotency using the trip_id as the Stripe idempotency key, preventing duplicate charges during retries. Failed payments are retried 3 times with exponential backoff, then moved to a DLQ for manual resolution. Driver payouts are batched daily via Stripe Connect transfers, aggregated by the payout consumer.

---

### Q6. How do you design the geospatial index for fast driver lookup?

**Interview Answer**

Redis geospatial index (`GEOADD` / `GEORADIUS`) is the primary index for nearby driver queries, providing O(log(N) + M) performance where N is the total drivers and M is the result set size. For 1 million drivers, a `GEORADIUS` query returns in under 1ms. However, Redis geospatial index doesn't support filtering by driver status, so I combine it with a Redis Set per zone: `zone:{zone_id}:available_drivers`. The matching service first finds drivers within radius, then filters by set membership. For city-level partitioning, I divide the city into a grid of 1km × 1km cells, each stored as a separate geospatial key. This reduces the search space dramatically — instead of searching 1 million drivers, I search only the 4-9 adjacent cells.

---

### Q7. How do you handle driver ETA calculations?

**Interview Answer**

I estimate driver ETA by calculating the distance between driver and rider using the Haversine formula (for straight-line distance) and multiplying by a local speed factor (typically 20-30 km/h in urban areas). For more accuracy, I integrate with a routing API (Google Maps or OSRM) to get actual driving distance and time, but only for the top 3-5 candidate drivers to avoid excessive API calls. The ETA is cached in Redis for 30 seconds. In Rust, the Haversine calculation is a simple function using `f64` trigonometry. For the routing API, I use `reqwest` with a 500ms timeout and fall back to the Haversine approximation if the API is slow. The ETA displayed to the rider updates every 10 seconds via WebSocket.

---

### Q8. How do you ensure system availability during peak hours?

**Interview Answer**

I deploy the matching service with 95th percentile latency targets under 200ms, which requires at least 3 replicas across 2 AZs. Auto-scaling is configured to add pods when Redis queue depth (ride requests) exceeds 1000. The location service is stateless and scales to 20+ replicas. PostgreSQL uses read replicas for analytics queries and the primary for trip writes, with PgBouncer for connection pooling. Redis runs in cluster mode with 6 shards and automatic failover. I implement a "degraded mode" where, if the matching service is overloaded, it expands the search radius to 10km instead of 5km and accepts the first available driver rather than scoring, trading match quality for response time. Load testing with `wrk` simulates 10x peak traffic to validate capacity.

---

### Q9. How do you handle data consistency across microservices?

**Interview Answer**

I use the Saga pattern for multi-service transactions. For example, creating a trip involves the matching service (find a driver), trip service (create trip record), and payment service (authorize payment). Each step publishes a Kafka event, and the next service consumes it. If a step fails, a compensating event is published to undo previous steps. I implement this in Rust using a state machine per saga, stored in PostgreSQL with the saga state and individual step statuses. The Kafka consumer processes events idempotently using the saga_id as a deduplication key. For strong consistency within a single service, I use PostgreSQL transactions. For cross-service consistency, I accept eventual consistency (within 1-2 seconds) via Kafka event propagation.

---

### Q10. How do you implement ride tracking and trip history?

**Interview Answer**

During a ride, the driver's location is streamed via WebSocket to the trip service, which stores GPS breadcrumbs (lat, lng, timestamp, speed) in TimescaleDB — a PostgreSQL extension optimized for time-series data. The rider's app receives location updates via WebSocket in real time, showing the car moving on the map. For trip history, the rider can view past trips with route polylines, which are generated from the GPS breadcrumbs using the Douglas-Peucker algorithm (implemented in Rust) to reduce point density. The trip history API queries TimescaleDB with time-range filters and returns paginated results. I use TimescaleDB continuous aggregates to pre-compute daily and weekly summaries (total distance, average speed, fare) for the analytics dashboard.
