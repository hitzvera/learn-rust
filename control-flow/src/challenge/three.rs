/**
 * 🟣 Level 3 — If Let
 *
 * ⚠️ CHALLENGE MODE: No solutions provided!
 *
 * Goal: Learn concise pattern matching with `if let`
 */

// ============================================================================
// Challenge 1: Basic If Let
// ============================================================================

/**
 * Create a function `check_some` that takes an Option<i32> and:
 * - Prints "Value: {n}" if Some(n)
 * - Prints "No value" if None
 *
 * HINT: Use `if let` for the Some case, `else` for None
 */
pub fn check_some(opt: Option<i32>) {
    if let Some(n) = opt {
        println!("Value: {n}")
    } else {
        println!("No value")
    }
}

// ============================================================================
// Challenge 2: If Let with Result
// ============================================================================

/**
 * Create a function `try_parse` that takes a Result<i32, String> and:
 * - Returns the value if Ok
 * - Returns -1 if Err (ignore the error message)
 *
 * HINT: Use `if let` for the Ok case
 */
pub fn try_parse(result: Result<i32, String>) -> i32 {
    // YOUR CODE HERE
    if let Ok(n) = result { n } else { -1 }
}

// ============================================================================
// Challenge 3: If Let with Enum
// ============================================================================

/**
 * Create an enum `Event` with variants:
 * - Click { x: i32, y: i32 }
 * - KeyPress(char)
 * - Scroll(i32) (positive = up, negative = down)
 *
 * Create a function `handle_click` that:
 * - Returns "Clicked at ({x}, {y})" if it's a Click event
 * - Returns "Not a click" for other events
 *
 * HINT: Use `if let` to match only the Click variant
 */

// Define the enum here
enum Event {
    Click { x: i32, y: i32 },
    KeyPress(char),
    Scroll(i32),
}

pub fn handle_click(event: Event) -> String {
    if let Event::Click { x, y } = event {
        format!("Clicked at ({x}, {y})")
    } else {
        format!("Not a click")
    }
}

// ============================================================================
// Challenge 4: If Let with Reference
// ============================================================================

/**
 * Create a function `get_name_length` that takes an Option<&str> and:
 * - Returns the length of the string if Some(name)
 * - Returns 0 if None
 *
 * HINT: Use `if let` with pattern matching
 */
pub fn get_name_length(opt: Option<&str>) -> usize {
    if let Some(n) = opt { n.len() } else { 0 }
}

// ============================================================================
// Challenge 5: If Let vs Match
// ============================================================================

/**
 * Create TWO functions that do the same thing:
 *
 * 1. `using_match` - uses match expression
 * 2. `using_if_let` - uses if let
 *
 * Both should:
 * - Take an Option<i32>
 * - Return the value doubled if Some(n)
 * - Return 0 if None
 *
 * After implementing both, think about:
 * - Which is more readable?
 * - When would you use each?
 */
pub fn using_match(opt: Option<i32>) -> i32 {
    match opt {
        Some(n) => n * 2,
        None => 0,
    }
}

pub fn using_if_let(opt: Option<i32>) -> i32 {
    if let Some(n) = opt { n * 2 } else { 0 }
}

// ============================================================================
// Challenge 6: Chaining If Let
// ============================================================================

/**
 * Create a function `process_data` that takes two Option<i32> values:
 * - If BOTH are Some, return their sum
 * - If EITHER is None, return 0
 *
 * HINT: You can chain if let conditions
 */
pub fn process_data(a: Option<i32>, b: Option<i32>) -> i32 {
    if let Some(n) = a {
        if let Some(m) = b { n + m } else { 0 }
    } else {
        0
    }
}

// ============================================================================
// Challenge 7: If Let with Struct
// ============================================================================

/**
 * Create a struct `User` with fields: id (u32), name (Option<String>)
 *
 * Create a function `get_user_name` that:
 * - Returns "User {name}" if the user has a name (Some)
 * - Returns "Anonymous User" if the user has no name (None)
 *
 * HINT: Use if let to check the Option inside the struct
 */

// Define the struct here
struct User {
    id: u32,
    name: Option<String>,
}

pub fn get_user_name(user: User) -> String {
    if let Some(name) = user.name {
        format!("User {name}")
    } else {
        format!("Anonymous User")
    }
}

// ============================================================================
// Challenge 8: When NOT to Use If Let
// ============================================================================

/**
 * Create a function `full_match` that takes an Option<i32> and returns:
 * - "Positive: {n}" if Some(n) and n > 0
 * - "Negative: {n}" if Some(n) and n < 0
 * - "Zero" if Some(0)
 * - "Nothing" if None
 *
 * HINT: This is better with match than if let!
 * Think about why.
 */
pub fn full_match(opt: Option<i32>) -> String {
    match opt {
        Some(n) if n > 0 => format!("Positive: {n}"),
        Some(n) if n < 0 => format!("Negative: {n}"),
        Some(0) => "Zero".to_string(),
        None => "Nothing".to_string(),
        _ => "Error".to_string(),
    }
}

// ============================================================================
// Test Runner (DO NOT MODIFY)
// ============================================================================

pub fn run_challenges() {
    println!("=== Level 3: If Let ===\n");

    println!("Challenge 1: check_some");
    print!("  Some(42) → ");
    check_some(Some(42));
    print!("  None → ");
    check_some(None);

    println!("\nChallenge 2: try_parse");
    println!("  Ok(100) → {}", try_parse(Ok(100)));
    println!("  Err(_) → {}", try_parse(Err(String::from("oops"))));

    println!("\nChallenge 3: handle_click");
    println!(
        "  Click {{ x: 10, y: 20 }} → {}",
        handle_click(Event::Click { x: 10, y: 20 })
    );
    println!("  KeyPress('a') → {}", handle_click(Event::KeyPress('a')));
    println!("  Scroll(5) → {}", handle_click(Event::Scroll(5)));

    println!("\nChallenge 4: get_name_length");
    println!("  Some(\"Rust\") → {}", get_name_length(Some("Rust")));
    println!("  None → {}", get_name_length(None));

    println!("\nChallenge 5: If Let vs Match");
    println!("  using_match(Some(5)) → {}", using_match(Some(5)));
    println!("  using_if_let(Some(5)) → {}", using_if_let(Some(5)));
    println!("  (Both should return 10)");

    println!("\nChallenge 6: process_data");
    println!("  Some(3), Some(7) → {}", process_data(Some(3), Some(7)));
    println!("  Some(3), None → {}", process_data(Some(3), None));

    println!("\nChallenge 7: get_user_name");
    let user_with_name = User {
        id: 1,
        name: Some(String::from("Alice")),
    };
    let user_without_name = User { id: 2, name: None };
    println!(
        "  User {{ id: 1, name: Some(\"Alice\") }} → {}",
        get_user_name(user_with_name)
    );
    println!(
        "  User {{ id: 2, name: None }} → {}",
        get_user_name(user_without_name)
    );

    println!("\nChallenge 8: full_match");
    println!("  Some(5) → {}", full_match(Some(5)));
    println!("  Some(-3) → {}", full_match(Some(-3)));
    println!("  Some(0) → {}", full_match(Some(0)));
    println!("  None → {}", full_match(None));

    println!("\n✅ Level 3 Complete!\n");
}
