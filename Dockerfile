FROM --platform=$BUILDPLATFORM rust:1.86-bullseye AS builder
RUN apt-get update && apt-get install -y musl-tools \
    && rustup target add x86_64-unknown-linux-musl
WORKDIR /app
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl \
    && strip target/x86_64-unknown-linux-musl/release/dinamify-poc

FROM gcr.io/distroless/static-debian12:nonroot
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/dinamify-poc /dinamify-poc
ENTRYPOINT ["/dinamify-poc"]