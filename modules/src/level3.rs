/**
 * 🟠 Level 3 — Paths & Use Keyword
 *
 * ⚠️ CHALLENGE MODE: No solutions provided!
 *
 * Goal: Master absolute/relative paths and the use keyword
 */

// ============================================================================
// Challenge 1: Absolute vs Relative Paths
// ============================================================================

/**
 * TASK:
 * Given this module structure:
 * 
 * mod front_of_house {
 *     pub mod hosting {
 *         pub fn add_to_waitlist() {}
 *     }
 * }
 * 
 * Write TWO function calls to `add_to_waitlist()`:
 * 1. Using an ABSOLUTE path (starting with `crate::`)
 * 2. Using a RELATIVE path (starting from current module)
 * 
 * HINT: Absolute = crate::front_of_house::hosting::add_to_waitlist()
 *       Relative = front_of_house::hosting::add_to_waitlist()
 */

pub fn call_with_absolute_path() {
    // YOUR CODE HERE - use absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
}

pub fn call_with_relative_path() {
    // YOUR CODE HERE - use relative path
    // front_of_house::hosting::add_to_waitlist();
}

// ============================================================================
// Challenge 2: Using self, super, and crate
// ============================================================================

/**
 * QUESTION 1:
 * What does `self` refer to in a path?
 * 
 * A) The parent module
 * B) The current module
 * C) The crate root
 * 
 * Store your answer:
 */
pub const SELF_REFERS_TO: &str = "TODO";

/**
 * QUESTION 2:
 * What does `super` refer to in a path?
 * 
 * A) The parent module
 * B) The current module
 * C) The crate root
 * 
 * Store your answer:
 */
pub const SUPER_REFERS_TO: &str = "TODO";

/**
 * QUESTION 3:
 * What does `crate` refer to in a path?
 * 
 * A) The parent module
 * B) An external crate
 * C) The crate root
 * 
 * Store your answer:
 */
pub const CRATE_REFERS_TO: &str = "TODO";

// ============================================================================
// Challenge 3: Fix the Path Errors
// ============================================================================

/**
 * TASK:
 * The code below has path errors. Fix them using correct paths.
 */

mod back_of_house {
    pub struct Chef {
        pub name: String,
    }
    
    pub fn cook() -> String {
        String::from("Cooking...")
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn seat_customer() {}
    }
}

pub fn run_service() {
    // Fix these paths:
    
    // ERROR 1: This path is wrong - fix it
    // let chef = back_of_house::Chef { name: String::from("Mario") };
    
    // ERROR 2: This path is wrong - fix it  
    // let food = back_of_house::cook();
    
    // ERROR 3: This path is wrong - fix it
    // front_of_house::hosting::seat_customer();
}

// ============================================================================
// Challenge 4: Bringing Items into Scope with `use`
// ============================================================================

/**
 * TASK:
 * Refactor the code below using the `use` keyword to reduce repetition.
 * 
 * Original code (repetitive):
 * 
 * mod garden {
 *     pub struct Plant;
 *     pub struct Tree;
 *     pub struct Flower;
 * }
 * 
 * pub fn create_garden() {
 *     let plant = garden::Plant;
 *     let tree = garden::Tree;
 *     let flower = garden::Flower;
 * }
 * 
 * Refactored requirements:
 * 1. Use `use` to bring all three types into scope
 * 2. Then use them without the `garden::` prefix
 * 
 * BONUS: Use the glob operator (*) to import everything at once
 */

mod garden {
    pub struct Plant;
    pub struct Tree;
    pub struct Flower;
}

pub fn create_garden() {
    // YOUR CODE HERE - use 'use' keyword
    // use crate::garden::{Plant, Tree, Flower};
    
    let plant = Plant;
    let tree = Tree;
    let flower = Flower;
}

// ============================================================================
// Challenge 5: Use with Nested Paths
// ============================================================================

/**
 * TASK:
 * Use nested path syntax to simplify these imports:
 * 
 * Original:
 * use crate::garden::vegetables::Asparagus;
 * use crate::garden::vegetables::Carrot;
 * use crate::garden::fruits::Apple;
 * use crate::garden::fruits::Orange;
 * 
 * Refactored requirements:
 * 1. Group vegetables together
 * 2. Group fruits together
 * 
 * HINT: use crate::garden::vegetables::{Asparagus, Carrot};
 */

