#!/usr/bin/env bash
set -euo pipefail

PLUGIN_NAME="codex-computer-use-x11"
MARKETPLACE_NAME="codex-computer-use-x11"
DRY_RUN=0
ACTIVATE_ACCESSIBILITY=0
REPORT_JSON=""

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
REPO_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

while [ "$#" -gt 0 ]; do
    case "$1" in
        --activate-accessibility)
            ACTIVATE_ACCESSIBILITY=1
            ;;
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
Usage: scripts/install-codex-plugin.sh [--dry-run] [--activate-accessibility] [--report-json [path|-]]

Install the standalone codex-computer-use-x11 MCP plugin into user-local Codex
plugin state without sudo.
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
VERSION="$(python3 - "$REPO_DIR/Cargo.toml" <<'PY'
import pathlib
import re
import sys

text = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
match = re.search(r'(?m)^version\s*=\s*"([^"]+)"', text)
if not match:
    raise SystemExit("could not read package version from Cargo.toml")
print(match.group(1))
PY
)"

CACHE_ROOT="$CODEX_HOME/plugins/cache/$MARKETPLACE_NAME/$PLUGIN_NAME"
CACHE_VERSION="$CACHE_ROOT/$VERSION"
MARKETPLACE_ROOT="$CODEX_HOME/plugins/marketplaces/$MARKETPLACE_NAME"
MARKETPLACE_FILE="$MARKETPLACE_ROOT/.agents/plugins/marketplace.json"
MARKETPLACE_PLUGIN_LINK="$MARKETPLACE_ROOT/plugins/$PLUGIN_NAME"
STATE_DIR="$CODEX_HOME/state/$PLUGIN_NAME"
INSTALL_MANIFEST="$STATE_DIR/install-manifest.json"

emit_report_json() {
    python3 - "$@" <<'PY'
import json
import os
import pathlib
import subprocess
import sys
from typing import Any

(
    dry_run,
    activate_accessibility,
    codex_home,
    config_file,
    cache_version,
    cache_root,
    marketplace_root,
    marketplace_file,
    marketplace_plugin_link,
    report_json,
) = sys.argv[1:]

dry_run_bool = dry_run == "1"
activate_accessibility_bool = activate_accessibility == "1"

def path_state(path: str) -> dict[str, Any]:
    p = pathlib.Path(path)
    return {"exists": p.exists()}

entries: list[dict[str, Any]] = [
    {
        "surface": "plugin_path",
        "path_or_key": "plugin_cache",
        "path": cache_version,
        "before": path_state(cache_version),
        "after": {"exists": True},
        "installer_changed": True,
        "completed": False,
    },
    {
        "surface": "plugin_path",
        "path_or_key": "plugin_latest",
        "path": str(pathlib.Path(cache_root) / "latest"),
        "before": path_state(str(pathlib.Path(cache_root) / "latest")),
        "after": {"exists": True},
        "installer_changed": True,
        "completed": False,
    },
    {
        "surface": "plugin_path",
        "path_or_key": "plugin_marketplace",
        "path": marketplace_file,
        "before": path_state(marketplace_file),
        "after": {"exists": True},
        "installer_changed": True,
        "completed": False,
    },
    {
        "surface": "plugin_path",
        "path_or_key": "plugin_marketplace_link",
        "path": marketplace_plugin_link,
        "before": path_state(marketplace_plugin_link),
        "after": {"exists": True},
        "installer_changed": True,
        "completed": False,
    },
    {
        "surface": "plugin_config",
        "path_or_key": "plugin_config",
        "path": config_file,
        "before": path_state(config_file),
        "after": {"contains_owned_sections": True},
        "installer_changed": True,
        "completed": False,
    },
]

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

if activate_accessibility_bool:
    gsettings_raw = run_stdout(["gsettings", "get", "org.gnome.desktop.interface", "toolkit-accessibility"])
    if gsettings_raw is None:
        toolkit_before: Any = None
        toolkit_present = False
    else:
        value = gsettings_raw.strip().lower()
        toolkit_before = True if value == "true" else False if value == "false" else value
        toolkit_present = True
    toolkit_changed = toolkit_before is not True
    toolkit_completed = False
    if not dry_run_bool:
        toolkit_completed = (not toolkit_changed) or run_action(["gsettings", "set", "org.gnome.desktop.interface", "toolkit-accessibility", "true"])
    entries.append(
        {
            "surface": "gsettings",
            "path_or_key": "org.gnome.desktop.interface toolkit-accessibility",
            "before": {"present": toolkit_present, "value": toolkit_before},
            "after": {"present": True, "value": True},
            "installer_changed": toolkit_changed,
            "completed": toolkit_completed,
        }
    )

    env_raw = run_stdout(["systemctl", "--user", "show-environment"]) or ""
    activation_env: dict[str, str] = {}
    for line in env_raw.splitlines():
        if "=" in line:
            key, value = line.split("=", 1)
            activation_env[key] = value

    def env_entry(key: str, desired_present: bool, desired_value: str | None) -> dict[str, Any]:
        before_present = key in activation_env
        before_value = activation_env.get(key)
        after = {"present": desired_present}
        if desired_present:
            after["value"] = desired_value
        installer_changed = (before_present != desired_present) or (
            desired_present and before_value != desired_value
        )
        completed = False
        if not dry_run_bool:
            if not installer_changed:
                completed = True
            elif desired_present:
                completed = run_action(["systemctl", "--user", "set-environment", f"{key}={desired_value}"])
                completed = run_action(["dbus-update-activation-environment", "--systemd", f"{key}={desired_value}"]) and completed
            else:
                completed = run_action(["systemctl", "--user", "unset-environment", key])
        return {
            "surface": "activation_env",
            "path_or_key": key,
            "before": {"present": before_present, "value": before_value},
            "after": after,
            "installer_changed": installer_changed,
            "completed": completed,
        }

    no_at_bridge_value = activation_env.get("NO_AT_BRIDGE")
    if no_at_bridge_value == "1":
        entries.append(env_entry("NO_AT_BRIDGE", False, None))
    else:
        entries.append(env_entry("NO_AT_BRIDGE", "NO_AT_BRIDGE" in activation_env, no_at_bridge_value))
    entries.append(env_entry("GTK_MODULES", True, "gail:atk-bridge"))
    entries.append(env_entry("QT_ACCESSIBILITY", True, "1"))

report = {
    "schema_version": 1,
    "operation": "install-codex-plugin",
    "dry_run": dry_run_bool,
    "activate_accessibility": activate_accessibility_bool,
    "codex_home": codex_home,
    "entries": entries,
}

payload = json.dumps(report, indent=2, sort_keys=True) + "\n"
if report_json == "-":
    sys.stdout.write(payload)
else:
    path = pathlib.Path(report_json)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(payload, encoding="utf-8")
PY
}

