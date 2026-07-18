//! # `unsafe` Rust
//! 
//! ## Interview Question
//! 
//! Explain `unsafe` Rust.
//! 
//! ## Interview Answer
//! 
//! > "Rust is a memory-safe language by default, but some low-level operations cannot be verified by the compiler. For these cases, Rust provides the `unsafe` keyword.
//! >
//! > Unsafe code allows operations such as dereferencing raw pointers, calling unsafe functions, accessing mutable static variables, implementing unsafe traits, and working with unions. The programmer becomes responsible for maintaining Rust's safety guarantees.
//! >
//! > I avoid unsafe in application-level backend code whenever possible. It's mainly used in systems programming, FFI with C libraries, operating systems, high-performance libraries, and low-level runtime implementations. If I need unsafe, I keep it minimal, well-documented, and isolated."
//! 
//! ---
//! 
//! ## Follow-up Questions & Answers
//! 
//! ### Q1. What is `unsafe` in Rust?
//! 
//! **Interview Answer:**
//! 
//! > "`unsafe` allows specific low-level operations that the compiler cannot verify for safety, while still preserving most of Rust's ownership and borrowing rules."
//! 
//! ---
//! 
//! ### Q2. Why does Rust have `unsafe`?
//! 
//! **Interview Answer:**
//! 
//! > "Because certain systems programming tasks, such as FFI, raw pointer manipulation, and runtime implementations, cannot be expressed entirely in safe Rust."
//! 
//! ---
//! 
//! ### Q3. What operations require `unsafe`?
//! 
//! **Interview Answer:**
//! 
//! > "Dereferencing raw pointers, calling unsafe functions, accessing mutable static variables, implementing unsafe traits, and accessing union fields."
//! 
//! ---
//! 
//! ### Q4. Does `unsafe` disable the borrow checker?
//! 
//! **Interview Answer:**
//! 
//! > "No. Ownership, borrowing, and lifetime rules still apply. `unsafe` only permits specific operations that require programmer guarantees."
//! 
//! ---
//! 
//! ### Q5. Is `unsafe` bad?
//! 
//! **Interview Answer:**
//! 
//! > "No. It's an essential feature for low-level programming, but it should be minimized, carefully reviewed, and isolated from the rest of the codebase."
//! 
//! ---
//! 
//! ### Q6. Where is `unsafe` commonly used?
//! 
//! **Interview Answer:**
//! 
//! > "In FFI with C libraries, operating systems, device drivers, memory allocators, async runtimes, and high-performance libraries."
//! 
//! ---
//! 
//! ### Q7. Does Tokio use `unsafe` internally?
//! 
//! **Interview Answer:**
//! 
//! > "Yes. Tokio uses carefully audited `unsafe` code internally for performance and low-level runtime implementation while exposing a safe public API."
//! 
//! ---
//! 
//! ### Q8. Should backend applications use `unsafe`?
//! 
//! **Interview Answer:**
//! 
//! > "Generally very little. Most backend applications rely on safe abstractions provided by the Rust ecosystem."
//! 
//! ---
//! 
//! ### Q9. How do you minimize risks when using `unsafe`?
//! 
//! **Interview Answer:**
//! 
//! > "I keep unsafe blocks as small as possible, document the required safety invariants, and expose safe APIs around the unsafe implementation."
//! 
//! ---
//! 
//! ### Q10. Have you written `unsafe` code in production?
//! 
//! **Interview Answer:**
//! 
//! > "Only when absolutely necessary. In most backend work with Axum, Tokio, sqlx, and PostgreSQL, I rely on safe abstractions because they already encapsulate the required unsafe code internally."

pub const TOPIC: &str = "`unsafe` Rust";
