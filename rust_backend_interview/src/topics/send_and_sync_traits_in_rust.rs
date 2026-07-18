//! # `Send` and `Sync` Traits in Rust
//! 
//! ## Interview Question
//! 
//! Explain `Send` and `Sync` Traits in Rust.
//! 
//! ## Interview Answer
//! 
//! > "Send and Sync are marker traits that define thread safety in Rust.
//! >
//! > A type implementing `Send` can safely transfer ownership from one thread to another. A type implementing `Sync` can safely be shared between multiple threads through immutable references.
//! >
//! > Most primitive types and standard collections are both Send and Sync. However, types like `Rc<T>` are neither Send nor Sync because they use non-atomic reference counting. For multithreaded applications, Rust provides `Arc<T>`, which uses atomic operations and is thread-safe.
//! >
//! > In backend applications using Tokio or Axum, understanding Send and Sync is essential because asynchronous tasks often run on different threads."
//! 
//! ---
//! 
//! ## Follow-up Questions & Answers
//! 
//! ### Q1. What is `Send`?
//! 
//! **Interview Answer:**
//! 
//! > "`Send` is a marker trait indicating that ownership of a value can safely move from one thread to another."
//! 
//! ---
//! 
//! ### Q2. What is `Sync`?
//! 
//! **Interview Answer:**
//! 
//! > "`Sync` is a marker trait indicating that immutable references to a value can safely be shared across multiple threads."
//! 
//! ---
//! 
//! ### Q3. What's the difference between `Send` and `Sync`?
//! 
//! **Interview Answer:**
//! 
//! > "`Send` is about transferring ownership between threads, while `Sync` is about safely sharing references between threads."
//! 
//! ---
//! 
//! ### Q4. Why is `Rc<T>` not `Send`?
//! 
//! **Interview Answer:**
//! 
//! > "Because `Rc` uses non-atomic reference counting. Multiple threads updating the reference count simultaneously could cause race conditions."
//! 
//! ---
//! 
//! ### Q5. Why is `Arc<T>` thread-safe?
//! 
//! **Interview Answer:**
//! 
//! > "`Arc` uses atomic reference counting, ensuring updates to the reference count are synchronized across threads."
//! 
//! ---
//! 
//! ### Q6. Is `String` Send?
//! 
//! **Interview Answer:**
//! 
//! > "Yes. `String` owns its data, so ownership can safely move to another thread."
//! 
//! ---
//! 
//! ### Q7. Why is `Mutex<T>` often used with `Arc<T>`?
//! 
//! **Interview Answer:**
//! 
//! > "`Arc` enables shared ownership across threads, while `Mutex` ensures only one thread modifies the shared data at a time."
//! 
//! ---
//! 
//! ### Q8. How are `Send` and `Sync` used in Tokio?
//! 
//! **Interview Answer:**
//! 
//! > "Tokio may move async tasks between worker threads, so captured values often need to implement `Send`. Shared application state is commonly stored in `Arc` and protected with synchronization primitives when mutable."
//! 
//! ---
//! 
//! ### Q9. Are `Send` and `Sync` manually implemented often?
//! 
//! **Interview Answer:**
//! 
//! > "Rarely. Most types receive them automatically when all their fields satisfy the required thread-safety guarantees."
//! 
//! ---
//! 
//! ### Q10. Why are `Send` and `Sync` important in backend development?
//! 
//! **Interview Answer:**
//! 
//! > "They enable safe multithreading, allowing request handlers, background jobs, and shared application state to work correctly without data races."

pub const TOPIC: &str = "`Send` and `Sync` Traits in Rust";
