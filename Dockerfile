FROM rust:1.80-slim AS base

WORKDIR /analyzer

COPY . .

RUN cargo build --release

FROM rust:1.80-slim

WORKDIR /opt/analyzer

RUN rustup component add clippy

COPY ./bin/run.sh ./bin/
COPY --from=base /analyzer/target/release/rust-analyzer ./bin/rust-analyzer

ENTRYPOINT ["bin/run.sh"]
