# 🦀 Control Flow Challenges

Comprehensive exercises for mastering Rust control flow patterns: `match`, `if let`, and `let...else`.

## ⚠️ IMPORTANT: Challenge Mode!

**No solutions are provided in this chapter!**

This is intentional. You'll learn more by struggling with the problems yourself.

## 📚 What You'll Learn

| Level | Topic | Focus |
|-------|-------|-------|
| 🟢 | Level 1 | Match Basics | Basic pattern matching |
| 🔵 | Level 2 | Match with Data | Extracting data from enums |
| 🟣 | Level 3 | If Let | Concise single-pattern matching |
| 🟡 | Level 4 | Let Else | Early return pattern |
| 🔴 | Level 5 | Choose the Right Tool | When to use each pattern |

## 🚀 Quick Start

```bash
cd control-flow

# Run challenges (they will fail until you implement them!)
cargo run --bin control-flow

# Edit src/challenge/one.rs and implement the functions
```

## 📁 Structure

```
control-flow/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    └── challenge/
        ├── mod.rs
        ├── one.rs    # Level 1: Match Basics
        ├── two.rs    # Level 2: Match with Data
        ├── three.rs  # Level 3: If Let
        ├── four.rs   # Level 4: Let Else
        └── five.rs   # Level 5: Choose the Right Tool
```

## 💡 Pattern Quick Reference

### Match Expression
```rust
match value {
    Pattern1 => action1,
    Pattern2 => action2,
    _ => default,
}
```
**Use when:** You need to handle multiple patterns

### If Let
```rust
if let Pattern = value {
    // Do something
} else {
    // Do something else
}
```
**Use when:** You only care about one pattern

### Let Else
```rust
let Pattern = value else {
    return early_value;
};
// Continue with unwrapped value
```
**Use when:** You want early return on failure

## 🎯 Challenge Tips

1. **Start with Level 1** - Don't skip ahead
2. **Read the hints** - They point you in the right direction
3. **Use the compiler** - Rust's error messages are helpful!
4. **Test often** - Run `cargo run` after each challenge
5. **Don't peek** - No solutions provided, struggle is part of learning!

## 📖 When You're Stuck

1. Re-read The Rust Book chapter on control flow
2. Check the hints in each challenge
3. Look at the test runner to understand expected output
4. Ask for help (but don't copy solutions!)

## ✅ Completion Checklist

- [ ] Level 1: All 7 match challenges
- [ ] Level 2: All 8 match with data challenges
- [ ] Level 3: All 8 if let challenges
- [ ] Level 4: All 10 let else challenges
- [ ] Level 5: All 10 "choose the right tool" challenges
- [ ] Reflection: Answer the questions in Level 5

## 🎓 After Completing

You should be able to:
- Choose between match, if let, and let...else appropriately
- Extract data from enum variants
- Handle Option and Result elegantly
- Write idiomatic Rust control flow

## 📖 Resources

- [The Rust Book - Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [The Rust Book - Pattern Matching](https://doc.rust-lang.org/book/ch18-00-patterns.html)
- [Rust by Example - Match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)
- [Rust by Example - If Let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [Rust by Example - Let Else](https://doc.rust-lang.org/rust-by-example/flow_control/let_else.html)

---

**Author:** @hitzvera  
**Created:** April 2026  
**Rust Edition:** 2024

**Remember:** The struggle is where learning happens. Don't give up! 💪🦀
