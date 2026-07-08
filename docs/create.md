# `odk create`

`odk create` runs an interactive Odoo project generator.

```bash
odk create
```

## Interactive Workflow

The generator asks for:

- Project Name
- Git Repository
- Odoo Version
- Python Version
- PostgreSQL Version
- Use Docker
- Generate PyCharm
- Generate VS Code

Supported choices:

| Odoo Version | Python Versions |
| --- | --- |
| 19.0 | 3.12, 3.13 |
| 18.0 | 3.11, 3.12 |
| 17.0 | 3.11 |

If the selected Python version is not compatible with the selected Odoo
version, ODK exits with a friendly error before cloning or generating files.

## Workflow

ODK performs these steps:

1. Clone the repository with `git clone`.
2. Install Python with `uv python install <version>`.
3. Create `.venv` with `uv venv .venv --python <version>`.
4. Create `addons/`, `custom/`, `docker/`, and `scripts/`.
5. Render project files from Tera templates.
6. Render PyCharm and VS Code configuration when selected.
7. Generate `uv.lock`.

ODK never uses `python -m venv`.

## Template Variables

Templates receive:

- `project_name`
- `odoo_version`
- `python_version`
- `postgres_version`
- `database_name`

The database name is derived from the project name and normalized for
PostgreSQL.
