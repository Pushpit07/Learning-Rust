fn main() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person = true;
    
    if age >= 21 && check_id || knows_person {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }
    
    // Shorthand if
    let is_of_age = if age >= 21 {true} else {false};
}
