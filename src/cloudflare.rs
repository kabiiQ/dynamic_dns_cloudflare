use serde::{Serialize, Deserialize};
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::Client;

#[derive(Deserialize)]
pub struct ZoneLookup {
    pub result: Vec<Zone>
}

#[derive(Deserialize)]
pub struct Zone {
    pub id: String
}

#[derive(Deserialize)]
pub struct DNSLookup {
    pub result: Vec<DNSRecord>
}

#[derive(Serialize, Deserialize)]
pub struct DNSRecord {
    #[serde(rename="type")]
    pub record_type: String,
    pub id: Option<String>,
    pub name: String,
    pub content: String
}

#[derive(Deserialize)]
pub struct CFResponse {
    pub success: bool
}

pub struct Cloudflare {
    headers: HeaderMap,
    client: Client
}

impl Cloudflare {
    pub fn create(email: &String, key: &String) -> Cloudflare {
        let mut headers = HeaderMap::new();
        headers.insert(header::USER_AGENT, HeaderValue::from_static("kabii/1.0 cloudflare_dynamic_dns"));
        headers.insert("X-Auth-Email", HeaderValue::from_str(email).unwrap());
        headers.insert("X-Auth-Key", HeaderValue::from_str(key).unwrap());

        Cloudflare {
            headers,
            client: reqwest::Client::new()
        }
    }

    pub fn get_zone(&self, domain: &String) -> Result<String, String> {
        let request = self.client
            .get(&format!("https://api.cloudflare.com/client/v4/zones?name={name}",
                         name = domain))
            .headers(self.headers.clone())
            .send();
        let zone_object = match request {
            Ok(mut response) => {
                if response.status().is_success() {
                    response.json::<ZoneLookup>().unwrap()
                } else {
                    return Err(format!("Cloudflare API connection failed. Status: {}/{}",
                                       response.status().as_u16(),
                                       response.status().canonical_reason().unwrap_or("")))
                }
            },
            Err(_) => return Err("Cloudflare API returned an unexpected response. (1)".to_string())
        };
        let zone_id = match zone_object.result.get(0) {
            Some(zone) => zone.id.clone(),
            None => return Err(format!("Cloudflare reported there was no zone for the domain target {}.", domain))
        };
        Ok(zone_id)
    }

    pub fn get_record(&self, zone: &String, record: &String) -> Result<(String, String), String> {
        let request = self.client
            .get(&format!("https://api.cloudflare.com/client/v4/zones/{zone}/dns_records?type=A&name={name}",
                         zone = zone,
                         name = record))
            .headers(self.headers.clone())
            .send();

        // connection was checked when call to get_zone was made so we can relax here
        let record_object = request.unwrap().json::<DNSLookup>().unwrap();
        let response = match record_object.result.get(0) {
            Some(record) => (record.id.clone().unwrap(), record.content.clone()),
            None => return Err(format!("Cloudflare reported that there was no DNS (type A) record for {}", record))
        };
        Ok(response)
    }

    pub fn update_record(&self, zone_id: &String, record_id: &String, record_name: &String, new_ip: &String) -> Result<(), ()> {
        let new_record = DNSRecord {
            record_type: "A".to_string(),
            id: None, // not used for PUT request
            name: record_name.clone(),
            content: new_ip.clone()
        };
        let request = self.client
            .put(&format!("https://api.cloudflare.com/client/v4/zones/{zone}/dns_records/{record}",
                          zone = zone_id,
                          record = record_id))
            .json(&new_record)
            .headers(self.headers.clone())
            .send();
        if let Ok(mut response) = request {
            if let Ok(json) = response.json::<CFResponse>() {
                if json.success {
                    return Ok(());
                }
            }
        }
        Err(())
    }
}