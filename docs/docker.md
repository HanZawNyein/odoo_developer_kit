# Docker Development

When Docker is enabled, ODK generates:

```text
compose.yaml
Dockerfile
config/odoo.conf
odoo.conf
odoo_pg_pass
```

Start the generated Docker project:

```bash
docker compose up -d
```

Stop it:

```bash
docker compose down
```

Open Odoo:

```text
http://localhost:8069
```

Docker uses `config/odoo.conf`. Local editor runs use root `odoo.conf`.
