FROM rust:1

WORKDIR /usr/src/hexil-back
COPY . .

RUN cargo install --path .

EXPOSE 80
CMD ["hexil-back", "0.0.0.0:80"]
