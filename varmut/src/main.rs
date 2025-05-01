// Variables and Mutability
// Every variables in rust in immutable by default
fn main() {
    println!("Hello, world!");
    // immutable variable
    let a = 50;
    println!("The value of variable of a is {}", a);
    // Mutable variable
    let mut x = 1;
    let x = 20;
    println!("The value of x has been change from 1 to 20 with mut method. The value is {}", x);
}
