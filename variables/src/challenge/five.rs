/**
 * 🔴 Level 5 — Shadowing Challenge
 *
 * YOUR TASK: Implement the functions below using variable shadowing.
 * Each function should use shadowing to transform data through multiple steps.
 *
 * RULES:
 * 1. Use variable shadowing (let x = ...; let x = ...)
 * 2. Each step should transform the data in some way
 * 3. Do NOT use mutation (no let mut)
 * 4. Make each shadowing step meaningful
 *
 * TEST YOUR SOLUTION: Run `cargo run` to see if all tests pass
 */

/// Challenge 1: Price Calculator
///
/// Calculate the final price of an item after applying:
/// 1. Base price (input)
/// 2. Apply discount percentage (e.g., 20% off)
/// 3. Add tax percentage (e.g., 8% tax)
/// 4. Convert to cents (integer) for payment processing
/// 5. Format as currency string (e.g., "$123.45")
///
/// HINT: Use shadowing to transform:
/// f64 (base price) → f64 (discounted) → f64 (with tax) → i64 (cents) → String (formatted)
///
/// Example: calculate_price(100.0, 20.0, 8.0) should return "$86.40"
/// - $100.00 base
/// - $80.00 after 20% discount
/// - $86.40 after 8% tax
/// - 8640 cents
/// - "$86.40" formatted
pub fn calculate_price(base_price: f64, discount_percent: f64, tax_percent: f64) -> String {
    // TODO: Implement using shadowing
    // Step 1: Apply discount
    // Step 2: Add tax
    // Step 3: Convert to cents (round to nearest cent)
    // Step 4: Format as currency string

    let price = base_price - (base_price * (discount_percent / 100.0));

    let price = price + (price * (tax_percent / 100.0));

    let price = (price * 100.0).round() as i64;

    let price = format!("${}.{:02}", price / 100, price % 100);

    price
}

/// Challenge 2: Username Normalizer
///
/// Normalize a username for a social media platform:
/// 1. Trim whitespace
/// 2. Convert to lowercase
/// 3. Replace spaces with underscores
/// 4. Remove any non-alphanumeric characters (except underscores)
/// 5. Truncate to max 20 characters
/// 6. If empty after sanitization, return "user_" + timestamp
///
/// HINT: Use shadowing to transform through each step
/// &str → &str → String → String → String → String → String
///
/// Example: normalize_username("  John Doe!!!  ") should return "john_doe"
pub fn normalize_username(input: &str) -> String {
    // TODO: Implement using shadowing
    // Step 1: Trim
    // Step 2: Lowercase
    // Step 3: Replace spaces with underscores
    // Step 4: Remove special characters
    // Step 5: Truncate to 20 chars
    // Step 6: Handle empty case

    let username = input.trim();

    let username = username.to_lowercase();

    let username = username.replace(' ', "_");

    let username: String = username
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect();

    let username: String = if username.len() > 20 {
        username[..20].to_string()
    } else {
        username
    };

    let username = if username.is_empty() {
        "user_".to_string()
    } else {
        username
    };

    username
}

/// Challenge 3: Score Normalizer
///
/// Normalize a game score to a 0-100 scale:
/// 1. Parse the raw score string (could be "85", "85.5", or invalid)
/// 2. Clamp to valid range 0-150 (max possible raw score)
/// 3. Apply difficulty multiplier (e.g., 1.2 for hard mode)
/// 4. Convert to percentage of max possible (150)
/// 5. Round to nearest integer
/// 6. Return as Result<u8, String> (u8 because 0-100 fits)
///
/// HINT: Use shadowing to handle the transformation pipeline
/// &str → f64 → f64 → f64 → f64 → u8
///
/// Example: normalize_score("85.5", 1.2) should return Ok(68)
/// - 85.5 parsed
/// - 85.5 clamped (already valid)
/// - 102.6 after difficulty multiplier
/// - 68.4 as percentage of 150
/// - 68 rounded
pub fn normalize_score(raw_score: &str, difficulty_multiplier: f64) -> Result<u8, String> {
    // TODO: Implement using shadowing
    // Step 1: Parse string to f64
    // Step 2: Clamp to 0-150
    // Step 3: Apply difficulty multiplier
    // Step 4: Convert to percentage (score / 150 * 100)
    // Step 5: Round to nearest integer
    // Step 6: Convert to u8

    let score: f64 = raw_score
        .parse()
        .map_err(|_| String::from("Invalid score format"))?;

    let score = score.clamp(0.0, 150.0);

    let score = score * difficulty_multiplier;

    let score = (score / 150.0) * 100.0;

    let score = score.round() as u8;

    Ok(score)
}

