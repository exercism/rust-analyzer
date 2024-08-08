# rust-analyzer

_Not to be confused with the [rust-analyzer of the Rust project][rust-project-rust-analyzer]!_

`rust-analyzer` is a little wrapper around clippy that massages its output to adhere to Exercism's [analyzer interface][analyzer-interface].
It is used to provide clippy's feedback to submissions on [Exercism's Rust track][exercism-rust].
Please find general documentation about Exercism's language analyzers [here][analyzers-doc].

## Usage

To run the utility using Cargo execute the following command:

```shell
cargo run -- <slug> <solution_dir> <output_dir>
```

In the context of the automatic mentoring the utility is invoked inside the Docker container via the `bin/run.sh` script.

## Building

You should have the latest stable version of Rust [installed][install-rust] on you machine.
Then from the project root run:

```shell
cargo build
```

## Testing

The main tests are run in docker, use the following script:

```shell
./bin/run-tests-in-docker.sh
```

For unit tests:

```shell
cargo test
```

[rust-project-rust-analyzer]: https://github.com/rust-lang/rust-analyzer
[analyzer-interface]: https://exercism.org/docs/building/tooling/analyzers/interface
[exercism-rust]: https://exercism.org/tracks/rust
[analyzers-doc]: https://exercism.org/docs/building/tooling/analyzers
[install-rust]: https://www.rust-lang.org/tools/install
