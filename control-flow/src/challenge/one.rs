/**
 * 🟢 Level 1 — Match Basics
 *
 * ⚠️ CHALLENGE MODE: No solutions provided!
 *
 * Goal: Understand basic match expression syntax
 */

// ============================================================================
// Challenge 1: Basic Match
// ============================================================================

/**
 * Create a function `get_day_name` that takes a number (1-7)
 * and returns the day name as a &str.
 *
 * Examples:
 * - 1 → "Monday"
 * - 2 → "Tuesday"
 * - 7 → "Sunday"
 * - Anything else → "Invalid day"
 *
 * HINT: Use match expression
 */
pub fn get_day_name(day: u8) -> &str {
    // YOUR CODE HERE
    // TODO: Implement using match
    todo!()
}

// ============================================================================
// Challenge 2: Match with Boolean
// ============================================================================

/**
 * Create a function `yes_no` that takes a bool and returns
 * "Yes" for true, "No" for false.
 *
 * HINT: Match on the boolean value
 */
pub fn yes_no(value: bool) -> &str {
    // YOUR CODE HERE
    todo!()
}

// ============================================================================
// Challenge 3: Match with Multiple Patterns
// ============================================================================

/**
 * Create a function `get_grade_message` that takes a grade (A-F)
 * and returns a message.
 *
 * Rules:
 * - A or B → "Great job!"
 * - C or D → "Keep trying!"
 * - F → "See you next semester"
 *
 * HINT: Use | to match multiple patterns
 */
pub fn get_grade_message(grade: char) -> &str {
    // YOUR CODE HERE
    todo!()
}

// ============================================================================
// Challenge 4: Match Guard
// ============================================================================

/**
 * Create a function `classify_number` that takes an i32 and returns:
 * - "Positive and even" if positive AND even
 * - "Positive and odd" if positive AND odd
 * - "Negative" if negative
 * - "Zero" if zero
 *
 * HINT: Use match with if guards (pattern if condition)
 */
pub fn classify_number(num: i32) -> &str {
    // YOUR CODE HERE
    todo!()
}

// ============================================================================
// Challenge 5: Match with Ranges
// ============================================================================

/**
 * Create a function `get_age_group` that takes an age and returns:
 * - 0-12 → "Child"
 * - 13-17 → "Teenager"
 * - 18-64 → "Adult"
 * - 65+ → "Senior"
 *
 * HINT: Use range patterns (start..=end)
 */
pub fn get_age_group(age: u8) -> &str {
    // YOUR CODE HERE
    todo!()
}

// ============================================================================
// Challenge 6: Traffic Light State Machine
// ============================================================================

/**
 * Create an enum `TrafficLight` with variants: Red, Yellow, Green
 *
 * Create a function `get_action` that takes a TrafficLight and returns:
 * - Red → "STOP"
 * - Yellow → "CAUTION"
 * - Green → "GO"
 */

// Define the enum here

pub fn get_action(light: TrafficLight) -> &str {
    // YOUR CODE HERE
    todo!()
}

// ============================================================================
// Challenge 7: Count Vowels
// ============================================================================

/**
 * Create a function `is_vowel` that takes a char and returns true
 * if it's a vowel (a, e, i, o, u - case insensitive).
 *
 * Examples:
 * - 'a' → true
 * - 'E' → true
 * - 'x' → false
 *
 * HINT: Match on the char with multiple patterns
 */
pub fn is_vowel(c: char) -> bool {
    // YOUR CODE HERE
    todo!()
}

// ============================================================================
// Test Runner (DO NOT MODIFY)
// ============================================================================

pub fn run_challenges() {
    println!("=== Level 1: Match Basics ===\n");
    
    println!("Challenge 1: get_day_name");
    println!("  1 → {}", get_day_name(1));
    println!("  7 → {}", get_day_name(7));
    println!("  10 → {}", get_day_name(10));
    
    println!("\nChallenge 2: yes_no");
    println!("  true → {}", yes_no(true));
    println!("  false → {}", yes_no(false));
    
    println!("\nChallenge 3: get_grade_message");
    println!("  A → {}", get_grade_message('A'));
    println!("  C → {}", get_grade_message('C'));
    println!("  F → {}", get_grade_message('F'));
    
    println!("\nChallenge 4: classify_number");
    println!("  4 → {}", classify_number(4));
    println!("  7 → {}", classify_number(7));
    println!("  -3 → {}", classify_number(-3));
    println!("  0 → {}", classify_number(0));
    
    println!("\nChallenge 5: get_age_group");
    println!("  5 → {}", get_age_group(5));
    println!("  15 → {}", get_age_group(15));
    println!("  30 → {}", get_age_group(30));
    println!("  70 → {}", get_age_group(70));
    
    println!("\nChallenge 6: get_action");
    // Test after you define TrafficLight
    
    println!("\nChallenge 7: is_vowel");
    println!("  'a' → {}", is_vowel('a'));
    println!("  'E' → {}", is_vowel('E'));
    println!("  'x' → {}", is_vowel('x'));
    
    println!("\n✅ Level 1 Complete!\n");
}
