# Input Validation

## Interview Question

How do you implement input validation in a Rust Axum backend to prevent security vulnerabilities?

## Interview Answer

Input validation ensures that data from clients conforms to expected formats, types, and constraints before processing. In a Rust Axum backend, I use Axum extractors (Json, Query, Path) with serde for deserialization and validation, the validator crate for custom validation rules, and type-safe domain models that prevent invalid states. Every API endpoint validates: required fields are present, types are correct (integers where expected), lengths are within bounds, formats match patterns (email, UUID), and business rules are satisfied (age > 18). Validation fails fast — invalid requests are rejected before reaching business logic. Rust's type system provides a strong foundation, but runtime validation is still necessary for external input.

---

## Follow-up Questions & Answers

### Q1. What is the difference between validation and sanitization?

**Interview Answer**

Validation checks whether input meets expected criteria and rejects it if not. Sanitization modifies input to make it safe (e.g., trimming whitespace, encoding HTML entities, removing special characters). Validation is preferred because it preserves intent — a user who submits invalid input should know it is invalid, not have their input silently modified. Sanitization is appropriate for fields where user formatting should be normalized (names, addresses) but not for security-critical fields. In Rust, use the validator crate for validation rules and custom sanitization functions for data normalization.

---

### Q2. How do you use Axum extractors for automatic input validation?

**Interview Answer**

Axum's Json extractor deserializes request bodies using serde, rejecting malformed JSON automatically. For custom validation, derive the validator::Validate trait on request structs and add validation attributes: #[validate(length(min = 1, max = 100))] for strings, #[validate(range(min = 0))] for numbers, #[validate(email)] for emails. In the handler, call validate() before processing: let input = input.validate().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?. This ensures every request is validated before business logic executes. Path and Query extractors provide similar type-safe validation for URL parameters.

---

### Q3. What is deserialization bombing and how do you prevent it?

**Interview Answer**

Deserialization bombing (or billion laughs attack) occurs when an attacker sends deeply nested or recursive JSON/YAML that consumes excessive memory during parsing. In Rust, serde has built-in recursion limits, but you should also set limits on request body size using tower_http::limit::RequestBodyLimitLayer. Limit JSON nesting depth, string lengths, and array sizes. For example, set a 1MB body limit: RequestBodyLimitLayer::new(1024 * 1024). Additionally, use timeout middleware to prevent slow loris attacks where an attacker sends data slowly to tie up resources.

---

### Q4. How do you validate email addresses and other format-specific fields?

**Interview Answer**

For email validation, use the validator crate's #[validate(email)] attribute which validates RFC 5322 compliance. For other formats: UUIDs with uuid::Uuid::parse_str(), URLs with url::Url::parse(), dates with chrono::NaiveDate::parse_from_str(), phone numbers with regex patterns. Never implement format validation manually — use well-tested libraries. For custom formats, use #[validate(regex(path = "MY_REGEX"))] with a lazy_static pattern. Example: #[validate(regex(path = " PHONE_REGEX ", message = "Invalid phone number"))]. Always validate at the API boundary, not deep in the business logic.

---

### Q5. How do you prevent NoSQL injection in non-relational databases?

**Interview Answer**

NoSQL injection occurs when user input is used to construct MongoDB queries or similar. For example, passing {"$gt": ""} as a username could match all users. In Rust with MongoDB, use typed queries instead of raw BSON: use bson::doc! with properly typed values, and validate input types before querying. The same principle as SQL injection applies: never construct queries from raw user input. Validate that input matches expected types (string, not object) before using it in queries. Use the mongodb crate's typed collection API rather than raw command execution.

---

### Q6. What is the role of type safety in input validation?

**Interview Answer**

Rust's type system provides compile-time validation that eliminates entire classes of bugs. Use newtypes to represent validated data: Email(String) where the constructor validates the format, PositiveInt(u64) where the constructor ensures > 0. This creates validated domains that can only contain valid data. In Axum, extractors can return validated types, so handlers only receive valid data. Example: struct ValidEmail(String); impl ValidEmail { fn new(s: String) -> Result<Self, ValidationError> { /* validate */ } }. This moves validation to the type system boundary.

---

### Q7. How do you handle validation errors in a user-friendly way?

**Interview Answer**

Return structured error responses that identify which fields failed validation and why. Use the serde_json format: {"errors": {"email": ["Invalid email format"], "age": ["Must be at least 18"]}}. The validator crate provides detailed error messages. In Axum, implement a custom IntoResponse for validation errors that formats them consistently. Use HTTP 400 Bad Request for validation failures. Log validation errors for monitoring but do not expose internal implementation details. For API clients, provide a machine-readable error format; for web interfaces, map errors to user-friendly messages.

---

### Q8. How do you validate file uploads securely?

**Interview Answer**

Validate: file size (set maximum), file type (check MIME type AND magic bytes, not just extension), file content (scan for malware if applicable), and filename (sanitize to prevent path traversal). In Axum, use Multipart to receive uploads, validate each part: check Content-Type, validate file size before reading into memory, use the file_magic crate to verify actual file type, and strip path components from filenames. Never trust the client-provided filename or content type — validate both. Store uploads outside the web root to prevent direct access. Limit concurrent uploads to prevent resource exhaustion.

---

### Q9. How do you validate and sanitize user-generated content (HTML, Markdown)?

**Interview Answer**

Never trust user-generated content — always sanitize before rendering. For HTML, use the ammonia crate (Rust port of Google's DOMPurify) to strip dangerous tags and attributes: ammonia::clean(user_html). For Markdown, use pulldown-cmark for parsing and ammonia for sanitizing the rendered HTML. Whitelist allowed tags and attributes rather than blacklisting dangerous ones. Strip JavaScript event handlers (onclick, onerror), protocol handlers (javascript:, data:), and form elements. For user content displayed to other users, sanitize on output, not input — this preserves the original content while preventing XSS on display.

---

### Q10. How do you implement input validation in a microservices architecture?

**Interview Answer**

Validate input at the API gateway level (first line of defense) and again at each service boundary (defense in depth). The gateway handles basic format validation, while services handle business-specific validation. Use a shared validation library (crate) that all services import to ensure consistent rules. For inter-service communication, validate inputs from other services — never assume upstream validation is complete. In Rust, create a common crate with validated request/response types that all services depend on. Use contract testing (Pact, etc.) to verify that services agree on input formats.
