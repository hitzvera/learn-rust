/**
 * 🟡 Level 2 — Modules & Privacy
 *
 * ⚠️ CHALLENGE MODE: No solutions provided!
 *
 * Goal: Master module declaration and visibility (pub keyword)
 */

// ============================================================================
// Challenge 1: Basic Module Declaration
// ============================================================================

/**
 * TASK:
 * Create a module hierarchy for a restaurant:
 * 
 * crate
 * └── front_of_house
 *     ├── hosting
 *     └── serving
 * 
 * Requirements:
 * 1. Declare the `front_of_house` module inline (with curly braces)
 * 2. Inside it, declare `hosting` and `serving` submodules
 * 3. In `hosting`, add a function `fn add_to_waitlist() {}`
 * 4. In `serving`, add a function `fn take_order() {}`
 * 
 * HINT: Start with this structure:
 * 
 * mod front_of_house {
 *     mod hosting {
 *         fn add_to_waitlist() {}
 *     }
 *     
 *     mod serving {
 *         fn take_order() {}
 *     }
 * }
 */

// YOUR CODE HERE (declare the module structure above)

// ============================================================================
// Challenge 2: Making Modules Public
// ============================================================================

/**
 * TASK:
 * The code below doesn't compile because modules are private by default.
 * Fix it by adding `pub` keywords where needed.
 * 
 * Current code (won't compile):
 * 
 * mod front_of_house {
 *     mod hosting {
 *         fn add_to_waitlist() {}
 *     }
 * }
 * 
 * pub fn eat_at_restaurant() {
 *     crate::front_of_house::hosting::add_to_waitlist();
 * }
 * 
 * fn main() {
 *     eat_at_restaurant();
 * }
 * 
 * Rules:
 * - Make ONLY the minimum items public to make it compile
 * - Think about what needs to be public at each level
 */

// Fix this code:
mod restaurant {
    mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaurant() {
        crate::restaurant::front_of_house::hosting::add_to_waitlist();
    }
}

// ============================================================================
// Challenge 3: Privacy Rules Quiz
// ============================================================================

/**
 * QUESTION 1:
 * If a parent module is public, are its children automatically public?
 * 
 * A) Yes
 * B) No
 * 
 * Store your answer:
 */
pub const PUB_MODULE_CHILDREN: &str = "TODO";

/**
 * QUESTION 2:
 * Can child modules access private items in parent modules?
 * 
 * A) Yes
 * B) No
 * 
 * Store your answer:
 */
pub const CHILD_ACCESS_PARENT: &str = "TODO";

/**
 * QUESTION 3:
 * Can sibling modules access each other's private items?
 * 
 * A) Yes
 * B) No
 * 
 * Store your answer:
 */
pub const SIBLING_ACCESS: &str = "TODO";

// ============================================================================
// Challenge 4: Fix the Privacy Errors
// ============================================================================

/**
 * TASK:
 * The following code has privacy errors. Fix them by adding `pub` keywords.
 * 
 * HINT: You need to make a path of public items from the caller to the target.
 */

mod cafe {
    mod back_of_house {
        struct Chef {
            name: String,
        }
        
        fn cook() {}
    }
    
    mod front_of_house {
        struct Server {
            name: String,
        }
        
        fn serve() {}
    }
}

pub fn run_restaurant() {
    // This should work after you fix the privacy:
    // let chef = cafe::back_of_house::Chef { name: String::from("Mario") };
    // cafe::back_of_house::cook();
}

// ============================================================================
// Challenge 5: Struct Field Visibility
// ============================================================================

/**
 * TASK:
 * Create a `Person` struct with the following visibility:
 * - `name` field: public
 * - `age` field: public
 * - `secret` field: private
 * 
 * Also create:
 * - A public constructor function `pub fn new(name: &str, age: u8, secret: &str) -> Person`
 * - A public method `pub fn get_secret(&self) -> &str` to access the secret
 * 
 * HINT: Struct fields are private by default, even in a public struct!
 */

// YOUR CODE HERE:
// pub struct Person {
//     pub name: String,
//     pub age: u8,
//     secret: String,
// }
//
// impl Person {
//     pub fn new(name: &str, age: u8, secret: &str) -> Self {
//         // YOUR CODE HERE
//     }
//     
//     pub fn get_secret(&self) -> &str {
//         // YOUR CODE HERE
//     }
// }

// ============================================================================
// Challenge 6: Enum Visibility
// ============================================================================

/**
 * TASK:
 * Create a `TrafficLight` enum that is public with public variants.
 * 
 * Requirements:
 * - Enum should be public
 * - All variants (Red, Yellow, Green) should be accessible from outside
 * - Add a method `pub fn message(&self) -> &str` that returns:
 *   - "STOP" for Red
 *   - "CAUTION" for Yellow
 *   - "GO" for Green
 */

// YOUR CODE HERE:
// pub enum TrafficLight {
//     Red,
//     Yellow,
//     Green,
// }
//
// impl TrafficLight {
//     pub fn message(&self) -> &str {
//         // YOUR CODE HERE
//     }
// }

// ============================================================================
// Test Runner
// ============================================================================

pub fn run_challenges() {
    println!("=== Level 2: Modules & Privacy ===\n");
    
    println!("Challenge 1: Basic Module Declaration");
    println!("  Check: Can you access hosting::add_to_waitlist() from eat_at_restaurant?\n");
    
    println!("Challenge 2: Making Modules Public");
    println!("  Check: Does the code compile after adding pub keywords?\n");
    
    println!("Challenge 3: Privacy Rules Quiz");
    println!("  Q1 (pub module children): {}", PUB_MODULE_CHILDREN);
    println!("  Expected: B\n");
    println!("  Q2 (child access parent): {}", CHILD_ACCESS_PARENT);
    println!("  Expected: A\n");
    println!("  Q3 (sibling access): {}", SIBLING_ACCESS);
    println!("  Expected: B\n");
    
    println!("Challenge 4: Fix Privacy Errors");
    println!("  Check: Can you create a Chef and call cook()?\n");
    
    println!("Challenge 5: Struct Field Visibility");
    println!("  Check: Can you access name/age but not secret directly?\n");
    
    println!("Challenge 6: Enum Visibility");
    println!("  Check: Do all variants work from outside the module?\n");
    
    // Verify quiz answers
    let mut score = 0;
    if PUB_MODULE_CHILDREN == "B" { score += 1; }
    if CHILD_ACCESS_PARENT == "A" { score += 1; }
    if SIBLING_ACCESS == "B" { score += 1; }
    
    println!("Quiz Score: {}/3", score);
    
    if score == 3 {
        println!("✅ Level 2 Complete!\n");
    } else {
        println!("⚠️ Review privacy rules and try again!\n");
    }
}
