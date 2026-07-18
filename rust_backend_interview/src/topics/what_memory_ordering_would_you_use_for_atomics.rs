//! # What memory ordering would you use for atomics?
//! 
//! ## Interview Question
//! 
//! What memory ordering would you use for atomics?
//! 
//! ## Interview Answer
//! 
//! "The strongest ordering is `SeqCst`. Weaker orderings like `Acquire`, `Release`, and `Relaxed` improve performance when stronger guarantees are unnecessary."

pub const TOPIC: &str = "What memory ordering would you use for atomics?";
