#!/usr/bin/env bash
set -euo pipefail

PLUGIN_NAME="codex-computer-use-x11"
MARKETPLACE_NAME="codex-computer-use-x11"
DRY_RUN=0
REPORT_JSON=""

while [ "$#" -gt 0 ]; do
    case "$1" in
        --dry-run)
            DRY_RUN=1
            ;;
        --report-json)
            if [ "${2:-}" != "" ] && [ "${2#-}" = "$2" ]; then
                REPORT_JSON="$2"
                shift
            else
                REPORT_JSON="-"
            fi
            ;;
        -h|--help)
            cat <<HELP
Usage: scripts/uninstall-codex-plugin.sh [--dry-run] [--report-json [path|-]]

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
STATE_DIR="$CODEX_HOME/state/$PLUGIN_NAME"
INSTALL_MANIFEST="$STATE_DIR/install-manifest.json"

restore_accessibility_from_manifest() {
    python3 - "$@" <<'PY'
import json
import pathlib
import subprocess
import sys
from typing import Any

dry_run, manifest_path, report_json = sys.argv[1:]
dry_run_bool = dry_run == "1"
manifest = pathlib.Path(manifest_path)
outcomes: list[dict[str, Any]] = []
blockers: list[dict[str, Any]] = []

def run_stdout(args: list[str]) -> str | None:
    try:
        proc = subprocess.run(args, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
    except FileNotFoundError:
        return None
    if proc.returncode != 0:
        return None
    return proc.stdout

def run_action(args: list[str]) -> bool:
    if dry_run_bool:
        return True
    try:
        proc = subprocess.run(args, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
    except FileNotFoundError:
        return False
    return proc.returncode == 0

def current_gsetting() -> Any:
    raw = run_stdout(["gsettings", "get", "org.gnome.desktop.interface", "toolkit-accessibility"])
    if raw is None:
        return None
    value = raw.strip().lower()
    if value == "true":
        return True
    if value == "false":
        return False
    return value

def current_activation_env() -> dict[str, str]:
    raw = run_stdout(["systemctl", "--user", "show-environment"]) or ""
    result: dict[str, str] = {}
    for line in raw.splitlines():
        if "=" in line:
            key, value = line.split("=", 1)
            result[key] = value
    return result

if manifest.exists():
    data = json.loads(manifest.read_text(encoding="utf-8"))
    activation_env = current_activation_env()
    for entry in data.get("entries", []):
        if not entry.get("installer_changed") or not entry.get("completed"):
            continue
        surface = entry.get("surface")
        key = entry.get("path_or_key")
        before = entry.get("before", {})
        after = entry.get("after", {})
        if surface == "gsettings" and key == "org.gnome.desktop.interface toolkit-accessibility":
            current = current_gsetting()
            expected_after = after.get("value")
            if current != expected_after:
                blockers.append({"path_or_key": key, "reason": "drift", "current": current, "expected_after": expected_after})
                continue
            restore_value = before.get("value")
            ok = run_action(["gsettings", "set", "org.gnome.desktop.interface", "toolkit-accessibility", "true" if restore_value is True else "false"])
            outcomes.append({"path_or_key": key, "surface": surface, "restored": ok, "dry_run": dry_run_bool})
        elif surface == "activation_env":
            current_present = key in activation_env
            current_value = activation_env.get(key)
            expected_present = bool(after.get("present"))
            expected_value = after.get("value")
            if current_present != expected_present or (expected_present and current_value != expected_value):
                blockers.append({
                    "path_or_key": key,
                    "reason": "drift",
                    "current": {"present": current_present, "value": current_value},
                    "expected_after": after,
                })
                continue
            before_present = bool(before.get("present"))
            before_value = before.get("value")
            if before_present:
                ok1 = run_action(["systemctl", "--user", "set-environment", f"{key}={before_value}"])
                ok2 = run_action(["dbus-update-activation-environment", "--systemd", f"{key}={before_value}"])
                ok = ok1 and ok2
            else:
                ok1 = run_action(["systemctl", "--user", "unset-environment", key])
                # dbus-update-activation-environment has no portable unset primitive; systemd user env is authoritative here.
                ok = ok1
            outcomes.append({"path_or_key": key, "surface": surface, "restored": ok, "dry_run": dry_run_bool})

report = {
    "schema_version": 1,
    "operation": "uninstall-codex-plugin",
    "dry_run": dry_run_bool,
    "manifest": str(manifest),
    "manifest_present": manifest.exists(),
    "outcomes": outcomes,
    "blockers": blockers,
}

if report_json:
    payload = json.dumps(report, indent=2, sort_keys=True) + "\n"
    if report_json == "-":
        sys.stdout.write(payload)
    else:
        path = pathlib.Path(report_json)
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(payload, encoding="utf-8")
elif blockers:
    for blocker in blockers:
        print(f"rollback blocker: {blocker}", file=sys.stderr)
    raise SystemExit(1)
PY
}

if [ "$DRY_RUN" -eq 1 ]; then
    if [ -n "$REPORT_JSON" ]; then
        restore_accessibility_from_manifest "$DRY_RUN" "$INSTALL_MANIFEST" "$REPORT_JSON"
        exit 0
    fi
    cat <<PLAN
DRY RUN: uninstall $PLUGIN_NAME
Would remove owned cache namespace: $CACHE_NAMESPACE
Would remove owned marketplace: $MARKETPLACE_ROOT
Would remove owned Codex config sections from: $CONFIG_FILE
No files were written.
PLAN
    exit 0
fi

restore_accessibility_from_manifest "$DRY_RUN" "$INSTALL_MANIFEST" "$REPORT_JSON"

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

if [ -z "$REPORT_JSON" ]; then
    echo "Uninstalled $PLUGIN_NAME from owned user-local Codex paths"
    echo "Removed cache namespace: $CACHE_NAMESPACE"
    echo "Removed marketplace: $MARKETPLACE_ROOT"
    echo "Updated config: $CONFIG_FILE"
fi
