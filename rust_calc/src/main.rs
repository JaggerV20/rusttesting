use std::io;

fn main() {
    //Input test
    println!("Enter a number: ");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    //Shadowing
    let user_input: u32 = user_input.trim().parse().expect("Please type a number!");

    println!("User Input: {user_input}");
}
