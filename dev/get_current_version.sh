#!/bin/bash
set -eo pipefail

cd "$(git rev-parse --show-toplevel)"

# I have no idea how this regex works, it's a combination
# of Copilot halucinations and throwing spaghetti at the wall.
# This would be much nicer with nushell.
# But it's an additional install and this regex works.
grep -oP '(?<=^version = ").*?(?=")' Cargo.toml
