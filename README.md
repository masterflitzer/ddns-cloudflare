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
