#!/bin/bash
set -Eeuo pipefail
SCRIPT_DIR="$(cd -P "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec python3 "$SCRIPT_DIR/codex-desktop-linux-x11-feature.py" install "$@"
