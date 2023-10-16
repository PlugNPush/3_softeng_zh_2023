# heavily inspired by:
# https://peterprototypes.com/blog/rust-dockerfile-boilerplate/

# debian based
FROM rust:latest as build

WORKDIR /work

ENV TRUNK_ARCHIVE=trunk-x86_64-unknown-linux-gnu.tar.gz
ENV TRUNK_VERSION=0.17.5
ENV TRUNK_SHA256_SUM=c675099200ff4e13579e4a3fbfbb6dc11375a4b779c2a9efd374f61d360ac7c7

RUN curl -L -O --proto '=https' --tlsv1.2 -sSf "https://github.com/thedodd/trunk/releases/download/v$TRUNK_VERSION/$TRUNK_ARCHIVE"
RUN echo "$TRUNK_SHA256_SUM  $TRUNK_ARCHIVE" | sha256sum -c - || exit 1
RUN tar xf "$TRUNK_ARCHIVE" -C /bin 

# dummy build to get the dependencies compiled and cached
ENV CARGO_CARGO_NEW_VCS="none"
RUN cargo new --lib models && \
    cargo new --lib docs && \
    cargo new app && \
    cargo new server
COPY app/Cargo.toml ./app/
COPY server/Cargo.toml ./server/
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
RUN cargo build --release

# actual build
COPY . .
RUN cd app && trunk build --release
RUN cargo build --release --bin server

# not much bigger than alpine, avoids musl issues
FROM debian:stable-slim as runtime

COPY --from=build /work/target/release/server /usr/bin/server

ENTRYPOINT [ "server" ]