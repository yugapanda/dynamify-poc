FROM rust:1.86.0-bullseye AS build

WORKDIR /app
COPY . .

# 必要なツールとライブラリをインストール
RUN apt update && apt install -y \
    musl-tools \
    musl-dev \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Rustのターゲットを追加
RUN rustup target add x86_64-unknown-linux-musl \
    && rustup component add rust-src

# OpenSSLの環境変数を設定
ENV OPENSSL_DIR=/usr
ENV OPENSSL_INCLUDE_DIR=/usr/include
ENV OPENSSL_LIB_DIR=/usr/lib

# ビルドを実行
RUN cargo build --release --target=x86_64-unknown-linux-musl

# 実行用のイメージ
FROM alpine:latest

# 必要な証明書をインストール
RUN apk add --no-cache ca-certificates

# ビルドしたバイナリをコピー
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/dinamify-poc /usr/local/bin/dinamify-poc

EXPOSE 8080
CMD ["dinamify-poc"]