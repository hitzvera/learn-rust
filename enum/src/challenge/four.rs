/**
 * 🟡 Level 4 — Option & Result (Rust's Error Handling)
 *
 * Goal: Master Rust's null safety and error handling
 */

// ============================================================================
// Exercise 1: Option<T> Basics
// ============================================================================

/**
 * Create functions that return Option<i32>:
 * - `find_max(a, b)`: Returns Some(max) if both positive, None otherwise
 * - `safe_divide(a, b)`: Returns Some(a/b) if b != 0, None otherwise
 * - `parse_positive(s)`: Returns Some(number) if string is positive number, None otherwise
 */

fn find_max(a: i32, b: i32) -> Option<i32> {
    if a > 0 && b > 0 {
        Some(if a > b { a } else { b })
    } else {
        None
    }
}

fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b != 0 {
        Some(a / b)
    } else {
        None
    }
}

fn parse_positive(s: &str) -> Option<i32> {
    match s.parse::<i32>() {
        Ok(num) if num > 0 => Some(num),
        _ => None,
    }
}

pub fn exercise_1() {
    println!("Exercise 1 - Option Basics:");
    
    // find_max
    println!("  find_max(10, 20) = {:?}", find_max(10, 20));
    println!("  find_max(-5, 10) = {:?}", find_max(-5, 10));
    
    // safe_divide
    println!("  safe_divide(100, 5) = {:?}", safe_divide(100, 5));
    println!("  safe_divide(100, 0) = {:?}", safe_divide(100, 0));
    
    // parse_positive
    println!("  parse_positive(\"42\") = {:?}", parse_positive("42"));
    println!("  parse_positive(\"-5\") = {:?}", parse_positive("-5"));
    println!("  parse_positive(\"abc\") = {:?}", parse_positive("abc"));
}

// ============================================================================
// Exercise 2: Option Methods (unwrap, unwrap_or, map, and_then)
// ============================================================================

fn get_user_age(username: &str) -> Option<i32> {
    match username {
        "mujahid" => Some(25),
        "lupi" => Some(24),
        "admin" => Some(30),
        _ => None,
    }
}

pub fn exercise_2() {
    println!("\nExercise 2 - Option Methods:");
    
    // unwrap_or (provide default)
    let age1 = get_user_age("mujahid").unwrap_or(0);
    let age2 = get_user_age("unknown").unwrap_or(0);
    println!("  mujahid's age: {}", age1);
    println!("  unknown's age: {} (default)", age2);
    
    // map (transform the value inside Option)
    let next_year = get_user_age("mujahid").map(|age| age + 1);
    println!("  mujahid's age next year: {:?}", next_year);
    
    // and_then (chain operations that return Option)
    let can_vote = get_user_age("mujahid").and_then(|age| {
        if age >= 17 {
            Some("✅ Can vote")
        } else {
            Some("❌ Too young to vote")
        }
    });
    println!("  Voting status: {:?}", can_vote);
    
    // is_some and is_none
    println!("  get_user_age(\"lupi\").is_some() = {}", get_user_age("lupi").is_some());
    println!("  get_user_age(\"ghost\").is_none() = {}", get_user_age("ghost").is_none());
}

// ============================================================================
// Exercise 3: Result<T, E> Basics
// ============================================================================

#[derive(Debug, PartialEq)]
enum MathError {
    DivisionByZero,
    NegativeNumber,
    Overflow,
}

fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn sqrt_approx(n: i32) -> Result<i32, MathError> {
    if n < 0 {
        Err(MathError::NegativeNumber)
    } else {
        Ok((n as f64).sqrt() as i32)
    }
}

pub fn exercise_3() {
    println!("\nExercise 3 - Result Basics:");
    
    // Successful operations
    match divide(100, 5) {
        Ok(result) => println!("  ✅ 100 / 5 = {}", result),
        Err(e) => println!("  ❌ Error: {:?}", e),
    }
    
    // Error case
    match divide(100, 0) {
        Ok(result) => println!("  ✅ Result: {}", result),
        Err(e) => println!("  ❌ Error: {:?}", e),
    }
    
    // sqrt tests
    match sqrt_approx(16) {
        Ok(result) => println!("  ✅ √16 ≈ {}", result),
        Err(e) => println!("  ❌ Error: {:?}", e),
    }
    
    match sqrt_approx(-4) {
        Ok(result) => println!("  ✅ Result: {}", result),
        Err(e) => println!("  ❌ Error: {:?}", e),
    }
}

