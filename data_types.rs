fn main() {
    let _x = 1;
    
    let _y = 2.5;
    
    let _z: i64 = 4545454545;
    
    // find max size
    println!("Max i32: {}", std::i32::MAX);
    
    println!("Max i64: {}", std::i64::MAX);
    
    let is_active = true;
    
    let is_greater: bool = 10 < 5;
    
    let a1 = 'a';
    let face = '\u{1F600}';
    
    println!("{:?}", (_x, _y, _z, is_active, is_greater, a1, face));
}
