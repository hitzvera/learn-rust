//! # Enum Challenge Module
//!
//! This module contains comprehensive exercises for learning Rust enums,
//! designed to take you from beginner to advanced understanding through hands-on practice.
//!
//! ## 📚 Learning Objectives
//!
//! By completing these exercises, you will understand:
//!
//! - **Basic Enums** - Defining and using enum variants
//! - **Enum with Data** - Attaching data to variants (tuples, structs)
//! - **Pattern Matching** - Using `match` to handle enum variants
//! - **Option<T>** - Rust's null safety with Option enum
//! - **Result<T, E>** - Error handling with Result enum
//! - **Methods on Enums** - Implementing impl blocks for enums
//!
//! ## 🎯 Difficulty Levels
//!
//! | Level | Name | Focus | Files |
//! |-------|------|-------|-------|
//! | 🟢 | Level 1 | Basic Enums | [`one.rs`] |
//! | 🔵 | Level 2 | Enums with Data | [`two.rs`] |
//! | 🟣 | Level 3 | Pattern Matching | [`three.rs`] |
//! | 🟡 | Level 4 | Option & Result | [`four.rs`] |
//! | 🔴 | Level 5 | Advanced Challenges | [`five.rs`] |
//!
//! ## 🚀 How to Use
//!
//! ### Run Individual Exercises
//!
//! ```bash
//! # Level 1: Basics
//! cargo run --bin enum  # Uncomment challange::one::exercise() in main.rs
//!
//! # Level 5: Advanced Challenges
//! cargo run --bin enum  # Default: runs challange::five::run_tests()
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
//! ### What Makes Enums Special in Rust?
//!
//! Rust enums are **algebraic data types** (ADTs), much more powerful than enums in other languages:
//!
//! ```rust
//! // C-style enum (like other languages)
//! enum Direction {
//!     North,
//!     South,
//!     East,
//!     West,
//! }
//!
//! // Rust superpower: Enums with data!
//! enum Message {
//!     Quit,                       // No data
//!     Move { x: i32, y: i32 },   // Struct-like
//!     Write(String),              // Tuple-like
//!     ChangeColor(i32, i32, i32), // Multiple values
//! }
//! ```
//!
//! ### Pattern Matching with match
//!
//! ```rust
//! let msg = Message::Write(String::from("Hello"));
//!
//! match msg {
//!     Message::Quit => println!("Quit"),
//!     Message::Move { x, y } => println!("Move to ({}, {})", x, y),
//!     Message::Write(text) => println!("Write: {}", text),
//!     Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
//! }
//! ```
//!
//! ### Option<T> - No More Null!
//!
//! ```rust
//! let some_number: Option<i32> = Some(5);
//! let no_number: Option<i32> = None;
//!
//! match some_number {
//!     Some(n) => println!("Number: {}", n),
//!     None => println!("No number"),
//! }
//! ```
//!
//! ## 📖 Additional Resources
//!
//! - [The Rust Book - Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
//! - [The Rust Book - Pattern Matching](https://doc.rust-lang.org/book/ch18-00-patterns.html)
//! - [Rust by Example - Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)
//!
//! ## ✅ Completion Checklist
//!
//! - [ ] Complete all Level 1-3 exercises
//! - [ ] Understand Option and Result in Level 4
//! - [ ] Pass all tests in Level 5
//! - [ ] Create your own enum with methods
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
