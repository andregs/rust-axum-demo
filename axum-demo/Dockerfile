ARG BASE
FROM $BASE as builder

FROM debian:bullseye-slim AS runtime
WORKDIR app
COPY --from=builder /app/application.toml .
COPY --from=builder /app/target/debug/axum-demo .
ENTRYPOINT ["./axum-demo"]
