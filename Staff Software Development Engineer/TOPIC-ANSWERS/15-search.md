# LEVEL 15 — Search Systems

### 0314. Elasticsearch

Distributed search/analytics engine on inverted indexes (Lucene). Full-text, aggregations, near-real-time search. Ops-heavy clusters; mapping design matters.

**Talk track:** *"Elasticsearch is distributed Lucene — search and aggs at scale with careful mapping and cluster ops."*

---

### 0315. OpenSearch

Open-source fork/continuation lineage of Elasticsearch. Similar concepts: indices, shards, mappings, queries. Choose based on license/vendor needs.

**Talk track:** *"OpenSearch is the open distributed search stack — same core ideas as ES."*

---

### 0316. Inverted Index

Map term → list of documents containing it. The heart of full-text search. Forward index is doc → terms; inverted makes term lookup fast.

**Talk track:** *"Inverted indexes answer 'which docs contain this word?' instantly."*

---

### 0317. Tokenization

Split text into tokens (words), often lowercased, stemmed, stop-word filtered. Analyzer chains define search behavior — must match index-time and query-time analysis.

**Talk track:** *"Tokenization decides what 'matching text' even means."*

---

### 0318. Ranking

Score which docs are most relevant (BM25, TF-IDF, learning-to-rank, business boosts). Search quality is ranking quality plus recall.

**Talk track:** *"Ranking turns matches into a useful order — relevance is a product feature."*

---

### 0319. Full Text Search

Query natural language / text fields beyond exact equality: fuzziness, phrases, boolean queries. Different from SQL `LIKE '%foo%'` which doesn't scale.

**Talk track:** *"Full-text search is relevance over analyzed text — not SQL wildcard scans."*
