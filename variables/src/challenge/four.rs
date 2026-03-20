/**
 * 🟡 Level 4 — Shadowing in Production
 *
 * Real-world scenarios where shadowing is incredibly useful
 */

/// Example 1: Data Transformation Pipeline
/// Converting user input through multiple validation/sanitization steps
pub fn user_registration_pipeline(raw_input: &str) -> Result<String, String> {
    // Step 1: Trim whitespace (shadow to show transformation)
    let raw_input = raw_input.trim();

    // Step 2: Convert to lowercase for consistency
    let raw_input = raw_input.to_lowercase();

    // Step 3: Validate length (now we have a String, not &str)
    if raw_input.len() < 3 {
        return Err("Username too short".to_string());
    }

    // Step 4: Sanitize - remove special characters
    let raw_input: String = raw_input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect();

    Ok(raw_input)
}

/// Example 2: Configuration Loading with Defaults
/// Building config from multiple sources (env vars, files, defaults)
pub fn load_configuration() -> AppConfig {
    // Start with default config
    let config = AppConfig::default();

    // Override with file config (shadow to merge)
    let config = config.merge_from_file("app.toml");

    // Override with environment variables (shadow again)
    let config = config.merge_from_env();

    // Final validation (shadow to validated config)
    let config = config.validate().expect("Invalid configuration");

    config
}

/// Example 3: Mathematical Computations
/// Step-by-step calculations where each step has meaning
pub fn calculate_financial_metrics(revenue: f64, costs: f64, tax_rate: f64) -> FinancialMetrics {
    // Gross profit
    let profit = revenue - costs;

    // Tax amount
    let profit = profit * tax_rate;
    let tax = profit;

    // Net profit (shadow again)
    let profit = (revenue - costs) - tax;

    // Profit margin percentage
    let profit = (profit / revenue) * 100.0;

    FinancialMetrics {
        gross_profit: revenue - costs,
        tax_amount: tax,
        net_profit: (revenue - costs) - tax,
        profit_margin: profit,
    }
}

/// Example 4: String Processing Pipeline
/// Common in web servers, log processing, data ETL
pub fn process_log_entry(raw_log: &str) -> Option<String> {
    // Remove timestamp prefix
    let raw_log = raw_log.strip_prefix("[INFO] ").unwrap_or(raw_log);
    let raw_log = raw_log.strip_prefix("[ERROR] ").unwrap_or(raw_log);
    let raw_log = raw_log.strip_prefix("[WARN] ").unwrap_or(raw_log);

    // Trim whitespace
    let raw_log = raw_log.trim();

    // Convert to uppercase for standardization
    let raw_log = raw_log.to_uppercase();

    // Truncate if too long (shadow with sliced version)
    let raw_log = if raw_log.len() > 50 {
        raw_log[..50].to_string()
    } else {
        raw_log
    };

    Some(raw_log)
}

/// Example 5: Type Transformation Chain
/// Converting data through multiple representations
pub fn parse_and_validate_number(input: &str) -> Result<i32, String> {
    // Parse to string first (trimmed)
    let input = input.trim();

    // Parse to integer
    let input: i32 = input.parse().map_err(|_| "Not a valid number")?;

    // Validate range
    let input = if input < 0 {
        return Err("Number must be positive".to_string());
    } else {
        input
    };

    // Apply business rule: cap at 100
    let input = if input > 100 { 100 } else { input };

    Ok(input)
}

// Supporting types for examples
#[derive(Debug, Default)]
pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
    pub debug_mode: bool,
}

impl AppConfig {
    pub fn default() -> Self {
        Self {
            database_url: "localhost:5432".to_string(),
            port: 8080,
            debug_mode: false,
        }
    }

    pub fn merge_from_file(self, _path: &str) -> Self {
        // Simulated file loading
        Self {
            database_url: "prod-db:5432".to_string(),
            ..self
        }
    }

    pub fn merge_from_env(self) -> Self {
        // Simulated env var loading
        Self { port: 3000, ..self }
    }

    pub fn validate(self) -> Result<Self, String> {
        if self.port == 0 {
            return Err("Port cannot be 0".to_string());
        }
        Ok(self)
    }
}

#[derive(Debug)]
pub struct FinancialMetrics {
    pub gross_profit: f64,
    pub tax_amount: f64,
    pub net_profit: f64,
    pub profit_margin: f64,
}

/// Run all examples
pub fn run_examples() {
    println!("\n=== Shadowing in Production Examples ===\n");

    // Example 1: User registration pipeline
    println!("1. Data Transformation Pipeline:");
    let username = "  John_Doe123!  ";
    match user_registration_pipeline(username) {
        Ok(clean) => println!("   ✓ Clean username: '{}'", clean),
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // Example 2: Configuration loading
    println!("\n2. Configuration Loading with Defaults:");
    let config = load_configuration();
    println!("   ✓ Database URL: {}", config.database_url);
    println!("   ✓ Port: {}", config.port);
    println!("   ✓ Debug mode: {}", config.debug_mode);

    // Example 3: Financial calculations
    println!("\n3. Mathematical Computations:");
    let metrics = calculate_financial_metrics(100000.0, 60000.0, 0.25);
    println!("   ✓ Gross Profit: ${:.2}", metrics.gross_profit);
    println!("   ✓ Tax Amount: ${:.2}", metrics.tax_amount);
    println!("   ✓ Net Profit: ${:.2}", metrics.net_profit);
    println!("   ✓ Profit Margin: {:.1}%", metrics.profit_margin);

    // Example 4: Log processing
    println!("\n4. String Processing Pipeline:");
    let log = "[ERROR]   Something went wrong!!!   ";
    match process_log_entry(log) {
        Some(processed) => println!("   ✓ Processed log: '{}'", processed),
        None => println!("   ✗ Failed to process log"),
    }

    // Example 5: Number parsing
    println!("\n5. Type Transformation Chain:");
    let number = "  150  ";
    match parse_and_validate_number(number) {
        Ok(n) => println!("   ✓ Parsed and validated: {} (capped at 100)", n),
        Err(e) => println!("   ✗ Error: {}", e),
    }

    println!("\n=== All examples completed ===\n");
}
