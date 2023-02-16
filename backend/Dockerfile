FROM rust:1 AS builder
WORKDIR /home/rust/app
COPY . .

RUN cargo build --release

FROM debian:bookworm

COPY --from=builder /home/rust/app/target/release/hexer-backend .

EXPOSE 80
CMD ["./hexer-backend", "0.0.0.0:80"]
