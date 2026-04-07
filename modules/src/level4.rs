/**
 * 🔴 Level 4 — File Separation & Re-exporting
 *
 * ⚠️ CHALLENGE MODE: No solutions provided!
 *
 * Goal: Master separating modules into files and creating clean public APIs
 */

// ============================================================================
// Challenge 1: Understanding File Structure
// ============================================================================

/**
 * QUESTION 1:
 * Given this module declaration in lib.rs:
 * 
 * pub mod garden;
 * 
 * Where does the compiler look for the garden module code?
 * 
 * A) src/garden.rs only
 * B) src/garden/mod.rs only
 * C) src/garden.rs OR src/garden/mod.rs
 * 
 * Store your answer:
 */
pub const MODULE_FILE_LOCATION: &str = "TODO";

/**
 * QUESTION 2:
 * You have this structure:
 * 
 * src/
 * ├── lib.rs
 * └── garden.rs
 * 
 * In garden.rs, you declare: pub mod vegetables;
 * 
 * Where does the compiler look for vegetables?
 * 
 * A) src/vegetables.rs
 * B) src/garden/vegetables.rs or src/garden/vegetables/mod.rs
 * C) src/garden/vegetables.rs only
 * 
 * Store your answer:
 */
pub const SUBMODULE_FILE_LOCATION: &str = "TODO";

// ============================================================================
// Challenge 2: Convert Inline Module to File
// ============================================================================

/**
 * TASK:
 * Convert this inline module structure to separate files.
 * 
 * Current structure (in this file):
 */

pub mod garden {
    pub mod vegetables {
        pub struct Asparagus;
        pub struct Carrot;
        
        pub fn grow_asparagus() -> Asparagus {
            Asparagus
        }
    }
    
    pub mod fruits {
        pub struct Apple;
        pub struct Orange;
    }
}

/**
 * Your task:
 * 1. Create src/garden.rs with the garden module contents
 * 2. Create src/garden/vegetables.rs with vegetables
 * 3. Create src/garden/fruits.rs with fruits
 * 4. Update lib.rs to declare: pub mod garden;
 * 
 * File structure should be:
 * 
 * modules/
 * ├── Cargo.toml
 * └── src/
 *     ├── lib.rs (contains: pub mod garden;)
 *     └── garden/
 *         ├── mod.rs (or garden.rs at src level)
 *         ├── vegetables.rs
 *         └── fruits.rs
 * 
 * HINT: For submodules, create a directory named after the parent module
 */

// After conversion, this should still work:
pub fn test_garden() {
    let asparagus = garden::vegetables::Asparagus;
    let apple = garden::fruits::Apple;
}

// ============================================================================
// Challenge 3: When to Use mod.rs vs Named File
// ============================================================================

/**
 * QUESTION:
 * What's the difference between these two approaches?
 * 
 * Approach A:
 * src/
 * ├── lib.rs
 * └── garden.rs
 * 
 * Approach B:
 * src/
 * ├── lib.rs
 * └── garden/
 *     └── mod.rs
 * 
 * When would you choose Approach B over Approach A?
 * 
 * Write your answer:
 */
pub const MOD_RS_EXPLANATION: &str = "TODO: Explain when to use mod.rs";

// ============================================================================
// Challenge 4: Create a Clean Public API with pub use
// ============================================================================

/**
 * TASK:
 * Create a library with internal structure but clean public API.
 * 
 * Internal structure (should be hidden):
 * - crate::internal::database::Database
 * - crate::internal::cache::Cache
 * - crate::internal::config::Config
 * 
 * Public API (what users see):
 * - crate::Database
 * - crate::Cache
 * - crate::Config
 * 
 * Requirements:
 * 1. Create the internal module structure
 * 2. Use `pub use` to re-export at crate root
 * 3. Users should NOT need to know about the `internal` module
 */

mod internal {
    pub mod database {
        pub struct Database {
            pub connection_string: String,
        }
        
        impl Database {
            pub fn new(conn: &str) -> Self {
                Database {
                    connection_string: conn.to_string(),
                }
            }
        }
    }
    
    pub mod cache {
        pub struct Cache {
            pub size: usize,
        }
        
        impl Cache {
            pub fn new(size: usize) -> Self {
                Cache { size }
            }
        }
    }
    
    pub mod config {
        pub struct Config {
            pub debug: bool,
        }
        
        impl Config {
            pub fn new(debug: bool) -> Self {
                Config { debug }
            }
        }
    }
}

// YOUR CODE HERE - Re-export with pub use
// pub use crate::internal::database::Database;
// pub use crate::internal::cache::Cache;
// pub use crate::internal::config::Config;

pub fn create_app() {
    // After re-export, this clean API should work:
    let db = Database::new("postgres://localhost");
    let cache = Cache::new(1000);
    let config = Config::new(true);
}

