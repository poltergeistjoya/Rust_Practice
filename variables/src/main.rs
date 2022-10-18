fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6; 
    println!("The value of x is {x}");

    //const declaration
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    
    //shadowing creates new variables, older variables with same name
    //are overshadowed by new ones
    let y = 5;
    let y = y + 1; 
    {
        let y = y *2;
        println!("The value of y in the inner scope is {y}");
    }
    println!("The value of y is: {y}");

    //shadowing allows for variable with same name to have different types
    let spaces = "    ";
    let spaces = spaces.len();
    println!("{spaces}")
}
