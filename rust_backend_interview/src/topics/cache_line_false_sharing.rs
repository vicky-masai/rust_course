//! # cache line false sharing
//! 
//! ## Interview Question
//! 
//! Explain cache line false sharing.
//! 
//! ## Interview Answer
//! 
//! "False sharing occurs when unrelated variables occupy the same CPU cache line. Different cores repeatedly invalidate each other's cache, hurting performance."

pub const TOPIC: &str = "cache line false sharing";
