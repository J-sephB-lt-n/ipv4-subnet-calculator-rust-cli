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
    let cidr_num: u32 = subnet_captures
        .get(5)
        .unwrap()
        .as_str()
        .parse::<u32>()
        .expect("Invalid CIDR");
    let num_avail_ip_addresses: u32 = 2u32.pow(32u32-cidr_num);
    println!("There are {} IP addresses in this subnet\n\t(2 are reserved - 1 for network address and 1 for broadcast address)", num_avail_ip_addresses);
    // println!("{:#?}", &octets);
    // println!("{}", &cidr_num);
}
