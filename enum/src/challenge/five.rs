/**
 * 🔴 Level 5 — Advanced Enum Challenges
 *
 * Goal: Apply everything you've learned in real-world scenarios
 */

// ============================================================================
// Challenge 1: State Machine (Vending Machine)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
enum VendingState {
    Idle,
    WaitingForMoney,
    HasCredit(i32),
    Dispensing(String),
}

#[derive(Debug)]
enum VendingEvent {
    InsertCoin(i32),
    SelectItem(String, i32),
    Cancel,
    Dispense,
}

struct VendingMachine {
    state: VendingState,
    items: Vec<(String, i32)>,
}

impl VendingMachine {
    fn new() -> Self {
        VendingMachine {
            state: VendingState::Idle,
            items: vec![
                (String::from("Coffee"), 5000),
                (String::from("Tea"), 4000),
                (String::from("Soda"), 6000),
                (String::from("Water"), 3000),
            ],
        }
    }

    fn process_event(&mut self, event: VendingEvent) -> String {
        let old_state = std::mem::replace(&mut self.state, VendingState::Idle);

        let (next_state, message) = match (old_state, event) {
            // From Idle state
            (VendingState::Idle, VendingEvent::InsertCoin(amount)) => (
                VendingState::HasCredit(amount),
                format!("💵 Inserted Rp {}. Credit: Rp {}", amount, amount),
            ),
            (VendingState::Idle, VendingEvent::SelectItem(item, price)) => (
                VendingState::WaitingForMoney,
                format!("📦 Selected {}. Please insert Rp {}", item, price),
            ),

            // From HasCredit state
            (VendingState::HasCredit(credit), VendingEvent::InsertCoin(amount)) => {
                let new_credit = credit + amount;
                (
                    VendingState::HasCredit(new_credit),
                    format!("💵 Inserted Rp {}. Total credit: Rp {}", amount, new_credit),
                )
            }
            (VendingState::HasCredit(credit), VendingEvent::SelectItem(item, price)) => {
                if credit >= price {
                    let change = credit - price;
                    (
                        VendingState::Dispensing(item.clone()),
                        format!("✅ Dispensing {}. Change: Rp {}", item, change),
                    )
                } else {
                    (
                        VendingState::HasCredit(credit),
                        format!(
                            "❌ Insufficient credit. Need Rp {}, have Rp {}",
                            price - credit,
                            credit
                        ),
                    )
                }
            }
            (VendingState::HasCredit(credit), VendingEvent::Cancel) => (
                VendingState::Idle,
                format!("🔄 Cancelled. Refund: Rp {}", credit),
            ),

            // From Dispensing state
            (VendingState::Dispensing(item), VendingEvent::Dispense) => {
                (VendingState::Idle, format!("🎉 Enjoy your {}!", item))
            }

            // Default
            (s, _) => (s, "⚠️ Invalid action".to_string()),
        };
        self.state = next_state;
        message
    }

    fn show_menu(&self) {
        println!("  ╔════════════════════════╗");
        println!("  ║   VENDING MACHINE      ║");
        println!("  ╠════════════════════════╣");
        for (i, (item, price)) in self.items.iter().enumerate() {
            println!("  ║ {}. {:<15} Rp {:>4} ║", i + 1, item, price);
        }
        println!("  ╚════════════════════════╝");
    }
}

pub fn challenge_1() {
    println!("Challenge 1 - Vending Machine State Machine:");

    let mut machine = VendingMachine::new();
    machine.show_menu();

    println!("\n  Simulation:");
    println!(
        "  {}",
        machine.process_event(VendingEvent::InsertCoin(10000))
    );
    println!(
        "  {}",
        machine.process_event(VendingEvent::SelectItem(String::from("Coffee"), 5000))
    );
    println!("  {}", machine.process_event(VendingEvent::Dispense));
    println!(
        "  {}",
        machine.process_event(VendingEvent::InsertCoin(5000))
    );
    println!(
        "  {}",
        machine.process_event(VendingEvent::SelectItem(String::from("Tea"), 4000))
    );
    println!("  {}", machine.process_event(VendingEvent::Dispense));
}

// ============================================================================
// Challenge 2: AST (Abstract Syntax Tree) for Simple Expressions
// ============================================================================

#[derive(Debug, Clone)]
enum Expr {
    Number(i32),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn evaluate(&self) -> Result<i32, &'static str> {
        match self {
            Expr::Number(n) => Ok(*n),
            Expr::Add(left, right) => Ok(left.evaluate()? + right.evaluate()?),
            Expr::Subtract(left, right) => Ok(left.evaluate()? - right.evaluate()?),
            Expr::Multiply(left, right) => Ok(left.evaluate()? * right.evaluate()?),
            Expr::Divide(left, right) => {
                let r = right.evaluate()?;
                if r == 0 {
                    Err("Division by zero")
                } else {
                    Ok(left.evaluate()? / r)
                }
            }
        }
    }

    fn to_string(&self) -> String {
        match self {
            Expr::Number(n) => n.to_string(),
            Expr::Add(l, r) => format!("({} + {})", l.to_string(), r.to_string()),
            Expr::Subtract(l, r) => format!("({} - {})", l.to_string(), r.to_string()),
            Expr::Multiply(l, r) => format!("({} * {})", l.to_string(), r.to_string()),
            Expr::Divide(l, r) => format!("({} / {})", l.to_string(), r.to_string()),
        }
    }
}

