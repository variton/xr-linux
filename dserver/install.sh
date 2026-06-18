#!/usr/bin/env bash
set -euo pipefail

help_me() {
  cat <<EOF
Usage: $0 <container_name>

Checks:
  - docker is installed
  - current directory contains a Cargo.toml
  - Cargo.toml contains the expected package metadata
  - the specified container exists and is running
  - target/release/dserver exists (builds it if needed)

Then copies:
  target/release/dserver -> <container_name>:/usr/local/bin/dserver

Examples:
  $0 my-container
  $0 --help
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  help_me
  exit 0
fi

if [[ $# -ne 1 ]]; then
  help_me
  exit 1
fi

CONTAINER_NAME="$1"

# Check docker is installed
if ! command -v docker >/dev/null 2>&1; then
  echo "Error: docker is not installed or not in PATH." >&2
  exit 1
fi

# Check Cargo.toml exists
if [[ ! -f "Cargo.toml" ]]; then
  echo "Error: Cargo.toml not found in current directory." >&2
  exit 1
fi

# Verify Cargo.toml contains the expected package metadata
if ! grep -q '^name = "dserver"$' Cargo.toml ||
   ! grep -q '^version = "0.1.0"$' Cargo.toml ||
   ! grep -q '^edition = "2024"$' Cargo.toml; then
  echo "Error: Cargo.toml does not match the expected dserver package metadata." >&2
  exit 1
fi

# Check container exists
if ! docker container inspect "$CONTAINER_NAME" >/dev/null 2>&1; then
  echo "Error: container '$CONTAINER_NAME' does not exist." >&2
  exit 1
fi

# Check container is running
CONTAINER_RUNNING=$(docker inspect -f '{{.State.Running}}' "$CONTAINER_NAME")

if [[ "$CONTAINER_RUNNING" != "true" ]]; then
  echo "Container '$CONTAINER_NAME' is not running."
  echo "Please start it first:"
  echo
  echo "    docker start $CONTAINER_NAME"
  exit 1
fi

BIN_PATH="target/release/dserver"

# Build binary if needed
if [[ ! -x "$BIN_PATH" ]]; then
  echo "Release binary not found. Building..."
  cargo build --release
fi

# Verify build succeeded
if [[ ! -x "$BIN_PATH" ]]; then
  echo "Error: build completed but '$BIN_PATH' was not found." >&2
  exit 1
fi

# Copy binary into container
docker cp "$BIN_PATH" "${CONTAINER_NAME}:/usr/local/bin/dserver"

echo "Successfully copied:"
echo "  $BIN_PATH"
echo "to:"
echo "  ${CONTAINER_NAME}:/usr/local/bin/dserver"
