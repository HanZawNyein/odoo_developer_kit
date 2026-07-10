# `odk doctor`

`odk doctor` validates the local Odoo developer environment and prints friendly, actionable output.

```bash
odk doctor
```

## What It Checks

ODK checks only the tools that are part of the supported Odoo workflow.

| Tool | Detection |
| --- | --- |
| `uv` | `uv --version` |
| Docker | `docker --version` |
| Python | `python --version`, then `python3 --version` |
| PyCharm | platform-specific app or command detection |
| VS Code | `code --version` |
| PostgreSQL | `psql --version` |
| `wkhtmltopdf` | `wkhtmltopdf --version` |

## Example: Ready Environment

```text
Odoo Developer Kit Doctor

[✓] uv
    Version: 0.8.4

[✓] Docker
    Version: 28.3.2

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

## Example: Missing Tool

```text
[✗] wkhtmltopdf
    Not installed

Suggestion:
    brew install wkhtmltopdf
```

!!! info "No hard crash"
    Missing commands are handled as normal diagnostic results. ODK never panics because a tool is absent from the machine.

## PyCharm Detection

PyCharm requires platform-specific handling:

- macOS scans `/Applications/PyCharm*.app` and `~/Applications/PyCharm*.app`
- Linux checks the `pycharm` command and `which pycharm`
- Windows scans common JetBrains locations under Program Files

## Supported Platforms

| OS | Status |
| --- | --- |
| macOS | Supported |
| Linux | Supported |
| Windows | Supported |
