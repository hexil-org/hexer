FROM rust:1

WORKDIR /usr/src/hexer-back
COPY . .

RUN cargo install --path .

EXPOSE 80
CMD ["hexer-back", "0.0.0.0:80"]
