use regex::Regex;
use std::io::{self, Write};

fn main() {
    print!("Enter a valid IPv4 CIDR (e.g. '192.168.1.1/22'): ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    user_input = user_input.trim().to_string();

    println!("received input '{}'", &user_input);

    let parse_subnet_regex =
        Regex::new(r"(\d{1,3}).(\d{1,3}).(\d{1,3}).(\d{1,3})\/(\d{1,2})").unwrap();
    let subnet_captures = parse_subnet_regex.captures(&user_input).unwrap();
    let octets: [u8; 4] = [1, 2, 3, 4].map(|i| {
        subnet_captures
            .get(i)
            .unwrap()
            .as_str()
            .parse::<u8>()
            .expect("Invalid octet")
    });
    let cidr: u8 = subnet_captures
        .get(5)
        .unwrap()
        .as_str()
        .parse::<u8>()
        .expect("Invalid CIDR");
    println!("{:#?}", &octets);
    println!("{}", &cidr);
}
