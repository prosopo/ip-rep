use serde_json::Value;
use std::{env, net::IpAddr};

pub async fn display_geoip2_data(ip: &IpAddr) {
    match get_geoip2_data(ip).await {
        Ok(response) => print_geoip2_data(&response),
        Err(e) => {
            eprintln!("Failed to retrieve geolocation data: {}", e);
            std::process::exit(1);
        }
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
    println!("\n----------------");
    println!("GeoIP2 Data Summary:");
    println!("----------------\n");
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
        println!("Traits:");
        for (key, value) in traits.iter() {
            let formatted_value = match value {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => "Unsupported value type".to_string(),
            };
            println!("{}: {}", key, formatted_value);
        }
    }
    println!();
}
