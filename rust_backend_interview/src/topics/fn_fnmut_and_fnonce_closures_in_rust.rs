//! # `Fn`, `FnMut`, and `FnOnce` Closures in Rust
//! 
//! ## Interview Question
//! 
//! Explain `Fn`, `FnMut`, and `FnOnce` Closures in Rust.
//! 
//! ## Interview Answer
//! 
//! > "Closures in Rust are anonymous functions that can capture variables from their surrounding environment. Depending on how they capture and use those variables, they implement one or more of the closure traits: `Fn`, `FnMut`, or `FnOnce`.
//! >
//! > `Fn` captures values immutably and can be called multiple times without modifying captured data. `FnMut` captures values mutably, allowing modification while still being callable multiple times. `FnOnce` takes ownership of captured values and may consume them, so it can only be called once.
//! >
//! > In backend applications, closures are commonly used with iterators, async tasks, middleware, callbacks, and thread spawning. Understanding these traits helps avoid ownership and borrowing errors."
//! 
//! ---
//! 
//! ## Follow-up Questions & Answers
//! 
//! ### Q1. What is a closure?
//! 
//! **Interview Answer:**
//! 
//! > "A closure is an anonymous function that can capture variables from its surrounding environment."
//! 
//! ---
//! 
//! ### Q2. What is `Fn`?
//! 
//! **Interview Answer:**
//! 
//! > "`Fn` captures variables by immutable reference and can be called multiple times without modifying the captured values."
//! 
//! ---
//! 
//! ### Q3. What is `FnMut`?
//! 
//! **Interview Answer:**
//! 
//! > "`FnMut` captures variables by mutable reference, allowing the closure to modify captured data while still being callable multiple times."
//! 
//! ---
//! 
//! ### Q4. What is `FnOnce`?
//! 
//! **Interview Answer:**
//! 
//! > "`FnOnce` captures ownership of variables. Since ownership may be consumed, the closure is guaranteed to be callable only once."
//! 
//! ---
//! 
//! ### Q5. What's the difference between `Fn`, `FnMut`, and `FnOnce`?
//! 
//! **Interview Answer:**
//! 
//! > "`Fn` only reads captured data, `FnMut` modifies captured data, and `FnOnce` takes ownership of captured data."
//! 
//! ---
//! 
//! ### Q6. Why does `tokio::spawn()` commonly use `move` closures?
//! 
//! **Interview Answer:**
//! 
//! > "Because spawned tasks may outlive the current function. Moving ownership into the task ensures captured values remain valid for the task's lifetime."
//! 
//! ---
//! 
//! ### Q7. Which closure trait is the most restrictive?
//! 
//! **Interview Answer:**
//! 
//! > "`FnOnce` is the most restrictive because it may consume captured values, preventing repeated calls."
//! 
//! ---
//! 
//! ### Q8. Where are closures commonly used in backend development?
//! 
//! **Interview Answer:**
//! 
//! > "Closures are used extensively in iterators, async tasks, middleware, callbacks, request processing pipelines, and concurrent task execution."
//! 
//! ---
//! 
//! ### Q9. Can a closure implement more than one closure trait?
//! 
//! **Interview Answer:**
//! 
//! > "Yes. Depending on how it captures variables, a closure may implement multiple closure traits. For example, an `Fn` closure also satisfies `FnMut` and `FnOnce`."
//! 
//! ---
//! 
//! ### Q10. How do you use closures in your backend projects?
//! 
//! **Interview Answer:**
//! 
//! > "I use closures with iterator chains, async task spawning, request handlers, middleware, callback registration, retry logic, and functional-style data transformations. Understanding the closure traits helps me choose the correct ownership model and avoid borrowing issues."

pub const TOPIC: &str = "`Fn`, `FnMut`, and `FnOnce` Closures in Rust";
