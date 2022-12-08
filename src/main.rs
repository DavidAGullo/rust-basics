//! This is not a fully working program
//! # Variable Printing Examples
//! 
//! ```
//! fn main() 
//! ```
//! This provides main function all available variables in scope of the function
//! this will do nothing currently
fn main() {
    let name :&str = "John";
    let age :i32 = 43;
    let weight = 180.5;
    let isMale = true;
    let isTall = false;
    // Printing all variables
    println!("{} is {} years old, weighs {} pounds, isMale: {}, isTall: {}", name, age, weight, isMale, isTall);

    //Where is matters
    failedPrint();

}

fn failedPrint() {
    let amount = 100000000000000000000000;
    println!("Amount: {}", amount); // This will error out

    //error: literal out of range for `i32`
}