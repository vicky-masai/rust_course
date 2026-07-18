# Design YouTube

## Interview Question

How would you design a video streaming platform like YouTube using Rust?

## Interview Answer

I would design YouTube with an upload service that accepts video files via resumable uploads (tus protocol), a video processing pipeline that transcodes videos into multiple resolutions using FFmpeg workers, and a CDN for global content delivery. The metadata service (Axum + PostgreSQL) stores video titles, descriptions, channel info, and view counts. A recommendation service uses collaborative filtering on user watch history stored in Kafka events. The search service uses Elasticsearch for full-text search across titles, descriptions, and tags. Analytics events (views, watch time, likes) flow through Kafka into a data warehouse for creator dashboards. Redis caches hot video metadata and trending lists. The architecture handles 500 hours of video uploaded per minute and serves 1 billion hours of video per day.

---

## Follow-up Questions & Answers

### Q1. How do you handle large video uploads?

**Interview Answer**

I implement resumable uploads using the tus protocol, which splits files into 5MB chunks and allows uploads to resume from the last successful chunk. The Axum upload service generates a pre-signed S3 URL for each chunk, and the client uploads directly to S3, bypassing the application server for data transfer. The upload service stores upload progress in Redis, so if a connection drops, the client can resume from the last chunk. For a 1GB video, this means 200 chunk uploads, each completing in under a second on a decent connection. The total upload time is dominated by the user's bandwidth, not server processing. Once all chunks are uploaded, the service triggers the transcoding pipeline via Kafka.

---

### Q2. How do you design the video transcoding pipeline?

**Interview Answer**

