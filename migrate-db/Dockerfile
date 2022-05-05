ARG BASE
FROM $BASE as builder

FROM debian:bullseye-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/debug/migrate-db .
ENTRYPOINT ["./migrate-db"]
