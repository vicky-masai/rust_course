//! # What is a self-referential struct, and why is it difficult?
//! 
//! ## Interview Question
//! 
//! What is a self-referential struct, and why is it difficult?
//! 
//! ## Interview Answer
//! 
//! "A self-referential struct contains pointers to its own fields. Moving the struct invalidates those pointers, so Rust requires `Pin` or specialized crates to make this safe."

pub const TOPIC: &str = "What is a self-referential struct, and why is it difficult?";
