/*
Loops;
    1. 
*/
fn main() {
    let mut number = 0;
    let result = loop {
        number += 1;
        if number == 24 {
            break number - 300;
        }
    };
    println!("The result of the loop is {result}");

    // Nested Loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1; // this one will make to 9 so it will stop and count another one 
        }

        count += 1;
    };
    println!("End count = {count}");


    // While loop. This conditions runs and until a certain condition is met
    let mut days = 0;
    while days <= 10{
        println!("The number of days is {}", days);
        days += 1;
    };
    println!("LIFTOFF!!!");

    // for loop. it is used to iterable over a list or array
    let num = [1,2,3,4,5,6,7,8,9];
    let letter = ["a","b","c","d","d"];
    for number in num {
        println!("Itering over num array. Number {number}");
    };
    for alpha in letter {
        println!("Itering over a list. This is alphabet {alpha}.");
    };
}