pub fn challenge_2() {
    println!("\nChallenge 2 - Expression AST:");

    // (5 + 3) * 2
    let expr1 = Expr::Multiply(
        Box::new(Expr::Add(
            Box::new(Expr::Number(5)),
            Box::new(Expr::Number(3)),
        )),
        Box::new(Expr::Number(2)),
    );

    // (10 - 4) / 2
    let expr2 = Expr::Divide(
        Box::new(Expr::Subtract(
            Box::new(Expr::Number(10)),
            Box::new(Expr::Number(4)),
        )),
        Box::new(Expr::Number(2)),
    );

    // (5 + 3) * (10 - 4)
    let expr3 = Expr::Multiply(
        Box::new(Expr::Add(
            Box::new(Expr::Number(5)),
            Box::new(Expr::Number(3)),
        )),
        Box::new(Expr::Subtract(
            Box::new(Expr::Number(10)),
            Box::new(Expr::Number(4)),
        )),
    );

    println!("  Expression: {}", expr1.to_string());
    match expr1.evaluate() {
        Ok(result) => println!("  Result: {}", result),
        Err(e) => println!("  Error: {}", e),
    }

    println!("  Expression: {}", expr2.to_string());
    match expr2.evaluate() {
        Ok(result) => println!("  Result: {}", result),
        Err(e) => println!("  Error: {}", e),
    }

    println!("  Expression: {}", expr3.to_string());
    match expr3.evaluate() {
        Ok(result) => println!("  Result: {}", result),
        Err(e) => println!("  Error: {}", e),
    }
}

// ============================================================================
// Challenge 3: Command Pattern with Enums
// ============================================================================

#[derive(Debug)]
enum Command {
    Create { name: String },
    Read { id: i32 },
    Update { id: i32, data: String },
    Delete { id: i32 },
    List,
    Exit,
}

impl Command {
    fn parse(input: &str) -> Option<Command> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            return None;
        }

        match parts[0].to_lowercase().as_str() {
            "create" => {
                if parts.len() >= 2 {
                    Some(Command::Create {
                        name: parts[1..].join(" "),
                    })
                } else {
                    None
                }
            }
            "read" => {
                if parts.len() >= 2 {
                    parts[1].parse::<i32>().ok().map(|id| Command::Read { id })
                } else {
                    None
                }
            }
            "update" => {
                if parts.len() >= 3 {
                    parts[1].parse::<i32>().ok().map(|id| Command::Update {
                        id,
                        data: parts[2..].join(" "),
                    })
                } else {
                    None
                }
            }
            "delete" => {
                if parts.len() >= 2 {
                    parts[1]
                        .parse::<i32>()
                        .ok()
                        .map(|id| Command::Delete { id })
                } else {
                    None
                }
            }
            "list" => Some(Command::List),
            "exit" | "quit" => Some(Command::Exit),
            _ => None,
        }
    }

    fn execute(&self) -> String {
        match self {
            Command::Create { name } => format!("✅ Created: {}", name),
            Command::Read { id } => format!("📖 Reading item #{}", id),
            Command::Update { id, data } => format!("✏️ Updated #{}: {}", id, data),
            Command::Delete { id } => format!("🗑️ Deleted item #{}", id),
            Command::List => format!("📋 Listing all items..."),
            Command::Exit => format!("👋 Goodbye!"),
        }
    }
}

pub fn challenge_3() {
    println!("\nChallenge 3 - Command Pattern:");

    let commands = vec![
        "create Expense Tracker",
        "read 1",
        "update 1 Add coffee expense",
        "list",
        "delete 1",
        "invalid command",
        "exit",
    ];

    for cmd_str in commands {
        println!("  Input: '{}'", cmd_str);
        match Command::parse(cmd_str) {
            Some(cmd) => println!("  {}", cmd.execute()),
            None => println!("  ❌ Invalid command"),
        }
    }
}

// ============================================================================
// Challenge 4: Tree Data Structure
// ============================================================================

#[derive(Debug)]
enum Tree<T> {
    Empty,
    Leaf(T),
    Node(Box<Tree<T>>, Box<Tree<T>>),
}

impl<T: Clone + std::fmt::Display> Tree<T> {
    fn depth(&self) -> i32 {
        match self {
            Tree::Empty => 0,
            Tree::Leaf(_) => 1,
            Tree::Node(left, right) => 1 + left.depth().max(right.depth()),
        }
    }

    fn size(&self) -> i32 {
        match self {
            Tree::Empty => 0,
            Tree::Leaf(_) => 1,
            Tree::Node(left, right) => 1 + left.size() + right.size(),
        }
    }

    fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        match self {
            Tree::Empty => false,
            Tree::Leaf(v) => v == value,
            Tree::Node(left, right) => left.contains(value) || right.contains(value),
        }
    }
}

pub fn challenge_4() {
    println!("\nChallenge 4 - Tree Data Structure:");

    // Create a tree:
    //       Node
    //      /    \
    //   Leaf(1)  Node
    //           /    \
    //       Leaf(2)  Leaf(3)

    let tree: Tree<i32> = Tree::Node(
        Box::new(Tree::Leaf(1)),
        Box::new(Tree::Node(Box::new(Tree::Leaf(2)), Box::new(Tree::Leaf(3)))),
    );

    println!("  Tree depth: {}", tree.depth());
    println!("  Tree size: {}", tree.size());
    println!("  Contains 2: {}", tree.contains(&2));
    println!("  Contains 5: {}", tree.contains(&5));
}

// ============================================================================
// Main challenge runner
// ============================================================================

pub fn run_tests() {
    println!("=== Level 5: Advanced Challenges ===\n");
    challenge_1();
    challenge_2();
    challenge_3();
    challenge_4();
    println!("\n🎉 All challenges completed!");
}
