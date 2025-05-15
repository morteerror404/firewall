FROM rust:1.65 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app

COPY --from=builder /app/target/release/lateralizador .
COPY config /app/config

RUN apt-get update && \
    apt-get install -y iptables && \
    rm -rf /var/lib/apt/lists/*

CMD ["./lateralizador"]