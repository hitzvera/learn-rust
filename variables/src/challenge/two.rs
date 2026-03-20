/**
 * 2. Type Annotation

Create:

A variable y with value 10

Explicitly set its type to i32

Then:

Change it to u32

Try assigning -1 and observe

👉 Goal: Signed vs unsigned types
 */

pub fn two() {
    let y: i32 = 10;
    // let y: u32 = -1; result an error cannot apply unary operator
    println!("The value of y is: {y}");
}
