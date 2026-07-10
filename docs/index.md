# Odoo Developer Kit (ODK)

[![Tests](https://github.com/HanZawNyein/odoo_developer_kit/actions/workflows/test.yml/badge.svg)](https://github.com/HanZawNyein/odoo_developer_kit/actions/workflows/test.yml)
[![Docs](https://github.com/HanZawNyein/odoo_developer_kit/actions/workflows/docs.yml/badge.svg)](https://hanzawnyein.github.io/odoo_developer_kit/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

**Odoo Developer Kit (`odk`)** is a Rust CLI for Odoo developers.
It helps teams diagnose local development machines and generate clean, consistent Odoo development projects.

ODK is designed with a small and focused command surface:

```bash
odk doctor
odk create
odk upgrade
```

---

## What Is ODK?

ODK is a developer toolkit for building professional Odoo development environments.

It helps Odoo developers:

* Check required local tools.
* Generate clean Odoo project scaffolding.
* Create a Python environment using `uv`.
* Generate Docker development files.
* Generate PyCharm configuration.
* Generate VS Code configuration.
* Standardize project setup across a team.

The goal is simple:

> Run `odk doctor`, fix your machine, then run `odk create`.

---

## Features

* Environment diagnostics for Odoo development tools
* Interactive Odoo project generator
* `uv` based Python environment creation
* Docker development setup
* PyCharm run configuration generation
* VS Code launch configuration generation
* Cross-platform support for macOS, Linux, and Windows
* GitHub Release installers with `cargo-dist`
* Clean generated project folder without ODK source files

---

## Installation

### Install from GitHub Release

#### macOS / Linux

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/HanZawNyein/odoo_developer_kit/releases/latest/download/odoo-developer-kit-installer.sh | sh
```

#### Windows PowerShell

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/HanZawNyein/odoo_developer_kit/releases/latest/download/odoo-developer-kit-installer.ps1 | iex"
```

After installation, verify:

```bash
odk doctor
```

If `odk` is not found after installation, add Cargo bin to your PATH.

For macOS or Linux:

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

For bash:

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

---

### Install from GitHub Source

```bash
cargo install --git https://github.com/HanZawNyein/odoo_developer_kit.git --locked
```

Verify:

```bash
odk doctor
```

---

### Install from Local Checkout

```bash
git clone https://github.com/HanZawNyein/odoo_developer_kit.git
cd odoo_developer_kit
cargo install --path . --locked
```

Verify:

```bash
odk doctor
```

---

## Usage

### Check your machine

```bash
odk doctor
```

Example output:

```text
Odoo Developer Kit Doctor

[✓] uv
    Version: 0.11.21

[✓] Docker
    Version: 28.3.2

[✓] PyCharm
    Version: 2026.1.4

[✓] VS Code
    Version: 1.127.0

[✓] PostgreSQL
    Version: 18.4

[✓] wkhtmltopdf
    Version: 0.12.6

Environment ready!
```

---

### Create a project

```bash
odk create
```

Example interactive flow:

```text
Odoo Project Creator

Project Name:
> sample

Project Path:
  sample (default)
> /Users/agga/Documents/python-dev/odoo-dev

Git Repository:
> 

Odoo Source Code Path:
> /Users/agga/src/odoo

Odoo Version:
  19.0 (detected)

Python Version:
  3.8
  3.9
  3.10
  3.11
  3.12
  3.13
> 3.10

Use Docker:
  yes/no
> yes

PostgreSQL Version:
  17 (default)
  16
> 17

Generate PyCharm:
  yes/no
> yes

Generate VS Code:
  yes/no
> yes
```

The project path is the parent directory, so this example creates
`/Users/agga/Documents/python-dev/odoo-dev/sample`.

ODK then:

1. Creates a clean project directory.
2. Installs the selected Python version with `uv`.
3. Creates `.venv` using `uv venv`.
4. Installs `<odoo_source_path>/requirements.txt` with `uv pip install -r`.
5. Generates Odoo project files from templates.
6. Generates Docker files when selected.
7. Generates PyCharm configuration when selected.
8. Generates VS Code configuration when selected.
9. Runs `uv lock`.

ODK does **not** use `python -m venv`.

---

## Commands

| Command      | Purpose                                    | Best time to run                                                       |
| ------------ | ------------------------------------------ | ---------------------------------------------------------------------- |
| `odk doctor` | Validate the local development environment | Before onboarding, after OS upgrades, or before starting a new project |
| `odk create` | Generate a clean Odoo development project  | When bootstrapping a new Odoo development environment                  |
| `odk upgrade` | Install the latest ODK release | When `odk doctor` reports that a newer version is available |

---

## `odk doctor`

`odk doctor` checks the tools commonly needed for Odoo development.

Current checks:

* `uv`
* Docker
* PyCharm
* VS Code
* PostgreSQL
* `wkhtmltopdf`

Example:

```bash
odk doctor
```

Output:

```text
Odoo Developer Kit Doctor

[✓] uv
    Version: 0.11.21

[✓] Docker
    Version: 28.3.2

[✓] PyCharm
    Version: 2026.1.4

[✓] VS Code
    Version: 1.127.0

[✓] PostgreSQL
    Version: 18.4

[✓] wkhtmltopdf
    Version: 0.12.6

Environment ready!
```

If a tool is missing, ODK shows an installation suggestion for your operating system.

If a newer ODK release is available, `odk doctor` prints a warning and suggests `odk upgrade`.

---

## `odk create`

`odk create` starts an interactive Odoo project generator.

It asks for:

* Project name
* Project path
* Optional Git repository
* Odoo version
* Python version
* Docker usage
* PostgreSQL version
* PyCharm configuration
* VS Code configuration

Supported Odoo versions:

```text
19.0
18.0
17.0
```

Supported Python versions:

```text
3.8
3.9
3.10
3.11
3.12
3.13
```

Supported PostgreSQL versions for Docker:

```text
17
16
```

---

## Generated Project Structure

When Docker, PyCharm, and VS Code are enabled, a generated project can look like this:

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

When Docker is disabled, ODK can generate:

```text
project/
├── .venv/
├── addons/
├── .idea/
├── .vscode/
├── odoo.conf
├── .gitignore
├── README.md
├── pyproject.toml
└── uv.lock
```

Depending on your selections, `.idea/`, `.vscode/`, and Docker files may not be generated.

---

## Files That Should Not Be Generated

ODK should generate a clean Odoo project only.

The generated project should **not** include ODK source repository files such as:

```text
.github/
Cargo.lock
Cargo.toml
docs/
src/
templates/
tests/
install.sh
mkdocs.yml
```

If these files appear inside a generated Odoo project, it usually means the ODK repository itself was cloned into the target folder instead of generating a clean project scaffold.

---

## Docker Development

When Docker is enabled, ODK generates:

```text
compose.yaml
Dockerfile
config/odoo.conf
odoo.conf
odoo_pg_pass
```

The default Docker setup includes:

* Odoo service
* PostgreSQL service
* Docker volumes
* PostgreSQL password secret
* Custom addons volume

Start the generated Docker project:

```bash
docker compose up -d
```

Stop it:

```bash
docker compose down
```

Open Odoo:

```text
http://localhost:8069
```

---

## PyCharm Support

When PyCharm generation is enabled, ODK creates `.idea` configuration files.

The generated PyCharm run configuration uses the generated `uv` environment and
points to the Odoo source path entered during project creation. It runs:

```text
<Odoo Source Code Path>/odoo-bin -c $PROJECT_DIR$/odoo.conf
```

ODK also generates the project module, VCS mapping, inspection profile, Black SDK,
and shared run configuration. Session-specific `workspace.xml` remains ignored and
is created by PyCharm when needed.

---

## VS Code Support

When VS Code generation is enabled, ODK creates:

```text
.vscode/settings.json
.vscode/launch.json
```

The generated launch configuration runs:

```text
${workspaceFolder}/odoo-bin -c odoo.conf
```

This works best when your project layout has `odoo-bin` at the project root.

---

## Development

Clone the repository:

```bash
git clone https://github.com/HanZawNyein/odoo_developer_kit.git
cd odoo_developer_kit
```

Run tests:

```bash
cargo test --all-features
```

Run formatting check:

```bash
cargo fmt --all -- --check
```

Run clippy:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Run all checks:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

Run locally:

```bash
cargo run -- doctor
cargo run -- create
```

Install locally:

```bash
cargo install --path . --locked --force
```

---

## Release

ODK uses `cargo-dist` for release builds and GitHub Release installers.

Check release plan:

```bash
dist plan --tag v0.1.1
```

Example output:

```text
announcing v0.1.1
  odoo-developer-kit 0.1.1
    source.tar.gz
    odoo-developer-kit-installer.sh
    odoo-developer-kit-installer.ps1
    sha256.sum
    odoo-developer-kit-aarch64-apple-darwin.tar.xz
    odoo-developer-kit-aarch64-unknown-linux-gnu.tar.xz
    odoo-developer-kit-x86_64-apple-darwin.tar.xz
    odoo-developer-kit-x86_64-pc-windows-msvc.zip
    odoo-developer-kit-x86_64-unknown-linux-gnu.tar.xz
```

To publish a release, make sure the version in `Cargo.toml` matches the Git tag.

Example:

```toml
version = "0.1.1"
```

Then tag:

```bash
git tag v0.1.1
git push origin v0.1.1
```

Important rule:

```text
Cargo.toml version = 0.1.1
Git tag            = v0.1.1
```

If the version and tag do not match, `dist` will fail with:

```text
This workspace doesn't have anything for dist to Release!
```

---

## Troubleshooting

### `cargo dist init` says no such command

If this happens:

```text
error: no such command: `dist`
```

Check if `dist` exists:

```bash
ls ~/.cargo/bin | grep dist
```

If the binary is installed as `dist`, run:

```bash
~/.cargo/bin/dist --version
~/.cargo/bin/dist init
```

If you want to use it as `cargo dist`, create a symlink:

```bash
ln -s ~/.cargo/bin/dist ~/.cargo/bin/cargo-dist
```

Then check:

```bash
cargo dist --version
```

---

### GitHub Release installer returns 404

If this command returns 404:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/HanZawNyein/odoo_developer_kit/releases/latest/download/odoo-developer-kit-installer.sh | sh
```

It means the installer asset does not exist in the latest GitHub Release yet.

Check:

```text
GitHub Repo → Releases → Latest release → Assets
```

The release should contain:

```text
odoo-developer-kit-installer.sh
odoo-developer-kit-installer.ps1
```

If only `.zip` or `.tar.xz` files exist, the release workflow did not generate global installer artifacts.

---

### `odk` command not found

Add Cargo bin to PATH:

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

Then check:

```bash
odk doctor
```

---

## Project Status

ODK currently focuses on two commands:

```bash
odk doctor
odk create
```

Future ideas:

* `odk addon create`
* `odk run`
* `odk update`
* `odk test`
* non-interactive `odk create` flags
* improved Docker templates
* Odoo source path support
* enterprise addons path support

---

## License

This project is licensed under the Apache-2.0 License.

See [LICENSE](LICENSE) for details.

---

## Author

Created by [Han Zaw Nyein](https://github.com/HanZawNyein).

Repository:

```text
https://github.com/HanZawNyein/odoo_developer_kit
```

Documentation:

```text
https://hanzawnyein.github.io/odoo_developer_kit/
```
