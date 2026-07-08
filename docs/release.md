# Release

ODK uses `cargo-dist` to build binary releases for macOS, Linux, and Windows.

## Release Trigger

Releases are created from Git tags that start with `v`.

```bash
git tag v0.1.0
git push origin v0.1.0
```

The release workflow then:

1. Runs formatting, Clippy, and unit tests.
2. Builds release artifacts with `cargo-dist`.
3. Uploads artifacts for each supported platform.
4. Creates a GitHub Release with generated release notes.

## Supported Targets

| Platform | Target |
| --- | --- |
| macOS ARM64 | `aarch64-apple-darwin` |
| macOS x64 | `x86_64-apple-darwin` |
| Linux x64 | `x86_64-unknown-linux-gnu` |
| Linux ARM64 | `aarch64-unknown-linux-gnu` |
| Windows x64 | `x86_64-pc-windows-msvc` |

## Preflight Checks

Before creating a tag, run:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

The pull request workflow also runs `dist plan` so release configuration problems are caught before merge.

!!! warning "Tags publish releases"
    Pushing a matching `v*` tag is a publishing action. Only tag commits that are ready to become public releases.
