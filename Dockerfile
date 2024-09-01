FROM rust:1.80-slim AS base

WORKDIR /analyzer

COPY . .

RUN cargo build --release

# cargo-local-registry stuff is copied from the test runner
FROM rust:1.80 AS build-cargo-local-registry

# install cargo-local-registry
RUN cargo install cargo-local-registry
# download popular crates to local registry
WORKDIR /local-registry
COPY local-registry/* ./
RUN cargo generate-lockfile && cargo local-registry --sync Cargo.lock .

FROM rust:1.80-slim

WORKDIR /opt/analyzer

RUN rustup component add clippy

COPY ./bin/run.sh ./bin/
COPY --from=base /analyzer/target/release/rust-analyzer ./bin/rust-analyzer
COPY --from=build-cargo-local-registry /local-registry local-registry/
# configure local-registry
RUN echo '[source.crates-io]\n\
    registry = "https://github.com/rust-lang/crates.io-index"\n\
    replace-with = "local-registry"\n\
    \n\
    [source.local-registry]\n\
    local-registry = "/opt/analyzer/local-registry/"\n' >> $CARGO_HOME/config.toml

ENTRYPOINT ["bin/run.sh"]
