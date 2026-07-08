# Odoo Developer Kit (ODK)

[![Tests](https://github.com/odoo-developer-kit/odoo-developer-kit/actions/workflows/test.yml/badge.svg)](https://github.com/odoo-developer-kit/odoo-developer-kit/actions/workflows/test.yml)
[![Docs](https://github.com/odoo-developer-kit/odoo-developer-kit/actions/workflows/docs.yml/badge.svg)](https://odoo-developer-kit.github.io/odoo-developer-kit/)
[![Crates.io](https://img.shields.io/crates/v/odoo-developer-kit.svg)](https://crates.io/crates/odoo-developer-kit)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Odoo Developer Kit (`odk`) is a Flutter-like developer toolkit for building professional Odoo development environments.

ODK intentionally exposes only two commands:

```bash
odk doctor
odk create
```

## What Is ODK?

ODK is a Rust CLI that helps Odoo developers bootstrap and validate local development environments. It checks the core tools used by Odoo projects and generates consistent project scaffolding with `uv`, Docker, PyCharm, VS Code, and Tera templates.

## Features

- Environment diagnostics for Odoo development tools
- Interactive Odoo project generator
- `uv` based Python environment creation
- Docker development setup
- PyCharm run configuration generation
- VS Code launch configuration generation
- Cross-platform support for macOS, Linux, and Windows
- Binary releases with `cargo-dist`

## Installation

Install from the official installer:

```bash
curl -LsSf https://odk.dev/install.sh | sh
```

Install with Cargo:

```bash
cargo install odoo-developer-kit
```

Verify:

```bash
odk doctor
```

## Usage

Check your environment:

```bash
odk doctor
```

Create a project:

```bash
odk create
```

## Commands

### `odk doctor`

Checks:

- `uv`
- Python
- PyCharm
- VS Code
- PostgreSQL
- `wkhtmltopdf`

Example:

```text
Odoo Developer Kit Doctor

[✓] uv
    Version: 0.8.4

[✓] Python
    Version: 3.13.5

[✓] PyCharm
    Version: 2026.1

[✓] VS Code
    Version: 1.104

[✓] PostgreSQL
    Version: 17

[✓] wkhtmltopdf
    Version: 0.12.6

Environment ready!
```

### `odk create`

Starts an interactive generator:

```text
Odoo Project Creator

Project Name:
> geaai_odoo

Git Repository:
> git@github.com:company/template.git

Odoo Version:
  19.0
  18.0
  17.0
> 19.0

Python Version:
  3.13
  3.12
  3.11
> 3.13
```

ODK then:

1. Clones the repository with `git clone`.
2. Installs Python with `uv python install <version>`.
3. Creates `.venv` with `uv venv .venv --python <version>`.
4. Generates folders and project files from Tera templates.
5. Generates PyCharm and VS Code configuration when selected.
6. Runs `uv lock`.

ODK never uses `python -m venv`.

## Generated Project Structure

```text
project/
├── .venv/
├── addons/
├── custom/
├── docker/
├── scripts/
├── .idea/
├── .vscode/
├── compose.yaml
├── Dockerfile
├── .env
├── .gitignore
├── odoo.conf
├── README.md
├── pyproject.toml
└── uv.lock
```

## Supported OS

- macOS
- Linux
- Windows

## Development Guide

Clone:

```bash
git clone https://github.com/odoo-developer-kit/odoo-developer-kit.git
cd odoo-developer-kit
```

Run tests:

```bash
cargo test
```

Run formatting and lint checks:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

Run locally:

```bash
cargo run -- doctor
cargo run -- create
```

Build a release binary:

```bash
cargo build --release
```

## License

Odoo Developer Kit is licensed under the [Apache License 2.0](LICENSE).
