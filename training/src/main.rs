#![allow(warnings)]

mod database;
pub mod helpers;
#[tokio::main]
async fn main() {
    let my_name = helpers::helpersmod::get_full_name("Titus", " Kiptoo", " Kibet");
    println!("My name is {my_name}");

    println!("\n🔥 All users in database:");
    match database::get_user_full_names().await {
        Ok(names) => {
            for name in names {
                println!("👤 {}", name);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to fetch names: {e}");
        }
    }
    let age = helpers::privatefns::get_age(23);
    println!("The new age is {0}", age);
}
