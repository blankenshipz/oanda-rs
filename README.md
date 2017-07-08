![docs.rs badge](https://docs.rs/mio/badge.svg?version=0.1.1)

# OANDARS

`oandars` is a Rust wrapper for the `oanda` [Rest-V20](http://developer.oanda.com/rest-live-v20/introduction/) API

## Missing Features

This library does not yet implement all of the features of the `oanda` API, if a feature you're interested in is missing please feel free to [open an issue](https://github.com/blankenshipz/oanda-rs/issues)

## Usage

See the [examples](examples/) and the [documentation](https://docs.rs/oandars/0.1.1/oandars/) for details.

## Running tests in Development

This project uses `docker` and `docker-compose` with these tools installed you can run the test suite with:

```sh
docker-compose run --rm lib
```

The default command is:

```sh
cargo test -- --nocapture
```
