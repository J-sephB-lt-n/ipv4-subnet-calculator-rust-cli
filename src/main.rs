use std::io::{self, Write};

fn main() {
    print!("Enter a valid IPv4 CIDR (e.g. '192.168.1.1/22'): ");
    io::stdout().flush().unwrap();
    
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    user_input = user_input.trim().to_string();

    println!("received input '{}'", &user_input);
}
