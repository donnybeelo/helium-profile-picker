# Helium Profile Picker

A small, beautiful, cross-platform profile picker for Helium, rewritten in Rust.

## Highlights

- Rust + egui/eframe
- Cross-platform: Linux, macOS, Windows
- No runtime Python or GTK dependencies
- Modern dark UI inspired by the original Python version
- Profile avatars and custom avatars when available
- Falls back to initials if no image is available

## Build

```bash
cargo build --release
```

The release binary will be in:

```bash
./target/release/helium-profile-picker
```

## Run

```bash
cargo run -- "https://example.com"
```

Or without a URL:

```bash
cargo run
```

## Environment variables

- `HELIUM_CONFIG_DIR` — override the Helium profile directory
- `HELIUM_BIN` — override the Helium executable path

## Notes

The app reads Helium's `Local State` file to discover profiles and preserve their display order.
If the built-in avatar images can be found from Chrome/Chromium/Edge resource packs, those are extracted and cached locally.
