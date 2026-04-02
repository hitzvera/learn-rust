/**
 * 🔵 Level 2 — Match with Data
 *
 * ⚠️ CHALLENGE MODE: No solutions provided!
 *
 * Goal: Learn to extract data from enum variants using match
 */

// ============================================================================
// Challenge 1: Option Pattern Matching
// ============================================================================

/**
 * Create a function `unwrap_option` that takes an Option<i32> and returns:
 * - The number if Some(n)
 * - -1 if None
 *
 * HINT: Match on the Option enum
 */
pub fn unwrap_option(opt: Option<i32>) -> i32 {
    // YOUR CODE HERE
    todo!()
}

// ============================================================================
// Challenge 2: Result Pattern Matching
// ============================================================================

/**
 * Create a function `handle_result` that takes a Result<i32, String> and returns:
 * - "Success: {value}" if Ok(value)
 * - "Error: {message}" if Err(message)
 *
 * Example:
 * - Ok(42) → "Success: 42"
 * - Err("Not found") → "Error: Not found"
 */
pub fn handle_result(result: Result<i32, String>) -> String {
    // YOUR CODE HERE
    todo!()
}

// ============================================================================
// Challenge 3: Message Enum
// ============================================================================

/**
 * Create an enum `Message` with these variants:
 * - Quit (no data)
 * - Move { x: i32, y: i32 } (struct-style)
 * - Write(String) (tuple-style)
 * - ChangeColor(i32, i32, i32) (RGB)
 *
 * Create a function `describe_message` that returns a description string
 * for each variant.
 *
 * Examples:
 * - Message::Quit → "Quit"
 * - Message::Move { x: 10, y: 20 } → "Move to (10, 20)"
 * - Message::Write(String::from("Hi")) → "Write: Hi"
 * - Message::ChangeColor(255, 0, 0) → "Color: RGB(255, 0, 0)"
 */

// Define the enum here

pub fn describe_message(msg: Message) -> String {
    // YOUR CODE HERE
    todo!()
}

// ============================================================================
// Challenge 4: Nested Pattern Matching
// ============================================================================

/**
 * Create a function `get_nested_value` that takes an Option<Option<i32>>
 * and returns:
 * - The inner value if Some(Some(n))
 * - 0 if Some(None) or None
 *
 * Examples:
 * - Some(Some(5)) → 5
 * - Some(None) → 0
 * - None → 0
 */
pub fn get_nested_value(opt: Option<Option<i32>>) -> i32 {
    // YOUR CODE HERE
    todo!()
}

// ============================================================================
// Challenge 5: Tuple Destructuring
// ============================================================================

/**
 * Create a function `get_first` that takes a tuple (i32, i32, i32)
 * and returns only the first value using pattern matching.
 *
 * HINT: Use _ to ignore values you don't need
 */
pub fn get_first(tuple: (i32, i32, i32)) -> i32 {
    // YOUR CODE HERE
    todo!()
}

// ============================================================================
// Challenge 6: Struct Destructuring
// ============================================================================

/**
 * Create a struct `Person` with fields: name (String), age (u8)
 *
 * Create a function `greet` that takes a Person and returns:
 * - "Hello, {name}!" if age >= 18
 * - "Hi there, {name}!" if age < 18
 *
 * HINT: Destructure the struct in the match pattern
 */

// Define the struct here

pub fn greet(person: Person) -> String {
    // YOUR CODE HERE
    todo!()
}

// ============================================================================
// Challenge 7: Coin Flip Game
// ============================================================================

/**
 * Create an enum `Coin` with variants: Heads, Tails
 *
 * Create a function `flip_coin` that takes a Coin and returns:
 * - "Heads! You win 10 points!" if Heads
 * - "Tails! You win 5 points!" if Tails
 *
 * Then create `flip_and_count` that takes a Coin and current score,
 * and returns the new score after flipping.
 */

// Define the enum here

pub fn flip_coin(coin: Coin) -> &str {
    // YOUR CODE HERE
    todo!()
}

pub fn flip_and_count(coin: Coin, score: i32) -> i32 {
    // YOUR CODE HERE
    todo!()
}

// ============================================================================
// Challenge 8: HTTP Response Handler
// ============================================================================

/**
 * Create an enum `HttpStatus` with variants:
 * - Ok (200)
 * - Created (201)
 * - NotFound (404)
 * - ServerError (500)
 * - Unknown(u16) (for other codes)
 *
 * Create a function `handle_status` that returns a message for each status.
 */

// Define the enum here

pub fn handle_status(status: HttpStatus) -> &str {
    // YOUR CODE HERE
    todo!()
}

// ============================================================================
// Test Runner (DO NOT MODIFY)
// ============================================================================

pub fn run_challenges() {
    println!("=== Level 2: Match with Data ===\n");
    
    println!("Challenge 1: unwrap_option");
    println!("  Some(42) → {}", unwrap_option(Some(42)));
    println!("  None → {}", unwrap_option(None));
    
    println!("\nChallenge 2: handle_result");
    println!("  Ok(100) → {}", handle_result(Ok(100)));
    println!("  Err(\"Failed\") → {}", handle_result(Err(String::from("Failed"))));
    
    println!("\nChallenge 3: describe_message");
    // Test after you define Message
    
    println!("\nChallenge 4: get_nested_value");
    println!("  Some(Some(5)) → {}", get_nested_value(Some(Some(5))));
    println!("  Some(None) → {}", get_nested_value(Some(None)));
    println!("  None → {}", get_nested_value(None));
    
    println!("\nChallenge 5: get_first");
    println!("  (1, 2, 3) → {}", get_first((1, 2, 3)));
    
    println!("\nChallenge 6: greet");
    // Test after you define Person
    
    println!("\nChallenge 7: flip_coin");
    // Test after you define Coin
    
    println!("\nChallenge 8: handle_status");
    // Test after you define HttpStatus
    
    println!("\n✅ Level 2 Complete!\n");
}
