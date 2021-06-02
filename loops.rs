fn main() {
    // Loops
    
    let mut count = 0;
    // Infinite_loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);
        
    //     if count == 10 {
    //         break;
    //     }
    // }
    
    // While
    while count <= 100 {
        if count%15 == 0 {
            println!("FizzBuzz");
        } else if count%3 == 0 {
            println!("Fizz");
        } else if count%5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }
        
        count += 1;
    }
    
    // For
    for x in 0..100 {
        if x%10 == 0 {
            println!("Div");
        }
    }
}
