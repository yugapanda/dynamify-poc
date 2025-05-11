# --- builder --------------------------------------------------
FROM rust:1.86-bullseye AS builder
WORKDIR /app

# OpenSSL 開発パッケージを追加（glibc 用）
RUN apt-get update && \
    apt-get install -y libssl-dev pkg-config

# ここは **デフォルト target (x86_64-unknown-linux-gnu)**
COPY . .
RUN cargo build --release \
    && strip target/release/dinamify-poc             # サイズ削減

# --- runtime --------------------------------------------------
FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app
COPY --from=builder /app/target/release/dinamify-poc .
ENTRYPOINT ["/app/dinamify-poc"]