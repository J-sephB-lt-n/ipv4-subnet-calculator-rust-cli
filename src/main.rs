use regex::Regex;
use std::io::{self, Write};

// TODO: Does cidr_num need to be as big as u32?

fn main() {
    print!("Enter a valid IPv4 CIDR (e.g. '192.168.1.1/22'): ");
    io::stdout().flush().unwrap();
    let mut user_cidr_input = String::new();
    io::stdin()
        .read_line(&mut user_cidr_input)
        .expect("Failed to read line");
    user_cidr_input = user_cidr_input.trim().to_string();

    let (octets, cidr_num) = parse_user_cidr_input(&user_cidr_input);

    let num_avail_ip_addresses: u32 = 2u32.pow(32u32 - cidr_num);
    println!(
        "There are {} IP addresses in this subnet",
        num_avail_ip_addresses
    );
    println!("{} IP addresses are assignable to hosts (1 IP is reserved for network address and 1 for broadcast address)", num_avail_ip_addresses-2);
    let binary_num_str = octets_to_binary_num_str(&octets);
    let start_ip_str = format!("{:0<32}", &binary_num_str[..cidr_num as usize]);
    let end_ip_str = format!("{:1<32}", &binary_num_str[..cidr_num as usize]);
    println!(
        "Subnet start IP: {}.{}.{}.{}\nSubnet end IP: {}.{}.{}.{}",
        &start_ip_str[0..8],
        &start_ip_str[8..16],
        &start_ip_str[16..24],
        &start_ip_str[24..32],
        &end_ip_str[0..8],
        &end_ip_str[8..16],
        &end_ip_str[16..24],
        &end_ip_str[24..32]
    );
}

fn parse_user_cidr_input(user_cidr_input: &str) -> ([u8; 4], u32) {
    let parse_subnet_regex =
        Regex::new(r"(\d{1,3}).(\d{1,3}).(\d{1,3}).(\d{1,3})\/(\d{1,2})").unwrap();
    let subnet_captures = parse_subnet_regex.captures(&user_cidr_input).unwrap();
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

    (octets, cidr_num)
}

fn octets_to_binary_num_str(octets: &[u8; 4]) -> String {
    let binary_num_str: String = octets
        .iter()
        .map(|octet| format!("{:08b}", octet))
        .collect();
    binary_num_str
}
