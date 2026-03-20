# 📝 Rust Learning Progress Log

> Detailed tracking of my Rust learning journey

---

## 2026

### March 2026

#### Week 12 (March 16-22, 2026)

**🎯 Goals:**
- [ ] Review Rust basics
- [ ] Complete Chapter 3 exercises
- [ ] Start CLI Todo App project

**📚 Study Sessions:**

| Date | Topic | Time Spent | Notes |
|------|-------|------------|-------|
| Mar 20 | Repo setup & roadmap | 30 min | Created comprehensive README with learning path |

**✅ Completed:**
- Set up GitHub tracking system
- Created learning roadmap
- Reviewed Hello World and Guessing Game

**📝 Notes:**
- Need to focus on ownership concepts
- Want to build practical projects soon

**🐛 Challenges:**
- None yet - just getting back into it

**💡 Insights:**
- Tracking progress on GitHub helps with accountability
- Should commit code more regularly

---

#### Week 11 (March 9-15, 2026)

*No activity recorded*

---

## 2024

### September 2024

#### Week 39 (Sep 23-29, 2024)

**🎯 Goals:**
- [x] Install Rust
- [x] Write first program
- [x] Understand Cargo

**📚 Study Sessions:**

| Date | Topic | Time Spent | Notes |
|------|-------|------------|-------|
| Sep 29 | Setup & Hello World | 1 hour | Installed Rust, created first project |

**✅ Completed:**
- Rust installation
- Hello World project
- Created GitHub repository

**📝 Notes:**
- Installation was smooth with rustup
- Cargo is intuitive coming from other package managers

**🐛 Challenges:**
- Understanding borrow checker (will be ongoing 😅)

**💡 Insights:**
- Rust's compiler errors are very helpful
- Syntax is cleaner than expected

---

#### Week 40 (Sep 30 - Oct 6, 2024)

**🎯 Goals:**
- [x] Complete Guessing Game tutorial
- [ ] Start Chapter 3

**📚 Study Sessions:**

| Date | Topic | Time Spent | Notes |
|------|-------|------------|-------|
| Oct 2 | Guessing Game | 2 hours | Built the classic CLI game |

**✅ Completed:**
- Guessing Game project
- Learned about:
  - `use` statements
  - `rand` crate
  - User input with `io`
  - Pattern matching with `match`
  - Loops with `loop`

**📝 Notes:**
- The guessing game was fun!
- Pattern matching is powerful
- Need to practice more with Result types

**🐛 Challenges:**
- Understanding `Result` and `unwrap()`
- When to use `match` vs `if let`

**💡 Insights:**
- Rust makes you think about error handling upfront
- The type system prevents many bugs

---

## 📊 Progress Statistics

### Time Tracking

| Month | Hours Spent | Projects | Chapters |
|-------|-------------|----------|----------|
| Sep 2024 | 3 | 2 | 2 |
| Oct 2024 | 0 | 0 | 0 |
| Nov 2024 | 0 | 0 | 0 |
| Dec 2024 | 0 | 0 | 0 |
| Jan 2026 | 0 | 0 | 0 |
| Feb 2026 | 0 | 0 | 0 |
| Mar 2026 | 0.5 | 0 | 0 |

**Total:** 3.5 hours | 2 projects | 2 chapters

### Skill Development

```
Rust Proficiency Self-Assessment (1-10):

Syntax:        ████████░░ 8/10
Ownership:     ████░░░░░░ 4/10
Borrowing:     ███░░░░░░░ 3/10
Lifetimes:     ██░░░░░░░░ 2/10
Traits:        ███░░░░░░░ 3/10
Generics:      ███░░░░░░░ 3/10
Error Handling:████░░░░░░ 4/10
Async/Await:   ░░░░░░░░░░ 0/10
Macros:        ░░░░░░░░░░ 0/10
```

---

## 🎯 Milestones

### Achieved ✅

- [x] **Sep 29, 2024:** First Rust program
- [x] **Oct 2, 2024:** First complete project (Guessing Game)
- [x] **Mar 20, 2026:** Set up progress tracking system

### Upcoming 🎯

- [ ] **Week 13, 2026:** Complete Chapter 3
- [ ] **Week 14, 2026:** Complete Chapter 4 (Ownership)
- [ ] **Week 16, 2026:** First personal project (CLI Todo)
- [ ] **Week 20, 2026:** Contribute to open source
- [ ] **Week 24, 2026:** Build web API

---

## 📖 Book Progress

```
The Rust Programming Language Progress:

Chapter 1:  [████████████████████] 100% ✅
Chapter 2:  [████████████████████] 100% ✅
Chapter 3:  [████░░░░░░░░░░░░░░░░]  20% 
Chapter 4:  [░░░░░░░░░░░░░░░░░░░░]   0%
Chapter 5:  [░░░░░░░░░░░░░░░░░░░░]   0%
Chapter 6:  [░░░░░░░░░░░░░░░░░░░░]   0%
Chapter 7:  [░░░░░░░░░░░░░░░░░░░░]   0%
Chapter 8:  [░░░░░░░░░░░░░░░░░░░░]   0%
Chapter 9:  [░░░░░░░░░░░░░░░░░░░░]   0%
Chapter 10: [░░░░░░░░░░░░░░░░░░░░]   0%
Chapter 11: [░░░░░░░░░░░░░░░░░░░░]   0%
Chapter 12: [░░░░░░░░░░░░░░░░░░░░]   0%
Chapter 13: [░░░░░░░░░░░░░░░░░░░░]   0%
Chapter 14: [░░░░░░░░░░░░░░░░░░░░]   0%
Chapter 15: [░░░░░░░░░░░░░░░░░░░░]   0%
Chapter 16: [░░░░░░░░░░░░░░░░░░░░]   0%
Chapter 17: [░░░░░░░░░░░░░░░░░░░░]   0%
Chapter 18: [░░░░░░░░░░░░░░░░░░░░]   0%
Chapter 19: [░░░░░░░░░░░░░░░░░░░░]   0%
Chapter 20: [░░░░░░░░░░░░░░░░░░░░]   0%

Overall: [██░░░░░░░░░░░░░░░░░░░░] 10%
```

---

## 🔧 Code Snippets & Learnings

### March 20, 2026 - Repo Setup

Learned about GitHub CLI for repo management:
```bash
# Clone repository
gh repo clone hitzvera/learn-rust

# Create issues for tracking
gh issue create --title "Complete Chapter 3" --label "learning"

# Track progress
gh project item-add
```

### October 2, 2024 - Guessing Game

Key learnings from the guessing game:
```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

**Key Concepts:**
- `use` statements for imports
- `match` for pattern matching
- `loop` for infinite loops
- `Result` type for error handling
- Mutable variables with `mut`

---

## 📝 Reflections

### What's Working Well
- ✅ Following The Rust Book structure
- ✅ Building projects as I learn
- ✅ Tracking progress on GitHub

### What Needs Improvement
- ❌ Consistency (long gaps between study sessions)
- ❌ Taking notes while learning
- ❌ Practicing exercises

### Action Items
1. [ ] Set aside 30 min daily for Rust
2. [ ] Write notes after each chapter
3. [ ] Do all exercises in The Book
4. [ ] Build one project per month

---

**Last Updated:** March 20, 2026  
**Next Review:** March 27, 2026
