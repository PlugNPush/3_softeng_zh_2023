#!/bin/bash
set -eo pipefail

cd "$(git rev-parse --show-toplevel)"

# The purpose of this script is to determine whether or not CI should
# cut a new release. It does this by checking if the current version
# specified in the workspace Cargo.toml matches the one of the latest
# release on GitLab

if [ "$(git branch --show-current)" != "main" ] ; then
    exit 1
fi

last_released_version="TODO_curl_from_gitlab_after_first_release"

current_version="$(./dev/get_current_version.sh)"

if [ "$last_released_version" == "$current_version" ]; then
    exit 1
fi
