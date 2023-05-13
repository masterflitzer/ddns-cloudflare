# DDNS Cloudflare

[![CI/CD](https://github.com/masterflitzer/ddns-cloudflare/actions/workflows/main.yml/badge.svg)](https://github.com/masterflitzer/ddns-cloudflare/actions/workflows/main.yml)

## Example Configuration

```toml
api_token = ""
# Use the IPv6 address the OS prefers for outgoing connections (often temporary addresses therefore discouraged)
use_preferred_ipv6 = false

[[zones]]
name = "example.com"
records = ["@", "www"]

[[zones]]
name = "example.org"
records = ["wiki"]

```

## Crontab

```bash
@reboot root ddns_cloudflare > /var/log/ddns_cloudflare.log 2>&1
@hourly root ddns_cloudflare > /var/log/ddns_cloudflare.log 2>&1
```
