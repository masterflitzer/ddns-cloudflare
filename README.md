# DDNS Cloudflare

[![CI/CD](https://github.com/masterflitzer/ddns-cloudflare/actions/workflows/main.yml/badge.svg)](https://github.com/masterflitzer/ddns-cloudflare/actions/workflows/main.yml)

## Example Configuration

```toml
api_token = ""

[ipv6]
# Prefer EUI-64 IPv6 address if available (has highest priority if true)
prefer_eui64 = true
# Prefer the IPv6 address that is used for outgoing connections (allows DDNS with privacy extensions)
prefer_outgoing = false

[records]
"example.com" = ["@", "www"]
"example.org" = ["wiki"]
```

## Crontab

```bash
@reboot root ddns_cloudflare > /var/log/ddns_cloudflare.log 2>&1
@hourly root ddns_cloudflare > /var/log/ddns_cloudflare.log 2>&1
```