// ============================================================================
// Challenge 5: Separating a Large Module
// ============================================================================

/**
 * TASK:
 * You have a large `models` module with many structs. Separate it into files.
 * 
 * Create this structure:
 * 
 * src/
 * ├── lib.rs
 * └── models/
 *     ├── mod.rs (declares submodules + re-exports)
 *     ├── user.rs
 *     ├── post.rs
 *     └── comment.rs
 * 
 * Requirements:
 * 1. Each file contains one struct (User, Post, Comment)
 * 2. models/mod.rs declares all submodules
 * 3. Use `pub use` in mod.rs to re-export all types
 * 4. Users can do: use crate::models::{User, Post, Comment};
 */

// Create the files as described above

// After separation, this should work:
pub fn create_content() {
    // let user = models::User::new("Alice");
    // let post = models::Post::new("Hello!", &user);
    // let comment = models::Comment::new("Nice!", &post);
}

// ============================================================================
// Challenge 6: The Glob Operator (*)
// ============================================================================

/**
 * TASK:
 * Use the glob operator to import all items from a module.
 * 
 * Scenario:
 * - You have a module with many types
 * - You want to import them all at once
 * 
 * Requirements:
 * 1. Create a `prelude` module with 5+ types
 * 2. Import them all using `use crate::prelude::*;`
 * 3. Use all types without prefix
 * 
 * WARNING: Glob imports can cause name conflicts! Be careful.
 */

pub mod prelude {
    pub struct Type1;
    pub struct Type2;
    pub struct Type3;
    pub struct Type4;
    pub struct Type5;
    
    pub fn helper1() {}
    pub fn helper2() {}
}

pub fn use_prelude() {
    // YOUR CODE HERE - use glob operator
    // use crate::prelude::*;
    
    let t1 = Type1;
    let t2 = Type2;
    // ... etc
}

// ============================================================================
// Challenge 7: Handling Name Conflicts
// ============================================================================

/**
 * TASK:
 * Two modules export types with the same name. Handle the conflict.
 * 
 * Scenario:
 * - crate::http::Request (for HTTP requests)
 * - crate::db::Request (for database queries)
 * 
 * Requirements:
 * 1. Import both without conflict
 * 2. Use `as` to rename at least one
 * 3. Create instances of both
 */

pub mod http {
    pub struct Request {
        pub url: String,
    }
}

pub mod db {
    pub struct Request {
        pub query: String,
    }
}

pub fn handle_requests() {
    // YOUR CODE HERE - resolve conflict
    // use crate::http::Request as HttpRequest;
    // use crate::db::Request as DbRequest;
    
    let http_req = http::Request { url: String::from("/") };
    let db_req = db::Request { query: String::from("SELECT *") };
}

// ============================================================================
// Challenge 8: Best Practices for Module Organization
// ============================================================================

/**
 * QUESTION:
 * List 3 best practices for organizing Rust modules.
 * 
 * Consider:
 * - When to split into files
 * - How to name modules
 * - What to make public vs private
 * - How to structure public APIs
 * 
 * Write your answer:
 */
pub const MODULE_BEST_PRACTICES: &str = "TODO: List 3 best practices";

// ============================================================================
// Test Runner
// ============================================================================

pub fn run_challenges() {
    println!("=== Level 4: File Separation & Re-exporting ===\n");
    
    println!("Challenge 1: File Structure Quiz");
    println!("  Q1 (module location): {}", MODULE_FILE_LOCATION);
    println!("  Expected: C\n");
    println!("  Q2 (submodule location): {}", SUBMODULE_FILE_LOCATION);
    println!("  Expected: B\n");
    
    println!("Challenge 2: Convert to Files");
    println!("  Check: Create the file structure and verify it compiles\n");
    
    println!("Challenge 3: mod.rs Explanation");
    println!("  Your answer: {}\n", MOD_RS_EXPLANATION);
    
    println!("Challenge 4: Clean Public API");
    println!("  Check: Can you use Database, Cache, Config without 'internal::'?\n");
    
    println!("Challenge 5: Separate Large Module");
    println!("  Check: Create models/ directory with separate files\n");
    
    println!("Challenge 6: Glob Operator");
    println!("  Check: Does 'use crate::prelude::*;' work?\n");
    
    println!("Challenge 7: Name Conflicts");
    println!("  Check: Can you use both Request types?\n");
    
    println!("Challenge 8: Best Practices");
    println!("  Your answer: {}\n", MODULE_BEST_PRACTICES);
    
    // Verify quiz answers
    let mut score = 0;
    if MODULE_FILE_LOCATION == "C" { score += 1; }
    if SUBMODULE_FILE_LOCATION == "B" { score += 1; }
    
    println!("Quiz Score: {}/2", score);
    
    if score == 2 {
        println!("✅ Level 4 Complete!\n");
    } else {
        println!("⚠️ Review file separation concepts and try again!\n");
    }
}
