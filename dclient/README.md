# dclient

`dclient` is a small asynchronous TCP client written in Rust. It connects to a
TCP server, sends a text message, closes the write side of the connection, reads
the server response, and prints that response to standard output.

## Contents

- `src/main.rs` - command-line parsing and TCP client implementation.
- `Cargo.toml` - Rust package metadata and dependencies.
- `Cargo.lock` - locked dependency versions for reproducible builds.

## Requirements

- Rust toolchain with Cargo installed.
- A reachable TCP server that accepts plain text input and returns a response.

The project uses:

- `tokio` for asynchronous networking and runtime support.
- `clap` for command-line argument parsing.

## Build

Build a debug binary:

```sh
cargo build
```

Build an optimized release binary:

```sh
cargo build --release
```

The release executable is created at:

```sh
target/release/dclient
```

## Usage

Run the client with a target host, TCP port, and message:

```sh
cargo run -- --host <HOST> --port <PORT> <MESSAGE>
```

Example:

```sh
cargo run -- --host 127.0.0.1 --port 9000 "hello from dclient"
```

After building a release binary, you can run it directly:

```sh
./target/release/dclient --host 127.0.0.1 --port 9000 "hello from dclient"
```

Messages can also be passed as multiple words. The client joins them with
spaces before sending:

```sh
cargo run -- --host example.com --port 1234 hello from dclient
```
## Deploy on host machine

```sh
cargo install --path <path_on_host_machine>
```
Force reinstall after changes:

```sh
cargo install --path <path_on_host_machine> --force
```

## Behavior

1. Parses `--host`, `--port`, and the message from the command line.
2. Opens a TCP connection to `<HOST>:<PORT>`.
3. Sends the message bytes exactly as provided, without appending a newline.
4. Shuts down the write side of the TCP stream.
5. Reads all response bytes until the server closes the connection.
6. Prints the response as UTF-8, replacing invalid bytes when necessary.

## Testing

Run the test suite with:

```sh
cargo test
```

The project currently has no unit tests, so this command verifies that the
crate compiles in test mode.

## Notes

- Use `--host` instead of `-h`, because `-h` is reserved by Clap for help output
  and conflicts with the current host short option in debug builds.
- The server must close the connection after responding. Otherwise, the client
  will continue waiting while reading the response.
