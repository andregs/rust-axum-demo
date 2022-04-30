```sh
DATABASE_URL=$(dasel -f application.toml -r toml local.database_url) cargo sqlx prepare --merged
```
