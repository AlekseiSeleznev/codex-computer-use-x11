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
INSTALL_PLUGIN=1
PATCH_LIVE_AUTO=1
REQUIRE_LIVE=0
REPORT_JSON=""

usage() {
    cat <<HELP
Usage: scripts/install-x11-provider-takeover.sh [options]

One-command rollout for X11 Computer Use takeover:
  1. install/enable the standalone codex-computer-use-x11 plugin;
  2. apply the target source overlay with --provider x11 --mode takeover;
  3. patch live computer-use-settings assets when the live assets directory is writable.

Options:
  --target <path>          Target codex-desktop-linux-full checkout.
  --codex-home <path>      CODEX_HOME for user-local plugin install.
  --live-assets-dir <path> Live webview assets directory (default: /opt/codex-desktop/content/webview/assets).
  --no-plugin              Skip standalone plugin install.
  --no-live-assets         Do not patch live assets; source overlay/rebuild path only.
  --require-live-assets    Fail if live assets cannot be patched.
  --report-json <path>     Write aggregate rollout report (default: target overlay reports dir).
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
            INSTALL_PLUGIN=0
            ;;
        --no-live-assets)
            PATCH_LIVE_AUTO=0
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
    REPORT_JSON="$REPORT_DIR/install-x11-provider-takeover-$STAMP.json"
fi
SOURCE_REPORT="$REPORT_DIR/source-overlay-$STAMP.json"
PLUGIN_REPORT="$REPORT_DIR/plugin-install-$STAMP.json"

mkdir -p "$(dirname "$REPORT_JSON")" "$REPORT_DIR"

PLUGIN_STATUS="skipped"
if [ "$INSTALL_PLUGIN" -eq 1 ]; then
    echo "==> Installing standalone codex-computer-use-x11 plugin"
    plugin_cmd=("$SCRIPT_DIR/install-codex-plugin.sh" "--activate-accessibility")
    if [ "$DRY_RUN" -eq 1 ]; then
        plugin_cmd+=("--dry-run" "--report-json" "$PLUGIN_REPORT")
        PLUGIN_STATUS="dry-run"
    else
        PLUGIN_STATUS="installed"
    fi
    if [ -n "$CODEX_HOME_ARG" ]; then
        CODEX_HOME="$CODEX_HOME_ARG" "${plugin_cmd[@]}"
    else
        "${plugin_cmd[@]}"
    fi
fi

LIVE_STATUS="skipped"
source_cmd=(
    "$SCRIPT_DIR/install-codex-source-overlay.sh"
    --target "$TARGET"
    --provider x11
    --mode takeover
    --report-json "$SOURCE_REPORT"
)
if [ "$DRY_RUN" -eq 1 ]; then
    source_cmd+=(--dry-run)
fi

if [ "$PATCH_LIVE_AUTO" -eq 1 ]; then
    if [ -d "$LIVE_ASSETS_DIR" ] && compgen -G "$LIVE_ASSETS_DIR/computer-use-settings-*.js" >/dev/null; then
        if [ "$DRY_RUN" -eq 1 ] || [ -w "$LIVE_ASSETS_DIR" ]; then
            source_cmd+=(--patch-live-assets --live-assets-dir "$LIVE_ASSETS_DIR")
            LIVE_STATUS=$([ "$DRY_RUN" -eq 1 ] && echo "dry-run" || echo "patched")
        elif [ "$REQUIRE_LIVE" -eq 1 ]; then
            echo "live assets directory is not writable: $LIVE_ASSETS_DIR" >&2
            echo "Run the live-asset patch step with appropriate permissions, or use --no-live-assets for source-only rollout." >&2
            exit 1
        else
            LIVE_STATUS="skipped:not-writable"
            echo "WARN: live assets directory is not writable; skipping live asset patch: $LIVE_ASSETS_DIR" >&2
            echo "WARN: rebuild/reinstall Codex Desktop from the patched target, or rerun with permissions for live assets." >&2
        fi
    elif [ "$REQUIRE_LIVE" -eq 1 ]; then
        echo "no computer-use-settings-*.js live assets found in: $LIVE_ASSETS_DIR" >&2
        exit 1
    else
        LIVE_STATUS="skipped:not-found"
        echo "WARN: no live computer-use-settings assets found; source overlay will still be applied." >&2
    fi
fi

echo "==> Applying target provider takeover overlay"
"${source_cmd[@]}"

python3 - "$REPORT_JSON" "$SOURCE_REPORT" "$PLUGIN_REPORT" "$TARGET" "$PLUGIN_STATUS" "$LIVE_STATUS" "$LIVE_ASSETS_DIR" "$DRY_RUN" "$CODEX_HOME_ARG" <<'PY'
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
dry_run = sys.argv[8] == "1"
codex_home_arg = sys.argv[9]
source_report = json.loads(source_report_path.read_text(encoding="utf-8")) if source_report_path.is_file() else None
plugin_report = json.loads(plugin_report_path.read_text(encoding="utf-8")) if plugin_report_path.is_file() else None
report = {
    "operation": "install-x11-provider-takeover",
    "target": target,
    "plugin": {
        "name": "codex-computer-use-x11",
        "status": plugin_status,
        "activate_accessibility": plugin_status != "skipped",
        "report": str(plugin_report_path) if plugin_report_path.is_file() else None,
        "manifest": str(pathlib.Path(codex_home_arg).joinpath("state/codex-computer-use-x11/install-manifest.json")) if codex_home_arg else None,
        "details": plugin_report,
    },
    "source_overlay": source_report,
    "live_assets": {"dir": live_assets_dir, "status": live_status},
    "dry_run": dry_run,
    "restart_hint": "Fully restart Codex Desktop after this installer. If live assets were skipped, rebuild/reinstall the patched target first.",
}
report_path.write_text(json.dumps(report, indent=2, ensure_ascii=False, sort_keys=True) + "\n", encoding="utf-8")
print(f"Aggregate report: {report_path}")
PY

echo "==> X11 provider takeover rollout complete"
echo "Target: $TARGET"
echo "Plugin: $PLUGIN_STATUS"
echo "Live assets: $LIVE_STATUS"
echo "Restart hint: fully restart Codex Desktop. If live assets were skipped, rebuild/reinstall the patched target first."
