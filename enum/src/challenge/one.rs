/**
 * 🟢 Level 1 — Basic Enums (Foundation)
 *
 * Goal: Understand how to define and use simple enums
 */

// ============================================================================
// Exercise 1: Define a Simple Enum
// ============================================================================

/**
 * Create an enum called `Direction` with 4 variants:
 * - North
 * - South
 * - East
 * - West
 *
 * Then create a variable `dir` set to Direction::North
 * and print it.
 */
pub fn exercise_1() {
    // Define the enum
    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    // Create a variable
    let dir = Direction::North;

    // Print it (use {:?} for debug printing)
    println!("Exercise 1 - Direction: {:?}", dir);
}

// ============================================================================
// Exercise 2: Enum in a Function
// ============================================================================

/**
 * Create a function `get_opposite` that takes a Direction
 * and returns the opposite direction:
 * - North ↔ South
 * - East ↔ West
 *
 * Use match to handle each variant.
 */
fn get_opposite(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}

pub fn exercise_2() {
    let north = Direction::North;
    let south = get_opposite(north);

    println!("Exercise 2 - Opposite of North: {:?}", south);

    // Test all directions
    println!("  East → {:?}", get_opposite(Direction::East));
    println!("  West → {:?}", get_opposite(Direction::West));
}

// ============================================================================
// Exercise 3: Enum with impl Block
// ============================================================================

/**
 * Add an impl block to Direction with a method `abbreviate()`
 * that returns a &str:
 * - North → "N"
 * - South → "S"
 * - East → "E"
 * - West → "W"
 */
impl Direction {
    fn abbreviate(&self) -> &str {
        match self {
            Direction::North => "N",
            Direction::South => "S",
            Direction::East => "E",
            Direction::West => "W",
        }
    }
}

pub fn exercise_3() {
    let directions = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    println!("Exercise 3 - Abbreviations:");
    for dir in &directions {
        println!("  {:?} → {}", dir, dir.abbreviate());
    }
}

// ============================================================================
// Main exercise runner
// ============================================================================

pub fn exercise() {
    println!("=== Level 1: Basic Enums ===\n");
    exercise_1();
    println!();
    exercise_2();
    println!();
    exercise_3();
    println!();
}

// Need to redefine Direction at module level for exercises 2 and 3
#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}
