# Generated Project Structure

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

The root `odoo.conf` is for local runs. `config/odoo.conf` is for Docker.

ODK does not generate `.env` or `.env.example`.
