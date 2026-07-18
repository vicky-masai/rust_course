//! # `Cow<T>` (Copy-on-Write) in Rust
//! 
//! ## Interview Question
//! 
//! Explain `Cow<T>` (Copy-on-Write) in Rust.
//! 
//! ## Interview Answer
//! 
//! > "Cow, or Copy-on-Write, is a smart pointer that can either borrow data or own it. Initially it borrows existing data without allocating new memory. If the data needs to be modified, Cow automatically clones it and switches to owned storage.
//! >
//! > This optimization reduces unnecessary allocations and copies, especially when most data is read-only. In backend applications, Cow is useful for string processing, configuration loading, HTTP headers, serialization, and APIs where data is usually borrowed but may occasionally require modification."
//! 
//! ---
//! 
//! ## Follow-up Questions & Answers
//! 
//! ### Q1. What is `Cow<T>`?
//! 
//! **Interview Answer:**
//! 
//! > "`Cow<T>` is a smart pointer implementing Copy-on-Write. It borrows data when possible and clones it only when modification becomes necessary."
//! 
//! ---
//! 
//! ### Q2. Why do we use `Cow`?
//! 
//! **Interview Answer:**
//! 
//! > "To reduce unnecessary allocations and copies while still allowing mutation when required."
//! 
//! ---
//! 
//! ### Q3. What does Copy-on-Write mean?
//! 
//! **Interview Answer:**
//! 
//! > "It means data is shared in a borrowed form until a write operation occurs. At that point, a private owned copy is created."
//! 
//! ---
//! 
//! ### Q4. What are the two variants of `Cow`?
//! 
//! **Interview Answer:**
//! 
//! > "`Cow` has `Borrowed` and `Owned` variants."
//! 
//! ---
//! 
//! ### Q5. When does `Cow` allocate memory?
//! 
//! **Interview Answer:**
//! 
//! > "Only when the borrowed data needs to be modified or an owned value is created directly."
//! 
//! ---
//! 
//! ### Q6. Is `Cow` always faster?
//! 
//! **Interview Answer:**
//! 
//! > "It's beneficial when most data is read-only. If nearly every value is modified, the cloning overhead may outweigh its advantages."
//! 
//! ---
//! 
//! ### Q7. Where is `Cow` commonly used?
//! 
//! **Interview Answer:**
//! 
//! > "String processing, HTTP headers, serialization, parsers, configuration management, and APIs that may return either borrowed or owned data."
//! 
//! ---
//! 
//! ### Q8. What's the difference between `Cow<str>` and `String`?
//! 
//! **Interview Answer:**
//! 
//! > "`String` always owns its data, while `Cow<str>` can either borrow an existing string or own one, depending on the situation."
//! 
//! ---
//! 
//! ### Q9. Does `Cow` improve backend performance?
//! 
//! **Interview Answer:**
//! 
//! > "Yes, particularly in read-heavy workloads where avoiding unnecessary allocations reduces memory usage and CPU overhead."
//! 
//! ---
//! 
//! ### Q10. Have you used `Cow` in production?
//! 
//! **Interview Answer:**
//! 
//! > "I've used it in scenarios involving string processing and APIs where data is usually read without modification. It's particularly useful for optimizing allocations while preserving a clean API."

pub const TOPIC: &str = "`Cow<T>` (Copy-on-Write) in Rust";
