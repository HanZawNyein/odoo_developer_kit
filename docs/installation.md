# Installation

ODK can be installed from a prebuilt release binary or from Cargo. The release installer is the recommended path for developers who just want the `odk` command.

## Recommended Installer

```bash
curl -LsSf https://odk.dev/install.sh | sh
```

The installer:

- detects the operating system
- detects CPU architecture
- downloads the matching GitHub release asset
- installs `odk` into `~/.local/bin`
- verifies the binary before exiting

!!! note "PATH"
    If `~/.local/bin` is not already on your `PATH`, the installer prints the shell export line to add it.

## Install With Cargo

Use Cargo when you already have a Rust toolchain installed:

```bash
cargo install odoo-developer-kit
```

Verify the command:

```bash
odk doctor
```

## GitHub Releases

ODK publishes binaries for the platforms below.

| Platform | Architecture | Target |
| --- | --- | --- |
| macOS | ARM64 | `aarch64-apple-darwin` |
| macOS | x64 | `x86_64-apple-darwin` |
| Linux | x64 | `x86_64-unknown-linux-gnu` |
| Linux | ARM64 | `aarch64-unknown-linux-gnu` |
| Windows | x64 | `x86_64-pc-windows-msvc` |

Download the archive for your platform from GitHub Releases, extract the `odk` binary, and place it on your `PATH`.

## Build From Source

```bash
git clone https://github.com/odoo-developer-kit/odoo-developer-kit.git
cd odoo-developer-kit
cargo build --release
```

The release binary is written to:

=== "macOS / Linux"

    ```bash
    target/release/odk
    ```

=== "Windows"

    ```powershell
    target\release\odk.exe
    ```

## Verify Installation

```bash
odk doctor
```

If any required tool is missing, ODK prints a platform-specific installation suggestion.
