# Rust on Kubernetes - Axum demo

This is a work in progress. It will be just like https://github.com/andregs/rust-auth-demo but using Axum instead of Rocket.

## Execute

Start a kubernetes cluster with [minikube](https://minikube.sigs.k8s.io/docs/start/):

```sh
minikube start --cpus=4
```

Build and deploy everything, in dev mode:

```sh
skaffold dev --iterative-status-check --trigger manual
```

### Notes

Repository created with `cargo init`.

https://github.com/tokio-rs/axum

https://docs.rs/axum/latest/axum/

