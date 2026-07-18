//! # How does the Rust borrow checker work internally?
//! 
//! ## Interview Question
//! 
//! How does the Rust borrow checker work internally?
//! 
//! ## Interview Answer
//! 
//! "The borrow checker builds a control-flow graph and performs lifetime analysis. It ensures at compile time that there is either one mutable reference or multiple immutable references, preventing data races and use-after-free. Modern Rust uses Non-Lexical Lifetimes (NLL), allowing borrows to end earlier than their lexical scope."

pub const TOPIC: &str = "How does the Rust borrow checker work internally?";
