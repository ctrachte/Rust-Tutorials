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
// fn main() {
//     //Shadowing is different from marking a variable as mut, because we’ll get a compile-time error
//     // if we accidentally try to reassign to this variable without using the let keyword.
//     // By using let, we can perform a few transformations on a value but have the variable
//     // be immutable after those transformations have been completed.
//     let x = 5;

//     let x = x + 1;

//     let x = x * 2;

//     println!("The value of x is: {}", x);

//     //The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again,
//     // we can change the type of the value but reuse the same name. 
//     let spaces = "   ";
//     let spaces = spaces.len();
//     println!("the number of spaces is {}", spaces);
// }

// Destructuring a tuple:
// fn main() {

//     //The variable tup binds to the entire tuple, because a tuple is considered a single compound element.
//     // To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
//     let tup: (i32, f64, u8) = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
//     println!("The value of z is: {}", z);

// }

// If/Else example:
// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//     // let number = if condition { 5 } else { "six" }; // not allowed

//     println!("The value of number is: {}", number);
// }

// Loop example:
// fn main() {
//     // Before the loop, we declare a variable named counter and initialize it to 0.
//     let mut counter = 0;
//     // Then we declare a variable named result to hold the value returned from the loop.
//     let result = loop {
//         // On every iteration of the loop, we add 1 to the counter variable, and then check whether the counter is equal to 10. 
//         counter += 1;
//         if counter == 10 {
//             // When it is, we use the break keyword with the value counter * 2. 
//             break counter * 2; // After the loop, we use a semicolon to end the statement that assigns the value to result.
//         }
//     };
//     // Finally, we print the value in result, which in this case is 20.
//     println!("The result is {}", result);
// }

// Indicating a return type
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
fn main() {
    let x = five();
    let five_plus_one = plus_one(five());
    println!("The value of x is: {}", x);
    println!("The value of x plus one is: {}", five_plus_one);
}