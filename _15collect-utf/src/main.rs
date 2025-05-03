#! [allow(warnings)]
/*
UTF8 data types 
*/

fn main() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Updating the string by using push_str
    let mut h = String::from("foo");
    h.push_str("bar");

    println!("The new string is {h}");

    let mut club = String::from("Man");
    club.push_str("chester");
    club.push_str(" United");
    club.push_str(" FC");
    println!("My football club is {club}");


    // format! macro concation 
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let s3 = String::from("!");

    let greeting = format!("{s1}{s2}{s3}");

    println!("The greeting is {greeting}");

}
