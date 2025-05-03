#! [allow(warnings)]
/*
Structs
it defines a name with a pieces of data 
*/
fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        phonenumber: u64,
        active: bool,
    };

    let user1 = User {
        username:String::from("toshcode"),
        email: String::from("dev@gmail.com"),
        sign_in_count: 2,
        phonenumber: 25470000,
        active: false,

    };
    // building struct from a build_user function
    fn build_user(username: String, email: String, phonenumber: u64) -> User {
        User {
        active: true,
        email,
        username,
        sign_in_count: 2,
        phonenumber,
    }
    };
    let user2 = User{
        email: String::from("dev.sudo@gmail.com"),
        ..user1
    };
    println!("The email of user2 is {}", user2.email);
    println!("The email of user1 is {}", user1.email);



}
