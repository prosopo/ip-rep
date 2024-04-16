use clap::{Command, Arg};
use dotenv::dotenv;
use reqwest;
use serde_json::Value;
use std::env;
use std::net::IpAddr;
use tokio;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables from .env file
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

    match get_geoip2_data(&ip).await {
        Ok(response) => print_geoip2_data(&response),
        Err(e) => {
            eprintln!("Failed to retrieve geolocation data: {}", e);
            std::process::exit(1);
        }
    }
}


async fn get_geoip2_data(ip: &IpAddr) -> Result<Value, reqwest::Error> {
    let account_id = env::var("GEOIP2_ACCOUNT_ID").expect("GEOIP2_ACCOUNT_ID not set in .env file");
    let license_key = env::var("GEOIP2_LICENSE_KEY").expect("GEOIP2_LICENSE_KEY not set in .env file");

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

