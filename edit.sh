#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [[ -x "$SCRIPT_DIR/target/release/envinject" ]]; then
  exec "$SCRIPT_DIR/target/release/envinject" gui
elif [[ -x "$SCRIPT_DIR/target/debug/envinject" ]]; then
  exec "$SCRIPT_DIR/target/debug/envinject" gui
else
  cd "$SCRIPT_DIR"
  exec cargo run -- gui
fi
