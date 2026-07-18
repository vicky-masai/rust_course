//! # Lifetimes (`'a`) in Rust
//! 
//! ## Interview Question
//! 
//! Explain Lifetimes (`'a`) in Rust.
//! 
//! ## Interview Answer
//! 
//! > "A lifetime in Rust defines how long a reference remains valid. Since Rust allows borrowing without transferring ownership, the compiler must ensure that references never outlive the data they point to.
//! >
//! > Rust performs lifetime analysis during compilation using the borrow checker. In most cases, lifetimes are inferred automatically, but when multiple references are involved, we sometimes provide explicit lifetime annotations to describe the relationship between them.
//! >
//! > Lifetimes don't extend the life of data. They simply help the compiler verify that borrowed references remain valid. This prevents dangling references and provides memory safety without requiring a garbage collector."
//! 
//! ---
//! 
//! ## Follow-up Questions & Answers
//! 
//! ### Q1. What is a lifetime in Rust?
//! 
//! **Interview Answer:**
//! 
//! > "A lifetime defines how long a reference is valid. It ensures a reference never outlives the data it points to."
//! 
//! ---
//! 
//! ### Q2. Why do we need lifetimes?
//! 
//! **Interview Answer:**
//! 
//! > "We need lifetimes to prevent dangling references. They allow the compiler to verify that borrowed data remains valid throughout its use."
//! 
//! ---
//! 
//! ### Q3. Does a lifetime increase the lifetime of an object?
//! 
//! **Interview Answer:**
//! 
//! > "No. Lifetimes don't extend how long an object lives. They only describe how long a reference can safely access that object."
//! 
//! ---
//! 
//! ### Q4. What is a dangling reference?
//! 
//! **Interview Answer:**
//! 
//! > "A dangling reference points to memory that has already been freed. Rust prevents this at compile time using lifetime analysis."
//! 
//! ---
//! 
//! ### Q5. What is Lifetime Elision?
//! 
//! **Interview Answer:**
//! 
//! > "Lifetime Elision is a feature where the Rust compiler automatically infers lifetime annotations in common cases, so developers usually don't need to write them explicitly."
//! 
//! ---
//! 
//! ### Q6. When do you need explicit lifetime annotations?
//! 
//! **Interview Answer:**
//! 
//! > "Explicit lifetime annotations are needed when a function accepts multiple references and returns one of them, so the compiler understands the relationship between the input and output references."
//! 
//! ---
//! 
//! ### Q7. Are lifetimes checked at compile time or runtime?
//! 
//! **Interview Answer:**
//! 
//! > "They are checked entirely at compile time, so there is no runtime overhead."
//! 
//! ---
//! 
//! ### Q8. What happens if a reference outlives its owner?
//! 
//! **Interview Answer:**
//! 
//! > "The code fails to compile because Rust detects that the reference would become invalid."
//! 
//! ---
//! 
//! ### Q9. Why don't we write lifetimes everywhere?
//! 
//! **Interview Answer:**
//! 
//! > "Because Rust uses Lifetime Elision rules to infer them automatically in most common situations."
//! 
//! ---
//! 
//! ### Q10. How are lifetimes useful in backend applications?
//! 
//! **Interview Answer:**
//! 
//! > "Lifetimes allow us to safely borrow large request objects, configuration data, and database models without cloning them. This reduces memory allocations and improves backend performance while maintaining memory safety."

pub const TOPIC: &str = "Lifetimes (`'a`) in Rust";
