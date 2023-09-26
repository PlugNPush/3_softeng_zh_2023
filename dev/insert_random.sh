#!/bin/bash
set -eo pipefail

uuid="$(uuidgen)"

# ISO 8601
timestamp="$(date -u +%Y-%m-%dT%H:%M:%S%Z)"

temperature="$RANDOM"

body="$(printf '{
    "id": "%s",
    "timestamp": "%s",
    "temperature": %s
}' "$uuid" "$timestamp" "$temperature")"

curl -X POST \
    -H "Content-Type: application/json" \
    -d "$body" \
    localhost:4000/api/measurements

echo "inserted temperature '$temperature'"
