# rust-analyzer

_Not to be confused with the [rust-analyzer of the Rust project][rust-project-rust-analyzer]!_

`rust-analyzer` is a static analysis utility that is used for the automatic mentoring of exercise solutions of [Exercism's Rust track][exercism-rust].
Please find general documentation about Exercism's language analyzers [here][analyzers-doc].

## Supported exercises

- [`reverse-string`](./src/analyzers/reverse_string/README.md)
- [`gigasecond`](./src/analyzers/gigasecond.rs)
- [`clock`](./src/analyzers/clock.rs)

## Usage

The utility can be used on the local machine. It accepts two required parameters:

- `--path` (`-p`) - path to the solution directory.
- `--slug` (`-s`) - the slug of the exercise that is being analyzed.

For example:
```shell
rust-analyzer -p ~/solution-238382y7sds7fsadfasj23j/ -s reverse-string
```

In the context of the automatic mentoring the utility is invoked inside the Docker container via the `bin/analyze.sh` script.

## Building

### Using Cargo

You should have the latest stable version of Rust [installed][install-rust] on you machine.
Then from the project root run:

```shell
cargo build
```

To run the utility using Cargo execute the following command:

```shell
cargo run -- -s exercise_slug -p /path/to/the/solution/directory
```

To run tests use the following command:
```shell
cargo test
```

### Using Docker

From the project root use the following command:

```shell
docker build -t rust_analyzer .
```

And to run the tests in docker:

```shell
./bin/run-tests-in-docker.sh
```

[rust-project-rust-analyzer]: https://github.com/rust-lang/rust-analyzer
[exercism-rust]: https://exercism.org/tracks/rust
[analyzers-doc]: https://exercism.org/docs/building/tooling/analyzers
[install-rust]: https://www.rust-lang.org/tools/install
