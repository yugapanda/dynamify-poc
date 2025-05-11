FROM rust:1.86.0-bullseye AS build

COPY . /app

WORKDIR /app


# ビルドを実行
RUN cargo build --release

# 実行用のイメージ
FROM gcr.io/distroless/static-debian11

# ビルドしたバイナリをコピー
COPY --from=build /app/target/release/dinamify-poc /usr/local/bin/dinamify-poc

EXPOSE 8080
CMD ["/usr/local/bin/dinamify-poc"]