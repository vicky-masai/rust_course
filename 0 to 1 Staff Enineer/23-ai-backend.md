# LEVEL 23 — AI Backend Engineering

### 0430. Embeddings

Vectors representing meaning of text/images. Similar meanings ⇒ nearby vectors. Foundation of semantic search and RAG. Dimension and model choice matter; embed consistently (same model for index and query).

**Talk track:** *"Embeddings turn content into vectors so 'similar meaning' becomes distance math."*

---

### 0431. Vector Databases

Stores embeddings with similarity search (ANN indexes: HNSW, IVF). Pinecone, Weaviate, pgvector, Milvus, etc. Filter + vector hybrid queries are common production needs.

**Talk track:** *"Vector DBs retrieve nearest neighbors at scale — metadata filters make them product-ready."*

---

### 0432. RAG

Retrieval-Augmented Generation: fetch relevant docs, stuff into prompt, then generate. Grounds LLMs in your data; reduces some hallucinations; needs chunking, retrieval quality, and citation UX.

**Talk track:** *"RAG is search + LLM — quality is mostly retrieval and chunk design."*

---

### 0433. Model Context Protocol (MCP)

Open protocol for connecting AI assistants to tools/data sources with a standard interface. Lets agents use external systems without one-off integrations per app.

**Talk track:** *"MCP standardizes how models talk to tools and context providers."*

---

### 0434. Prompt Engineering

Crafting instructions that steer model behavior: roles, constraints, examples, output schemas. Version prompts like code; evaluate systematically, don't only vibe-test.

**Talk track:** *"Prompts are product logic — version, test, and constrain outputs."*

---

### 0435. Context Engineering

Broader than prompts: what goes into the window — history, retrieved docs, tool results, system policies — under token limits. Prioritize, compress, and structure context.

**Talk track:** *"Context engineering is packing the right information into a finite window."*

---

### 0436. Streaming LLM Responses

Token-by-token delivery (SSE/WebSocket) for lower time-to-first-token UX. Handle cancel, partial JSON, and backpressure on the client.

**Talk track:** *"Streaming improves perceived latency — design for cancel and partial output."*

---

### 0437. AI Agent Architecture

LLM loops with tools: plan → act → observe → repeat. Needs budgets, guardrails, deterministic fallbacks, and audit logs. Unbounded agents are liability.

**Talk track:** *"Agents are LLMs with tools and loops — constrain steps, tools, and spend."*

---

### 0438. LLM Gateway

Central proxy for model providers: auth, routing, rate limits, logging, PII scrubbing, cost controls, failover between vendors.

**Talk track:** *"An LLM gateway is the API platform layer for model access — policy and observability in one place."*

---

### 0439. Model Routing

Send requests to different models by complexity/cost/latency/modality. Cheap model for classify; strong model for hard reasoning. Needs evaluation harnesses.

**Talk track:** *"Routing picks the right model per request — cost and quality dial."*

---

### 0440. Inference Optimization

Make serving faster/cheaper: quantization, batching, caching prompts/results, speculative decoding, right hardware. Backend concern when you self-host or care about unit economics.

**Talk track:** *"Inference optimization is systems engineering on model serving — batch, quantize, cache, measure."*
