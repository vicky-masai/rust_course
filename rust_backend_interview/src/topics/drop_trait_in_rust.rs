//! # `Drop` Trait in Rust
//! 
//! ## Interview Question
//! 
//! Explain `Drop` Trait in Rust.
//! 
//! ## Interview Answer
//! 
//! > "The `Drop` trait allows a type to define custom cleanup logic that runs automatically when a value goes out of scope. Rust uses the RAII (Resource Acquisition Is Initialization) pattern, so resources such as heap memory, files, sockets, and database connections are released deterministically when ownership ends.
//! >
//! > The `drop()` method is called automatically by the compiler. Developers should not call it directly; instead, the `std::mem::drop()` function can be used to explicitly release a value early.
//! >
//! > In backend applications, the Drop trait is essential for automatically releasing database connections, file handles, locks, network sockets, and other resources without requiring a garbage collector."
//! 
//! ---
//! 
//! ## Follow-up Questions & Answers
//! 
//! ### Q1. What is the `Drop` trait?
//! 
//! **Interview Answer:**
//! 
//! > "The `Drop` trait allows a type to define custom cleanup logic that executes automatically when the value goes out of scope."
//! 
//! ---
//! 
//! ### Q2. When is `drop()` called?
//! 
//! **Interview Answer:**
//! 
//! > "It's called automatically by the compiler when the owner of the value goes out of scope or when ownership is explicitly passed to `std::mem::drop()`."
//! 
//! ---
//! 
//! ### Q3. What is RAII?
//! 
//! **Interview Answer:**
//! 
//! > "RAII stands for Resource Acquisition Is Initialization. It ties the lifetime of a resource to the lifetime of its owning object, ensuring deterministic cleanup."
//! 
//! ---
//! 
//! ### Q4. Can you call `drop()` directly?
//! 
//! **Interview Answer:**
//! 
//! > "No. The `Drop::drop()` method cannot be called directly. To release a value early, use `std::mem::drop()`."
//! 
//! ---
//! 
//! ### Q5. In what order are variables dropped?
//! 
//! **Interview Answer:**
//! 
//! > "Variables are dropped in reverse order of their creation within the same scope."
//! 
//! ---
//! 
//! ### Q6. How is `Drop` used in backend applications?
//! 
//! **Interview Answer:**
//! 
//! > "It's used to automatically release database connections, file handles, network sockets, mutex guards, transactions, and other resources."
//! 
//! ---
//! 
//! ### Q7. What happens if a panic occurs?
//! 
//! **Interview Answer:**
//! 
//! > "During stack unwinding, Rust runs `Drop` for values on the stack, ensuring resources are cleaned up. However, if the program is configured to `panic = abort`, destructors are not run because the process terminates immediately."
//! 
//! ---
//! 
//! ### Q8. Can every type implement `Drop`?
//! 
//! **Interview Answer:**
//! 
//! > "Yes, but a type can implement the `Drop` trait only once. Rust automatically invokes it when the value's lifetime ends."
//! 
//! ---
//! 
//! ### Q9. Why is `Drop` important for memory safety?
//! 
//! **Interview Answer:**
//! 
//! > "It guarantees deterministic cleanup without relying on a garbage collector, preventing many resource leaks and ensuring predictable resource management."
//! 
//! ---
//! 
//! ### Q10. How do you use `Drop` in your backend projects?
//! 
//! **Interview Answer:**
//! 
//! > "Most of the time I rely on libraries like `sqlx`, Tokio, and the standard library, which implement `Drop` internally. Database connections, transaction guards, file handles, and mutex guards are automatically cleaned up when they go out of scope, reducing the risk of resource leaks."

pub const TOPIC: &str = "`Drop` Trait in Rust";
