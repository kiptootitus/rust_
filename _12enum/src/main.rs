#! [allow(warnings)]
/*
Enum is a spec way to have more types of a data like ip address and it can be used anywhere in the code 
*/
fn main() {
    enum IpAddrKind {
        V4(String),
        V6(String),
    };

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    println!("The Ip address kind of 4 is", );

}
