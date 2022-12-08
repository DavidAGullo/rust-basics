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
    let is_male = true;
    let is_tall = false;
    // Printing all variables
    println!("{} is {} years old, weighs {} pounds, isMale: {}, isTall: {}", name, age, weight, is_male, is_tall);

    //Where is matters
    //failedPrint();
    good_print();

}

fn failed_print() {
    //let amount = 10000000000000000;
    let amount = 2147483647;
    println!("Amount: {}", amount); // This will error out

    //error: literal out of range for `i32`
}
fn good_print() {
    let amount :i64 = 10000000000000000;
    println!("Amount: {}", amount); // This will error out

    //error: literal out of range for `i32`
}