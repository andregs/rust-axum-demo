# App Configuration

Sensible defaults are hard-coded in [axum_demo::config::Config](../axum-demo/src/config/mod.rs).

Multiple [profiles](./base/application.toml) can be declared to override such defaults. The toml is just a convenient way, but the app also accept environment variables. That's how we provide e.g. the active profile and secrets like db credentials to deployment resources.

Sensitive config data -- like db credentials -- could be encrypted and stored in Kubernetes secrets using something like [Sealed Secrets](https://github.com/bitnami-labs/sealed-secrets).
