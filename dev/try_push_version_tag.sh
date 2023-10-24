#!/bin/bash
set -eo pipefail

cd "$(git rev-parse --show-toplevel)"

# If the version was updated in Cargo.toml
# and a corresponding git tag doesn't exist yet, push one.

# read version from Cargo.toml
version=$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)

git config user.name  "Release Bot"
git config user.email "release.bot@gitlab.local"

# grep for version in output of git tag
if git tag | grep -q "^$version$"; then
    echo "Tag $version already exists, nothing to do."
else
    echo "Tag $version doesn't exist yet, creating and pushing it"
    git tag -a "$version" -m "Version $version"
    git push origin "$version"
fi
