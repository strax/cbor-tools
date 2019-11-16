# cbor-tools

CLI tool for working with [CBOR][1] data. Current features are converting CBOR to JSON and filtering a CBOR sequence to its first item.

## Usage

```
USAGE:
    cbor [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    dump    converts the input CBOR data to JSON
    head    prints the first item of the input CBOR sequence to stdout
    help    Prints this message or the help of the given subcommand(s)
```

Both `dump` and `head` commands take an optional `FILE` argument that specifies the input file.
If it is not specified, data will be read from the standard input.

## Building

`cbor-tools` is written in Rust and targets the latest stable toolchain.
To build and install the `cbor` binary on your local system, run the following command from the repository root:

```
cargo install --path .
```

This will build the binary with the release profile and symlink the `cbor` binary to the [default Cargo install location](https://doc.rust-lang.org/cargo/commands/cargo-install.html).

[1]: https://tools.ietf.org/html/rfc7049
