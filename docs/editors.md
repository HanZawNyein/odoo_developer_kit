# Editor Support

ODK can generate PyCharm and VS Code configuration during `odk create`.

## PyCharm

The generated PyCharm run configuration points to:

```text
$PROJECT_DIR$/odoo-bin -c odoo.conf
```

It uses the generated `.venv` interpreter.

## VS Code

The generated VS Code configuration points to:

```text
${workspaceFolder}/.venv/bin/python
${workspaceFolder}/odoo-bin -c odoo.conf
```

Both editors use root `odoo.conf` for local development.
