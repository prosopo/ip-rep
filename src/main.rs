use clap::{Arg, Command};
use dotenv::dotenv;
use ipinfo::{IpInfo, IpInfoConfig};
use reqwest;
use serde_json::Value;
use std::env;
use std::net::IpAddr;
use tokio;

#[tokio::main]
async fn main() {
    print_ascii_art();
    dotenv().ok(); // Load environment variables from .env file

    let ip = parse_command_line_arguments();
    retrieve_and_print_geoip2_data(&ip).await;
    perform_ipinfo_lookup(&ip).await;
}

fn print_ascii_art() {
    println!(
        r"
------------------------------------------------------
_____                                   _____ _____   
|  __ \                                 |_   _|  __ \  
| |__) | __ ___  ___  ___  _ __   ___     | | | |__) | 
|  ___/ '__/ _ \/ __|/ _ \| '_ \ / _ \    | | |  ___/  
| |   | | | (_) \__ \ (_) | |_) | (_) |  _| |_| |      
|_|   |_|  \___/|___/\___/| .__/ \___/  |_____|_|      
    /\               | |  | |                          
   /  \   _ __   __ _| |_ |_| _______ _ __             
  / /\ \ | '_ \ / _` | | | | |_  / _ \ '__|            
 / ____ \| | | | (_| | | |_| |/ /  __/ |               
/_/    \_\_| |_|\__,_|_|\__, /___\___|_|               
                         __/ |                         
                        |___/   
------------------------------------------------------                       
"
    );
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

async fn retrieve_and_print_geoip2_data(ip: &IpAddr) {
    match get_geoip2_data(ip).await {
        Ok(response) => print_geoip2_data(&response),
        Err(e) => {
            eprintln!("Failed to retrieve geolocation data: {}", e);
            std::process::exit(1);
        }
    }
}

async fn perform_ipinfo_lookup(ip: &IpAddr) {
    let ipinfo_token = env::var("IP_INFO_TOKEN").expect("IP_INFO_TOKEN not set in .env file");
    let config = IpInfoConfig {
        token: Some(ipinfo_token),
        ..Default::default()
    };
    let mut ipinfo = IpInfo::new(config).expect("Failed to construct IpInfo");

    match ipinfo.lookup(&ip.to_string()).await {
        Ok(result) => println!("{} lookup result: {:?}", ip, result),
        Err(e) => println!("Error occurred: {}", e),
    }
}

async fn get_geoip2_data(ip: &IpAddr) -> Result<Value, reqwest::Error> {
    let account_id = env::var("GEOIP2_ACCOUNT_ID").expect("GEOIP2_ACCOUNT_ID not set in .env file");
    let license_key =
        env::var("GEOIP2_LICENSE_KEY").expect("GEOIP2_LICENSE_KEY not set in .env file");

    let url = format!("https://geolite.info/geoip/v2.1/city/{}?pretty", ip);
    let client = reqwest::Client::new();
    client
        .get(url)
        .basic_auth(account_id, Some(license_key))
        .send()
        .await?
        .json::<Value>()
        .await
}

fn print_geoip2_data(response: &Value) {
    if let Some(city) = response["city"]["names"]["en"].as_str() {
        println!("City: {}", city);
    }
    if let Some(country) = response["country"]["names"]["en"].as_str() {
        println!("Country: {}", country);
    }
    if let Some(continent) = response["continent"]["names"]["en"].as_str() {
        println!("Continent: {}", continent);
    }
    if let Some(subdivisions) = response["subdivisions"].as_array() {
        for subdivision in subdivisions {
            if let Some(sub_name) = subdivision["names"]["en"].as_str() {
                println!("Subdivision: {}", sub_name);
            }
        }
    }
    if let Some(traits) = response["traits"].as_object() {
        println!("{:#?}", traits);
    }
}
