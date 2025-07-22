use std::io;

fn main() -> ! {

    println!("Reliquary is working!");

    // Define magic item categories and print status
    let common: i32 = 0;
    let uncommon: i32 = 0;
    let rare: i32 = 0;
    let very_rare: i32 = 0;
    let legendary: i32 = 0;

    println!("You have {common} common, {uncommon} uncommon, {rare} rare, {very_rare} very rare, and {legendary} legendary magic items. ");

    loop {
        println!("To log a new common magic item, enter a number:");

        // Obtain user input as {input}
        let mut user_input: String = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Line should be a valid string.");
        
        let common: i32 = match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

    println!("You have {common} common magic items.");

    }
    
}