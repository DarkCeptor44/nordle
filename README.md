# Nordle

[![crates.io](https://img.shields.io/crates/v/nordle.svg)](https://crates.io/crates/nordle)

A self-hostable Wordle clone without limits, you might think it comes from Nord + Wordle but it's actually Not-Wordle. There is no persistence, the server uses a LRU cache to store the words and game session IDs. Every reload gives you a new session ID with a new word to guess, and the only reward for guessing correctly is a "good job".

The word list is from [this](https://gist.github.com/scholtes/94f3c0303ba6a7768b47583aff36654d) Gist by scholtes.

## Getting Started

You can run your own Nordle server either by installing it from crates.io or by building it from source.

### From crates.io

If you just want to run the server, this is the easiest way, just install it using cargo:

```bash
cargo install nordle
```

This will compile the binary and place it in your cargo bin path. You can then start the server by running:

```bash
nordle
```

### From Source

1. **Prerequisites:** Ensure you have the [Rust toolchain](https://rustup.rs/) installed (Edition 2024 requires Rust 1.85+).
2. Clone the repo:

    ```bash
    git clone https://github.com/DarkCeptor44/nordle.git
    cd nordle
    ```

3. Run in development mode (the server serves the HTML file locally instead of embedding it):

    ```bash
    cargo run
    ```

    If you want to see debugging details, add the `--debug` flag:

    ```bash
    cargo run -- --debug
    ```

4. (Optional) Install it with release mode:

    ```bash
    cargo install --path .
    ```

## Usage

Once the server is running, open your browser and go to <http://localhost:8080>, or whatever it says in the logs. You can change the host and port with the CLI flags:

```bash
$ nordle -h
A Wordle clone without limits

Usage: nordle [OPTIONS]

Options:
  -H, --host <HOST>              Host to listen on [env: NORDLE_HOST=] [default: 0.0.0.0]
  -p, --port <PORT>              Port to listen on [env: NORDLE_PORT=] [default: 8080]
      --debug                    Enable debug logging [env: NORDLE_DEBUG=y]
  -c, --cache-size <CACHE_SIZE>  Cache size [env: NORDLE_CACHE_SIZE=] [default: 10]
  -h, --help                     Print help
  -V, --version                  Print version
```

There are also environment variables, see [Environment Variables](#environment-variables).

## Environment Variables

The following environment variables are currently supported, they are used if the CLI flags are not set:

| Variable | Default | Description |
| --- | --- | --- |
| `NORDLE_CACHE_SIZE` | `10` | Size of the LRU cache |
| `NORDLE_DEBUG` | (empty) | Enable debug logging |
| `NORDLE_HOST` | `0.0.0.0` | Host to listen on |
| `NORDLE_PORT` | `8080` | Port to listen on |

## License

This project is licensed under the [Mozilla Public License, version 2.0](https://www.mozilla.org/MPL/2.0/). See the [LICENSE](LICENSE) file for details.
