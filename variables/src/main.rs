// demonstrates immutability
// fn main() {
//     let mut x = 5; // we must assign 'mut'  to make x mutable
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
//     const SPEED_OF_LIGHT: u32 = 299_792_458;
//     println!("The value of the speed of light is: {} m/s", SPEED_OF_LIGHT);
//     // CONSTANTS are always immutable, and the type must be annotated.
// }
//In Rust, the compiler guarantees that when you state that a value won’t change, it really won’t change.
// That means that when you’re reading and writing code, you don’t have to keep track of how and where a value might change.
// Your code is thus easier to reason through.

// Shadowing:
fn main() {
    //Shadowing is different from marking a variable as mut, because we’ll get a compile-time error
    // if we accidentally try to reassign to this variable without using the let keyword.
    // By using let, we can perform a few transformations on a value but have the variable
    // be immutable after those transformations have been completed.
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    //The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again,
    // we can change the type of the value but reuse the same name. 
    let spaces = "   ";
    let spaces = spaces.len();
    println!("the length of spaces is {}", spaces)
}
