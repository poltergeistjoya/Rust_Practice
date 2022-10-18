fn main() {
    let guess: u32 = "42".parse().expect("Not a Number!");

    //scalar types are singular values: ints floats, bools, characters

    // //Integer overflow in release mode this will make 256 = 1 by wrapping
    // let x: u8 = 256;
    // println!("{x}");

    let x = 2.0;
    let y: f32 = 3.0;

    //addition
    let sum = 5 + 10;

    //Subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    //division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;

    //remainder
    let remainder = 43 % 5;

    println!("sum:{sum}, difference:{difference}, product:{product}, quotient:{quotient}, floored:{floored}, remainder:{remainder}");

    //char types can be anything which the character encoding allows it to be
    let c = 'z';
    let z: char = '≈'; //chars are denoted with single quote! not double!!

    //compound types can group multiple values into one type: tuples and arrays
    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //use pattern matching to get values out of tuples - destructuring seems kind of stupid tbh when you can just use the .
    let(a,b,c) = tup;
    println!("The value of b is: {b}");
    let five_hundred = tup.0;
    println!("{five_hundred}");

    //empty tuple is known as unit
    //expressions implicitly return unit val if they don't return any other value
    let empty_tup = ();

    //arrays, every elt must be of the same type
    //helpful when you want data on stack and not heap
    //use vectors when size of array needs to be changed
    let array = [1,2,3,4,5];
    let arr: [i32; 5] = [1,2,3,4,5];
    let arr_of_five_threes = [3;5];

    let first = array[0];
}
