use clap::{Command, Arg};
use std::net::IpAddr;
use tokio;

#[tokio::main]
async fn main() {
    let matches = Command::new("IP Reputation CLI")
        .version("1.0")
        .author("Your Name <your_email@example.com>")
        .about("Checks the reputation of an IP address")
        .arg(Arg::new("IP")
            .help("The IP address to check")
            .required(true)
            .index(1))
        .get_matches();

    let ip_str = matches.get_one::<String>("IP").expect("IP argument missing");
    let ip: IpAddr = ip_str.parse().expect("Invalid IP address");

    let score = get_reputation_score(ip).await;
    println!("Reputation score for {}: {}", ip, score);
}

async fn get_reputation_score(ip: IpAddr) -> i32 {
    // Placeholder for the real implementation
    println!("Checking reputation for {}", ip);
    100
}
