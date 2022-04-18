# Rust on Kubernetes - Axum demo

Repository created with `cargo init`.

https://github.com/tokio-rs/axum

https://docs.rs/axum/latest/axum/

### Execute

Execute the following commands to start the cluster and deploy PostgreSQL & Redis.

```sh
minikube start --mount --mount-string=$PWD:/mnt/host --cpus=4;
skaffold dev --trigger=manual --iterative-status-check
```
