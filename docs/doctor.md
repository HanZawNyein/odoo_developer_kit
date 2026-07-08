# `odk doctor`

`odk doctor` checks the local Odoo development environment.

```bash
odk doctor
```

## Checks

ODK checks only these tools:

- `uv`
- Python
- PyCharm
- VS Code
- PostgreSQL
- `wkhtmltopdf`

## Detection

ODK uses system commands such as:

- `uv --version`
- `python --version`
- `python3 --version`
- `code --version`
- `psql --version`
- `wkhtmltopdf --version`

PyCharm detection is platform-specific:

- macOS: scans `/Applications/PyCharm*.app` and `~/Applications/PyCharm*.app`
- Linux: checks `pycharm` and `which pycharm`
- Windows: scans common Program Files JetBrains locations

## Example Output

```text
Odoo Developer Kit Doctor

[✓] uv
    Version: 0.8.4

[✓] Python
    Version: 3.13.5

[✓] PyCharm
    Version: 2026.1

[✓] VS Code
    Version: 1.104

[✓] PostgreSQL
    Version: 17

[✓] wkhtmltopdf
    Version: 0.12.6

Environment ready!
```
