# 🦀 Rust Modules & Packages Challenges

**Based on:** [Rust Book Chapter 7](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

## 📚 Learning Objectives

By completing these challenges, you will master:

- **Packages & Crates** - Understanding the difference, binary vs library crates
- **Module System** - Organizing code with `mod`
- **Paths** - Absolute (`crate::`) vs relative paths
- **Privacy** - Using `pub` to expose items
- **Use Keyword** - Bringing paths into scope
- **Re-exporting** - Using `pub use` to create public APIs
- **File Separation** - Moving modules to separate files

---

## 🎯 Challenge Levels

| Level | Focus | Difficulty |
|-------|-------|------------|
| 🟢 | Level 1: Packages & Crates Basics | Easy |
| 🟡 | Level 2: Modules & Privacy | Medium |
| 🟠 | Level 3: Paths & Use Keyword | Medium |
| 🔴 | Level 4: File Separation & Re-exporting | Hard |
| 🟣 | Level 5: Real-World Project Structure | Expert |

---

## 🚀 How to Use

```bash
cd modules

# Run challenges
cargo run

# Run tests
cargo test
```

---

## 📖 Key Concepts Cheat Sheet

### Packages vs Crates
```
Package = Cargo.toml + 1+ crates
Crate = Library (lib.rs) OR Binary (main.rs)
```

### Module Declaration
```rust
// Inline module
mod garden {
    pub struct Plant;
}

// External module (in src/garden.rs)
mod garden;
```

### Paths
```rust
// Absolute path
crate::garden::vegetables::Asparagus

// Relative path
self::vegetables::Asparagus
super::garden::vegetables::Asparagus
```

### Visibility
```rust
pub mod garden;        // Public module
pub struct Plant;      // Public struct
pub fn grow() {}       // Public function
```

### Use Keyword
```rust
use crate::garden::Plant;           // Bring into scope
use crate::garden::{Plant, Water};  // Multiple items
use crate::garden::*;               // Glob operator
pub use crate::garden::Plant;       // Re-export
```

---

## ✅ Completion Checklist

- [ ] Level 1: Create package with binary + library
- [ ] Level 2: Implement module hierarchy with proper visibility
- [ ] Level 3: Use absolute/relative paths correctly
- [ ] Level 4: Separate modules into files
- [ ] Level 5: Build a complete project structure

---

**Author:** @hitzvera  
**Created:** April 6, 2026  
**Rust Edition:** 2024
