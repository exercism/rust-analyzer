# rust-analyzer

`rust-analyzer` is a static analysis utility that is used for the automatic mentoring of the common [Rust track](https://exercism.io/tracks/rust) solutions. It is being developed as part of the [Exercism Strategy initiative](https://exercism.io/strategy). The general design of the utility conforms to the [Interface document](https://github.com/exercism/automated-mentoring-support/blob/master/docs/interface.md).

## Supported exercises

- `reverse-string`

## Usage

The utility can be used on the local machine. It accepts two required parameters:

- `--path` (`-p`) - path to the solution directory.
- `--slug` (`-s`) - the slug of the exercise that is being analyzed.

For example:
```shell
$ rust-analyzer -p ~/solution-238382y7sds7fsadfasj23j/ -s reverse-string
```

In the context of the automatic mentoring the utility is invoked inside the Docker container via the `bin/analyze.sh` script.

## Building

### Using Cargo

You should have the latest stable version of Rust installed on you machine (for instance using [rustup](https://rustup.rs/)).
Then from the project root run
```shell
$ cargo build
```
To run the utility using Cargo execute the following command:
```shell
$ cargo run -- -s exercise_slug -p /path/to/the/solution/directory
```

To run test use the following command:
```shell
$ cargo test
```

### Using Docker

From the project root use the following command:

```shell
$ docker build -t rust_analyzer .
```

## Notes

If you are using `rust-analyzer` to analyze a solution on your machine, please note that it expects that the solution file `lib.rs` is located in the root of the solution directory, in contrast to the typical Rust exercise layout, where the solution is located in the `src/lib.rs`.
