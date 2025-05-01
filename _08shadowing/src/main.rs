/* 
Shadowing 
Shadowing is different from mutability is that mutability uses `mut` which makes the variable reassignable.
*/
fn main() {
    // original variable
    let x = 5;
    // this variable will overshadow the original variable and it can be overshadow by the other variable declare at the last line of scoper
    let x = x + 7;
    let x = x * 2;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    // this is shadowing
    let spaces = "";
    let spaces = spaces.len();

    println!("The number of spaces legnth is {spaces}");

    // this is mutability which returns error during compiling 
    let spaces = "   ";
    spaces = spaces.len(); // this will return error
}

// Shadowing helps us to reuse names without reassignig the values