/**
 * 🔴 Level 5 — Choose the Right Tool
 *
 * ⚠️ CHALLENGE MODE: No solutions provided!
 *
 * Goal: Learn when to use match vs if let vs let...else
 *
 * This is the hardest level - you decide which pattern to use!
 */

// ============================================================================
// Challenge 1: HTTP Request Handler
// ============================================================================

/**
 * Create an enum `HttpMethod` with: GET, POST, PUT, DELETE
 *
 * Create a function `handle_request` that:
 * - Returns "Fetching data" for GET
 * - Returns "Creating resource" for POST
 * - Returns "Updating resource" for PUT
 * - Returns "Deleting resource" for DELETE
 *
 * QUESTION: Which pattern did you use? Why?
 * A: i use match because the option is many it'll become easier to read if i use match instead of let if or let else
 */

// Define the enum here
enum HttpMethod {
    POST,
    PUT,
    GET,
    DELETE,
}

pub fn handle_request(method: HttpMethod) -> &'static str {
    match method {
        HttpMethod::POST => "Creating resource",
        HttpMethod::PUT => "Updating resource",
        HttpMethod::GET => "Fetching data",
        HttpMethod::DELETE => "Deleting resource",
    }
}

// ============================================================================
// Challenge 2: Optional Configuration
// ============================================================================

/**
 * Create a function `get_port` that takes an Option<u16> and:
 * - Returns the port if Some(port)
 * - Returns 8080 as default if None
 *
 * QUESTION: Which pattern is most idiomatic here?
 * A: i'll use let if because it's more concise and straightforward for this simple case of checking if the option is Some or None
 */
pub fn get_port(port: Option<u16>) -> u16 {
    let Some(p) = port else { return 8080 };
    p
}

// ============================================================================
// Challenge 3: Command Parser
// ============================================================================

/**
 * Create an enum `Command` with:
 * - Help (no data)
 * - Run { script: String }
 * - Stop(u32) (process ID)
 * - Unknown(String)
 *
 * Create a function `execute_command` that returns a description string.
 *
 * QUESTION: match, if let, or let...else?
 *
 */

// Define the enum here
enum Command {
    Help,
    Run { script: String },
    Stop(u32),
    Unknown(String),
}

pub fn execute_command(cmd: Command) -> String {
    match cmd {
        Command::Help => "Help".to_string(),
        Command::Run { script } => format!("Run: {script}"),
        Command::Stop(pid) => format!("Stop process {pid}"),
        Command::Unknown(cmd) => format!("Unknown command: {cmd}"),
    }
}

// ============================================================================
// Challenge 4: Safe Division
// ============================================================================

/**
 * Create a function `safe_divide` that takes two Option<i32> values:
 * - Returns Ok(a / b) if both are Some and b != 0
 * - Returns Err("Missing operand") if either is None
 * - Returns Err("Division by zero") if b is 0
 *
 * QUESTION: How do you combine let...else with regular if?
 */
pub fn safe_divide(a: Option<i32>, b: Option<i32>) -> Result<i32, &'static str> {
    let (Some(x), Some(y)) = (a, b) else {
        return Err("Missing operand");
    };
    if y == 0 {
        Err("Division by zero")
    } else {
        Ok(x / y)
    }
}

// ============================================================================
// Challenge 5: First Non-None
// ============================================================================

/**
 * Create a function `first_valid` that takes three Option<i32> values:
 * - Returns the first Some value (checking left to right)
 * - Returns -1 if all are None
 *
 * QUESTION: Chain let...else or use match?
 */
pub fn first_valid(a: Option<i32>, b: Option<i32>, c: Option<i32>) -> i32 {
    match (a, b, c) {
        (Some(val_one), _, _) => val_one,
        (None, Some(val_two), _) => val_two,
        (None, None, Some(val_three)) => val_three,
        _ => -1,
    }
}

// ============================================================================
// Challenge 6: Payment Processor
// ============================================================================

