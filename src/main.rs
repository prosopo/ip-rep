mod ascii_banner;
mod get_abuse_ipdb;
mod get_freegeoip;
mod get_geoip2;
mod get_ipinfo;

use ascii_banner::print_ascii_art;
use clap::{Arg, Command};
use dotenv::dotenv;
use get_abuse_ipdb::display_ip_abuse_data;
use get_freegeoip::display_freegeoip_data;
use get_geoip2::display_geoip2_data;
use get_ipinfo::display_ipinfo_data;
use std::net::IpAddr;
use tokio;

#[tokio::main]
async fn main() {
    print_ascii_art();
    dotenv().ok(); // Load environment variables from .env file

    let ip = parse_command_line_arguments();

    // Geolocation data

    display_geoip2_data(&ip).await;
    display_ipinfo_data(&ip).await;
    display_freegeoip_data(&ip).await;

    // Reputational data

    display_ip_abuse_data(&ip).await;
}

fn parse_command_line_arguments() -> IpAddr {
    let matches = Command::new("IP Reputation CLI")
        .version("1.0")
        .author("Your Name <your_email@example.com>")
        .about("Checks the reputation of an IP address")
        .arg(
            Arg::new("IP")
                .help("The IP address to check")
                .required(true)
                .index(1),
        )
        .get_matches();

    matches
        .get_one::<String>("IP")
        .expect("IP argument missing")
        .parse()
        .expect("Invalid IP address")
}
