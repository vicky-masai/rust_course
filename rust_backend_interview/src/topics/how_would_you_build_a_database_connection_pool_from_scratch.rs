//! # How would you build a database connection pool from scratch?
//! 
//! ## Interview Question
//! 
//! How would you build a database connection pool from scratch?
//! 
//! ## Interview Answer
//! 
//! "Maintain reusable connections in a synchronized queue, allocate on demand, return connections after use, enforce maximum pool size, idle timeout, health checks, and waiting queues."

pub const TOPIC: &str = "How would you build a database connection pool from scratch?";
