//! # `PhantomData<T>` in Rust
//! 
//! ## Interview Question
//! 
//! Explain `PhantomData<T>` in Rust.
//! 
//! ## Interview Answer
//! 
//! > "`PhantomData<T>` is a zero-sized marker type used to tell the Rust compiler that a struct logically contains or is associated with a generic type, even if that type isn't stored as a field.
//! >
//! > It influences compile-time checks such as ownership, variance, auto traits like `Send` and `Sync`, and drop checking, but it has no runtime memory cost.
//! >
//! > In practice, `PhantomData` is mainly used when building generic libraries, custom smart pointers, FFI wrappers, iterators, and low-level abstractions. In typical backend applications using Axum and Tokio, it's uncommon to use directly."
//! 
//! ---
//! 
//! ## Follow-up Questions & Answers
//! 
//! ### Q1. What is `PhantomData<T>`?
//! 
//! **Interview Answer:**
//! 
//! > "`PhantomData<T>` is a zero-sized marker type that tells the compiler a type is logically associated with `T`, even if it doesn't store a value of type `T`."
//! 
//! ---
//! 
//! ### Q2. Why do we use `PhantomData`?
//! 
//! **Interview Answer:**
//! 
//! > "To provide compile-time information about ownership, lifetimes, variance, or generic associations without storing additional runtime data."
//! 
//! ---
//! 
//! ### Q3. Does `PhantomData` allocate memory?
//! 
//! **Interview Answer:**
//! 
//! > "No. It's a zero-sized type and has no runtime memory overhead."
//! 
//! ---
//! 
//! ### Q4. Is `PhantomData` used in normal backend development?
//! 
//! **Interview Answer:**
//! 
//! > "Rarely. It's mostly used when building libraries, custom abstractions, FFI wrappers, or advanced generic components."
//! 
//! ---
//! 
//! ### Q5. What problems does `PhantomData` solve?
//! 
//! **Interview Answer:**
//! 
//! > "It helps the compiler correctly reason about ownership, lifetimes, drop checking, and generic relationships when no actual value of the generic type is stored."
//! 
//! ---
//! 
//! ### Q6. What is a Zero-Sized Type (ZST)?
//! 
//! **Interview Answer:**
//! 
//! > "A Zero-Sized Type occupies no memory at runtime. `PhantomData` is a common example."
//! 
//! ---
//! 
//! ### Q7. Can `PhantomData` affect `Send` and `Sync`?
//! 
//! **Interview Answer:**
//! 
//! > "Yes. Because it informs the compiler about the associated type, it can influence automatically derived traits such as `Send` and `Sync`."
//! 
//! ---
//! 
//! ### Q8. Where is `PhantomData` commonly used?
//! 
//! **Interview Answer:**
//! 
//! > "In custom smart pointers, iterators, FFI wrappers, generic libraries, serializers, and low-level systems programming."
//! 
//! ---
//! 
//! ### Q9. Why is `PhantomData` considered a zero-cost abstraction?
//! 
//! **Interview Answer:**
//! 
//! > "Because it provides compile-time type information without adding runtime memory usage or execution overhead."
//! 
//! ---
//! 
//! ### Q10. Have you used `PhantomData` in your backend projects?
//! 
//! **Interview Answer:**
//! 
//! > "Not directly in everyday Axum or Tokio services. Most backend applications don't require it, but I understand its role in generic libraries, FFI, and advanced abstractions where compile-time type information is needed without storing actual values."

pub const TOPIC: &str = "`PhantomData<T>` in Rust";
