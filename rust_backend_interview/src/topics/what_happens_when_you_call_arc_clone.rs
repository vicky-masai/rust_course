//! # What happens when you call `Arc::clone()`?
//! 
//! ## Interview Question
//! 
//! What happens when you call `Arc::clone()`?
//! 
//! ## Interview Answer
//! 
//! "It performs an atomic increment of the reference count. The underlying data is not copied."

pub const TOPIC: &str = "What happens when you call `Arc::clone()`?";
