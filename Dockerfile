FROM rust:1.42-stretch as base

WORKDIR /analyzer

COPY . .

RUN apt-get update && \
    apt-get install -y musl musl-dev musl-tools && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target=x86_64-unknown-linux-musl && \
    cp target/x86_64-unknown-linux-musl/release/rust-analyzer ./bin/

FROM alpine:latest

WORKDIR /opt/analyzer

COPY --from=base /analyzer/bin/* ./bin/

ENTRYPOINT ["bin/analyze.sh"]