/// Challenge 4: Config Builder
///
/// Build a database configuration from environment:
/// 1. Start with default config
/// 2. Override with config file values (if they exist)
/// 3. Override with environment variables (if set)
/// 4. Validate: port must be 1024-65535, host must not be empty
/// 5. Return validated config or error message
///
/// HINT: Use shadowing to build up the config through each source
///
/// Example: build_config(Some("db.toml"), Some("5432"), None)
/// Should use file config, override port with 5432, keep file host
pub fn build_config(
    _config_file: Option<&str>,
    _env_port: Option<&str>,
    _env_host: Option<&str>,
) -> Result<DbConfig, String> {
    // TODO: Implement using shadowing
    // Step 1: Start with default config
    // Step 2: Apply file config if Some
    // Step 3: Apply env_port if Some
    // Step 4: Apply env_host if Some
    // Step 5: Validate port range and host not empty

    let config = DbConfig::default();

    let config = if let Some(file) = _config_file {
        DbConfig { ..config }
    } else {
        config
    };

    let config = if let Some(port) = _env_port {
        DbConfig {
            port: port
                .parse()
                .map_err(|_| String::from("Invalid port format"))?,
            ..config
        }
    } else {
        config
    };

    let config = if let Some(host) = _env_host {
        DbConfig {
            host: host.to_string(),
            ..config
        }
    } else {
        config
    };

    let config = if let Some(port) = _env_port {
        DbConfig {
            port: port
                .parse()
                .map_err(|_| String::from("Invalid port format"))?,
            ..config
        }
    } else {
        config
    };

    let config = if let Some(host) = _env_host {
        DbConfig {
            host: host.to_string(),
            ..config
        }
    } else {
        config
    };

    // Validate port range and host not empty
    if config.port < 1024 || config.port > 65534 {
        return Err(String::from("Port must be between 1024 and 65534"));
    }
    if config.host.is_empty() {
        return Err(String::from("Host must not be empty"));
    }

    Ok(config)
}

/// Challenge 5: Log Parser
///
/// Parse and enrich a log entry:
/// 1. Extract timestamp, level, and message from format: "[LEVEL] TIMESTAMP: MESSAGE"
/// 2. Normalize level to uppercase (INFO, WARN, ERROR)
/// 3. Parse timestamp to structured format (just keep as string for simplicity)
/// 4. Truncate message to 100 chars, add "..." if truncated
/// 5. Add metadata: length of original message
/// 6. Return structured LogEntry
///
/// HINT: Use shadowing to extract and transform each component
///
/// Example: parse_log("[info] 2024-01-15 10:30:00: User login successful from 192.168.1.1")
/// Should return LogEntry with level="INFO", timestamp="2024-01-15 10:30:00",
/// message="User login successful from 192.168.1.1", original_length=39
pub fn parse_log(raw_log: &str) -> Result<LogEntry, String> {
    // Step 1: Extract level
    let start = raw_log.find('[').ok_or("Missing '['")?;
    let end = raw_log.find(']').ok_or("Missing ']'")?;
    let level = &raw_log[start + 1..end];

    // Step 2: Normalize level (shadowing)
    let level = level.to_uppercase();

    // Step 3: Extract timestamp
    let rest = raw_log[end + 1..].trim();

    // split only once: timestamp vs message
    // Find the first ": " that separates timestamp from message
    let colon_pos = rest
        .find(": ")
        .ok_or("Invalid format: missing colon separator")?;
    let timestamp = rest[..colon_pos].trim().to_string();
    let raw_message = &rest[colon_pos + 2..]; // +2 to skip ": "
    let original_length = raw_message.len() + 1; // +1 for the leading space that was part of ": "

    let message = raw_message;

    // Step 5: Save original length

    // Step 6: Truncate if needed (shadowing again)
    let message = if message.chars().count() > 100 {
        let truncated: String = message.chars().take(100).collect();
        format!("{}...", truncated)
    } else {
        message.to_string()
    };

    // Step 7: Build struct
    Ok(LogEntry {
        level,
        timestamp: timestamp.to_string(),
        message,
        original_length,
    })
}

