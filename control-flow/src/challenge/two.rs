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
    match opt {
        Some(n) => n,
        None => -1,
    }
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
    match result {
        Ok(value) => format!("Success: {}", value),
        Err(message) => format!("Error: {}", message),
    }
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
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn describe_message(msg: Message) -> String {
    // YOUR CODE HERE
    match msg {
        Message::Quit => "Quit".into(),
        Message::Move { x, y } => format!("Move to ({}, {})", x, y),
        Message::Write(content) => format!("Write: {}", content),
        Message::ChangeColor(r, g, b) => format!("Color: RGB({}, {}, {})", r, g, b),
    }
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
    match opt {
        Some(Some(n)) => n,
        _ => 0,
    }
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
    let (first, _, _) = tuple;
    first
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
struct Person {
    name: String,
    age: u8,
}

pub fn greet(person: Person) -> String {
    let Person { name, age } = person;
    if age >= 18 {
        return format!("Hello, {name}!");
    }

    format!("Hi there, {name}!")
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
enum Coin {
    Heads,
    Tails,
}

pub fn flip_coin(coin: Coin) -> &'static str {
    match coin {
        Coin::Tails => "Tails! You win 5 points!",
        Coin::Heads => "Heads! You win 10 points!",
    }
}

pub fn flip_and_count(coin: Coin, score: i32) -> i32 {
    let additional_score = match coin {
        Coin::Heads => 10,
        Coin::Tails => 5,
    };

    score + additional_score
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
enum HttpStatus {
    Ok(u16),
    Created(u16),
    NotFound(u16),
    ServerError(u16),
    Unknown(u16),
}

pub fn handle_status(status: HttpStatus) -> &'static str {
    match status {
        HttpStatus::Created(201) => "Created",
        HttpStatus::Ok(200) => "Ok",
        HttpStatus::NotFound(404) => "Not Found",
        HttpStatus::ServerError(500) => "Server Error",
        _ => "Unknown",
    }
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
    println!(
        "  Err(\"Failed\") → {}",
        handle_result(Err(String::from("Failed")))
    );

    println!("\nChallenge 3: describe_message");
    println!("  Quit → {}", describe_message(Message::Quit));
    println!(
        "  Move {{ x: 10, y: 20 }} → {}",
        describe_message(Message::Move { x: 10, y: 20 })
    );
    println!(
        "  Write(\"Hi\") → {}",
        describe_message(Message::Write(String::from("Hi")))
    );
    println!(
        "  ChangeColor(255, 0, 0) → {}",
        describe_message(Message::ChangeColor(255, 0, 0))
    );

    println!("\nChallenge 4: get_nested_value");
    println!("  Some(Some(5)) → {}", get_nested_value(Some(Some(5))));
    println!("  Some(None) → {}", get_nested_value(Some(None)));
    println!("  None → {}", get_nested_value(None));

    println!("\nChallenge 5: get_first");
    println!("  (1, 2, 3) → {}", get_first((1, 2, 3)));

    println!("\nChallenge 6: greet");
    let adult = Person {
        name: String::from("Alice"),
        age: 25,
    };
    let child = Person {
        name: String::from("Bob"),
        age: 12,
    };
    println!("  Person {{ name: \"Alice\", age: 25 }} → {}", greet(adult));
    println!("  Person {{ name: \"Bob\", age: 12 }} → {}", greet(child));

    println!("\nChallenge 7: flip_coin");
    println!("  Heads → {}", flip_coin(Coin::Heads));
    println!("  Tails → {}", flip_coin(Coin::Tails));
    println!(
        "  flip_and_count(Heads, 20) → {}",
        flip_and_count(Coin::Heads, 20)
    );
    println!(
        "  flip_and_count(Tails, 20) → {}",
        flip_and_count(Coin::Tails, 20)
    );

    println!("\nChallenge 8: handle_status");
    println!("  Ok(200) → {}", handle_status(HttpStatus::Ok(200)));
    println!(
        "  Created(201) → {}",
        handle_status(HttpStatus::Created(201))
    );
    println!(
        "  NotFound(404) → {}",
        handle_status(HttpStatus::NotFound(404))
    );
    println!(
        "  ServerError(500) → {}",
        handle_status(HttpStatus::ServerError(500))
    );
    println!(
        "  Unknown(418) → {}",
        handle_status(HttpStatus::Unknown(418))
    );

    println!("\n✅ Level 2 Complete!\n");
}
