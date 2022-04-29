# Migrate DB

SQLx database migrations.

The following commands are supposed to be executed from workspace root dir.

### Execute

Before you can create or execute migrations you need a PostgreSQL server up and running (see [storage](../storage/)).

You can use the [sqlx](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli#sqlx-cli) CLI to create migration files and prepare the `sqlx-data.json`.

Create a migration file:

```sh
sqlx migrate add -r new-credentials-table
```

In order to execute only the DB migrations on Minikube, in dev mode:

```sh
skaffold dev --trigger manual --force -m migrate-db
```
