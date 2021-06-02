fn main() {
    // Vectors - Resizeable arrays
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    
    // Add elem
    numbers.push(6);
    
    println!("{:?}", numbers);
    
    // Get single val
    println!("Single val: {}", numbers[0]);
    
    // Re-assign
    numbers[3] = 40;
    
    numbers.pop();
    println!("{:?}", numbers);
    
    // Vectors are stack allocated
    println!("Array occupies: {} bytes", std::mem::size_of_val(&numbers));
    
    // Get slice
    let slice: &[i32] = &numbers[0..2];
    
    println!("SLice: {:?}", slice);
    
    // Loop
    for x in numbers.iter() {
        println!("Number: {}", x);
    }
    
    // Loop and mutate
    for x in numbers.iter_mut() {
        *x*=2;
    }
    
    println!("Numbers: {:?}", numbers);
}
