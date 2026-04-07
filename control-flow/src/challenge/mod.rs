//! # Control Flow Challenge Module
//!
//! This module contains comprehensive exercises for learning Rust control flow patterns:
//! `match`, `if let`, and `let else`.
//!
//! ## 📚 Learning Objectives
//!
//! By completing these exercises, you will understand:
//!
//! - **Match Expressions** - Pattern matching with enums and values
//! - **If Let** - Concise pattern matching for single patterns
//! - **Let Else** - Early return pattern for error handling
//! - **When to Use Each** - Choosing the right tool for the job
//!
//! ## 🎯 Difficulty Levels
//!
//! | Level | Name | Focus | Files |
//! |-------|------|-------|-------|
//! | 🟢 | Level 1 | Match Basics | [`one.rs`] |
//! | 🔵 | Level 2 | Match with Data | [`two.rs`] |
//! | 🟣 | Level 3 | If Let | [`three.rs`] |
//! | 🟡 | Level 4 | Let Else | [`four.rs`] |
//! | 🔴 | Level 5 | Choose the Right Tool | [`five.rs`] |
//!
//! ## 🚀 How to Use
//!
//! **IMPORTANT:** These are CHALLENGES, not tutorials. No solutions provided!
//!
//! ```bash
//! cd control-flow
//!
//! # Run Level 1
//! cargo run --bin control-flow
//!
//! # Edit src/challenge/one.rs and implement the functions
//! ```
//!
//! ## 💡 Key Concepts
//!
//! ### Match Expression
//! ```rust
//! match value {
//!     Pattern1 => action1,
//!     Pattern2 => action2,
//!     _ => default,
//! }
//! ```
//!
//! ### If Let
//! ```rust
//! if let Pattern = value {
//!     // Do something
//! }
//! ```
//!
//! ### Let Else
//! ```rust
//! let Pattern = value else {
//!     return early_value;
//! };
//! ```
//!
//! ## ✅ Completion Checklist
//!
//! - [ ] Complete all Level 1-2 match exercises
//! - [ ] Understand when to use `if let` vs `match`
//! - [ ] Master `let else` for error handling
//! - [ ] Choose the right pattern in Level 5
//!
//! ---
//!
//! **Author:** @hitzvera  
//! **Created:** April 2026  
//! **Rust Edition:** 2024

// Module structure
// pub mod five;
pub mod four;
// pub mod one;
// pub mod three;
// pub mod two;
