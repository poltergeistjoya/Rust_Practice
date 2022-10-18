fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(7, 'm');
    let x = five();
    println!{"The value of x is: {x}"}
    let x = plus_one(5);
    println!("The value of x is {x}")
}

//function only needs to be defined in same scope as call
//order doesn't matter, above function can call below function
fn another_function(x: i32){
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32{
    5
}

fn plus_one(x: i32) -> i32{
    x+1
}