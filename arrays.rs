fn main() {
    // Arrays - Fixed list
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    
    println!("{:?}", numbers);
    
    // Get single val
    println!("Single val: {}", numbers[0]);
    
    // Re-assign
    numbers[3] = 40;
    println!("{:?}", numbers);
    
    // Arrays are stack allocated
    println!("Array occupies: {} bytes", std::mem::size_of_val(&numbers));
    
    // Get slice
    let slice: &[i32] = &numbers[0..2];
    
    println!("SLice: {:?}", slice);
}
