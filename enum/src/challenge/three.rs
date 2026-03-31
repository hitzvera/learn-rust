/**
 * 🟣 Level 3 — Pattern Matching Mastery
 *
 * Goal: Master the `match` expression with enums
 */

// ============================================================================
// Exercise 1: Basic Match with Enum
// ============================================================================

#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn action(&self) -> &str {
        match self {
            TrafficLight::Red => "STOP",
            TrafficLight::Yellow => "CAUTION",
            TrafficLight::Green => "GO",
        }
    }
}

pub fn exercise_1() {
    let lights = [
        TrafficLight::Red,
        TrafficLight::Yellow,
        TrafficLight::Green,
    ];

    println!("Exercise 1 - Traffic Light Actions:");
    for light in &lights {
        println!("  {:?} → {}", light, light.action());
    }
}

// ============================================================================
// Exercise 2: Match with Data Extraction
// ============================================================================

#[derive(Debug)]
enum Package {
    Small { weight: f32 },
    Medium { weight: f32 },
    Large { weight: f32, dimensions: (i32, i32, i32) },
    Express(String), // destination
}

impl Package {
    fn shipping_cost(&self) -> f32 {
        match self {
            Package::Small { weight } => weight * 2.0,
            Package::Medium { weight } => weight * 3.0,
            Package::Large { weight, dimensions } => {
                let volume = dimensions.0 * dimensions.1 * dimensions.2;
                weight * 4.0 + (volume as f32 * 0.001)
            }
            Package::Express(_) => 50.0, // flat rate
        }
    }

    fn description(&self) -> String {
        match self {
            Package::Small { weight } => format!("Small package ({:.1}kg)", weight),
            Package::Medium { weight } => format!("Medium package ({:.1}kg)", weight),
            Package::Large { weight, dimensions } => {
                format!("Large package ({:.1}kg, {}x{}x{})", 
                    weight, dimensions.0, dimensions.1, dimensions.2)
            }
            Package::Express(dest) => format!("Express to {}", dest),
        }
    }
}

pub fn exercise_2() {
    let packages = vec![
        Package::Small { weight: 1.5 },
        Package::Medium { weight: 5.0 },
        Package::Large {
            weight: 10.0,
            dimensions: (30, 20, 15),
        },
        Package::Express(String::from("Jakarta")),
    ];

    println!("\nExercise 2 - Package Shipping:");
    for package in &packages {
        println!("  {}", package.description());
        println!("    Cost: Rp {:.2}", package.shipping_cost());
    }
}

// ============================================================================
// Exercise 3: Match with Guards (if conditions)
// ============================================================================

#[derive(Debug)]
enum Grade {
    A(i32), // score
    B(i32),
    C(i32),
    D(i32),
    F(i32),
}

impl Grade {
    fn feedback(&self) -> String {
        match self {
            Grade::A(score) if *score >= 95 => format!("Excellent! ({})", score),
            Grade::A(score) => format!("Great job! ({})", score),
            Grade::B(score) if *score >= 85 => format!("Good work! ({})", score),
            Grade::B(score) => format!("Above average ({})", score),
            Grade::C(score) => format!("Satisfactory ({})", score),
            Grade::D(score) => format!("Needs improvement ({})", score),
            Grade::F(score) => format!("Failed - retake required ({})", score),
        }
    }

    fn is_passing(&self) -> bool {
        match self {
            Grade::A(_) | Grade::B(_) | Grade::C(_) | Grade::D(_) => true,
            Grade::F(_) => false,
        }
    }
}

pub fn exercise_3() {
    let grades = vec![
        Grade::A(98),
        Grade::A(90),
        Grade::B(87),
        Grade::B(80),
        Grade::C(75),
        Grade::D(60),
        Grade::F(45),
    ];

    println!("\nExercise 3 - Grade Feedback:");
    for grade in &grades {
        let status = if grade.is_passing() { "✅" } else { "❌" };
        println!("  {} {:?} - {}", status, grade, grade.feedback());
    }
}

// ============================================================================
// Exercise 4: Nested Match
// ============================================================================

#[derive(Debug)]
enum PaymentMethod {
    Cash(i32),
    Card { card_type: CardType, amount: i32 },
    EWallet(EWallet, i32),
}

#[derive(Debug)]
enum CardType {
    Debit,
    Credit,
    Prepaid,
}

#[derive(Debug)]
enum EWallet {
    OVO,
    Gopay,
    Dana,
}

impl PaymentMethod {
    fn process(&self) -> String {
        match self {
            PaymentMethod::Cash(amount) => {
                format!("💵 Cash payment: Rp {}", amount)
            }
            PaymentMethod::Card { card_type, amount } => {
                let icon = match card_type {
                    CardType::Debit => "💳",
                    CardType::Credit => "💳",
                    CardType::Prepaid => "💳",
                };
                format!("{} {:?} card: Rp {}", icon, card_type, amount)
            }
            PaymentMethod::EWallet(wallet, amount) => {
                match wallet {
                    EWallet::OVO => format!("📱 OVO: Rp {}", amount),
                    EWallet::Gopay => format!("📱 Gopay: Rp {}", amount),
                    EWallet::Dana => format!("📱 Dana: Rp {}", amount),
                }
            }
        }
    }
}

pub fn exercise_4() {
    let payments = vec![
        PaymentMethod::Cash(50000),
        PaymentMethod::Card {
            card_type: CardType::Debit,
            amount: 150000,
        },
        PaymentMethod::Card {
            card_type: CardType::Credit,
            amount: 500000,
        },
        PaymentMethod::EWallet(EWallet::OVO, 75000),
        PaymentMethod::EWallet(EWallet::Gopay, 30000),
    ];

    println!("\nExercise 4 - Payment Processing:");
    for payment in &payments {
        println!("  {}", payment.process());
    }
}

// ============================================================================
// Main exercise runner
// ============================================================================

pub fn run_exercises() {
    println!("=== Level 3: Pattern Matching ===\n");
    exercise_1();
    exercise_2();
    exercise_3();
    exercise_4();
    println!();
}