mod garden {
    pub mod vegetables {
        pub struct Asparagus;
        pub struct Carrot;
    }
    
    pub mod fruits {
        pub struct Apple;
        pub struct Orange;
    }
}

pub fn create_garden_items() {
    // Add your 'use' statements here
    
    let asparagus = garden::vegetables::Asparagus;
    let carrot = garden::vegetables::Carrot;
    let apple = garden::fruits::Apple;
    let orange = garden::fruits::Orange;
}

// ============================================================================
// Challenge 6: The `as` Keyword (Renaming)
// ============================================================================

/**
 * TASK:
 * Two modules have types with the same name. Use `as` to rename them.
 * 
 * Scenario:
 * - crate::front_of_house::Plate (for serving)
 * - crate::back_of_house::Plate (for cooking)
 * 
 * Requirements:
 * 1. Import both Plate types
 * 2. Rename one of them using `as`
 * 3. Create instances of both
 */

mod front_of_house {
    pub struct Plate;
}

mod back_of_house {
    pub struct Plate;
}

pub fn use_plates() {
    // YOUR CODE HERE - use 'as' to rename
    // use crate::front_of_house::Plate as ServingPlate;
    // use crate::back_of_house::Plate as CookingPlate;
    
    let serving = front_of_house::Plate;
    let cooking = back_of_house::Plate;
}

// ============================================================================
// Challenge 7: Re-exporting with `pub use`
// ============================================================================

/**
 * TASK:
 * Create a public API using `pub use` to re-export internal items.
 * 
 * Scenario:
 * - You have internal module structure: crate::internal::engine::Engine
 * - You want users to access it as: crate::Engine
 * 
 * Requirements:
 * 1. Create an `internal` module with a nested `engine` submodule
 * 2. Put an `Engine` struct inside
 * 3. Use `pub use` to re-export Engine at the crate root
 */

mod internal {
    pub mod engine {
        pub struct Engine;
    }
}

// YOUR CODE HERE - re-export Engine
// pub use crate::internal::engine::Engine;

pub fn create_car() {
    // After re-export, this should work:
    let engine = Engine;
}

// ============================================================================
// Challenge 8: When to Use Absolute vs Relative Paths
// ============================================================================

/**
 * QUESTION:
 * When should you use absolute paths (crate::...) vs relative paths?
 * 
 * Write your answer as a string explaining:
 * - When absolute paths are better
 * - When relative paths are better
 * 
 * HINT: Think about code movement and refactoring
 */

pub const PATH_CHOICE_EXPLANATION: &str = "TODO: Write your explanation here";

// ============================================================================
// Test Runner
// ============================================================================

pub fn run_challenges() {
    println!("=== Level 3: Paths & Use Keyword ===\n");
    
    println!("Challenge 1: Absolute vs Relative Paths");
    println!("  Check: Do both functions compile and work?\n");
    
    println!("Challenge 2: self, super, crate Quiz");
    println!("  Q1 (self): {}", SELF_REFERS_TO);
    println!("  Expected: B\n");
    println!("  Q2 (super): {}", SUPER_REFERS_TO);
    println!("  Expected: A\n");
    println!("  Q3 (crate): {}", CRATE_REFERS_TO);
    println!("  Expected: C\n");
    
    println!("Challenge 3: Fix Path Errors");
    println!("  Check: Do all paths resolve correctly?\n");
    
    println!("Challenge 4: Use Keyword");
    println!("  Check: Can you use types without module prefix?\n");
    
    println!("Challenge 5: Nested Paths");
    println!("  Check: Did you group imports correctly?\n");
    
    println!("Challenge 6: The 'as' Keyword");
    println!("  Check: Can you use both Plate types?\n");
    
    println!("Challenge 7: Re-exporting");
    println!("  Check: Is Engine accessible at crate root?\n");
    
    println!("Challenge 8: Path Choice Explanation");
    println!("  Your answer: {}\n", PATH_CHOICE_EXPLANATION);
    
    // Verify quiz answers
    let mut score = 0;
    if SELF_REFERS_TO == "B" { score += 1; }
    if SUPER_REFERS_TO == "A" { score += 1; }
    if CRATE_REFERS_TO == "C" { score += 1; }
    
    println!("Quiz Score: {}/3", score);
    
    if score == 3 {
        println!("✅ Level 3 Complete!\n");
    } else {
        println!("⚠️ Review path concepts and try again!\n");
    }
}
