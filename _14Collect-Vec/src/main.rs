#! [allow(warnings)]

fn main() {
    let mut numbers =Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);
    numbers.push(5);
    println!("The new arrar is {:?}", numbers);
    vec_num();
}


fn vec_num(){
    // using macro way of creating a vector
    let mut _v:Vec<i32> = vec![];
    _v.push(20);
    _v.push(21);
    _v.push(22);
    _v.push(23);
    _v.push(24);    
    _v.push(25);
    _v.push(26);
    _v.push(27);
    let third = _v.get(2);
    match third {
        Some(third) => println!("The index from the get method is:  {third}"),
        None => println!("There is no third element "),
    }


}