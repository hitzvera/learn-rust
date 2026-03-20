# 🟢 Variables Challenge

> Master Rust variables through progressive exercises

## 📖 Module Overview

This module teaches Rust variables through **5 progressive levels**, from basic mutability to advanced shadowing patterns used in production code.

## 🎯 Learning Path

```
Level 1 🟢 → Level 2 🔵 → Level 3 🟣 → Level 4 🟡 → Level 5 🔴
Basics      Type         Constants   Shadowing   Challenges
            Annotation               Examples
```

## 📁 Module Structure

```
variables/
├── Cargo.toml              # Project configuration
├── src/
│   ├── main.rs             # Entry point (uncomment exercises)
│   └── challenge/
│       ├── mod.rs          # Module documentation
│       ├── one.rs          # 🟢 Basics (mutability)
│       ├── two.rs          # 🔵 Type annotations
│       ├── three.rs        # 🟣 Constants
│       ├── four.rs         # 🟡 Shadowing examples
│       └── five.rs         # 🔴 Shadowing challenges
```

## 🚀 Getting Started

### 1. Navigate to the module

```bash
cd variables
```

### 2. Run exercises

```bash
# Run Level 5 (default - tests)
cargo run

# Run specific level (edit src/main.rs to uncomment)
cargo run
```

### 3. Run tests

```bash
cargo test
```

## 📚 Exercise Details

### Level 1: Basics 🟢

**File:** `src/challenge/one.rs`

**Learn:**
- `let` vs `let mut`
- Immutability by default
- When mutation is necessary

**Exercise:**
```rust
let mut x = 5;
x = 6;
println!("The value of x is: {x}");
```

---

### Level 2: Type Annotations 🔵

**File:** `src/challenge/two.rs`

**Learn:**
- Explicit type annotations
- Signed (`i32`) vs unsigned (`u32`)
- Type inference

**Exercise:**
```rust
let y: i32 = 10;
// let y: u32 = -1; // Error!
```

---

### Level 3: Constants 🟣

**File:** `src/challenge/three.rs`

**Learn:**
- `const` keyword
- Compile-time values
- Naming conventions (SCREAMING_SNAKE_CASE)

**Exercise:**
```rust
const MAX_USERS: u32 = 1_000_000;
```

---

### Level 4: Shadowing Examples 🟡

**File:** `src/challenge/four.rs`

**Learn:**
- Real-world shadowing patterns
- Data transformation pipelines
- Configuration building

**Examples:**
- User registration pipeline
- Configuration loading
- Financial calculations
- Log processing

---

### Level 5: Shadowing Challenges 🔴

**File:** `src/challenge/five.rs`

**Test your skills:**
1. **Price Calculator** - Apply discount, tax, format currency
2. **Username Normalizer** - Sanitize user input
3. **Score Normalizer** - Parse, clamp, scale scores
4. **Config Builder** - Build config with defaults
5. **Log Parser** - Parse structured log entries

**Run tests:**
```bash
cargo run
```

## 💡 Key Concepts

### Shadowing vs Mutation

| Shadowing ✅ | Mutation ⚠️ |
|-------------|------------|
| `let x = x + 1;` | `let mut x = 5; x += 1;` |
| Type changes allowed | Same type only |
| Idiomatic Rust | Use when necessary |
| Clear data flow | In-place modification |

### When to Use Shadowing

```rust
// ✅ Transform data through stages
let input = "  Hello  ";
let input = input.trim();
let input = input.to_lowercase();
let input = input.replace(' ', "_");

// ✅ Change types
let text = "42";
let number = text.parse::<i32>().unwrap();
let doubled = number * 2;
let result = doubled.to_string();
```

## 📖 Additional Resources

- [The Rust Book - Variables](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/variable_bindings.html)
- [Rust Reference - Variables](https://doc.rust-lang.org/reference/variables.html)

## ✅ Completion Checklist

- [ ] Complete all 5 levels
- [ ] Pass all Level 5 tests
- [ ] Write your own shadowing example
- [ ] Explain shadowing vs mutation to someone else
- [ ] Move to [`ownership`](../ownership/) module

---

**Created:** March 2026  
**Author:** @hitzvera  
**Rust Edition:** 2024
