use std::io;

fn main() {

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Reliquary is working!");
}