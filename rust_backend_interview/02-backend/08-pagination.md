# Pagination

## Interview Question

Pagination.

## Interview Answer

"I prefer cursor-based pagination for large datasets because it's more scalable than OFFSET."

---

## Follow-up Questions & Answers

### Q1. Why is OFFSET-based pagination inefficient for large datasets?

**Interview Answer**

OFFSET skips rows at the database level, so `OFFSET 100000` still reads and discards 100,000 rows before returning results. As the offset grows, query time increases linearly even though the response size stays the same. This causes significant performance degradation on tables with millions of rows.

---

### Q2. How does cursor-based pagination work in practice?

**Interview Answer**

The client sends the last seen identifier, like a user ID or timestamp, and the server fetches the next page using a `WHERE id > cursor` clause. The query uses an index on the cursor column, so it's efficient regardless of page position. I typically use the primary key or a unique column as the cursor to ensure consistent ordering.

---

### Q3. How do you implement cursor-based pagination in Axum with sqlx?

**Interview Answer**

I define a `Pagination` struct with optional `cursor` and `limit` fields extracted from query parameters. The SQL query uses `WHERE id > $1 ORDER BY id ASC LIMIT $2` with the cursor value and a default limit of 20. The response includes a `next_cursor` field from the last item's ID so the client knows what to send for the next page.

---

### Q4. What happens when records are inserted or deleted while paginating?

**Interview Answer**

With cursor-based pagination using a stable column like an auto-incrementing ID, insertions and deletions between pages don't cause items to be skipped or duplicated. OFFSET pagination can skip or repeat rows when data changes between page requests. This stability is a major reason I prefer cursor-based pagination for real-time applications.

---

### Q5. How do you handle the "last page" problem with cursor pagination?

**Interview Answer**

I return a `has_more` boolean flag based on whether the query returned a full page of results. If the result set has fewer items than the requested limit, `has_more` is false and the client knows it has reached the end. I also include `next_cursor: null` when there are no more pages to make the response self-documenting.

---

### Q6. Can you use cursor-based pagination for reverse or bidirectional scrolling?

**Interview Answer**

Yes, for reverse scrolling I use `WHERE id < cursor ORDER BY id DESC LIMIT $1` to fetch previous pages. The response includes a `prev_cursor` field from the first item's ID. For chat applications or feeds that need both directions, I maintain both `next_cursor` and `prev_cursor` in each response.

---

### Q7. How do you handle pagination for sorted results that aren't by ID?

**Interview Answer**

When sorting by a non-unique column like `created_at`, I include a tiebreaker like the primary key in the cursor. The cursor becomes a composite value like `created_at,id` so the `WHERE` clause handles ties correctly. Without the tiebreaker, records with identical sort values could be skipped or repeated across pages.

---

### Q8. What response format do you use for paginated results?

**Interview Answer**

I return a JSON object with `data` containing the items array, `next_cursor` as a string or null, `has_more` as a boolean, and optionally `count` for the page size. This format is consistent across all endpoints and makes it easy for frontend teams to build generic pagination components. I also include `Link` headers with `rel="next"` for REST purists.

---
