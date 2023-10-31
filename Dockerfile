FROM rust:1.71-slim as base

WORKDIR /analyzer

RUN apt-get update && \
    apt-get install -y musl musl-dev musl-tools && \
    rustup target add x86_64-unknown-linux-musl

COPY . .

RUN cargo build --release --target=x86_64-unknown-linux-musl && \
    cp target/x86_64-unknown-linux-musl/release/rust-analyzer ./bin/

FROM alpine:latest

WORKDIR /opt/analyzer

COPY --from=base /analyzer/bin/* ./bin/

ENTRYPOINT ["bin/run.sh"]
