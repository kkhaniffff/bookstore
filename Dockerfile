FROM rust:slim-trixie AS builder

WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libssl-dev openssl
RUN cargo install sqlx-cli

COPY . .
ENV SQLX_OFFLINE=true
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN cargo build --release --locked

FROM debian:trixie-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/bookstore /app/bookstore
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY migrations /app/migrations

# Create startup script
RUN cat <<'EOF' > /app/start.sh
#!/bin/sh
set -e

echo "Running migrations..."
sqlx migrate run --source /app/migrations

echo "Starting application..."
exec /app/bookstore
EOF

RUN chmod +x /app/start.sh

CMD ["/app/start.sh"]
