//! # `Rc<T>` and `Arc<T>` in Rust
//! 
//! ## Interview Question
//! 
//! Explain `Rc<T>` and `Arc<T>` in Rust.
//! 
//! ## Interview Answer
//! 
//! > "Both `Rc<T>` and `Arc<T>` provide shared ownership of data through reference counting.
//! >
//! > `Rc`, which stands for Reference Counted, is designed for single-threaded applications. It keeps track of how many owners reference the same value and automatically frees the memory when the last owner is dropped.
//! >
//! > `Arc`, which stands for Atomic Reference Counted, provides the same functionality but uses atomic operations, making it safe to share data across multiple threads.
//! >
//! > In backend applications, I rarely use `Rc` because most web servers are multithreaded. Instead, I commonly use `Arc`, often combined with `Mutex` or `RwLock`, to share application state safely across concurrent requests."
//! 
//! ---
//! 
//! ## Follow-up Questions & Answers
//! 
//! ### Q1. What is `Rc<T>`?
//! 
//! **Interview Answer:**
//! 
//! > "`Rc<T>` is a reference-counted smart pointer that enables multiple ownership in a single-threaded environment."
//! 
//! ---
//! 
//! ### Q2. What is `Arc<T>`?
//! 
//! **Interview Answer:**
//! 
//! > "`Arc<T>` is an atomic reference-counted smart pointer that enables multiple ownership across multiple threads."
//! 
//! ---
//! 
//! ### Q3. What is the difference between `Rc` and `Arc`?
//! 
//! **Interview Answer:**
//! 
//! > "`Rc` is not thread-safe because it uses non-atomic reference counting. `Arc` uses atomic operations, making it safe for multithreaded applications."
//! 
//! ---
//! 
//! ### Q4. Why is `Arc` slower than `Rc`?
//! 
//! **Interview Answer:**
//! 
//! > "Because atomic reference counting requires CPU synchronization, which introduces a small performance overhead."
//! 
//! ---
//! 
//! ### Q5. When should you use `Rc`?
//! 
//! **Interview Answer:**
//! 
//! > "When multiple owners are needed in a single-threaded application, such as desktop applications or tree structures."
//! 
//! ---
//! 
//! ### Q6. When should you use `Arc`?
//! 
//! **Interview Answer:**
//! 
//! > "When shared ownership is required across multiple threads, such as web servers, background workers, or async runtimes."
//! 
//! ---
//! 
//! ### Q7. Why do Axum and Tokio commonly use `Arc`?
//! 
//! **Interview Answer:**
//! 
//! > "Because request handlers and async tasks may execute on different threads, shared application state must be thread-safe."
//! 
//! ---
//! 
//! ### Q8. Why is `Arc` often combined with `Mutex`?
//! 
//! **Interview Answer:**
//! 
//! > "`Arc` provides shared ownership, while `Mutex` provides safe mutable access. Together they allow multiple threads to share and modify data safely."
//! 
//! ---
//! 
//! ### Q9. Can `Rc<T>` be used with Tokio?
//! 
//! **Interview Answer:**
//! 
//! > "Generally no, because Tokio tasks may move between threads. `Rc` is not `Send`, so `Arc` is usually the correct choice."
//! 
//! ---
//! 
//! ### Q10. In your backend projects, where would you use `Arc<T>`?
//! 
//! **Interview Answer:**
//! 
//! > "I use `Arc` for shared application state such as database pools, Redis clients, configuration, caches, and services that need to be accessed concurrently by multiple request handlers or background workers."

pub const TOPIC: &str = "`Rc<T>` and `Arc<T>` in Rust";
