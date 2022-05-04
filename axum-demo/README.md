# Axum Demo Web Application

The following commands are supposed to be executed from workspace root dir.

### Execute

In order to deploy only the web app on Minikube, in dev mode:

```sh
skaffold dev --trigger manual -m axum-demo
```

```sh
DATABASE_URL=$(dasel -f application.toml default --format 'postgres://{{.db_username}}:{{.db_password}}@{{.db_host}}/{{.db_name}}') cargo sqlx prepare --merged
```
