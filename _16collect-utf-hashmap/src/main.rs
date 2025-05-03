fn main() {
use std::collections::HashMap;
let mut scores = HashMap::new();


scores.insert("Blue", 50);
scores.insert("Yellow", 100);

let team_name ="Yellow";

let score = scores.get(&team_name).copied().unwrap_or(0);
println!("Score for {} is {}", team_name, score);

}
