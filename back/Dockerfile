FROM rust:1

WORKDIR /usr/src/hexil-back
COPY . .

RUN cargo install --path .

CMD ["hexil-back"]
