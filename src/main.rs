use crate::cloudflare::Cloudflare;
use crate::ip::IPLookup;
use std::thread;
use std::time::Duration;
use std::io::Read;

mod config;
mod cloudflare;
mod ip;

fn main() {
    if let Err(err_str) = application() {
        // keep console open on error
        eprintln!("Error: {}", err_str);
        println!("Press return to exit.");
        let _char_trash = std::io::stdin().read(&mut []);
    }
}

fn application() -> Result<(), String> {
    let config = config::load_global_config()?;

    let cloudflare = Cloudflare::create(&config.cloudflare_email, &config.cloudflare_key);
    let ip_service = IPLookup::create(&config.ip_services);

    // call out to validate cloudflare login, dns record identity, and current public ip
    let zone_id = cloudflare.get_zone(&config.cf_domain_name)?;
    let (record_id, mut cf_ip) = cloudflare.get_record(&zone_id, &config.cf_record_name)?;
    println!("Cloudflare has IP {} saved for {}.", cf_ip, &config.cf_record_name);
    println!("Checking public IP state every {} seconds.", &config.frequency_seconds);
    let retry_timeout = Duration::from_secs(config.retry_timeout_seconds);
    let normal_delay = Duration::from_secs(config.frequency_seconds);

    // main application loop - get current ip, if changed then update on cloudflare
    let mut process = || {
        let current_ip = match ip_service.get_ip() {
            Ok(ip) => ip,
            Err(error) => { // failed to get any ip - likely client network outage. print error, retry later.
                eprintln!("{}", error);
                return Err(());
            }
        };
        if current_ip != cf_ip {
            let request = cloudflare.update_record(&zone_id, &record_id, &config.cf_record_name, &current_ip);
            if let Err(_) = request {
                eprintln!("Error sending IP change request to Cloudflare. (2)");
                return Err(())
            }
            println!("Cloudflare IP has been updated to {}", current_ip);
        } else {
            println!("\tNo IP change needed.");
        }
        cf_ip = current_ip;
        Ok(())
    };

    loop {
        let delay = match process() {
            Ok(_) => normal_delay,
            Err(_) => retry_timeout
        };
        thread::sleep(delay);
    }
}