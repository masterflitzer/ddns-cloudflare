# DDNS Cloudflare

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
@reboot root ddns_cloudflare &> /var/log/ddns_cloudflare.log
@hourly root ddns_cloudflare &> /var/log/ddns_cloudflare.log
```
