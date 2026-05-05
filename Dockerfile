############################# START SHARED LAYERS #############################

# IMPORTANT: This part of the build is shared between the test-runner and
# analyzer. It takes a relatively large amount of space: 850 MB for the Rust
# toolchain and 40 MB for the local cargo registry. Therefore, it's important
# to keep this in sync between the two images. A slight mismatch in these layers
# would lead to douple the storage requirement on Exercism's servers.

FROM rust:1.95.0 AS build-local-registry

WORKDIR /work
COPY local-registry/Cargo.toml .

RUN <<EOF
set -eux
apt-get update
apt-get install -y gcc openssl cmake
cargo install --locked cargo-local-registry
cargo generate-lockfile
cargo local-registry sync Cargo.lock /local-registry
EOF

# As of April 2026, we need to use the nightly toolchain to get JSON test output
# tracking issue: https://github.com/rust-lang/rust/issues/49359
#
# The official docker image for the nightly Rust toolchain is updated every
# day. We don't want to invalidate the build cache that often, so we pin it
# to a specific hash. To update, go to the following page, then navigate to
# "nightly-slim", select "linux/amd64" and copy the manifest digest.
# https://hub.docker.com/r/rustlang/rust/tags
#
FROM docker.io/rustlang/rust@sha256:3444fefbb69afbff45c0722c8045404c8e7f369c5202e916bd94f665b69f1b1c as rust-nightly-with-local-registry

# add local registry
COPY --from=build-local-registry /local-registry /local-registry
RUN <<EOF
echo "\
[source.crates-io]
registry = 'sparse+https://index.crates.io/'
replace-with = 'local-registry'

[source.local-registry]
local-registry = '/local-registry'" >> $CARGO_HOME/config.toml
EOF

############################## END SHARED LAYERS ##############################


FROM rust:1.95.0 AS build

WORKDIR /work
COPY . .
RUN cargo build --release


FROM rust-nightly-with-local-registry

RUN rustup component add clippy

WORKDIR /opt/analyzer
COPY --from=build /work/target/release/rust-analyzer bin/
COPY bin/run.sh bin/
ENTRYPOINT ["bin/run.sh"]
