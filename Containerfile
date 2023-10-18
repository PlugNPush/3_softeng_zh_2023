# heavily inspired by:
# https://peterprototypes.com/blog/rust-dockerfile-boilerplate/

# debian based
FROM docker.io/rust:1.73-buster AS build

# I'd rather take this from the TARGETPLATFORM arg, but that needs to be 
# further massaged which is not nicely possible within the constraints 
# of a containerfile (and the --arch arg does not seem to get passed 
# along, at least not in buildah + TARGETARCH is wrong).
ARG ARCH=x86_64

# need to build trunk from source for multiarch (or provide prebuilt 
# binaries ourselves)
RUN cargo install --locked trunk
RUN cargo install --locked wasm-bindgen-cli

ENV CARGO_BUILD_RUSTFLAGS="-C target-feature=+crt-static"
ENV CARGO_BUILD_TARGET="$ARCH-unknown-linux-musl"

WORKDIR /work

# dummy build to get the dependencies compiled and cached
ENV CARGO_CARGO_NEW_VCS="none"
RUN cargo new --lib models && \
    cargo new --lib docs && \
    cargo new app && \
    cargo new server
COPY app/Cargo.toml ./app/
COPY server/Cargo.toml ./server/
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
RUN sed -i "/^targets = / s/\]/, \"$CARGO_BUILD_TARGET\"\]/" rust-toolchain.toml
RUN cargo build --release

# actual build
COPY . .
RUN cd app && trunk build --release
RUN cargo build --release --bin server
RUN cp "/work/target/$CARGO_BUILD_TARGET/release/server" /server

FROM scratch

COPY --from=build /server /server

ENTRYPOINT [ "/server" ]
