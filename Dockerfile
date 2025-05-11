FROM rust:1.86.0-bullseye AS build

COPY . /app

RUN rustup target add x86_64-unknown-linux-musl && rustup component add rust-src

WORKDIR /app

RUN apt update && apt install -y musl-tools


RUN apt update && apt install -y musl-tools musl-dev pkg-config libssl-dev

FROM alpine

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/dinamify-poc /app/server
RUN chmod 777 /app/server
EXPOSE 80

WORKDIR /app

CMD ["./server"]

