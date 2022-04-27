# Migration

SQLx database migrations.

The following commands are supposed to be executed from workspace root dir.

### Execute

In order to execute only the DB migrations on Minikube, in dev mode:

```sh
skaffold dev --trigger manual -m migration
```
