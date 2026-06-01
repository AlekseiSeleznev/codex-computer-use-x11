#!/usr/bin/env bash
set -euo pipefail

PLUGIN_NAME="codex-computer-use-x11"
MARKETPLACE_NAME="codex-computer-use-x11"
DRY_RUN=0

while [ "$#" -gt 0 ]; do
    case "$1" in
        --dry-run)
            DRY_RUN=1
            ;;
        -h|--help)
            cat <<HELP
Usage: scripts/uninstall-codex-plugin.sh [--dry-run]

Remove the standalone codex-computer-use-x11 MCP plugin from user-local Codex
plugin state. Only owned codex-computer-use-x11 paths and config sections are
removed.
HELP
            exit 0
            ;;
        *)
            echo "unknown argument: $1" >&2
            exit 2
            ;;
    esac
    shift
done

if [ -z "${CODEX_HOME:-}" ]; then
    if [ -z "${HOME:-}" ]; then
        echo "HOME or CODEX_HOME is required" >&2
        exit 1
    fi
    CODEX_HOME="$HOME/.codex"
fi

CONFIG_FILE="${CODEX_CONFIG_FILE:-$CODEX_HOME/config.toml}"
CACHE_NAMESPACE="$CODEX_HOME/plugins/cache/$MARKETPLACE_NAME"
MARKETPLACE_ROOT="$CODEX_HOME/plugins/marketplaces/$MARKETPLACE_NAME"

if [ "$DRY_RUN" -eq 1 ]; then
    cat <<PLAN
DRY RUN: uninstall $PLUGIN_NAME
Would remove owned cache namespace: $CACHE_NAMESPACE
Would remove owned marketplace: $MARKETPLACE_ROOT
Would remove owned Codex config sections from: $CONFIG_FILE
No files were written.
PLAN
    exit 0
fi

rm -rf "$CACHE_NAMESPACE"
rm -rf "$MARKETPLACE_ROOT"

if [ -f "$CONFIG_FILE" ]; then
    python3 - "$CONFIG_FILE" <<'PY'
import pathlib
import re
import sys

config_path = pathlib.Path(sys.argv[1])
plugin_section = 'plugins."codex-computer-use-x11@codex-computer-use-x11"'
marketplace_section = "marketplaces.codex-computer-use-x11"

def is_owned(section: str) -> bool:
    return (
        section == plugin_section
        or section.startswith(plugin_section + ".")
        or section == marketplace_section
        or section.startswith(marketplace_section + ".")
    )

lines = config_path.read_text(encoding="utf-8").splitlines()
out = []
skip = False
section_re = re.compile(r"^\s*\[([^\]]+)\]\s*$")
for line in lines:
    match = section_re.match(line)
    if match:
        skip = is_owned(match.group(1))
        if skip:
            continue
    if not skip:
        out.append(line)

while len(out) > 1 and out[-1] == "" and out[-2] == "":
    out.pop()

config_path.write_text("\n".join(out).rstrip() + ("\n" if out else ""), encoding="utf-8")
PY
fi

echo "Uninstalled $PLUGIN_NAME from owned user-local Codex paths"
echo "Removed cache namespace: $CACHE_NAMESPACE"
echo "Removed marketplace: $MARKETPLACE_ROOT"
echo "Updated config: $CONFIG_FILE"
