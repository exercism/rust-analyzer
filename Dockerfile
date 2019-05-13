FROM rust:1.34-slim as base

WORKDIR /analyzer

COPY . .

RUN apt-get update && \
    apt-get install -y musl musl-dev musl-tools && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target=x86_64-unknown-linux-musl && \
    cp target/release/rust-analyzer ./bin/analyzer

FROM alpine:latest

WORKDIR /opt/analyzer

COPY --from=base /analyzer/bin/* ./bin/

RUN ls -1 bin

ENTRYPOINT ["bin/analyze.sh"]
