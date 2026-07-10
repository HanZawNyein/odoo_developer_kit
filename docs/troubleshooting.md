# Troubleshooting

## `odk` Is Not Found

Add Cargo bin to your shell path:

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

## Docker Files Were Not Generated

Run `odk create` again and answer `yes` when prompted for Docker.

## Local `odoo.conf` Has The Wrong Addons Path

Install the latest local build:

```bash
cargo install --path . --locked
```

Then regenerate the project. The root `odoo.conf` should contain the Odoo source addons path and the generated project `addons/` path.
