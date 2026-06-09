#!/bin/bash
set -e

if [ -z "$1" ]; then
    echo "Usage: $0 /path/to/stellar-core"
    exit 1
fi

STELLAR_CORE_PATH="$1"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

if [ ! -d "$STELLAR_CORE_PATH" ]; then
    echo "Error: Stellar Core path not found: $STELLAR_CORE_PATH"
    exit 1
fi

echo "Building SMR protocol library..."
cd "$PROJECT_ROOT"
cargo build --release

echo "Copying SMR library to Stellar Core..."
mkdir -p "$STELLAR_CORE_PATH/lib/smr"
cp -r "$PROJECT_ROOT/src" "$STELLAR_CORE_PATH/lib/smr/"
cp "$PROJECT_ROOT/Cargo.toml" "$STELLAR_CORE_PATH/lib/smr/"

echo "Integration complete. Update Stellar Core Cargo.toml to include smr-protocol dependency."