/**
 * Create an enum `PaymentMethod` with:
 * - Cash
 * - Card { last_four: String }
 * - Crypto { wallet: String, amount: f64 }
 *
 * Create a function `process_payment` that:
 * - Returns "Cash payment" for Cash
 * - Returns "Card ending in {last_four}" for Card
 * - Returns "Crypto: {amount} from {wallet}" for Crypto
 *
 * QUESTION: Which pattern handles this best?
 * i use match
 */

// Define the enum here
enum PaymentMethod {
    Cash,
    Card { last_four: String },
    Crypto { wallet: String, amount: f64 },
}

pub fn process_payment(payment: PaymentMethod) -> String {
    match payment {
        PaymentMethod::Cash => "Cash Payment".to_string(),
        PaymentMethod::Card { last_four } => format!("Card ending in {last_four}"),
        PaymentMethod::Crypto { wallet, amount } => format!("Crypto: {amount} from {wallet}"),
    }
}

// ============================================================================
// Challenge 7: Extract and Transform
// ============================================================================

/**
 * Create a function `extract_and_uppercase` that takes an Option<&str> and:
 * - Returns the string in UPPERCASE if Some(s)
 * - Returns "NO DATA" if None
 *
 * QUESTION: if let or let...else or match?
 */
pub fn extract_and_uppercase(opt: Option<&str>) -> String {
    let Some(data) = opt else {
        return "NO DATA".to_string();
    };

    data.to_uppercase()
}

// ============================================================================
// Challenge 8: Multi-Level Unwrapping
// ============================================================================

/**
 * Create a function `deep_extract` that takes Option<Option<Option<i32>>> and:
 * - Returns the innermost value if Some(Some(Some(n)))
 * - Returns 0 if any level is None
 *
 * QUESTION: How many let...else statements do you need?
 */
pub fn deep_extract(opt: Option<Option<Option<i32>>>) -> i32 {
    let Some(Some(Some(value))) = opt else {
        return 0;
    };

    value
}

// ============================================================================
// Challenge 9: Status Code Handler
// ============================================================================

/**
 * Create a function `handle_status` that takes a Result<i32, String> and:
 * - Returns "Success: {code}" if Ok(code) and code < 400
 * - Returns "Error: {code}" if Ok(code) and code >= 400
 * - Returns "Failed: {msg}" if Err(msg)
 *
 * QUESTION: How do you combine let...else with if conditions?
 */
pub fn handle_status(status: Result<i32, String>) -> String {
    match status {
        Ok(status_code) if status_code < 400 => format!("Success: {status_code}"),
        Ok(status_code) => format!("Error: {status_code}"),
        Err(msg) => format!("Failed: {msg}"),
    }
}

// ============================================================================
// Challenge 10: The Ultimate Challenge
// ============================================================================

/**
 * Create a struct `ApiResponse` with:
 * - status: u16
 * - data: Option<String>
 * - error: Option<String>
 *
 * Create a function `format_response` that:
 * - Returns "Data: {data}" if status < 400 AND data is Some
 * - Returns "Error: {error}" if status >= 400 AND error is Some
 * - Returns "Unknown error" if status >= 400 AND error is None
 * - Returns "No data" if status < 400 AND data is None
 *
 * This combines EVERYTHING. Good luck!
 */

// Define the struct here
struct ApiResponse {
    status: u16,
    data: Option<String>,
    error: Option<String>,
}

pub fn format_response(response: ApiResponse) -> String {
    match response {
        ApiResponse {
            status,
            data: Some(data),
            ..
        } if status < 400 => format!("Data: {data}"),
        ApiResponse {
            status,
            error: Some(error),
            ..
        } if status >= 400 => format!("Error: {error}"),
        ApiResponse {
            status,
            error: None,
            ..
        } if status >= 400 => "Unknown error".to_string(),
        ApiResponse {
            status, data: None, ..
        } if status < 400 => "No data".to_string(),
        _ => "Invalid response".to_string(),
    }
}

// ============================================================================
// Test Runner (DO NOT MODIFY)
// ============================================================================

