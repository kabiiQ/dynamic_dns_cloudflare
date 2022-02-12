# dynamic_dns_cloudflare

### Support the Developer

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/E1E5AF13X)

Made this application for my own use (and as a first project in Rust). If you have a use for a self-hosted DNS updater and use Cloudflare, I've improved it to be easily configurable.

## Binary Download
Binary releases can be found in the [Releases](https://github.com/kabiiQ/dynamic_dns_cloudflare/releases) section on Github. Direct [Windows binary link](https://github.com/kabiiQ/dynamic_dns_cloudflare/releases/download/1.0.0/dynamic_dns.exe) / [Linux binary link](https://github.com/kabiiQ/dynamic_dns_cloudflare/releases/download/1.0.0/dynamic_dns_linux)

## Configuration

On first run the app will generate a file `configuration.toml` in the same directory. 
```toml
cloudflare_email = ''
cloudflare_key = ''
cf_domain_name = 'example.com'
cf_record_name = 'root.example.com'
frequency_seconds = 600
retry_timeout_seconds = 60
ip_services = [
    'http://checkip.amazonaws.com',
    'http://myexternalip.com/raw',
    'http://www.trackip.net/ip',
]
```
This file should be fairly self-explanatory. To use the Cloudflare API on your behalf it requires the email login for the Cloudflare account as well as your API key which can be found on the Cloudflare site at the bottom of the My Profile page. `cloudflare_email`, `cloudflare_key`, `cf_domain_name`, and `cf_record_name` need to be changed. You can leave the other values as-is. 

`ip_services` will be checked in their listed order until one of them provides an IP successfully. `frequency-seconds` represents the "normal" frequency to check for public IP changes, you could lower this depending on your requirements. `retry_timeout_seconds` represents how soon to retry if a network error occurs either getting the public IP or contacting Cloudflare.
