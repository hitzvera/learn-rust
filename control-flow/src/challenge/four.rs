/**
 * 🟡 Level 4 — Let Else
 *
 * ⚠️ CHALLENGE MODE: No solutions provided!
 *
 * Goal: Master the `let...else` pattern for early returns
 *
 * NOTE: `let...else` is a newer Rust feature (stabilized in 1.65)
 * It's perfect for "guard" patterns and error handling.
 */

// ============================================================================
// Challenge 1: Basic Let Else
// ============================================================================

/**
 * Create a function `unwrap_or_zero` that takes an Option<i32> and:
 * - Returns the value if Some(n)
 * - Returns 0 if None
 *
 * HINT: Use `let...else` pattern
 *
 * Example structure:
 * let Some(value) = opt else {
 *     return default_value;
 * };
 */
pub fn unwrap_or_zero(opt: Option<i32>) -> i32 {
    let Some(n) = opt else { return 0 };
    n
}

// ============================================================================
// Challenge 2: Let Else with Result
// ============================================================================

/**
 * Create a function `parse_or_error` that takes a Result<i32, String> and:
 * - Returns the value if Ok
 * - Returns -999 if Err (as an error code)
 *
 * HINT: Use `let...else` with Result pattern
 */
pub fn parse_or_error(result: Result<i32, String>) -> i32 {
    let Ok(n) = result else { return -999 };
    n
}

// ============================================================================
// Challenge 3: Chaining Let Else
// ============================================================================

/**
 * Create a function `add_options` that takes two Option<i32> values:
 * - Returns the sum if BOTH are Some
 * - Returns 0 if EITHER is None
 *
 * HINT: Chain two `let...else` statements
 */
pub fn add_options(a: Option<i32>, b: Option<i32>) -> i32 {
    let (Some(x), Some(y)) = (a, b) else { return 0 };
    x + y
}

// ============================================================================
// Challenge 4: Let Else with Nested Options
// ============================================================================

/**
 * Create a function `get_inner` that takes an Option<Option<i32>> and:
 * - Returns the inner value if Some(Some(n))
 * - Returns -1 if Some(None) or None
 *
 * HINT: Use two `let...else` statements
 */
pub fn get_inner(opt: Option<Option<i32>>) -> i32 {
    // YOUR CODE HERE
    let Some(Some(n)) = opt else { return -1 };
    n
}

// ============================================================================
// Challenge 5: Let Else with Struct Fields
// ============================================================================

/**
 * Create a struct `Config` with fields:
 * - host: Option<String>
 * - port: Option<u16>
 * - timeout: Option<u32>
 *
 * Create a function `get_config_info` that:
 * - Returns "Host: {host}, Port: {port}, Timeout: {timeout}" if ALL are Some
 * - Returns "Incomplete config" if ANY is None
 *
 * HINT: Chain three `let...else` statements
 */

// Define the struct here
struct Config {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<u32>,
}

pub fn get_config_info(config: Config) -> String {
    let Some(host) = config.host else {
        return "Incomplete config".to_string();
    };
    let Some(port) = config.port else {
        return "Incomplete config".to_string();
    };
    let Some(timeout) = config.timeout else {
        return "Incomplete config".to_string();
    };

    format!("Host: {host}, Port: {port}, Timeout: {timeout}")
}

// ============================================================================
// Challenge 6: Let Else vs Match
// ============================================================================

/**
 * Create TWO functions:
 *
 * 1. `with_match` - uses match expression
 * 2. `with_let_else` - uses let...else
 *
 * Both should:
 * - Take an Option<String>
 * - Return the string length if Some(s)
 * - Return 0 if None
 *
 * After implementing, consider:
 * - Which feels more natural?
 * - When would you use each?
 */
pub fn with_match(opt: Option<String>) -> usize {
    match opt {
        Some(n) => n.len(),
        _ => 0,
    }
}

pub fn with_let_else(opt: Option<String>) -> usize {
    let Some(n) = opt else { return 0 };
    n.len()
}

// ============================================================================
// Challenge 7: Let Else with Custom Error
// ============================================================================

/**
 * Create a function `validate_age` that takes an Option<i32> and:
 * - Returns Ok(age) if Some(age) and age >= 18
 * - Returns Err("Too young") if Some(age) and age < 18
 * - Returns Err("Age not provided") if None
 *
 * HINT: Use `let...else` for the None case, then `if` for the age check
 */
pub fn validate_age(age_opt: Option<i32>) -> Result<i32, &'static str> {
    let Some(age) = age_opt else {
        return Err("Age not provided");
    };

    if age >= 18 { Ok(age) } else { Err("Too Young") }
}

