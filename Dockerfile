FROM debian:jessie

ENV RUST_VERSION=1.16.0
ENV RUST_TARGET=x86_64-unknown-linux-gnu

RUN \
  apt-get update && \
  DEBIAN_FRONTEND=noninteractive \
  apt-get install -y ca-certificates curl build-essential gcc libssl-dev openssl pkg-config && \
  curl \
    -sO \
    https://static.rust-lang.org/dist/rust-$RUST_VERSION-$RUST_TARGET.tar.gz \
    && \
  tar -xzf rust-$RUST_VERSION-$RUST_TARGET.tar.gz && \
  ./rust-$RUST_VERSION-$RUST_TARGET/install.sh --without=rust-docs && \
  DEBIAN_FRONTEND=noninteractive apt-get remove --purge -y curl && \
  rm -rf \
    ./rust-$RUST_VERSION-$RUST_TARGET \
    rust-$RUST_VERSION-$RUST_TARGET.tar.gz \
    /var/lib/apt/lists/* \
    /tmp/* \
    /var/tmp/*

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

COPY . /usr/src/app

CMD ["cargo", "build"]
