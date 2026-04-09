# 🦀 My Rust Learning Journey

> **Learning Rust one concept at a time**

[![Rust](https://img.shields.io/badge/rust-v1.75-orange.svg)](https://www.rust-lang.org)
[![Status](https://img.shields.io/badge/status-learning-green.svg)](https://github.com/hitzvera/learn-rust)
[![Started](https://img.shields.io/badge/started-September%202024-blue.svg)](https://github.com/hitzvera/learn-rust)

---

## 📊 Progress Overview

| Metric | Value |
|--------|-------|
| **Started** | September 29, 2024 |
| **Current Level** | Beginner → Intermediate |
| **Goal** | Build production-ready Rust applications |
| **Focus Areas** | Systems programming, CLI tools, WebAssembly |
| **Learning Method** | The Rust Book + Projects |

---

## ✅ Completed Projects

| Project | Status | Date | Description |
|---------|--------|------|-------------|
| **Hello World** | ✅ Done | Sep 2024 | First Rust program |
| **Guessing Game** | ✅ Done | Sep 2024 | Classic CLI game from The Book |

---

## 🎯 Learning Roadmap

### Phase 1: Rust Basics (The Rust Book) 📚

- [x] **Chapter 1:** Getting Started
  - [x] Hello World
  - [x] Cargo basics
  
- [x] **Chapter 2:** Guessing Game Tutorial
  - [x] User input
  - [x] Random numbers
  - [x] Loop and match
  
- [x] **Chapter 3:** Common Programming Concepts
  - [x] Variables & Mutability
  - [x] Data Types
  - [x] Functions
  - [x] Control Flow
  
- [ ] **Chapter 4:** Ownership
  - [ ] Stack & Heap
  - [ ] Ownership Rules
  - [ ] References & Borrowing
  - [ ] Slices
  
- [ ] **Chapter 5:** Structs
  - [ ] Defining Structs
  - [ ] Methods
  - [ ] Associated Functions
  
- [ ] **Chapter 6:** Enums & Pattern Matching
  - [ ] Defining Enums
  - [ ] The Option Enum
  - [ ] Match Control Flow
  
- [ ] **Chapter 7:** Modules
  - [ ] Packages & Crates
  - [ ] Module System
  - [ ] Paths
  
- [ ] **Chapter 8:** Collections
  - [ ] Vectors
  - [ ] Strings
  - [ ] Hash Maps
  
- [ ] **Chapter 9:** Error Handling
  - [ ] Unrecoverable Errors (panic!)
  - [ ] Recoverable Errors (Result)
  - [ ] When to Panic
  
- [ ] **Chapter 10:** Generics, Traits, Lifetimes
  - [ ] Generic Data Types
  - [ ] Traits
  - [ ] Lifetimes
  
- [ ] **Chapter 11-18:** Advanced Topics
  - [ ] Testing
  - [ ] I/O Project
  - [ ] Functional Features
  - [ ] Smart Pointers
  - [ ] Concurrency
  - [ ] OOP Features
  
- [ ] **Chapter 19-20:** Patterns & Advanced
  - [ ] Patterns
  - [ ] Unsafe Rust
  - [ ] Advanced Traits
  - [ ] Macros
  - [ ] Async/Await

---

### Phase 2: Build Projects 🛠️

| Project | Type | Status | Priority |
|---------|------|--------|----------|
| **CLI Todo App** | CLI | ⏳ Todo | High |
| **Expense Tracker** | CLI/Web | ⏳ Todo | High |
| **File Organizer** | CLI | ⏳ Todo | Medium |
| **REST API** | Web | ⏳ Todo | Medium |
| **Web Scraper** | Tool | ⏳ Todo | Low |
| **Game (Tetris)** | Game | ⏳ Todo | Fun |

---

### Phase 3: Advanced Topics 🚀

- [ ] **Async/Await Programming**
- [ ] **Macros (declarative & procedural)**
- [ ] **FFI (Foreign Function Interface)**
- [ ] **WebAssembly (WASM)**
- [ ] **Embedded Rust**
- [ ] **Performance Optimization**

---

## 📚 Learning Resources

### Primary
- [The Rust Programming Language (Book)](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)

### Practice
- [Exercism Rust Track](https://exercism.org/tracks/rust)
- [Advent of Code](https://adventofcode.com/)
- [Project Euler](https://projecteuler.net/)

### References
- [Rust Standard Library Docs](https://doc.rust-lang.org/std/)
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
- [This Week in Rust](https://this-week-in-rust.org/)

### Video Courses
- [Rust Crash Course (YouTube)](https://www.youtube.com/results?search_query=rust+programming)
- [Jon Gjengset Videos](https://www.youtube.com/c/JonGjengset)

---

## 📝 Weekly Progress Log

### Week 1 (Sep 29 - Oct 5, 2024)
- ✅ Set up Rust environment
- ✅ Created Hello World project
- ✅ Completed Guessing Game tutorial
- 📖 Reading: The Book Chapters 1-2

### Week 2 (Oct 6-12, 2024)
- ⏳ In Progress: Chapter 3 - Common Programming Concepts

---

## 🔧 Development Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version

# Create new project
cargo new project_name

# Run project
cargo run

# Build for release
cargo build --release

# Run tests
cargo test
```

---

## 📂 Repository Structure

```
learn-rust/
├── README.md                 # This file - learning roadmap
├── PROGRESS.md              # Detailed progress log
├── RESOURCES.md             # Additional resources & notes
├── learn-rust-book/         # Following The Rust Book
│   ├── Cargo.toml
│   ├── src/
│   └── guess_game/          # Chapter 2 project
└── projects/                # Personal projects
    ├── hello_world/         # First program
    └── [future projects]/
```

---

## 🎯 Next Steps

1. [ ] Complete Chapter 3 exercises
2. [ ] Build CLI Todo App
3. [ ] Contribute to open source Rust project
4. [ ] Write blog post about learning Rust

---

## 💭 Notes & Tips

> **Ownership is the hardest concept** - Don't rush it. Practice with small examples.

> **Use the compiler** - Rust's error messages are your friend. They often tell you exactly what to fix.

> **Clone before borrow** - When in doubt about ownership, cloning can help while learning.

---

## 📈 Stats

```rust
// Learning Stats
let mut rust_journey = RustJourney::new();
rust_journey.days_learning = 0;
rust_journey.projects_completed = 2;
rust_journey.chapters_completed = 2;
rust_journey.lines_of_code = 0;

println!("Keep learning! 🦀");
```

---

**Last Updated:** March 20, 2026  
**Maintained by:** [@hitzvera](https://github.com/hitzvera)

---

<div align="center">

### 🦀 Happy Coding! 🦀

*If you find this helpful, give it a ⭐!*

</div>
