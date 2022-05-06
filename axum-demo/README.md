# Axum Demo Web Application

The following commands are supposed to be executed from workspace root dir.

### Execute

In order to deploy only the web app on Minikube, in dev mode:

```sh
skaffold dev --trigger manual -m axum-demo
```

If you have the migrated db running, you can execute the web app locally:

```sh
cargo test && cargo run --bin axum-demo
```

You have to update `sqlx-data.json` when you change the SQL queries in your app code.
The SQLx cli tool that generates that file reads the DB credentials from an env var:

```sh
 DATABASE_URL='postgres://postgres:password for dev only!@localhost:5432' cargo sqlx prepare --merged
```
