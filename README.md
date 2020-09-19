# Browser Launcher

A utility for MS Windows to launch different programs or browsers when opening urls.

Heavily inspired by [DanTups BrowserSelector](https://github.com/DanTup/BrowserSelector).

## Setup

TBD

## Usage

```txt
browser_launcher.exe --register
    Register as web browser

browser_launcher.exe --unregister
    Unregister as web browser

browser_launcher.exe "http://example.org/"
    Launch url with the browser specified in the config.toml
```

## Example Config

```toml
[browser.firefox]
path = "C:\\Program Files\\Mozilla Firefox\\firefox.exe"
args = ["-osint","-url","{url}"]
matching = [ { pattern = "*" } ]

[browser.edge]
priority = 100
path = "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe"
args = ["{url}"]
matching = [
    { pattern = "microsoft.com" },
    { pattern = "*.microsoft.com" },
    { pattern = "office.com" },
    { pattern = "*.office.com" },
]

[browser.steam]
priority = 100
path = "D:\\Program Files (x86)\\Steam\\steam.exe"
args = ["--", "steam://openurl/{url}" ]
matching = [
    { pattern = "steamcommunity.com" },
    { pattern = "steampowered.com" },
    { pattern = "*.steamcommunity.com" },
    { pattern = "*.steampowered.com" }
]
```