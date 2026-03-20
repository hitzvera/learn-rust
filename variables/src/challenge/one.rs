/**
 * 🟢 Level 1 — Basics (If you get these wrong, reread)
1. Immutable vs Mutable

Write a program that:

Creates a variable x = 5

Try to change it to 6 (should fail)

Fix it properly

👉 Goal: Understand let vs let mut
 */
pub fn exercise() {
    // answer
    let mut x = 5;
    x = 6;

    println!("The value of x is: {x}");
}
