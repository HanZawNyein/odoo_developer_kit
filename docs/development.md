# Development

Clone the repository:

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
