# Publishing Documentation

ODK uses Material for MkDocs for documentation. The current repository workflow publishes documentation on pushes to `1.0` and `main` with:

```bash
mkdocs gh-deploy --force
```

This keeps documentation changes reviewable before they are merged.

## GitHub Pages URL

For this repository, GitHub Pages should publish to:

```text
https://hanzawnyein.github.io/odoo_developer_kit/
```

The matching `mkdocs.yml` setting is:

```yaml
site_url: https://hanzawnyein.github.io/odoo_developer_kit/
```

## Active GitHub Pages Workflow

The active workflow runs only on pushes:

```yaml
name: Docs

on:
  push:
    branches:
      - "1.0"
      - main

permissions:
  contents: write

jobs:
  deploy:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup Python
        uses: actions/setup-python@v5
        with:
          python-version: "3.x"

      - name: Cache MkDocs
        uses: actions/cache@v4
        with:
          key: mkdocs-material-${{ github.run_id }}
          path: ~/.cache
          restore-keys: |
            mkdocs-material-

      - name: Install documentation dependencies
        run: python -m pip install mkdocs-material

      - name: Deploy documentation
        run: mkdocs gh-deploy --force
```

## GitHub Pages Publishing Example

Material for MkDocs documents a simple GitHub Actions deployment flow using `mkdocs gh-deploy --force`. This is the same approach used by ODK:

```yaml
name: Publish Docs

on:
  push:
    branches:
      - main

permissions:
  contents: write

jobs:
  deploy:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Configure Git credentials
        run: |
          git config user.name github-actions[bot]
          git config user.email 41898282+github-actions[bot]@users.noreply.github.com

      - name: Setup Python
        uses: actions/setup-python@v5
        with:
          python-version: "3.x"

      - name: Cache MkDocs
        uses: actions/cache@v4
        with:
          key: mkdocs-material-${{ github.run_id }}
          path: ~/.cache
          restore-keys: |
            mkdocs-material-

      - name: Install documentation dependencies
        run: python -m pip install mkdocs-material

      - name: Deploy to GitHub Pages
        run: mkdocs gh-deploy --force
```

!!! note "GitHub Pages source"
    After the first deployment, set GitHub Pages to publish from the `gh-pages` branch in the repository settings.

## Manual Publishing

You can also publish manually from a local checkout:

```bash
python -m pip install mkdocs-material
mkdocs gh-deploy --force
```

Use manual publishing only when you intentionally want to update the public documentation outside the normal CI process.
