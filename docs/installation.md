# Installation

## Installer

The recommended installation path is the hosted installer:

```bash
curl -LsSf https://odk.dev/install.sh | sh
```

The installer detects the operating system and CPU architecture, downloads the
matching release asset, installs `odk` into `~/.local/bin`, and verifies the
binary.

## Cargo

Install from crates.io:

```bash
cargo install odoo-developer-kit
```

## GitHub Releases

ODK publishes binaries for:

- macOS ARM64
- macOS x64
- Linux x64
- Linux ARM64
- Windows x64

Download the asset for your platform from GitHub Releases and place `odk` on
your `PATH`.

## From Source

```bash
git clone https://github.com/odoo-developer-kit/odoo-developer-kit.git
cd odoo-developer-kit
cargo build --release
```

The binary is written to `target/release/odk`.
