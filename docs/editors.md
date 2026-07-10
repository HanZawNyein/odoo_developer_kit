# Editor Support

ODK can generate PyCharm and VS Code configuration during `odk create`.

## PyCharm

The generated PyCharm run configuration points to:

```text
<Odoo Source Code Path>/odoo-bin -c $PROJECT_DIR$/odoo.conf
```

It uses the generated `uv` environment, adds the external Odoo source as a module
content root, and generates the module, VCS, inspection, and shared run configuration
files under `.idea/`. PyCharm creates its own ignored `workspace.xml` when the project
is opened.

## VS Code

The generated VS Code configuration points to:

```text
${workspaceFolder}/.venv/bin/python
${workspaceFolder}/odoo-bin -c odoo.conf
```

Both editors use root `odoo.conf` for local development.