// ============================================================================
// Challenge 8: Real-World Example - User Authentication
// ============================================================================

/**
 * Create a struct `User` with fields:
 * - id: u32
 * - email: Option<String>
 * - verified: bool
 *
 * Create a function `send_verification_email` that:
 * - Returns Ok("Email sent to {email}") if email is Some AND verified is false
 * - Returns Err("User already verified") if verified is true
 * - Returns Err("No email on file") if email is None
 *
 * HINT: Use `let...else` for the email, then check verified
 */

// Define the struct here
struct User {
    id: u32,
    email: Option<String>,
    verified: bool,
}

pub fn send_verification_email(user: User) -> Result<String, &'static str> {
    let Some(email) = user.email else {
        return Err("No Email on file");
    };

    if user.verified {
        return Err("User already verified");
    }

    Ok(format!("Email sent to {email}"))
}

// ============================================================================
// Challenge 9: Let Else with Method Calls
// ============================================================================

/**
 * Create a function `parse_and_double` that takes an Option<&str> and:
 * - Parses the string to i32
 * - Returns the doubled value if parsing succeeds
 * - Returns 0 if None OR if parsing fails
 *
 * HINT: Use `let...else` twice - once for Option, once for Result
 */
pub fn parse_and_double(opt: Option<&str>) -> i32 {
    let Some(num_string) = opt else { return 0 };
    let Ok(new_num) = num_string.parse::<i32>() else {
        return 0;
    };
    new_num * 2
}

// ============================================================================
// Challenge 10: When NOT to Use Let Else
// ============================================================================

/**
 * Create a function `categorize` that takes an Option<i32> and returns:
 * - "Positive: {n}" if Some(n) and n > 0
 * - "Negative: {n}" if Some(n) and n < 0
 * - "Zero" if Some(0)
 * - "Nothing" if None
 *
 * HINT: This is better with match! Think about why let...else is awkward here.
 */
pub fn categorize(opt: Option<i32>) -> &'static str {
    match opt {
        Some(n) if n > 0 => "Positive",
        Some(n) if n < 0 => "Negative",
        Some(0) => "Zero",
        None => "Nothing",
        _ => "Unknown", // This case should never happen
    }
}

// ============================================================================
// Test Runner (DO NOT MODIFY)
// ============================================================================

pub fn run_challenges() {
    println!("=== Level 4: Let Else ===\n");

    println!("Challenge 1: unwrap_or_zero");
    println!("  Some(42) → {}", unwrap_or_zero(Some(42)));
    println!("  None → {}", unwrap_or_zero(None));

    println!("\nChallenge 2: parse_or_error");
    println!("  Ok(100) → {}", parse_or_error(Ok(100)));
    println!("  Err(_) → {}", parse_or_error(Err(String::from("oops"))));

    println!("\nChallenge 3: add_options");
    println!("  Some(3), Some(7) → {}", add_options(Some(3), Some(7)));
    println!("  Some(3), None → {}", add_options(Some(3), None));

    println!("\nChallenge 4: get_inner");
    println!("  Some(Some(5)) → {}", get_inner(Some(Some(5))));
    println!("  Some(None) → {}", get_inner(Some(None)));
    println!("  None → {}", get_inner(None));

    println!("\nChallenge 5: get_config_info");
    // Test after you define Config

    println!("\nChallenge 6: Let Else vs Match");
    println!(
        "  with_match(Some(String::from(\"Rust\"))) → {}",
        with_match(Some(String::from("Rust")))
    );
    println!(
        "  with_let_else(Some(String::from(\"Rust\"))) → {}",
        with_let_else(Some(String::from("Rust")))
    );

    println!("\nChallenge 7: validate_age");
    println!("  Some(25) → {:?}", validate_age(Some(25)));
    println!("  Some(15) → {:?}", validate_age(Some(15)));
    println!("  None → {:?}", validate_age(None));

    println!("\nChallenge 8: send_verification_email");
    // Test after you define User

    println!("\nChallenge 9: parse_and_double");
    println!("  Some(\"21\") → {}", parse_and_double(Some("21")));
    println!("  Some(\"abc\") → {}", parse_and_double(Some("abc")));
    println!("  None → {}", parse_and_double(None));

    println!("\nChallenge 10: categorize");
    println!("  Some(5) → {}", categorize(Some(5)));
    println!("  Some(-3) → {}", categorize(Some(-3)));
    println!("  Some(0) → {}", categorize(Some(0)));
    println!("  None → {}", categorize(None));

    println!("\n✅ Level 4 Complete!\n");
}
