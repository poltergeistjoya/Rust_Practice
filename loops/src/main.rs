fn main() {
    //loop runs FOREVER unless we break or ^C
    loop{
        println!("again!");
        break;
    }

    //can use loop to retry an op you know will fail 
    //aka continually check 
    let mut counter = 0; 
    
    let result = loop{
        counter +=1;
        
        if counter == 10{
            break counter *2;
        }
    };

    println!("The result is {result}");

    // //loop labels good to break a more outer loop
    // let mut count = 0;
    // 'counting_up: loop{
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count ==2 {
    //             break 'counting_up;
    //         }
    //         remaining -=1
    //     }
    //     count +=1;
    // }
    // println!("End count = {count}")

    

}
