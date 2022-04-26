# Rust on Kubernetes - Axum demo

This is a work in progress. It will be just like https://github.com/andregs/rust-auth-demo but using Axum instead of Rocket.

## tl;dr

Execute this demo app in [minikube](https://minikube.sigs.k8s.io/docs/start/) with:

```sh
minikube start --cpus=4
skaffold dev --trigger=manual
```

### Notes

Repository created with `cargo init`.

https://github.com/tokio-rs/axum

https://docs.rs/axum/latest/axum/

### Execute

Execute the following commands to start the cluster and deploy PostgreSQL & Redis.

```sh
minikube start --cpus=4;
skaffold dev --trigger=manual --iterative-status-check
```
