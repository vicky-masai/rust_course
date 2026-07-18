//! # Static Dispatch vs Dynamic Dispatch in Rust
//! 
//! ## Interview Question
//! 
//! Explain Static Dispatch vs Dynamic Dispatch in Rust.
//! 
//! ## Interview Answer
//! 
//! > "Rust supports two forms of polymorphism: static dispatch and dynamic dispatch.
//! >
//! > Static dispatch uses generics and trait bounds. The compiler knows the concrete type at compile time and generates specialized code through monomorphization. This provides maximum performance because method calls are resolved during compilation.
//! >
//! > Dynamic dispatch uses trait objects such as `dyn Trait`. The concrete type is determined at runtime through a vtable lookup. This adds a small runtime overhead but allows different types implementing the same trait to be stored together and selected dynamically.
//! >
//! > In backend applications, I prefer static dispatch for performance-critical paths such as repositories and services. I use dynamic dispatch when plugins, middleware, or runtime-configurable implementations are required."
//! 
//! ---
//! 
//! ## Follow-up Questions & Answers
//! 
//! ### Q1. What is Static Dispatch?
//! 
//! **Interview Answer:**
//! 
//! > "Static dispatch resolves method calls at compile time using generics and trait bounds, providing maximum performance."
//! 
//! ---
//! 
//! ### Q2. What is Dynamic Dispatch?
//! 
//! **Interview Answer:**
//! 
//! > "Dynamic dispatch resolves method calls at runtime using trait objects (`dyn Trait`) and a virtual table."
//! 
//! ---
//! 
//! ### Q3. What is `dyn Trait`?
//! 
//! **Interview Answer:**
//! 
//! > "`dyn Trait` is a trait object that enables dynamic dispatch by allowing different implementations of the same trait to be handled through a common interface."
//! 
//! ---
//! 
//! ### Q4. What is a VTable?
//! 
//! **Interview Answer:**
//! 
//! > "A VTable is a lookup table containing pointers to the methods implemented by a concrete type, allowing runtime method dispatch."
//! 
//! ---
//! 
//! ### Q5. Which is faster: Static or Dynamic Dispatch?
//! 
//! **Interview Answer:**
//! 
//! > "Static dispatch is faster because function calls are resolved at compile time, avoiding runtime vtable lookups."
//! 
//! ---
//! 
//! ### Q6. When should you use Static Dispatch?
//! 
//! **Interview Answer:**
//! 
//! > "For performance-critical code where the concrete type is known at compile time, such as repositories, services, and business logic."
//! 
//! ---
//! 
//! ### Q7. When should you use Dynamic Dispatch?
//! 
//! **Interview Answer:**
//! 
//! > "When the concrete implementation is determined at runtime, such as plugin systems, configurable middleware, or runtime-selected integrations."
//! 
//! ---
//! 
//! ### Q8. Does Dynamic Dispatch have a large performance penalty?
//! 
//! **Interview Answer:**
//! 
//! > "No. The overhead is usually just an additional vtable lookup, which is small but can matter in performance-critical paths."
//! 
//! ---
//! 
//! ### Q9. Can you store different implementations in the same collection?
//! 
//! **Interview Answer:**
//! 
//! > "Yes. Using trait objects like `Vec<Box<dyn Trait>>`, different implementations of the same trait can coexist in a single collection."
//! 
//! ---
//! 
//! ### Q10. Which do you prefer in backend development?
//! 
//! **Interview Answer:**
//! 
//! > "I default to static dispatch because of its performance and compile-time optimizations. I use dynamic dispatch only when runtime flexibility is required, such as plugin architectures or configurable integrations."

pub const TOPIC: &str = "Static Dispatch vs Dynamic Dispatch in Rust";
