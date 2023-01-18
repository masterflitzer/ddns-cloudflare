# DDNS Cloudflare

## Example Configuration

```toml
api_token = ""

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