// ============================================================================
// Exercise 4: Result with ? Operator
// ============================================================================

fn calculate_complex(a: i32, b: i32, c: i32) -> Result<i32, MathError> {
    // Use ? operator to propagate errors
    let step1 = divide(a, b)?;  // If error, returns early
    let step2 = sqrt_approx(step1)?;
    let result = step2 * c;
    Ok(result)
}

pub fn exercise_4() {
    println!("\nExercise 4 - Result with ? Operator:");
    
    // Success case
    match calculate_complex(100, 5, 2) {
        Ok(result) => println!("  ✅ calculate_complex(100, 5, 2) = {}", result),
        Err(e) => println!("  ❌ Error: {:?}", e),
    }
    
    // Division by zero
    match calculate_complex(100, 0, 2) {
        Ok(result) => println!("  ✅ Result: {}", result),
        Err(e) => println!("  ❌ Error: {:?}", e),
    }
    
    // Negative result from division
    match calculate_complex(-100, 5, 2) {
        Ok(result) => println!("  ✅ Result: {}", result),
        Err(e) => println!("  ❌ Error: {:?}", e),
    }
}

// ============================================================================
// Exercise 5: Real-world Example - Bank Account
// ============================================================================

#[derive(Debug)]
enum BankError {
    InsufficientFunds,
    InvalidAmount,
    AccountNotFound,
}

struct BankAccount {
    balance: i32,
}

impl BankAccount {
    fn new(balance: i32) -> Self {
        BankAccount { balance }
    }
    
    fn withdraw(&mut self, amount: i32) -> Result<i32, BankError> {
        if amount <= 0 {
            return Err(BankError::InvalidAmount);
        }
        if amount > self.balance {
            return Err(BankError::InsufficientFunds);
        }
        self.balance -= amount;
        Ok(self.balance)
    }
    
    fn deposit(&mut self, amount: i32) -> Result<i32, BankError> {
        if amount <= 0 {
            return Err(BankError::InvalidAmount);
        }
        self.balance += amount;
        Ok(self.balance)
    }
    
    fn transfer(&mut self, to: &mut BankAccount, amount: i32) -> Result<(i32, i32), BankError> {
        self.withdraw(amount)?;
        to.deposit(amount)?;
        Ok((self.balance, to.balance))
    }
}

pub fn exercise_5() {
    println!("\nExercise 5 - Bank Account Example:");
    
    let mut account1 = BankAccount::new(1000000);
    let mut account2 = BankAccount::new(500000);
    
    println!("  Initial balances:");
    println!("    Account 1: Rp {}", account1.balance);
    println!("    Account 2: Rp {}", account2.balance);
    
    // Deposit
    match account1.deposit(200000) {
        Ok(balance) => println!("  ✅ Deposited Rp 200,000 → Balance: Rp {}", balance),
        Err(e) => println!("  ❌ Error: {:?}", e),
    }
    
    // Withdraw
    match account1.withdraw(300000) {
        Ok(balance) => println!("  ✅ Withdrew Rp 300,000 → Balance: Rp {}", balance),
        Err(e) => println!("  ❌ Error: {:?}", e),
    }
    
    // Transfer
    match account1.transfer(&mut account2, 100000) {
        Ok((bal1, bal2)) => {
            println!("  ✅ Transferred Rp 100,000");
            println!("    Account 1: Rp {}", bal1);
            println!("    Account 2: Rp {}", bal2);
        }
        Err(e) => println!("  ❌ Transfer failed: {:?}", e),
    }
    
    // Insufficient funds
    match account1.withdraw(5000000) {
        Ok(_) => println!("  ✅ Withdrawal successful"),
        Err(e) => println!("  ❌ Error: {:?} (as expected)", e),
    }
}

// ============================================================================
// Main exercise runner
// ============================================================================

pub fn run_exercises() {
    println!("=== Level 4: Option & Result ===\n");
    exercise_1();
    exercise_2();
    exercise_3();
    exercise_4();
    exercise_5();
    println!();
}
