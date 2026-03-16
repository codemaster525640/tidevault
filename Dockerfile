# Offline-friendly multi-stage build using vendored crates.
FROM rust:1.83-bookworm AS builder
WORKDIR /app
COPY . .
ENV CARGO_NET_OFFLINE=true
RUN cargo build --release --workspace
RUN cargo test --workspace -- --test-threads=2

FROM debian:bookworm-slim
RUN useradd -m -u 10001 tidevault
COPY --from=builder /app/target/release/tidevault /usr/local/bin/tidevault
COPY --from=builder /app/target/release/tvctl /usr/local/bin/tvctl
USER tidevault
EXPOSE 8080
ENTRYPOINT ["tidevault"]
CMD ["--listen", "0.0.0.0:8080"]
