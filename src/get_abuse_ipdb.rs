use reqwest::{Client, Error};
use serde_json::Value;
use std::{env, net::IpAddr};

pub async fn display_ip_abuse_data(ip_address: &IpAddr) {
    match fetch_ip_abuse_data(ip_address).await {
        Ok(response) => print_ip_abuse_data(&response),
        Err(e) => {
            eprintln!("Failed to retrieve IP abuse data: {}", e);
            std::process::exit(1);
        }
    }
}

async fn fetch_ip_abuse_data(ip_address: &IpAddr) -> Result<Value, Error> {
    // Build the client
    let client = Client::new();

    let api_key = env::var("ABUSEIPDB_API_KEY").expect("ABUSEIPDB_API_KEY not set");

    // Construct the request URL and parameters
    let url = "https://api.abuseipdb.com/api/v2/check";
    let response = client
        .get(url)
        .query(&[
            ("ipAddress", ip_address.to_string()),
            ("maxAgeInDays", "90".to_string()),
            ("verbose", "".to_string()),
        ])
        .header("Key", api_key)
        .header("Accept", "application/json")
        .send()
        .await?
        .json::<Value>()
        .await;

    response
}

fn print_ip_abuse_data(response: &Value) {
    println!("\nIP Abuse Data Summary:");
    println!("----------------------");

    if let Some(data) = response["data"].as_object() {
        if let Some(ip) = data.get("ipAddress").and_then(Value::as_str) {
            println!("IP Address: {}", ip);
        }
        if let Some(country_code) = data.get("countryCode").and_then(Value::as_str) {
            println!("Country Code: {}", country_code);
        }
        if let Some(country_name) = data.get("countryName").and_then(Value::as_str) {
            println!("Country: {}", country_name);
        }
        if let Some(domain) = data.get("domain").and_then(Value::as_str) {
            println!("Domain: {}", domain);
        }
        if let Some(isp) = data.get("isp").and_then(Value::as_str) {
            println!("ISP: {}", isp);
        }
        if let Some(hostnames) = data.get("hostnames").and_then(Value::as_array) {
            for hostname in hostnames {
                if let Some(host) = hostname.as_str() {
                    println!("Hostname: {}", host);
                }
            }
        }
        if let Some(abuse_confidence_score) =
            data.get("abuseConfidenceScore").and_then(Value::as_i64)
        {
            println!("Abuse Confidence Score: {}", abuse_confidence_score);
        }
        if let Some(is_public) = data.get("isPublic").and_then(Value::as_bool) {
            println!("Is Public: {}", is_public);
        }
        if let Some(is_tor) = data.get("isTor").and_then(Value::as_bool) {
            println!("Is Tor Network: {}", is_tor);
        }
        if let Some(total_reports) = data.get("totalReports").and_then(Value::as_i64) {
            println!("Total Reports: {}", total_reports);
        }
        if let Some(last_reported_at) = data.get("lastReportedAt").and_then(Value::as_str) {
            println!("Last Reported At: {}", last_reported_at);
        } else {
            println!("Last Reported At: None");
        }
    } else {
        println!("Failed to parse data or data object missing");
    }

    println!();
}
