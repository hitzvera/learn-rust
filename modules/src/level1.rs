/**
 * 🟢 Level 1 — Packages & Crates Basics
 *
 * ⚠️ CHALLENGE MODE: No solutions provided!
 *
 * Goal: Understand the difference between packages, crates, and modules
 */

// ============================================================================
// Challenge 1: Identify Crate Type
// ============================================================================

/**
 * QUESTION 1:
 * You run `cargo new my_project` and see:
 * 
 * my_project/
 * ├── Cargo.toml
 * └── src/
 *     └── main.rs
 * 
 * What type of crate is this?
 * 
 * A) Library crate
 * B) Binary crate
 * C) Both
 * 
 * Store your answer in this constant (change "TODO" to "A", "B", or "C"):
 */
pub const CRATE_TYPE_ANSWER: &str = "TODO";

// ============================================================================
// Challenge 2: Library Crate Root
// ============================================================================

/**
 * QUESTION 2:
 * You run `cargo new my_lib --lib`. What file is created as the crate root?
 * 
 * A) src/main.rs
 * B) src/lib.rs
 * C) src/lib/main.rs
 * 
 * Store your answer:
 */
pub const LIB_CRATE_ROOT_ANSWER: &str = "TODO";

// ============================================================================
// Challenge 3: Package Contents
// ============================================================================

/**
 * QUESTION 3:
 * What can a single package contain?
 * 
 * A) Only one crate (either binary OR library)
 * B) Multiple binary crates and at most one library crate
 * C) Multiple binary crates and multiple library crates
 * 
 * Store your answer:
 */
pub const PACKAGE_CONTENTS_ANSWER: &str = "TODO";

// ============================================================================
// Challenge 4: Create a Binary + Library Package
// ============================================================================

/**
 * TASK:
 * 1. Create a new package that has BOTH a binary and a library
 * 2. The library should have a function `pub fn greet(name: &str) -> String`
 *    that returns "Hello, {name}!"
 * 3. The binary should call this function from the library and print it
 * 
 * HINT: You need both src/main.rs AND src/lib.rs
 * 
 * Steps:
 * 1. Create src/lib.rs with the greet function
 * 2. Modify src/main.rs to use the library and call greet()
 * 3. Run with: cargo run
 * 
 * Expected output: "Hello, Rustacean!"
 */

// In src/lib.rs, implement:
// pub fn greet(name: &str) -> String {
//     // YOUR CODE HERE
// }

// In src/main.rs, implement:
// use modules::greet;
// 
// fn main() {
//     println!("{}", greet("Rustacean"));
// }

// ============================================================================
// Challenge 5: Multiple Binary Crates
// ============================================================================

/**
 * TASK:
 * Create a package with TWO binary crates:
 * 
 * 1. Main binary (src/main.rs) - prints "Main binary running"
 * 2. Secondary binary (src/bin/secondary.rs) - prints "Secondary binary running"
 * 
 * HINT: Place additional binaries in src/bin/ directory
 * 
 * Run them with:
 * - cargo run (runs main binary)
 * - cargo run --bin secondary (runs secondary binary)
 */

// In src/main.rs:
// fn main() {
//     println!("Main binary running");
// }

// In src/bin/secondary.rs:
// fn main() {
//     println!("Secondary binary running");
// }

// ============================================================================
// Test Runner
// ============================================================================

pub fn run_challenges() {
    println!("=== Level 1: Packages & Crates Basics ===\n");
    
    println!("Challenge 1: Crate Type");
    println!("  Your answer: {}", CRATE_TYPE_ANSWER);
    println!("  Expected: B\n");
    
    println!("Challenge 2: Library Crate Root");
    println!("  Your answer: {}", LIB_CRATE_ROOT_ANSWER);
    println!("  Expected: B\n");
    
    println!("Challenge 3: Package Contents");
    println!("  Your answer: {}", PACKAGE_CONTENTS_ANSWER);
    println!("  Expected: B\n");
    
    println!("Challenge 4: Binary + Library Package");
    println!("  Check: Can you run 'cargo run' and see 'Hello, Rustacean!'?\n");
    
    println!("Challenge 5: Multiple Binaries");
    println!("  Check: Can you run 'cargo run --bin secondary'?\n");
    
    // Verify answers
    let mut score = 0;
    if CRATE_TYPE_ANSWER == "B" { score += 1; }
    if LIB_CRATE_ROOT_ANSWER == "B" { score += 1; }
    if PACKAGE_CONTENTS_ANSWER == "B" { score += 1; }
    
    println!("Quiz Score: {}/3", score);
    
    if score == 3 {
        println!("✅ Level 1 Complete!\n");
    } else {
        println!("⚠️ Review the concepts and try again!\n");
    }
}
