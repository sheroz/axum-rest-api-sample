FROM rust:1.85 as builder
WORKDIR /opt
COPY . .
RUN cargo build --release
RUN cp /opt/target/release/axum-web .
RUN cargo clean

FROM ubuntu:24.04
RUN apt-get update \
    && apt-get install -y --no-install-recommends vim \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /opt
COPY --from=builder /opt/axum-web .
EXPOSE 8080
CMD ["./axum-web"]
