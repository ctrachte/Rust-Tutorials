// demonstrates immutability
fn main() {
    let mut x = 5; // we must assign 'mut'  to make x mutable
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
//In Rust, the compiler guarantees that when you state that a value won’t change, it really won’t change.
// That means that when you’re reading and writing code, you don’t have to keep track of how and where a value might change.
// Your code is thus easier to reason through.