pub fn run_challenges() {
    println!("=== Level 5: Choose the Right Tool ===\n");

    println!("Challenge 1: handle_request");
    println!("  GET → {}", handle_request(HttpMethod::GET));
    println!("  POST → {}", handle_request(HttpMethod::POST));
    println!("  PUT → {}", handle_request(HttpMethod::PUT));
    println!("  DELETE → {}", handle_request(HttpMethod::DELETE));

    println!("\nChallenge 2: get_port");
    println!("  Some(3000) → {}", get_port(Some(3000)));
    println!("  None → {}", get_port(None));

    println!("\nChallenge 3: execute_command");
    println!("  Help → {}", execute_command(Command::Help));
    println!(
        "  Run {{ script: \"build.sh\" }} → {}",
        execute_command(Command::Run {
            script: "build.sh".to_string()
        })
    );
    println!("  Stop(1234) → {}", execute_command(Command::Stop(1234)));
    println!(
        "  Unknown(\"foo\") → {}",
        execute_command(Command::Unknown("foo".to_string()))
    );

    println!("\nChallenge 4: safe_divide");
    println!("  Some(10), Some(2) → {:?}", safe_divide(Some(10), Some(2)));
    println!("  Some(10), Some(0) → {:?}", safe_divide(Some(10), Some(0)));
    println!("  Some(10), None → {:?}", safe_divide(Some(10), None));

    println!("\nChallenge 5: first_valid");
    println!(
        "  None, None, Some(5) → {}",
        first_valid(None, None, Some(5))
    );
    println!("  None, None, None → {}", first_valid(None, None, None));

    println!("\nChallenge 6: process_payment");
    println!("  Cash → {}", process_payment(PaymentMethod::Cash));
    println!(
        "  Card {{ last_four: \"1234\" }} → {}",
        process_payment(PaymentMethod::Card {
            last_four: "1234".to_string()
        })
    );
    println!(
        "  Crypto {{ wallet: \"0xabc\", amount: 1.5 }} → {}",
        process_payment(PaymentMethod::Crypto {
            wallet: "0xabc".to_string(),
            amount: 1.5
        })
    );

    println!("\nChallenge 7: extract_and_uppercase");
    println!("  Some(\"rust\") → {}", extract_and_uppercase(Some("rust")));
    println!("  None → {}", extract_and_uppercase(None));

    println!("\nChallenge 8: deep_extract");
    println!(
        "  Some(Some(Some(42))) → {}",
        deep_extract(Some(Some(Some(42))))
    );
    println!("  Some(None, Some(42)) → {}", deep_extract(Some(None)));

    println!("\nChallenge 9: handle_status");
    println!("  Ok(200) → {}", handle_status(Ok(200)));
    println!("  Ok(404) → {}", handle_status(Ok(404)));
    println!(
        "  Err(\"Timeout\") → {}",
        handle_status(Err(String::from("Timeout")))
    );

    println!("\nChallenge 10: format_response");
    println!(
        "  status: 200, data: Some(\"Hello\") → {}",
        format_response(ApiResponse {
            status: 200,
            data: Some("Hello".to_string()),
            error: None
        })
    );
    println!(
        "  status: 404, error: Some(\"Not found\") → {}",
        format_response(ApiResponse {
            status: 404,
            data: None,
            error: Some("Not found".to_string())
        })
    );
    println!(
        "  status: 500, error: None → {}",
        format_response(ApiResponse {
            status: 500,
            data: None,
            error: None
        })
    );
    println!(
        "  status: 200, data: None → {}",
        format_response(ApiResponse {
            status: 200,
            data: None,
            error: None
        })
    );

    println!("\n🎉 Level 5 Complete!");
    println!("🎊 You've finished ALL control flow challenges!");
    println!("\n💡 Reflection Questions:");
    println!("  1. When do you prefer match over if let?");
    println!("  2. When is let...else the best choice?");
    println!("  3. What patterns feel most natural to you?");
    println!();
}