mark_plugin_manifest_completed() {
    python3 - "$INSTALL_MANIFEST" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
if not path.exists():
    raise SystemExit(0)
data = json.loads(path.read_text(encoding="utf-8"))
for entry in data.get("entries", []):
    if entry.get("surface") in {"plugin_path", "plugin_config"}:
        entry["completed"] = True
data["dry_run"] = False
path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")
PY
}

if [ "$DRY_RUN" -eq 1 ]; then
    if [ -n "$REPORT_JSON" ]; then
        emit_report_json \
            "$DRY_RUN" \
            "$ACTIVATE_ACCESSIBILITY" \
            "$CODEX_HOME" \
            "$CONFIG_FILE" \
            "$CACHE_VERSION" \
            "$CACHE_ROOT" \
            "$MARKETPLACE_ROOT" \
            "$MARKETPLACE_FILE" \
            "$MARKETPLACE_PLUGIN_LINK" \
            "$REPORT_JSON"
        exit 0
    fi
    cat <<PLAN
DRY RUN: install $PLUGIN_NAME
Would create owned cache: $CACHE_VERSION
Would update latest symlink: $CACHE_ROOT/latest
Would create owned marketplace: $MARKETPLACE_FILE
Would link marketplace plugin: $MARKETPLACE_PLUGIN_LINK
Would update Codex config sections in: $CONFIG_FILE
$(if [ "$ACTIVATE_ACCESSIBILITY" -eq 1 ]; then echo "Would activate Cinnamon/X11 accessibility baseline."; fi)
No files were written.
PLAN
    exit 0
fi

if [ -n "${CODEX_X11_PLUGIN_BINARY:-}" ]; then
    BINARY_SOURCE="$CODEX_X11_PLUGIN_BINARY"
else
    (cd "$REPO_DIR" && cargo build --release)
    BINARY_SOURCE="$REPO_DIR/target/release/codex-computer-use-x11"
fi

if [ ! -x "$BINARY_SOURCE" ]; then
    echo "plugin binary is not executable: $BINARY_SOURCE" >&2
    exit 1
fi

if [ "$ACTIVATE_ACCESSIBILITY" -eq 1 ]; then
    emit_report_json \
        "$DRY_RUN" \
        "$ACTIVATE_ACCESSIBILITY" \
        "$CODEX_HOME" \
        "$CONFIG_FILE" \
        "$CACHE_VERSION" \
        "$CACHE_ROOT" \
        "$MARKETPLACE_ROOT" \
        "$MARKETPLACE_FILE" \
        "$MARKETPLACE_PLUGIN_LINK" \
        "$INSTALL_MANIFEST"
fi

TMP_PLUGIN="$CACHE_ROOT/.install-$VERSION.$$"
rm -rf "$TMP_PLUGIN"
mkdir -p "$TMP_PLUGIN/.codex-plugin" "$TMP_PLUGIN/bin" "$TMP_PLUGIN/assets"
cp "$BINARY_SOURCE" "$TMP_PLUGIN/bin/codex-computer-use-x11"
chmod 0755 "$TMP_PLUGIN/bin/codex-computer-use-x11"
cp "$REPO_DIR/assets/app-icon.png" "$TMP_PLUGIN/assets/app-icon.png"

