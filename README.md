# DDNS Cloudflare

[![CI/CD](https://github.com/masterflitzer/ddns-cloudflare/actions/workflows/main.yml/badge.svg)](https://github.com/masterflitzer/ddns-cloudflare/actions/workflows/main.yml)

## Setup

- Change the `ASSET_NAME` variable according to your target platform (see [GitHub Releases](https://github.com/masterflitzer/ddns-cloudflare/releases))
- Download the app
- Edit the configuration (you can find an example below)
- Configure a cron job

```bash
ASSET_NAME="linux-aarch64-ddns_cloudflare"
curl -Lso /usr/local/sbin/ddns_cloudflare https://github.com/masterflitzer/ddns-cloudflare/releases/latest/download/${ASSET_NAME}
vim $(ddns_cloudflare --configuration)
vim /etc/cron.d/ddns-cloudflare
```

## Example Configuration

```toml
api_token = ""

[ipv6]
# Prefer EUI-64 IPv6 address if available (has highest priority if true)
prefer_eui64 = false
# Prefer the IPv6 address that is used for outgoing connections (allows DDNS with privacy extensions)
prefer_outgoing = false

[records]
"example.com" = ["@", "www"]
"example.org" = ["wiki"]
```

## Crontab

```bash
@reboot root /usr/local/sbin/ddns_cloudflare > /var/log/ddns_cloudflare.log 2>&1
@hourly root /usr/local/sbin/ddns_cloudflare > /var/log/ddns_cloudflare.log 2>&1
```
