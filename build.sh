#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

PROFILE="${1:-release}"

case "$PROFILE" in
  release)
    echo "[envinject] Building release..."
    cargo build --release
    BIN_PATH="$SCRIPT_DIR/target/release/envinject"
    ;;
  debug)
    echo "[envinject] Building debug..."
    cargo build
    BIN_PATH="$SCRIPT_DIR/target/debug/envinject"
    ;;
  *)
    echo "Usage: ./build.sh [release|debug]"
    exit 1
    ;;
esac

echo "[envinject] Done."
echo "[envinject] Binary: $BIN_PATH"
