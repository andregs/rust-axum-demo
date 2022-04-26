# Storage

PostgreSQL database and Redis services.

The following commands are supposed to be executed from workspace root dir.

### Execute

In order to deploy only the storage services on Minikube, in dev mode:

```sh
skaffold dev --trigger manual -m storage
```
