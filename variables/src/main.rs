//! # Variables Challenge - Main Entry Point
//!
//! This is the main entry point for the variables challenge exercises.
//! Uncomment the exercise you want to run.
//!
//! ## Quick Start
//!
//! ```bash
//! cargo run --bin variables
//! ```
//!
//! ## Exercise Progress
//!
//! - [x] Level 1: Basics (mutability)
//! - [x] Level 2: Type annotations
//! - [x] Level 3: Constants
//! - [x] Level 4: Shadowing examples
//! - [ ] Level 5: Shadowing challenges (in progress)

mod challenge;

fn main() {
    println!("🦀 Rust Variables Challenge\n");
    println!("Select an exercise to run by uncommenting it in main.rs\n");
    println!("=== Available Exercises ===\n");

    // ========================================
    // Level 1: Basics (Mutability)
    // ========================================
    // 🟢 Learn: let vs let mut
    //
    // Uncomment to run:
    // println!("🟢 Level 1: Basics");
    // challenge::one::exercise();

    // ========================================
    // Level 2: Type Annotations
    // ========================================
    // 🔵 Learn: Explicit types, signed vs unsigned
    //
    // Uncomment to run:
    // println!("🔵 Level 2: Type Annotations");
    // challenge::two::two();

    // ========================================
    // Level 3: Constants
    // ========================================
    // 🟣 Learn: const keyword, compile-time values
    //
    // Uncomment to run:
    // println!("🟣 Level 3: Constants");
    // challenge::three::exercise();

    // ========================================
    // Level 4: Shadowing Examples
    // ========================================
    // 🟡 Learn: Real-world shadowing patterns
    //
    // Uncomment to run:
    // println!("🟡 Level 4: Shadowing Examples");
    // challenge::four::run_examples();

    // ========================================
    // Level 5: Shadowing Challenges
    // ========================================
    // 🔴 Test: Advanced shadowing challenges
    //
    // Default: Runs automatically
    println!("🔴 Level 5: Shadowing Challenges");
    println!("Running tests...\n");
    challenge::five::run_tests();

    // ========================================
    // Next Steps
    // ========================================
    // After completing Level 5:
    // 1. Move to the `ownership` module
    // 2. Read The Rust Book Chapter 4
    // 3. Build your own project!
}
