#! [allow(warnings)]

fn main() {
    let mut numbers = vec![];
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);
    numbers.push(5);
    println!("The new arrar is {:?}", numbers);
    vec_num();
}


fn vec_num(){
    let mut _v:Vec<i32> = vec![];
    _v.push(20);
    _v.push(21);
    _v.push(22);
    _v.push(23);
    _v.push(24);    
    _v.push(25);
    _v.push(26);
    _v.push(27);

    println!("The new array _v using vec {:?}", _v);

}