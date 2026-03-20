//! # Variables Challenge Module
//!
//! This module contains a comprehensive set of exercises for learning Rust variables,
//! designed to take you from beginner to advanced understanding through hands-on practice.
//!
//! ## 📚 Learning Objectives
//!
//! By completing these exercises, you will understand:
//!
//! - **Immutability vs Mutability** - When to use `let` vs `let mut`
//! - **Type Annotations** - Explicit typing and type inference
//! - **Constants** - Using `const` for compile-time values
//! - **Variable Shadowing** - Transforming data through shadowing (not mutation!)
//! - **Type Conversions** - Safe casting and parsing
//!
//! ## 🎯 Difficulty Levels
//!
//! | Level | Name | Focus | Files |
//! |-------|------|-------|-------|
//! | 🟢 | Level 1 | Basics | [`one.rs`] |
//! | 🔵 | Level 2 | Type Annotations | [`two.rs`] |
//! | 🟣 | Level 3 | Constants | [`three.rs`] |
//! | 🟡 | Level 4 | Shadowing Examples | [`four.rs`] |
//! | 🔴 | Level 5 | Shadowing Challenges | [`five.rs`] |
//!
//! ## 🚀 How to Use
//!
//! ### Run Individual Exercises
//!
//! ```bash
//! # Level 1: Basics
//! cargo run --bin variables  # Uncomment challange::one::exercise() in main.rs
//!
//! # Level 5: Advanced Challenges
//! cargo run --bin variables  # Default: runs challange::five::run_tests()
//! ```
//!
//! ### Run Tests
//!
//! ```bash
//! cargo test
//! ```
//!
//! ## 💡 Key Concepts
//!
//! ### Variable Shadowing vs Mutation
//!
//! Rust encourages **shadowing** over mutation when possible:
//!
//! ```rust
//! // ❌ Mutation (not always necessary)
//! let mut x = 5;
//! x = x + 1;
//!
//! // ✅ Shadowing (preferred)
//! let x = 5;
//! let x = x + 1;
//! ```
//!
//! **Why shadowing?**
//! - Enables type transformations
//! - Makes data flow explicit
//! - Prevents accidental mutations
//! - More idiomatic Rust
//!
//! ### When to Use Each
//!
//! | Use Shadowing When | Use Mutation When |
//! |--------------------|-------------------|
//! | Transforming data types | Modifying in place |
//! | Data pipeline steps | Performance-critical loops |
//! | Configuration building | State machines |
//! | Parsing and validation | Buffer manipulation |
//!
//! ## 📖 Additional Resources
//!
//! - [The Rust Book - Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
//! - [Rust by Example - Variables](https://doc.rust-lang.org/rust-by-example/variable_bindings.html)
//! - [Rust Edition Guide 2024](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)
//!
//! ## ✅ Completion Checklist
//!
//! - [ ] Complete all Level 1-3 exercises
//! - [ ] Understand shadowing examples in Level 4
//! - [ ] Pass all tests in Level 5
//! - [ ] Write your own shadowing example
//!
//! ---
//!
//! **Author:** @hitzvera  
//! **Created:** March 2026  
//! **Rust Edition:** 2024

// Module structure
pub mod five;
pub mod four;
pub mod one;
pub mod three;
pub mod two;
