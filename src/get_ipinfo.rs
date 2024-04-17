use ipinfo::{IpDetails, IpInfo, IpInfoConfig};
use std::env;
use std::net::IpAddr;

pub async fn display_ipinfo_data(ip: &IpAddr) {
    let ipinfo_token = env::var("IP_INFO_TOKEN").expect("IP_INFO_TOKEN not set in .env file");
    let config = IpInfoConfig {
        token: Some(ipinfo_token),
        ..Default::default()
    };
    let mut ipinfo = IpInfo::new(config).expect("Failed to construct IpInfo");

    println!("\n----------------");
    println!("IPInfo Data Summary:");
    println!("----------------\n");

    match ipinfo.lookup(&ip.to_string()).await {
        Ok(details) => print_ip_details(&details),
        Err(e) => println!("Error occurred: {}", e),
    }
}

fn print_ip_details(details: &IpDetails) {
    println!("IP: {}", details.ip);
    if let Some(hostname) = &details.hostname {
        println!("Hostname: {}", hostname);
    }
    println!("City: {}", details.city);
    println!("Region: {}", details.region);
    if let Some(country_name) = &details.country_name {
        println!("Country: {} ({})", country_name, details.country);
    } else {
        println!("Country: {}", details.country);
    }
    if let Some(flag) = &details.country_flag {
        println!("Country Flag: {} {}", flag.emoji, flag.unicode);
    }
    if let Some(currency) = &details.country_currency {
        println!("Currency: {} ({})", currency.symbol, currency.code);
    }
    if let Some(cont) = &details.continent {
        println!("Continent: {}", cont.name);
    }
    println!("Location: {}", details.loc);
    if let Some(org) = &details.org {
        println!("Organization: {}", org);
    }
    if let Some(postal) = &details.postal {
        println!("Postal Code: {}", postal);
    }
    if let Some(timezone) = &details.timezone {
        println!("Time Zone: {}", timezone);
    }
}
