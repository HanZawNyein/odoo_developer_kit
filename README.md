# Odoo Developer Kit (ODK)

[![Tests](https://github.com/HanZawNyein/odoo_developer_kit/actions/workflows/test.yml/badge.svg)](https://github.com/HanZawNyein/odoo_developer_kit/actions/workflows/test.yml)
[![Docs](https://github.com/HanZawNyein/odoo_developer_kit/actions/workflows/docs.yml/badge.svg)](https://hanzawnyein.github.io/odoo_developer_kit/)
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
- Release automation with `cargo-dist`

## Installation

Install the latest source from GitHub:

```bash
cargo install --git https://github.com/HanZawNyein/odoo_developer_kit.git --locked
```

Install from a local checkout:

```bash
cargo install --path . --locked
```

Verify:

```bash
odk doctor
```

The GitHub release installer and `cargo install odoo-developer-kit` will be available after the first GitHub Release and crates.io publish.

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
- PyCharm
- VS Code
- PostgreSQL
- `wkhtmltopdf`

Example:

```text
Odoo Developer Kit Doctor

[✓] uv
    Version: 0.8.4

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
> sample

Project Path:
  sample (default)
> /Users/agga/Documents/python-dev/odoo-dev/sample

Git Repository:
> git@github.com:company/template.git

Odoo Version:
  19.0 (default)
  18.0
  17.0
> 19.0

Python Version:
  3.10
  3.13
  3.12
  3.11
  3.9
  3.8
> 3.10

Use Docker:
  yes/no
> yes

PostgreSQL Version:
  17 (default)
  16
> 17
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
├── config/
│   └── odoo.conf
├── .idea/
├── .vscode/
├── compose.yaml
├── Dockerfile
├── odoo.conf
├── .gitignore
├── odoo_pg_pass
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
git clone https://github.com/HanZawNyein/odoo_developer_kit.git
cd odoo_developer_kit
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

## Release

Releases are built with `cargo-dist` from version tags:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The release workflow runs tests, builds binaries for macOS, Linux, and Windows, and publishes a GitHub Release.

## License

Odoo Developer Kit is licensed under the [Apache License 2.0](LICENSE).
