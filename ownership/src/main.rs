#! allow
// Ownership rules 

// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.


// // Rule 1: Each value in Rust has an owner.
// fn main() {
//     let s1 = String :: from("Hello");
//     let len = calc_legnth(&s1);
//     println!("The length of owner is: {} and it is {}", len, s1);

// }

// fn calc_legnth(s: &String) -> usize{
//     s.len()
// }

// Rule 2: There can only be one owner at a time.
// fn main() {
//     let s1 = String::from("Rust");
//     let s2 = s1;
//     println!("These are the owners s1 , s2 and the real owner is {} s2",s2);
// }