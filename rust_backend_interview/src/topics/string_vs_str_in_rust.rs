//! # `String` vs `&str` in Rust
//! 
//! ## Interview Question
//! 
//! Explain `String` vs `&str` in Rust.
//! 
//! ## Interview Answer
//! 
//! > "The main difference is ownership. `String` is an owned, heap-allocated, and growable string type, whereas `&str` is a borrowed string slice that provides read-only access to string data.
//! >
//! > Since `String` owns its data, it can be modified, appended, or passed between functions by transferring ownership. `&str` doesn't own the data; it simply references an existing string, so it's lightweight and efficient for read-only operations.
//! >
//! > In production code, I prefer `&str` whenever I only need to read string data because it avoids unnecessary allocations. I use `String` when I need to own or modify the data."
//! 
//! ---
//! 
//! ## Follow-up Questions & Answers
//! 
//! ### Q1. What is the main difference between `String` and `&str`?
//! 
//! **Interview Answer:**
//! 
//! > "`String` owns its data and is heap allocated, while `&str` is a borrowed string slice that provides read-only access without owning the data."
//! 
//! ---
//! 
//! ### Q2. Which one is mutable?
//! 
//! **Interview Answer:**
//! 
//! > "`String` can be mutable and modified. `&str` is immutable because it's just a borrowed view of string data."
//! 
//! ---
//! 
//! ### Q3. Why is `String` stored on the heap?
//! 
//! **Interview Answer:**
//! 
//! > "Because its size can change at runtime. Heap allocation allows it to grow and shrink dynamically."
//! 
//! ---
//! 
//! ### Q4. Why do most Rust functions accept `&str` instead of `String`?
//! 
//! **Interview Answer:**
//! 
//! > "Because `&str` avoids ownership transfer and heap allocation, making APIs more flexible and efficient."
//! 
//! ---
//! 
//! ### Q5. Can a `String` become a `&str`?
//! 
//! **Interview Answer:**
//! 
//! > "Yes. We can borrow it using `&string` or `string.as_str()`."
//! 
//! ---
//! 
//! ### Q6. Can a `&str` become a `String`?
//! 
//! **Interview Answer:**
//! 
//! > "Yes. Using `to_string()` or `String::from()`, which allocates new heap memory."
//! 
//! ---
//! 
//! ### Q7. Which is faster?
//! 
//! **Interview Answer:**
//! 
//! > "`&str` is generally faster for read-only operations because it doesn't allocate memory. `String` is necessary when ownership or modification is required."
//! 
//! ---
//! 
//! ### Q8. Does a string literal (`"Rust"`) create a `String`?
//! 
//! **Interview Answer:**
//! 
//! > "No. A string literal has the type `&'static str` and is stored in the program's read-only binary."
//! 
//! ---
//! 
//! ### Q9. When do you choose `String` over `&str`?
//! 
//! **Interview Answer:**
//! 
//! > "I use `String` when I need ownership, need to modify the text, or need to return an owned string from a function."
//! 
//! ---
//! 
//! ### Q10. Which one do you use more in backend development?
//! 
//! **Interview Answer:**
//! 
//! > "For function parameters, I usually use `&str` because it's efficient. I use `String` when storing, modifying, or returning text."

pub const TOPIC: &str = "`String` vs `&str` in Rust";
