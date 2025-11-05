FROM rust:latest AS builder
WORKDIR /app

COPY Cargo.toml .
RUN mkdir src && echo "fn main() {println!(\"build placeholder\");}" > src/main.rs
RUN cargo build --release

COPY src src
RUN touch src/main.rs
RUN cargo build --release

RUN strip target/release/personal-website

FROM debian:bookworm-slim AS release
WORKDIR /app
COPY --from=builder /app/target/release/personal-website .

EXPOSE 8080
ENV RUST_LOG=info
CMD ["./personal-website"]
