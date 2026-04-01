/**
 * Exercise 3 — Floating Numbers

Create two variables:

price = 19.99
tax = 0.07

Both must use f64.

Then compute:

total = price + (price * tax)

Print the result.
 */

pub fn exercise_number_three() {
    let price: f64 = 19.99;
    let tax: f64 = 0.07;

    let total: f64 = price + (price * tax);

    println!("{total}")
}
