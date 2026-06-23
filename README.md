# Chromium Profile Picker

A cross-platform good-looking profile picker for Chromium-based browsers, written in Rust.

<img width="600" alt="image" src="https://github.com/user-attachments/assets/c3eef927-7363-484c-948d-f8cf447b1633" />

## Supported browsers

| Feature flag | Browser |
|---|---|
| `chrome` *(default)* | Google Chrome |
| `chromium` | Chromium |
| `brave` | Brave Browser |
| `edge` | Microsoft Edge |
| `vivaldi` | Vivaldi |
| `opera` | Opera |
| `arc` | Arc *(macOS / Windows only)* |
| `helium` | Helium |

## Installation

```bash
cargo install --git https://github.com/donnybeelo/chromium-profile-picker --no-default-features --features helium
```

Replace `helium` with the feature for your browser. Only one feature should be active at a time.

Locate your `.desktop` file for your browser and set `chromium-profile-picker` as the `Exec` command. Do the same for the "New Window" desktop action if it exists.

## Configuration

On first run the picker auto-detects the browser binary and profile directory and writes a config file to:

- **Linux:** `~/.config/net.donnybeelo.chromium-profile-picker/browser.json`
- **macOS:** `~/Library/Application Support/net.donnybeelo.chromium-profile-picker/browser.json`
- **Windows:** `%APPDATA%\net.donnybeelo.chromium-profile-picker\browser.json`

Edit this file to override any auto-detected paths. The config is regenerated automatically when you switch browser features.

## Build from source

```bash
cargo build --release --no-default-features --features helium
```

The release binary will be in:

```bash
./target/release/chromium-profile-picker
```

### Run

```bash
cargo run --no-default-features --features helium -- "https://example.com"
```

Or without a URL:

```bash
cargo run --no-default-features --features helium
```

## Environment variables

- `BROWSER_BIN` — override the browser executable path
- `BROWSER_CONFIG_DIR` — override the browser profile directory
