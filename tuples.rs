fn main() {
    // Max 12 elements
    let person: (&str, &str, i8) = ("Brad", "Mass", 27);
    
    println!("{} is from {} and is {}", person.0, person.1, person.2);
}
