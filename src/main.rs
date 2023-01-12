//! This is not a fully working program
//! its mostly just snippets of code that I have used to learn rust
//! 

//allows for dead code to be ignored
//Dead Code is code that is not used
#![allow(dead_code)]


/// ```
/// fn main() 
/// ```
/// This provides main function all available variables in scope of the function
/// this will do nothing currently
fn main() {
    let name :&str = "John";
    let age :i32 = 43;
    let weight = 180.5;
    let is_male = true;
    let is_tall = false;
    // Printing all variables
    //println!("{} is {} years old, weighs {} pounds, isMale: {}, isTall: {}", name, age, weight, is_male, is_tall);

    //Where is matters
    //failedPrint();
    //good_print();
    //variables_print();
    string_func();
}
/// ```
/// fn failed_print() 
/// ```
/// This code will always fail, because the number is too large for i32
/// 
fn failed_print() {
    //let amount = 10000000000000000;
    let amount = 2147483647;
    println!("Amount: {}", amount); // This will error out

    //error: literal out of range for `i32`
}

/// ```
/// fn good_print()
/// ```
/// This code will always work, because the number is large enough and can be stored in that data type
fn good_print() {
    let amount :i64 = 10000000000000000;
    println!("Amount: {}", amount); // This will error out

    //error: literal out of range for `i32`
}

/// ```
/// fn variables_print()
/// ```
/// This code will print out the variables a, b, and c
/// each variable is written in different ways to show what is available to use
fn variables_print() {
    // Assign A a value of 15, B a value of 😁, C a value of Bean
    let (a, b, c) = (15, '\u{1F601}', "Bean");
    println!("{} {} {}", a, b, c);
}
fn string_func() {
    let cat: &str = "cat";
    //Static string
    let dog: &'static str = "dog";
    //Immutable string
    let cow: String = String::new();
    let mut cow: String = String::from("cow");
    println!("{} {} {}", cat, dog, cow);
    let mut owner = format!("{} is owned by {}", cow, "John");
    println!("{}", owner);
    //Length of owner
    println!("Length of owner: {}", owner.len());
    cow.push_str(" is a cow"); // String version of .push()
    let mut pig = String::from("Turkey");
    pig.push_str(" is a pig");
    //Replace Turkey with Pig
    let new_pig = pig.replace("Turkey", "Pig");
    println!("{}", new_pig);
    //Check if string contains a word
    println!("Does owner contain 'is': {}", owner.contains("is"));
    //Loop through string by whitespace
    for word in owner.split_whitespace() {
        println!("{}", word);
    }
    //Create string with certain capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    //Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    println!("{}", s);

    //Constant string for googles url
    const GOOGLE_URL: &str = "https://www.google.com";
    println!("{}", GOOGLE_URL);

    //Arthmetic Examples
    let a = 10 + 3; //Addition operator 10 + 3 = 13 
    let b = 10 - 3; //Subtraction operator 10 - 3 = 7
    let c = 10 * 3; //Multiplication operator 10 * 3 = 30
    let d = 10 / 3; //Division operator 10 / 3 = 3 (integer division)
    let d_float= 10.0 / 3.0; //Division operator 10 / 3 = 3.3333333333333335 (float division)
    let e = 10 % 3; //Modulus operator  10 / 3 = 3 remainder 1

    println!("{} {} {} {} {} {}", a, b, c, d, d_float, e);
    //Increments and Decrements
    let mut f = 1;
    f += 1; // f = f + 1
    f -= 1; // f = f - 1
    f *= 2; // f = f * 2
    f /= 2; // f = f / 2
    f %= 2; // f = f % 2
    println!("{}", f); // f = 1

    //Relational Operators
    let g = 1 > 2; // Greater than operator 1 > 2 = false
    let h = 1 < 2; // Less than operator 1 < 2 = true
    let i = 1 >= 2; // Greater than or equal to operator 1 >= 2 = false
    let j = 1 <= 2; // Less than or equal to operator 1 <= 2 = true
    let k = 1 == 2; // Equal to operator 1 == 2 = false
    let l = 1 != 2; // Not equal to operator 1 != 2 = true

    println!("{} {} {} {} {} {}", g, h, i, j, k, l);

    //Logical Operators
    let m = true && false; // Logical AND operator true && false = false
    let n = true || false; // Logical OR operator true || false = true
    let o = !true; // Logical NOT operator !true = false

    println!("{} {} {}", m, n, o);

    //Bitwise Operators
    let p = 1 & 2; // Bitwise AND operator 1 & 2 = 0
    let q = 1 | 2; // Bitwise OR operator 1 | 2 = 3
    let r = 1 ^ 2; // Bitwise XOR operator 1 ^ 2 = 3
    let s = !1; // Bitwise NOT operator !1 = -2

    println!("{} {} {} {}", p, q, r, s);

    //Shift Operators
    let t = 1 << 2; // Shift left operator 1 << 2 = 4
    let u = 1 >> 2; // Shift right operator 1 >> 2 = 0

    println!("{} {}", t, u);

    //Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", tup.2);

    //Arrays
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);

    //Array with type
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);

    //Array with same value
    let a = [3; 5];
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);


}