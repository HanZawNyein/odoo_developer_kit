# Quickstart

Use `odk doctor` first, then generate a project with `odk create`.

```bash
odk doctor
odk create
```

If `odk doctor` reports that a newer ODK version is available, upgrade first:

```bash
odk upgrade
```

The create command asks for:

1. Project name
2. Project path
3. Git repository
4. Odoo source code path
5. Odoo version
6. Python version
7. Docker preference
8. Editor configuration preferences

For local development, ODK writes root `odoo.conf` with an `addons_path` built from the Odoo source checkout and the generated project `addons/` directory.

For Docker development, ODK also writes `config/odoo.conf` for the container.
