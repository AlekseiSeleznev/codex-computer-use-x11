#!/usr/bin/env bash
set -euo pipefail

resolve_script_dir() {
    local source="${BASH_SOURCE[0]}"
    local dir
    while [ -L "$source" ]; do
        dir="$(cd -P "$(dirname "$source")" && pwd)"
        source="$(readlink "$source")"
        case "$source" in
            /*) ;;
            *) source="$dir/$source" ;;
        esac
    done
    cd -P "$(dirname "$source")" && pwd
}

SCRIPT_DIR="$(resolve_script_dir)"
DEFAULT_TARGET="/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full"
TARGET="${CODEX_DESKTOP_LINUX_FULL_PATH:-$DEFAULT_TARGET}"
LIVE_ASSETS_DIR="/opt/codex-desktop/content/webview/assets"
CODEX_HOME_ARG=""
DRY_RUN=0
UNINSTALL_PLUGIN=1
CHECK_LIVE_ASSETS=1
REQUIRE_LIVE=0
REPORT_JSON=""

usage() {
    cat <<HELP
Usage: scripts/uninstall-x11-provider-takeover.sh [options]

One-command rollback for X11 Computer Use takeover:
  1. restore provider takeover source/live assets from owned backups;
  2. remove the standalone codex-computer-use-x11 plugin from user-local Codex state;
  3. verify provider takeover markers are absent from configured live assets.

Options:
  --target <path>          Target codex-desktop-linux-full checkout.
  --codex-home <path>      CODEX_HOME for user-local plugin uninstall.
  --live-assets-dir <path> Live webview assets directory (default: /opt/codex-desktop/content/webview/assets).
  --no-plugin              Skip standalone plugin uninstall.
  --no-live-assets         Do not scan/restore live assets.
  --require-live-assets    Fail if live assets directory is missing.
  --report-json <path>     Write aggregate rollback report.
  --dry-run                Print/apply no mutations; subcommands run in dry-run mode.
  -h, --help               Show this help.
HELP
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --target)
            TARGET="${2:?--target requires a path}"
            shift
            ;;
        --codex-home)
            CODEX_HOME_ARG="${2:?--codex-home requires a path}"
            shift
            ;;
        --live-assets-dir)
            LIVE_ASSETS_DIR="${2:?--live-assets-dir requires a path}"
            shift
            ;;
        --no-plugin)
            UNINSTALL_PLUGIN=0
            ;;
        --no-live-assets)
            CHECK_LIVE_ASSETS=0
            ;;
        --require-live-assets)
            REQUIRE_LIVE=1
            ;;
        --report-json)
            REPORT_JSON="${2:?--report-json requires a path}"
            shift
            ;;
        --dry-run)
            DRY_RUN=1
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "unknown argument: $1" >&2
            usage >&2
            exit 2
            ;;
    esac
    shift
done

TARGET="$(python3 -c 'import pathlib,sys; print(pathlib.Path(sys.argv[1]).expanduser().resolve())' "$TARGET")"
STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
REPORT_DIR="$TARGET/.codex-computer-use-x11-overlay/provider-takeover/reports"
if [ -z "$REPORT_JSON" ]; then
    REPORT_JSON="$REPORT_DIR/uninstall-x11-provider-takeover-$STAMP.json"
fi
mkdir -p "$(dirname "$REPORT_JSON")"
SOURCE_REPORT="$(dirname "$REPORT_JSON")/source-overlay-uninstall-$STAMP.json"
PLUGIN_REPORT="$(dirname "$REPORT_JSON")/plugin-uninstall-$STAMP.json"

live_status="skipped"
source_status="unknown"
plugin_status="skipped"
plugin_stdout=""
plugin_stderr=""

source_cmd=(
    python3 "$SCRIPT_DIR/codex-source-overlay.py" uninstall
    --target "$TARGET"
    --provider x11
    --mode takeover
    --report-json "$SOURCE_REPORT"
)
if [ "$DRY_RUN" -eq 1 ]; then
    source_cmd+=(--dry-run)
fi
if [ "$CHECK_LIVE_ASSETS" -eq 1 ]; then
    if [ -d "$LIVE_ASSETS_DIR" ]; then
        source_cmd+=(--live-assets-dir "$LIVE_ASSETS_DIR")
    elif [ "$REQUIRE_LIVE" -eq 1 ]; then
        echo "live assets directory is missing: $LIVE_ASSETS_DIR" >&2
        exit 1
    fi
fi

echo "==> Restoring provider takeover source/live overlay"
"${source_cmd[@]}"
source_status="clean"

if [ "$UNINSTALL_PLUGIN" -eq 1 ]; then
    echo "==> Removing standalone codex-computer-use-x11 plugin"
    plugin_cmd=("$SCRIPT_DIR/uninstall-codex-plugin.sh")
    if [ "$DRY_RUN" -eq 1 ]; then
        plugin_cmd+=(--dry-run)
        plugin_status="dry-run"
    else
        plugin_status="uninstalled"
    fi
    plugin_cmd+=(--report-json "$PLUGIN_REPORT")
    if [ -n "$CODEX_HOME_ARG" ]; then
        CODEX_HOME="$CODEX_HOME_ARG" CODEX_CONFIG_FILE="$CODEX_HOME_ARG/config.toml" "${plugin_cmd[@]}"
    else
        "${plugin_cmd[@]}"
    fi
fi

if [ "$CHECK_LIVE_ASSETS" -eq 1 ] && [ -d "$LIVE_ASSETS_DIR" ] && [ "$DRY_RUN" -eq 1 ]; then
    live_status="dry-run"
elif [ "$CHECK_LIVE_ASSETS" -eq 1 ] && [ -d "$LIVE_ASSETS_DIR" ]; then
    python3 - "$LIVE_ASSETS_DIR" <<'PY'
from pathlib import Path
import sys
live = Path(sys.argv[1])
needles = [
    'codex-computer-use-x11-provider-takeover:v1',
    'codexLinuxComputerUseTakeoverProvider',
    'codex-computer-use-x11-unavailable',
]
hits = []
for asset in sorted(live.glob('computer-use-settings-*.js')):
    text = asset.read_text(encoding='utf-8', errors='replace')
    for needle in needles:
        if needle in text:
            hits.append(f'{asset}:{needle}')
if hits:
    print('live asset takeover markers remain: ' + ', '.join(hits), file=sys.stderr)
    raise SystemExit(2)
PY
    live_status="clean"
elif [ "$CHECK_LIVE_ASSETS" -eq 1 ]; then
    live_status="skipped:not-found"
else
    live_status="skipped"
fi

python3 - "$REPORT_JSON" "$SOURCE_REPORT" "$PLUGIN_REPORT" "$TARGET" "$plugin_status" "$live_status" "$LIVE_ASSETS_DIR" "$DRY_RUN" <<'PY'
import json
import pathlib
import sys
report_path = pathlib.Path(sys.argv[1])
source_report_path = pathlib.Path(sys.argv[2])
plugin_report_path = pathlib.Path(sys.argv[3])
target = sys.argv[4]
plugin_status = sys.argv[5]
live_status = sys.argv[6]
live_assets_dir = sys.argv[7]
dry_run = sys.argv[8] == '1'
source_report = json.loads(source_report_path.read_text(encoding='utf-8')) if source_report_path.is_file() else None
plugin_report = json.loads(plugin_report_path.read_text(encoding='utf-8')) if plugin_report_path.is_file() else None
report = {
    'operation': 'uninstall-x11-provider-takeover',
    'target': target,
    'plugin': {'name': 'codex-computer-use-x11', 'status': plugin_status, 'report': str(plugin_report_path) if plugin_report_path.is_file() else None, 'details': plugin_report},
    'source_overlay': source_report,
    'live_assets': {'dir': live_assets_dir, 'status': live_status},
    'dry_run': dry_run,
    'restart_hint': 'Fully restart Codex Desktop so Electron/webview reloads bundled Computer Use settings assets.',
}
report_path.parent.mkdir(parents=True, exist_ok=True)
report_path.write_text(json.dumps(report, indent=2, ensure_ascii=False, sort_keys=True) + '\n', encoding='utf-8')
print(f'Aggregate report: {report_path}')
PY

echo "==> X11 provider takeover rollback complete"
echo "Target: $TARGET"
echo "Plugin: $plugin_status"
echo "Live assets: $live_status"
echo "Restart hint: fully restart Codex Desktop."
