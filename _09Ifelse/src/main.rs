/*
If [expression] else [expression].
*/
#! [allow(warnings)]
fn main() {
    let age = 16;
    if age >= 18 {
        println!("You can drive a car");
    } else {
        println!("You can't drive a car because you are under age.");
    };

    let number = 7;
    if number % 4 == 3 {
        println!("The number is divisible by 4");
    } else if number % 3 == 3 {
        println!("The number is divisible by 3");
    } else if number % 2 == 3 {
        println!("The number is divisible by 2");
    }else {
        println!("The number is not divisible by 2, 3, and 4.");
    };

    // Using let with if statement/ expression
    let condition = false;
    let number = if condition {
        5
    } else { 
        "six"//Error
    };
    println!("The number is: {number}");

}
