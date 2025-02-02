# DDNS Cloudflare

[![CI/CD](https://github.com/masterflitzer/ddns-cloudflare/actions/workflows/main.yml/badge.svg)](https://github.com/masterflitzer/ddns-cloudflare/actions/workflows/main.yml)

## Setup

- Change the `asset` variable according to your target platform (see [GitHub Releases](https://github.com/masterflitzer/ddns-cloudflare/releases))
- Change the `bin` variable if you wish to use a different destination
- Download the app
- Edit the configuration (you can find an example below)
- Configure a cron job

### Unix-like (bash)

- Depending on the selected destination you may have to become root: `sudo -i`

```bash
asset="linux-aarch64-ddns_cloudflare"; bin="/usr/local/sbin/ddns-cloudflare"; curl -Lso "${bin}.new" "https://github.com/masterflitzer/ddns-cloudflare/releases/latest/download/${asset}" && mv "${bin}.new" "${bin}" && chmod 0754 "${bin}"

vim "$(ddns-cloudflare --configuration)"
vim /etc/cron.d/ddns-cloudflare
```

### Windows (pwsh)

```pwsh
$asset = "windows-x86_64-ddns_cloudflare.exe"; $bin = "$env:LOCALAPPDATA/Programs/ddns-cloudflare/ddns-cloudflare.exe"; curl.exe -Lso "${bin}.new" "https://github.com/masterflitzer/ddns-cloudflare/releases/latest/download/${asset}" && mv -force "${bin}.new" "${bin}"
```

- Use **Task Scheduler** as an replacement for **cron** on Windows

## Configuration

- Print location of configuration file: `ddns-cloudflare --configuration`

```toml
# https://github.com/masterflitzer/ddns-cloudflare#readme

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
@reboot root /usr/local/sbin/ddns-cloudflare > /var/log/ddns-cloudflare.log 2>&1
@hourly root /usr/local/sbin/ddns-cloudflare > /var/log/ddns-cloudflare.log 2>&1
```
