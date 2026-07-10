# Odoo Developer Kit (ODK)

[![Tests](https://github.com/HanZawNyein/odoo_developer_kit/actions/workflows/test.yml/badge.svg)](https://github.com/HanZawNyein/odoo_developer_kit/actions/workflows/test.yml)
[![Docs](https://github.com/HanZawNyein/odoo_developer_kit/actions/workflows/docs.yml/badge.svg)](https://hanzawnyein.github.io/odoo_developer_kit/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Odoo Developer Kit (`odk`) is a Flutter-like developer toolkit for building professional Odoo development environments.

ODK intentionally exposes a small command set:

```bash
odk doctor
odk create
odk upgrade
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

Upgrade ODK:

```bash
odk upgrade
```

## Commands

### `odk doctor`

Checks:

- `uv`
- Docker
- PyCharm
- VS Code
- PostgreSQL
- `wkhtmltopdf`

Example:

```text
Odoo Developer Kit Doctor

[✓] uv
    Version: 0.8.4

[✓] Docker
    Version: 28.3.2

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

If a newer ODK release is available, `odk doctor` prints a warning and suggests `odk upgrade`.

### `odk create`

Starts an interactive generator:

```bash
odk create --project-name burma --project-path=/Users/agga/Documents/python-dev/odoo-dev --docker --pycharm --vscode --doctor
```

`--doctor` runs `odk doctor` before creation. Missing required values are still prompted.
`--projec-path` is accepted as an alias for `--project-path`.

```text
Odoo Project Creator

Project Name:
> sample

Project Path:
  sample (default)
> /Users/agga/Documents/python-dev/odoo-dev

Git Repository:
  leave empty to skip
> 

Odoo Source Code Path:
> /Users/agga/src/odoo

Odoo Version:
  19.1 (detected)

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

The project path is a parent directory. In this example, ODK creates the project at
`/Users/agga/Documents/python-dev/odoo-dev/sample`. You may also enter the complete
project path; ODK does not append the project name twice.

ODK then:

1. Clones the repository with `git clone` when a Git repository is provided.
2. Installs Python with `uv python install <version>`.
3. Creates `.venv` with `uv venv .venv --python <version>`.
4. Installs Odoo requirements with `uv pip install -r <odoo_source_path>/requirements.txt`.
   If requirement installation fails, ODK reports the error and continues so you can fix it manually.
5. Generates folders and project files from Tera templates.
6. Uses the stable Odoo Docker image tag for Docker builds, so Odoo `19.1` renders `FROM odoo:19.0`.
7. Generates PyCharm and VS Code configuration when selected.
8. Runs `uv lock`.

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
