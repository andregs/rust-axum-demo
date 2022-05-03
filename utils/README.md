```sh
DATABASE_URL=$(dasel -f application.toml default --format 'postgres://{{.db_username}}:{{.db_password}}@{{.db_host}}/{{.db_name}}') cargo sqlx prepare --merged
```