After upload, a Kafka consumer triggers the transcoding pipeline. The video worker downloads the original from S3 and uses FFmpeg (via Rust's `std::process::Command`) to transcode into multiple resolutions: 240p, 360p, 480p, 720p, 1080p, and optionally 4K. Each resolution is a separate FFmpeg process running in parallel using `tokio::process::Command`, which makes FFmpeg async and non-blocking. The output segments are stored in S3 and registered with a CDN. I also extract video thumbnails (3 per video at 25%, 50%, 75% duration) and generate HLS manifests (`.m3u8` playlists) for adaptive streaming. The entire pipeline is orchestrated by a Rust consumer that tracks progress and updates the video status in PostgreSQL from "processing" to "ready."

---

### Q3. How do you implement video streaming with adaptive bitrate?

**Interview Answer**

I use HLS (HTTP Live Streaming) for adaptive bitrate delivery. The transcoding pipeline generates multiple quality levels, each as a separate HLS playlist with 10-second TS segments. The player (client-side) downloads the master playlist, which lists all available qualities, and dynamically switches between them based on available bandwidth. On the server side, each HLS segment is stored in S3 and served via CloudFront CDN with long cache TTLs (24 hours). The Axum API serves only the manifest files (`.m3u8`), which are small and cacheable. I implement token-based access control: the API generates a signed URL with a 2-hour expiry for each video request, preventing unauthorized sharing. Segment-level CDN caching ensures even 4K videos stream smoothly globally.

---

### Q4. How do you design the recommendation system?

**Interview Answer**

The recommendation service uses a hybrid approach: collaborative filtering (users who watched X also watched Y) and content-based filtering (similar tags, categories, and channels). User watch events are streamed via Kafka, and a background Rust job computes a user-video affinity matrix using matrix factorization (ALS algorithm). The results are stored in Redis as sorted sets per user: `recommendations:{user_id}` with the affinity score as the score. When the home page is requested, the Axum service fetches the top 100 recommendations from Redis and applies business rules (diversity, freshness, popularity). Cold-start users get trending videos. The model is retrained every 6 hours using historical data. I use the `linfa` Rust ML crate for the matrix factorization.

---

### Q5. How do you implement the search functionality?

**Interview Answer**

I use Elasticsearch for full-text search across video titles, descriptions, tags, and channel names. The index schema maps each video as a document with fields for title (text, boosted 2x), description (text), tags (keyword), channel_name (text), view_count (integer), and upload_date (date). The Axum search service builds Elasticsearch queries using the `elastic` Rust crate, supporting autocomplete (using n-gram tokenizers), typo tolerance (using fuzzy queries), and filters (by duration, upload date, view count). Search results are cached in Redis for 60 seconds since search queries are frequently repeated. I also implement search ranking based on a combination of relevance score, view count, and recency, so fresh, popular content appears first.

---

### Q6. How do you track and aggregate analytics?

**Interview Answer**

Every video view generates a Kafka event with video_id, user_id, timestamp, watch_duration, device_type, and country. A Kafka consumer aggregates these events into real-time counters stored in Redis: `video:{id}:views` (total views), `video:{id}:watch_time` (total watch time). These counters are updated atomically using Redis `INCRBY` and `INCRBYFLOAT`. For creator dashboards, I use TimescaleDB continuous aggregates to compute hourly and daily summaries (views, watch time, revenue, subscriber growth) from the Kafka events. The analytics API queries these pre-aggregated views instead of raw events, keeping response times under 50ms. I also implement A/B test tracking by including a `variant` field in the event, enabling comparison of recommendation algorithm versions.

---

### Q7. How do you handle content moderation?

**Interview Answer**

Content moderation uses a multi-stage pipeline. When a video is uploaded, it goes through automated checks before transcoding: NSFW detection using a TensorFlow model served via a separate Rust gRPC service, copyright detection using YouTube's Content ID-like fingerprinting (audio and video hashing stored in PostgreSQL), and metadata scanning for banned keywords. Videos that fail automated checks are queued for human review in a moderation dashboard (Axum + HTMX). Human moderators approve, reject, or restrict (age-gate) videos. The moderation state machine is: uploaded → auto_review → (approved | rejected | human_review). All moderation decisions are logged for audit. I use `tokio::task::spawn_blocking` to run the ML inference without blocking the Tokio runtime.

---

### Q8. How do you scale the system for billions of views?

**Interview Answer**

At YouTube scale, the key bottleneck is serving video segments, which is handled entirely by the CDN — CloudFront can serve petabytes of data per day. The application layer (metadata, search, recommendations) scales independently. I deploy the metadata API with 50+ Axum instances across 3 regions, each hitting a Redis Cluster with 24 shards for hot data. PostgreSQL uses Citus (distributed PostgreSQL) for horizontal sharding of video metadata by channel_id. Kafka runs with 100+ partitions per topic for parallel processing of view events. Auto-scaling is driven by custom metrics: requests/second for API pods and Kafka consumer lag for processing pods. I use `cargo flamegraph` to optimize hot paths, and Rust's zero-cost abstractions ensure the application layer adds negligible overhead.

---

### Q9. How do you handle live streaming?

**Interview Answer**

Live streaming uses RTMP ingest and HLS/DASH delivery. Streamers push RTMP to an Axum-based ingest server that receives the stream and passes it to a live transcoder (FFmpeg with `-f hls` flag). The transcoder outputs HLS segments in real time, which are immediately uploaded to S3 and served via CDN. The HLS manifest is updated every 2-3 seconds with new segments, and viewers' players poll the manifest to receive new content. Latency is typically 10-30 seconds with HLS. For lower latency, I can implement WebRTC for the last-mile delivery (2-3 second latency). Live chat runs through a separate WebSocket service using Redis Pub/Sub for message fan-out. Stream health is monitored via real-time metrics: bitrate stability, frame drops, and viewer count, all exposed via Prometheus.

---

### Q10. How do you design the notification system for new uploads?

**Interview Answer**

When a creator uploads a video, the metadata service publishes a `video_published` event to Kafka. A notification consumer consumes these events, fetches the creator's subscriber list from PostgreSQL, and publishes individual notification events to a `notifications` Kafka topic partitioned by subscriber_id. The notification workers (Axum services) process these events and send push notifications via FCM/APNs, in-app notifications stored in PostgreSQL, and email digests (batched hourly for users who prefer email). I deduplicate notifications using a Redis set per subscriber: `notified:{subscriber_id}:{video_id}` with a 24-hour TTL, preventing duplicate notifications during retry. The system scales to millions of subscribers by parallelizing across Kafka partitions and batching notification sends.
