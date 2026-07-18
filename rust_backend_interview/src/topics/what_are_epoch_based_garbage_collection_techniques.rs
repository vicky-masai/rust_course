//! # What are epoch-based garbage collection techniques?
//! 
//! ## Interview Question
//! 
//! What are epoch-based garbage collection techniques?
//! 
//! ## Interview Answer
//! 
//! "They defer memory reclamation until all threads have moved past the epoch in which an object became unreachable. This is widely used in lock-free structures."
//! 
//! ---

pub const TOPIC: &str = "What are epoch-based garbage collection techniques?";
