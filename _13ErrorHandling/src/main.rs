#! [allow(warnings)]
// Error Handling
// fn main() {
    // Aprroach 1
    // enum Option<T> { // Defines the generic Option type
    //     Some(T), // Represens a value
    //     None, // Represents no value
    // };

    // // Aproach 2
    // enum Result<T, E> { // Define errors that cannot succedd 
    //     Ok(T), //
    //     Err(E),
    // };
    fn divideOption(numerator: f64, denominator: f64) -> Option<f64>{
        if denominator == 0.0 {
            None
        } else {
            Some (numerator/ denominator)
        }

    }
    
// };
fn divideResult(numerator: f64, denominator: f64) -> Result<f64, String>{
    if denominator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(numerator/ denominator)
    }
}
// Approach Option
fn main(){
    // let result = divideOption(10.0, 2.0);
    // match result {
    //     Some(x) => println!("Result {}", x),
    //     None => println!("Cannot divide by Zero"),
    // }
    match divideResult(10.00,7.958){
        Ok(result) => println!("The results is {}", result),
        Err(err) => println!("The error is {}", err),
    }
}





