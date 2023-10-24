#!/bin/bash
set -eo pipefail

cd "$(git rev-parse --show-toplevel)"

# Cuts a new release, only intended to be run in CI

if [ -z "$TARGET" ] ; then
    echo "TARGET is not set"
    exit 1
fi

./dev/setup.sh

# cargo-binstall should be available from the setup script.
# We need this to cross-compile for aarch64.
cargo binstall -y cross

cross build --release --target "$TARGET" --bin server

dirname="server-$TARGET"
mv "target/$TARGET/release/server" "$dirname"
