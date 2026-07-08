---
hide:
  - navigation
---

# Odoo Developer Kit

<section class="odk-hero">
  <div class="odk-hero__content">
    <p class="odk-eyebrow">Rust CLI for Odoo developers</p>
    <h1>Build consistent Odoo development environments in minutes.</h1>
    <p class="odk-lead">
      Odoo Developer Kit brings a focused, Flutter-like workflow to Odoo teams:
      diagnose the machine, generate the project, and start from a clean local setup.
    </p>
    <div class="odk-actions">
      <a class="md-button md-button--primary" href="installation/">Install ODK</a>
      <a class="md-button" href="create/">Create a project</a>
    </div>
  </div>
  <div class="odk-terminal" aria-label="ODK doctor output">
    <div class="odk-terminal__bar">
      <span></span><span></span><span></span>
    </div>
    <pre><code>$ odk doctor

Odoo Developer Kit Doctor

[✓] uv
    Version: 0.8.4

[✓] Python
    Version: 3.13.5

[✓] PostgreSQL
    Version: 17

Environment ready!</code></pre>
  </div>
</section>

## Why ODK

<div class="odk-grid">
  <article class="odk-card">
    <h3>Environment Diagnostics</h3>
    <p>Check exactly the tools an Odoo developer needs: uv, Python, PyCharm, VS Code, PostgreSQL, and wkhtmltopdf.</p>
  </article>
  <article class="odk-card">
    <h3>Project Generation</h3>
    <p>Clone a repository, create the Python environment with uv, and render consistent Odoo project files from templates.</p>
  </article>
  <article class="odk-card">
    <h3>Editor Ready</h3>
    <p>Generate PyCharm and VS Code configuration that points to the project virtual environment and Odoo entrypoint.</p>
  </article>
  <article class="odk-card">
    <h3>Docker Friendly</h3>
    <p>Generate a Docker development setup with PostgreSQL and Odoo wiring when the project needs containerized services.</p>
  </article>
</div>

## Quick Start

Install ODK:

```bash
curl -LsSf https://odk.dev/install.sh | sh
```

Check the machine:

```bash
odk doctor
```

Create an Odoo project:

```bash
odk create
```

!!! tip "Two commands, one workflow"
    ODK intentionally exposes only `odk doctor` and `odk create`. The goal is a small, reliable CLI surface that teams can standardize on.

## Command Map

| Command | Purpose | Best time to run |
| --- | --- | --- |
| `odk doctor` | Validate the local development environment | Before onboarding, after OS upgrades, or before starting a new project |
| `odk create` | Generate a complete Odoo development project | When bootstrapping a new repository checkout |
