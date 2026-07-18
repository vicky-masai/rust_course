//! # Move vs Copy vs Clone in Rust
//! 
//! ## Interview Question
//! 
//! Explain Move vs Copy vs Clone in Rust.
//! 
//! ## Interview Answer
//! 
//! > "Move, Copy, and Clone define how values are assigned in Rust.
//! >
//! > Move transfers ownership from one variable to another, making the original variable invalid. It's used for types that own heap memory, such as `String` and `Vec`.
//! >
//! > Copy creates an automatic bitwise copy of fixed-size types like integers, booleans, and characters. Both variables remain valid after the assignment.
//! >
//! > Clone performs an explicit deep copy by allocating new memory and copying the data. Since cloning is more expensive, I use it only when I need an independent copy of the data."
//! 
//! ---
//! 
//! ## Follow-up Questions & Answers
//! 
//! ### Q1. Why does `String` use Move instead of Copy?
//! 
//! **Interview Answer:**
//! 
//! > "Because `String` owns heap memory. Automatically copying it would duplicate heap allocations and could lead to double-free issues. Rust transfers ownership instead."
//! 
//! ---
//! 
//! ### Q2. Which types implement the `Copy` trait?
//! 
//! **Interview Answer:**
//! 
//! > "Primitive fixed-size types such as integers, booleans, characters, floating-point numbers, references, and tuples whose elements are all `Copy`."
//! 
//! ---
//! 
//! ### Q3. Why isn't `String` a `Copy` type?
//! 
//! **Interview Answer:**
//! 
//! > "`String` manages heap memory. Making it `Copy` would require implicit heap duplication, which is expensive and unsafe."
//! 
//! ---
//! 
//! ### Q4. When should you use `.clone()`?
//! 
//! **Interview Answer:**
//! 
//! > "Only when an independent copy of the data is required. If I only need temporary access, I prefer borrowing to avoid unnecessary allocations."
//! 
//! ---
//! 
//! ### Q5. Is `Clone` a shallow copy or a deep copy?
//! 
//! **Interview Answer:**
//! 
//! > "For types like `String` and `Vec`, `Clone` performs a deep copy by allocating new memory and copying the contents."
//! 
//! ---
//! 
//! ### Q6. Which operation is the fastest?
//! 
//! **Interview Answer:**
//! 
//! > "`Copy` and `Move` are generally very fast because they don't allocate heap memory. `Clone` is slower because it allocates memory and copies data."
//! 
//! ---
//! 
//! ### Q7. Can a type implement both `Copy` and `Drop`?
//! 
//! **Interview Answer:**
//! 
//! > "No. Types implementing `Copy` cannot implement `Drop` because copied values don't have unique ownership."
//! 
//! ---
//! 
//! ### Q8. How does Move improve performance?
//! 
//! **Interview Answer:**
//! 
//! > "Move transfers ownership without copying heap data, making assignments O(1) instead of O(n)."
//! 
//! ---
//! 
//! ### Q9. How do you reduce unnecessary cloning in backend applications?
//! 
//! **Interview Answer:**
//! 
//! > "I pass references using borrowing whenever possible and clone only when independent ownership is required."
//! 
//! ---
//! 
//! ### Q10. Which operation do you use most in production Rust code?
//! 
//! **Interview Answer:**
//! 
//! > "Borrowing is used most frequently. Moves happen naturally when ownership changes, and cloning is used sparingly because of its performance cost."

pub const TOPIC: &str = "Move vs Copy vs Clone in Rust";
