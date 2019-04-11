use std::str::FromStr;
use std::net::Ipv4Addr;
use reqwest::Client;

pub struct IPLookup {
    services: Vec<String>,
    client: Client
}

impl IPLookup{
    pub fn create(services: &Vec<String>) -> IPLookup {
        IPLookup { services: services.clone(), client: reqwest::Client::new() }
    }

    pub fn get_ip(&self) -> Result<String, String> {
        for service in &self.services {
            let request = self.client.get(service).send();
            if let Ok(mut response) = request {
                let body = match response.text() {
                    Ok(content) => content,
                    Err(_) => continue
                };
                let ip_str = body.trim();
                if let Ok(_) = Ipv4Addr::from_str(ip_str) {
                    println!("{} returned valid IP: {}.", service, ip_str);
                    return Ok(ip_str.to_string());
                }
            }
        }
        Err("No valid IP could be obtained. Network outage?".to_string())
    }
}