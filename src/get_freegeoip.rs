use serde_json::Value;
use std::net::IpAddr;

pub async fn display_freegeoip_data(ip: &IpAddr) {
    match get_freegeoip_data(ip).await {
        Ok(response) => print_freegeoip_data(&response),
        Err(e) => {
            eprintln!("Failed to retrieve geolocation data: {}", e);
            std::process::exit(1);
        }
    }
}

async fn get_freegeoip_data(ip: &IpAddr) -> Result<Value, reqwest::Error> {
    let url = format!("https://freegeoip.app/json/{}", ip);
    let client = reqwest::Client::new();
    client.get(url).send().await?.json::<Value>().await
}

fn print_freegeoip_data(data: &Value) {
    if let Some(obj) = data.as_object() {
        println!("\n----------------");
        println!("FreeGeoIP Data Summary:");
        println!("----------------\n");

        if let Some(ip) = obj.get("ip").and_then(|v| v.as_str()) {
            println!("IP Address: {}", ip);
        }
        if let Some(city) = obj.get("city").and_then(|v| v.as_str()) {
            println!("City: {}", city);
        }
        if let Some(country_name) = obj.get("country_name").and_then(|v| v.as_str()) {
            println!("Country: {}", country_name);
        }
        if let Some(country_code) = obj.get("country_code").and_then(|v| v.as_str()) {
            println!("Country Code: {}", country_code);
        }
        if let Some(region_name) = obj.get("region_name").and_then(|v| v.as_str()) {
            println!("Region: {}", region_name);
        }
        if let Some(zip_code) = obj.get("zip_code").and_then(|v| v.as_str()) {
            println!("ZIP Code: {}", zip_code);
        }
        if let Some(time_zone) = obj.get("time_zone").and_then(|v| v.as_str()) {
            println!("Time Zone: {}", time_zone);
        }
        if let Some(latitude) = obj.get("latitude").and_then(|v| v.as_f64()) {
            println!("Latitude: {:.6}", latitude);
        }
        if let Some(longitude) = obj.get("longitude").and_then(|v| v.as_f64()) {
            println!("Longitude: {:.6}", longitude);
        }
        if let Some(metro_code) = obj.get("metro_code").and_then(|v| v.as_i64()) {
            println!("Metro Code: {}", metro_code);
        }
        println!();
    } else {
        println!("Invalid data provided");
    }
}