python3 - "$TMP_PLUGIN/.codex-plugin/plugin.json" "$VERSION" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
version = sys.argv[2]
manifest = {
    "name": "codex-computer-use-x11",
    "version": version,
    "description": "Standalone X11/EWMH Computer Use MCP tools for Codex.",
    "author": {
        "name": "AlekseiSeleznev",
        "url": "https://github.com/AlekseiSeleznev"
    },
    "homepage": "https://github.com/AlekseiSeleznev/codex-computer-use-x11",
    "license": "MIT",
    "keywords": ["computer-use", "linux", "x11", "ewmh", "mcp"],
    "mcpServers": "./.mcp.json",
    "interface": {
        "displayName": "X11 Computer Use",
        "shortDescription": "Standalone x11_* tools for Linux X11/EWMH",
        "longDescription": "Provides standalone x11_* readiness diagnostics, window listing/focus, keyboard input, pointer actions, accessibility tree, app state, and target-window context tools for validating the codex-computer-use-x11 backend without replacing the bundled Computer Use plugin.",
        "developerName": "AlekseiSeleznev",
        "category": "Productivity",
        "websiteURL": "https://github.com/AlekseiSeleznev/codex-computer-use-x11",
        "logo": "./assets/app-icon.png",
        "defaultPrompt": [
            "Check whether standalone X11 Computer Use is ready with x11_doctor",
            "List X11 windows with x11_list_windows and inspect app state with x11_get_app_state",
            "Save a target context with x11_target_window before using verified x11_type_text, x11_click, x11_scroll, or x11_drag"
        ],
        "brandColor": "#1E293B",
        "screenshots": []
    }
}
path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
PY

python3 - "$TMP_PLUGIN/.mcp.json" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
manifest = {
    "mcpServers": {
        "codex-computer-use-x11": {
            "command": "./bin/codex-computer-use-x11",
            "args": ["mcp"],
            "cwd": "."
        }
    }
}
path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
PY

rm -rf "$CACHE_VERSION"
mkdir -p "$CACHE_ROOT"
mv "$TMP_PLUGIN" "$CACHE_VERSION"
if [ -e "$CACHE_ROOT/latest" ] && [ ! -L "$CACHE_ROOT/latest" ]; then
    rm -rf "$CACHE_ROOT/latest"
fi
ln -sfn "$VERSION" "$CACHE_ROOT/latest"

mkdir -p "$MARKETPLACE_ROOT/.agents/plugins" "$MARKETPLACE_ROOT/plugins"
python3 - "$MARKETPLACE_FILE" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
marketplace = {
    "name": "codex-computer-use-x11",
    "interface": {"displayName": "X11 Computer Use"},
    "plugins": [
        {
            "name": "codex-computer-use-x11",
            "source": {
                "source": "local",
                "path": "./plugins/codex-computer-use-x11"
            },
            "policy": {
                "installation": "AVAILABLE",
                "authentication": "ON_INSTALL"
            },
            "category": "Productivity"
        }
    ]
}
path.write_text(json.dumps(marketplace, indent=2) + "\n", encoding="utf-8")
PY
if [ -e "$MARKETPLACE_PLUGIN_LINK" ] && [ ! -L "$MARKETPLACE_PLUGIN_LINK" ]; then
    rm -rf "$MARKETPLACE_PLUGIN_LINK"
fi
ln -sfn "$CACHE_ROOT/latest" "$MARKETPLACE_PLUGIN_LINK"

mkdir -p "$(dirname "$CONFIG_FILE")"
python3 - "$CONFIG_FILE" "$MARKETPLACE_ROOT" <<'PY'
import datetime as dt
import json
import pathlib
import re
import sys

config_path = pathlib.Path(sys.argv[1])
marketplace_root = sys.argv[2]
plugin_section = 'plugins."codex-computer-use-x11@codex-computer-use-x11"'
marketplace_section = "marketplaces.codex-computer-use-x11"

def is_owned(section: str) -> bool:
    return (
        section == plugin_section
        or section.startswith(plugin_section + ".")
        or section == marketplace_section
        or section.startswith(marketplace_section + ".")
    )

text = config_path.read_text(encoding="utf-8") if config_path.exists() else ""
lines = text.splitlines()
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

while out and out[-1] == "":
    out.pop()

timestamp = dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")
owned = [
    f'[{plugin_section}]',
    "enabled = true",
    "",
    f"[{marketplace_section}]",
    f'last_updated = "{timestamp}"',
    'source_type = "local"',
    f"source = {json.dumps(marketplace_root)}",
]
if out:
    out.append("")
out.extend(owned)
config_path.write_text("\n".join(out) + "\n", encoding="utf-8")
PY

echo "Installed $PLUGIN_NAME $VERSION"
echo "Cache: $CACHE_VERSION"
echo "Marketplace: $MARKETPLACE_FILE"
mark_plugin_manifest_completed
echo "Config updated: $CONFIG_FILE"
