# 🦀 Enum Challenges

Comprehensive exercises for learning Rust enums, from basics to advanced patterns.

## 📚 What You'll Learn

| Level | Topic | Focus |
|-------|-------|-------|
| 🟢 | Level 1 | Basic Enums | Define enums, simple match |
| 🔵 | Level 2 | Enums with Data | Tuple & struct variants |
| 🟣 | Level 3 | Pattern Matching | Match guards, nested match |
| 🟡 | Level 4 | Option & Result | Null safety, error handling |
| 🔴 | Level 5 | Advanced | State machines, AST, trees |

## 🚀 Quick Start

```bash
cd enum

# Run Level 1 (Basic Enums)
cargo run --bin enum

# Run specific level (edit src/main.rs to uncomment)
cargo run --bin enum
```

## 📁 Structure

```
enum/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── challenge/
│       ├── mod.rs      # Module documentation
│       ├── one.rs      # Level 1: Basics
│       ├── two.rs      # Level 2: Data
│       ├── three.rs    # Level 3: Pattern Matching
│       ├── four.rs     # Level 4: Option/Result
│       └── five.rs     # Level 5: Advanced
```

## 💡 Key Concepts

### Enums with Data
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

### Pattern Matching
```rust
match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    Message::Write(text) => println!("Write: {}", text),
    _ => println!("Other"),
}
```

### Option<T>
```rust
let some: Option<i32> = Some(5);
let none: Option<i32> = None;
```

### Result<T, E>
```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

## 🎯 Challenges Preview

### Level 1: Traffic Light
```rust
enum TrafficLight { Red, Yellow, Green }
impl TrafficLight {
    fn action(&self) -> &str {
        match self {
            TrafficLight::Red => "STOP",
            TrafficLight::Yellow => "CAUTION",
            TrafficLight::Green => "GO",
        }
    }
}
```

### Level 5: Vending Machine State Machine
Complete state machine implementation with events and state transitions.

### Level 5: Expression AST
Build and evaluate mathematical expressions using recursive enums.

## ✅ Completion Checklist

- [ ] Complete all Level 1 exercises
- [ ] Understand enum variants with data (Level 2)
- [ ] Master pattern matching (Level 3)
- [ ] Use Option and Result confidently (Level 4)
- [ ] Build a state machine (Level 5)
- [ ] Create your own enum project

## 📖 Resources

- [The Rust Book - Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Rust by Example - Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)
- [Pattern Matching](https://doc.rust-lang.org/book/ch18-00-patterns.html)

---

**Author:** @hitzvera  
**Created:** March 2026  
**Rust Edition:** 2024
