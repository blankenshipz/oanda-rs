![docs.rs badge](https://docs.rs/mio/badge.svg?version=0.1.2)

# OANDARS

`oandars` is a Rust wrapper for the `oanda` [Rest-V20](http://developer.oanda.com/rest-live-v20/introduction/) API

## Missing Features

This library does not yet implement all of the features of the `oanda` API, if a feature you're interested in is missing please feel free to [open an issue](https://github.com/blankenshipz/oanda-rs/issues)

## Usage

See the [examples](examples/) and the [documentation](https://docs.rs/oandars/0.1.1/oandars/) for details

## Development

### Setup

1. Create an `Oanda` test account and name it `Testv20`
1. Copy the `.secrets.sample` to `.secrets` and update the variables based on your new test account

### Running Tests

The test suite can be run with `docker-compose`:

```sh
docker-compose run --rm lib
```

Or if you prefer to just use cargo:

```sh
cargo test -- --nocapture
```
