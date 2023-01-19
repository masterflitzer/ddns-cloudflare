# DDNS Cloudflare

## Example Configuration

```toml
api_token = ""
# Use the IPv6 address the OS prefers for outgoing connections (often temporary addresses therefore discouraged)
ipv6_preferred = false

[[zones]]
name = "example.com"
records = ["@", "www"]

[[zones]]
name = "mail.xyz"
records = ["imap", "smtp"]

```

## Crontab

```bash
@reboot root ddns_cloudflare &> /var/log/ddns_cloudflare.log
@hourly root ddns_cloudflare &> /var/log/ddns_cloudflare.log
```
