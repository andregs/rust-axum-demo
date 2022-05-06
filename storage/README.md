# Storage

PostgreSQL database and Redis services.

The following commands are supposed to be executed from workspace root dir.

### Execute

In order to deploy only the storage services on Minikube, in dev mode:

```sh
skaffold dev --trigger manual -m storage
```

TODO On prod, we need more than a single pod to guarantee HA.

Persistent storage, master/slave replication, backup routines etc. that's no trivial setup for this demo.
I should consider using some open-source k8s operator to handle that for me, like:
- https://github.com/zalando/postgres-operator
- https://access.crunchydata.com/documentation/postgres-operator/latest/

Consider using a single DB cluster, and each microservice would have its own db schema and credentials.
Ok, this way microservices won't be totally independent, but db-related ops like disaster recovery can be solved only once.
It's a good middle ground because microservices won't share data. After we're familiar with this model, we can think about splitting even more.
