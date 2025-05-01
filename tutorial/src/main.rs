#!allow

fn main(){
  // let x: i32 = -42;
  // let y: u64 = 100;
  // println!("Signed interger: {}", x);
  // println!("Unsigned interger: {}", y);

  // // compound data types 
  // // arrays, tuples, slices and strings 

  // let numbers: [i32;5]= [1,2,3,4,5];
  // println!("Array numbers: {:?}", numbers);

  // //Strings it is store in the heap storages 
  // let mut stone_cold: String = String :: from("Hell, ");
  // stone_cold.push_str("yeah");
  // println!("Stone cold says: {}", stone_cold);


  // // string slices it is stored in stacked and it is faster than the String
  // let strng: String = String :: from("Hello, there!");
  // let slice: &str = &strng[0..5];
  // println!("This is slice, {}", slice);
  tell_height(192);
  human_info("Titus", 24, 189.0 );

  let _x = {
    let qnty = 10;
    let price = 5;
    qnty * price // this is special to rust you can do mathematical calculations can be done at the end. This is an ex
  };
  println!("The result is: {}", _x);

  let y = add(5, 5);
  println!("The output is: {}", y);

  let height = 70.0;
  let weight = 75.0;
  let bmi = calc_bmi(height, weight);
  println!("Your bmi, {}", bmi);






}

fn tell_height(height: u32) {
  println!("Enter your height: {}.cm", height)
}


fn human_info(name: &str, age: u32, height: f32){
  println!("My name is {}. I am {} years old, and my height is {}.cm", name, age, height);
}


// function for returning values
fn add(a: i32, b:i32) -> i32{
  a + b
}

// Expressions and statements
// statements ends with ; almost all statements in Rust


// final example
fn calc_bmi(height_cm: f64, weight_kg:f64) -> f64{
  height_cm * weight_kg
}