FROM rust:1.65 as builder

WORKDIR /app
COPY . .

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    cargo build --release

FROM debian:bullseye-slim
WORKDIR /app

COPY --from=builder /app/target/release/homologacao-web .
COPY --from=builder /app/static ./static
COPY --from=builder /app/templates ./templates

ENV RUST_LOG=info
ENV PORT=8080

EXPOSE 8080

CMD ["./homologacao-web"]