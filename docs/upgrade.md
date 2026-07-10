# `odk upgrade`

`odk upgrade` installs the latest ODK GitHub Release.

```bash
odk upgrade
```

The command:

1. Checks the running ODK version.
2. Checks the latest GitHub Release.
3. Runs the release installer when an update is available.

On macOS and Linux, it uses the latest shell installer:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/HanZawNyein/odoo_developer_kit/releases/latest/download/odoo-developer-kit-installer.sh | sh
```

On Windows, it uses the latest PowerShell installer.

`odk doctor` also checks for a newer ODK release and prints a warning when the installed version is behind.
