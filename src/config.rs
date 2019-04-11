use std::fs::File;
use std::io::{ErrorKind, Write, Read};
use serde::{Serialize, Deserialize};
use url::Url;

#[derive(Serialize, Deserialize, Default)]
pub struct Configuration {
    pub cloudflare_email: String,
    pub cloudflare_key: String,
    pub cf_domain_name: String,
    pub cf_record_name: String,
    pub frequency_seconds: u64,
    pub retry_timeout_seconds: u64,
    pub ip_services: Vec<String>
}

impl Configuration {
    fn default_config() -> Configuration {
        Configuration {
            cloudflare_email: String::new(),
            cloudflare_key: String::new(),
            frequency_seconds: 600,
            retry_timeout_seconds: 60,
            ip_services: vec![
                "http://checkip.amazonaws.com".to_string(),
                "http://myexternalip.com/raw".to_string(),
                "http://www.trackip.net/ip".to_string(),
            ],
            cf_domain_name: "example.com".to_string(),
            cf_record_name: "root.example.com".to_string(),
        }
    }
}

pub fn load_global_config() -> Result<Configuration, String> {
    let path = "configuration.toml";
    let config = File::open(path);
    if let Err(error) = config {
        return match error.kind() {
            ErrorKind::NotFound => {
                let config = File::create(path);
                match config {
                    Ok(mut file) => {
                        let default_config = Configuration::default_config();
                        let default_config = toml::to_string_pretty(&default_config).unwrap();
                        let _write = file.write_all(default_config.as_bytes());
                        Err("Default configuration file has been created, please add the required information!".to_string())
                    }
                    Err(_) => Err("Unable to create default configuration file!".to_string())
                }
            }
            _ => Err("Error accessing configuration file!".to_string())
        }
    }
    let mut config_str = String::new();
    if let Err(_) = config.unwrap().read_to_string(&mut config_str) {
        return Err("Error reading configuration file!".to_string());
    }

    let config = match toml::from_str::<Configuration>(&config_str) {
        Ok(conf) => conf,
        Err(err) => return Err(format!("Error reading configuration file!\n{}", err.to_string()))
    };

    // validate config
    if config.cloudflare_email.trim().is_empty() {
        return Err("Config not complete: Email is required for login to Cloudflare.".to_string());
    }
    if config.cloudflare_key.trim().is_empty() {
        return Err("Config not complete: API key is required for authentication to Cloudflare.".to_string());
    }
    if config.cf_record_name.trim().is_empty() {
        return Err("Config not complete: address name for the A record to edit is required.".to_string());
    }
    let valid_urls: Vec<String> = config.ip_services.into_iter().filter( |service| {
        Url::parse(service.as_str()).is_ok()
    }).collect();
    if valid_urls.len() < 1 {
        return Err("Config not complete: provide at least one IP checker service. The default config provides 3 that are likely to exist long-term.".to_string());
    }
    let config = Configuration {
        ip_services: valid_urls,
        ..config
    };
    Ok(config)
}