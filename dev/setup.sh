#!/bin/bash
set -eo pipefail

cd "$(git rev-parse --show-toplevel)"

# for CI, where sudo is not available
if which sudo &> /dev/null; then
    _sudo="sudo"
else
    _sudo=""
fi

# install system dependencies
if grep -q "fedora" /etc/os-release; then
    packages=(
        "openssl-devel"
        "perl-FindBin"
        "perl-File-Compare"
    )
    sudo dnf install -y "${packages[@]}"
elif grep -q "debian" /etc/os-release; then
    packages=(
        "libssl-dev"
        "libfindbin-libs-perl"
    )
    $_sudo apt-get update -y
    $_sudo apt-get install -y "${packages[@]}"
else
    echo "Unknown OS, make sure you have the necessary"
    echo "packages installed."
fi

if ! which cargo-binstall &> /dev/null ; then
    echo "installing cargo-binstall to install other crates faster..."
    curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
fi
if ! which trunk &> /dev/null ; then
    echo "installing trunk for webassembly frontend..."
    cargo binstall -y trunk
fi
if ! which mdbook &> /dev/null ; then
    echo "installing mdbook to build the documentation..."
    cargo binstall -y mdbook
fi

if [ -z "$CI" ]; then
    # these dev tools are not needed in CI

    if ! which just &> /dev/null ; then
        echo "installing the just command runner..."
        cargo binstall -y just
    fi
    if ! which cargo-watch &> /dev/null ; then
        echo "installing cargo-watch for backend auto-recompilation..."
        cargo binstall -y cargo-watch
    fi
    if ! which zellij &> /dev/null ; then
        echo "installing zellij for the terminal workspace..."
        cargo binstall -y zellij
    fi
    if ! which d2 &> /dev/null || [[ "$(d2 --version)" != "v0.6.1" ]] ; then
        echo "installing d2 diagram renderer..."
        curl -fsSL https://d2lang.com/install.sh | sh -s -- --version v0.6.1
    fi
    if ! which watchexec &> /dev/null ; then
        echo "installing watchexec to watch for diagram changes..."
        cargo binstall -y watchexec-cli
    fi
fi
