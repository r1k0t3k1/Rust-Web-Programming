FROM rust:latest as builder
WORKDIR /home/rust
COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /rust-web-programming
COPY --from=builder /home/rust/target/release/rust-web-programming .
EXPOSE 9000