// Supporting types - DO NOT MODIFY
#[derive(Debug, PartialEq)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
}

impl DbConfig {
    pub fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            database: "myapp".to_string(),
            username: "admin".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LogEntry {
    pub level: String,
    pub timestamp: String,
    pub message: String,
    pub original_length: usize,
}

/// Test your implementations
pub fn run_tests() {
    println!("\n=== Running Shadowing Challenge Tests ===\n");

    let mut passed = 0;
    let mut failed = 0;

    // Test 1: Price Calculator
    print!("Test 1 - Price Calculator: ");
    match calculate_price(100.0, 20.0, 8.0) {
        result if result == "$86.40" => {
            println!("✓ PASS (got {})", result);
            passed += 1;
        }
        result => {
            println!("✗ FAIL (expected $86.40, got {})", result);
            failed += 1;
        }
    }

    // Test 2: Username Normalizer
    print!("Test 2 - Username Normalizer: ");
    match normalize_username("  John Doe!!!  ") {
        result if result == "john_doe" => {
            println!("✓ PASS (got {})", result);
            passed += 1;
        }
        result => {
            println!("✗ FAIL (expected john_doe, got {})", result);
            failed += 1;
        }
    }

    // Test 3: Score Normalizer
    print!("Test 3 - Score Normalizer: ");
    match normalize_score("85.5", 1.2) {
        Ok(68) => {
            println!("✓ PASS (got Ok(68))");
            passed += 1;
        }
        result => {
            println!("✗ FAIL (expected Ok(68), got {:?})", result);
            failed += 1;
        }
    }

    // Test 4: Config Builder
    print!("Test 4 - Config Builder: ");
    match build_config(None, Some("3306"), Some("prod.db.com")) {
        Ok(config) if config.port == 3306 && config.host == "prod.db.com" => {
            println!("✓ PASS (got {:?})", config);
            passed += 1;
        }
        result => {
            println!(
                "✗ FAIL (expected port=3306, host=prod.db.com, got {:?})",
                result
            );
            failed += 1;
        }
    }

    // Test 5: Log Parser
    print!("Test 5 - Log Parser: ");
    let log = "[info] 2024-01-15 10:30:00: User login successful from 192.168.1.1";
    match parse_log(log) {
        Ok(entry)
            if entry.level == "INFO"
                && entry.timestamp == "2024-01-15 10:30:00"
                && entry.message == "User login successful from 192.168.1.1"
                && entry.original_length == 39 =>
        {
            println!("✓ PASS");
            passed += 1;
        }
        result => {
            println!("✗ FAIL (got {:?})", result);
            failed += 1;
        }
    }

    println!(
        "\n=== Results: {}/{} tests passed ===\n",
        passed,
        passed + failed
    );

    if failed == 0 {
        println!("🎉 Congratulations! All tests passed!");
    } else {
        println!("⚠️  Some tests failed. Keep trying!");
    }
}
