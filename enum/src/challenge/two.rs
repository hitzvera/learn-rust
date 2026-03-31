/**
 * 🔵 Level 2 — Enums with Data (Rust Superpower!)
 *
 * Goal: Learn how to attach data to enum variants
 */

// ============================================================================
// Exercise 1: Enum with Tuple Variants
// ============================================================================

/**
 * Create an enum called `Message` with these variants:
 * - Quit (no data)
 * - Move(x: i32, y: i32) - tuple style
 * - Write(String) - single value
 * - ChangeColor(i32, i32, i32) - RGB values
 *
 * Create instances of each and print them.
 */
pub fn exercise_1() {
    enum Message {
        Quit,
        Move(i32, i32),
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let messages = [
        Message::Quit,
        Message::Move(10, 20),
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(255, 128, 0),
    ];

    println!("Exercise 1 - Messages with Data:");
    for msg in &messages {
        match msg {
            Message::Quit => println!("  • Quit"),
            Message::Move(x, y) => println!("  • Move to ({}, {})", x, y),
            Message::Write(text) => println!("  • Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("  • Color: RGB({}, {}, {})", r, g, b),
        }
    }
}

// ============================================================================
// Exercise 2: Enum with Struct-like Variants
// ============================================================================

/**
 * Create an enum called `StudentEvent` with struct-style variants:
 * - Exam { subject: String, score: i32 }
 * - Assignment { name: String, completed: bool }
 * - Holiday { name: String, days: i32 }
 *
 * Create one of each and display the info.
 */
pub fn exercise_2() {
    enum StudentEvent {
        Exam { subject: String, score: i32 },
        Assignment { name: String, completed: bool },
        Holiday { name: String, days: i32 },
    }

    let events = [
        StudentEvent::Exam {
            subject: String::from("Rust Programming"),
            score: 95,
        },
        StudentEvent::Assignment {
            name: String::from("Enum Challenges"),
            completed: true,
        },
        StudentEvent::Holiday {
            name: String::from("Weekend"),
            days: 2,
        },
    ];

    println!("\nExercise 2 - Student Events:");
    for event in &events {
        match event {
            StudentEvent::Exam { subject, score } => {
                println!("  📝 {} - Score: {}/100", subject, score);
            }
            StudentEvent::Assignment { name, completed } => {
                let status = if *completed { "✅" } else { "⏳" };
                println!("  {} {} - {}", status, name, if *completed { "Done" } else { "Pending" });
            }
            StudentEvent::Holiday { name, days } => {
                println!("  🏖️ {} - {} days", name, days);
            }
        }
    }
}

// ============================================================================
// Exercise 3: Mixed Enum (Tuple + Struct + No Data)
// ============================================================================

/**
 * Create an enum called `WebEvent` that mixes all styles:
 * - PageLoad (no data)
 * - Click { x: i32, y: i32 } (struct)
 * - KeyPress(char) (tuple)
 * - SubmitForm(String) (tuple - form data)
 * - Hover { element: String, duration_ms: i32 } (struct)
 *
 * Process each event with a match statement.
 */
pub fn exercise_3() {
    enum WebEvent {
        PageLoad,
        Click { x: i32, y: i32 },
        KeyPress(char),
        SubmitForm(String),
        Hover { element: String, duration_ms: i32 },
    }

    let events = vec![
        WebEvent::PageLoad,
        WebEvent::Click { x: 150, y: 300 },
        WebEvent::KeyPress('A'),
        WebEvent::SubmitForm(String::from("username=mujahid")),
        WebEvent::Hover {
            element: String::from("submit_button"),
            duration_ms: 1500,
        },
    ];

    println!("\nExercise 3 - Web Events:");
    for event in events {
        match event {
            WebEvent::PageLoad => println!("  📄 Page loaded"),
            WebEvent::Click { x, y } => println!("  🖱️ Click at ({}, {})", x, y),
            WebEvent::KeyPress(key) => println!("  ⌨️ Key pressed: '{}'", key),
            WebEvent::SubmitForm(data) => println!("  📤 Form submitted: {}", data),
            WebEvent::Hover { element, duration_ms } => {
                println!("  👆 Hovered on '{}' for {}ms", element, duration_ms);
            }
        }
    }
}

// ============================================================================
// Main exercise runner
// ============================================================================

pub fn run_exercises() {
    println!("=== Level 2: Enums with Data ===\n");
    exercise_1();
    exercise_2();
    exercise_3();
    println!();
}
