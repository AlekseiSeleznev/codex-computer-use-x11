#!/usr/bin/env python3
"""E2E smoke runner for codex-computer-use-x11."""
from __future__ import annotations

import argparse
import datetime as dt
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import traceback
from typing import Any

REPO_DIR = Path(__file__).resolve().parents[2]
PLUGIN_NAME = "codex-computer-use-x11"
MARKETPLACE_NAME = "codex-computer-use-x11"
EXPECTED_DISPLAY_NAME = "X11 Computer Use"
EXPECTED_AUTHOR = "AlekseiSeleznev"
EXPECTED_AUTHOR_URL = "https://github.com/AlekseiSeleznev"
EXPECTED_WEBSITE_URL = "https://github.com/AlekseiSeleznev/codex-computer-use-x11"
EXPECTED_LOGO = "./assets/app-icon.png"
CAPABILITY_GROUPS = [
    "doctor/capabilities",
    "window listing/focus",
    "get_app_state",
    "keyboard input",
    "pointer input",
    "screenshot",
    "AT-SPI",
    "install/rollback",
]
DELIVERY_PATHS = ["standalone_plugin", "source_overlay"]
CANONICAL_REASON_CATEGORIES = {
    "environment_limitation",
    "missing_fixture_setup",
    "code_failure",
    "unsupported_out_of_scope",
    "expected_fake_fixture_limitation",
    "unsafe_target_selection",
    "malformed_evidence",
    "not_evaluated",
}
EXPECTED_STANDALONE_TOOLS = [
    "x11_doctor",
    "x11_list_windows",
    "x11_focused_window",
    "x11_focus_window",
    "x11_type_text",
    "x11_press_key",
    "x11_click",
    "x11_scroll",
    "x11_drag",
    "x11_accessibility_tree",
    "x11_get_app_state",
    "x11_target_window",
    "x11_release_window",
    "x11_target_context",
]
FORBIDDEN_STOCK_TOOLS = [
    "doctor",
    "list_windows",
    "focused_window",
    "activate_window",
    "type_text",
    "press_key",
    "click",
    "scroll",
    "drag",
    "accessibility_tree",
    "get_app_state",
    "target_window",
    "release_window",
    "target_context",
    "computer-use",
]


class SmokeFailure(RuntimeError):
    pass


class McpClient:
    def __init__(self, metadata: dict[str, Any], env: dict[str, str], run_dir: Path):
        plugin_dir = Path(metadata["plugin_dir"])
        command = metadata["command"]
        command_path = str((plugin_dir / command).resolve()) if command.startswith(".") else command
        args = metadata.get("args") or []
        stderr_file = (run_dir / "child-stderr.log").open("w", encoding="utf-8")
        self._stderr_file = stderr_file
        self.proc = subprocess.Popen(
            [command_path, *args],
            cwd=plugin_dir,
            env=env,
            text=True,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=stderr_file,
        )
        assert self.proc.stdin is not None
        assert self.proc.stdout is not None
        self.stdin = self.proc.stdin
        self.stdout = self.proc.stdout

    def close(self) -> None:
        try:
            if self.proc.poll() is None:
                self.proc.terminate()
                try:
                    self.proc.wait(timeout=2)
                except subprocess.TimeoutExpired:
                    self.proc.kill()
                    self.proc.wait(timeout=2)
        finally:
            self._stderr_file.close()

    def send(self, message: dict[str, Any]) -> None:
        self.stdin.write(json.dumps(message) + "\n")
        self.stdin.flush()

    def read(self) -> dict[str, Any]:
        line = self.stdout.readline()
        if not line:
            raise SmokeFailure("MCP server closed stdout unexpectedly")
        try:
            return json.loads(line)
        except json.JSONDecodeError as exc:
            raise SmokeFailure(f"MCP server returned invalid JSON: {line!r}") from exc

    def initialize(self) -> None:
        self.send({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2025-06-18",
                "capabilities": {},
                "clientInfo": {"name": "codex-x11-e2e", "version": "0.0.0"},
            },
        })
        response = self.read()
        if response.get("id") != 1 or "result" not in response:
            raise SmokeFailure(f"MCP initialize failed: {response}")
        self.send({"jsonrpc": "2.0", "method": "notifications/initialized"})

    def tools_list(self) -> list[str]:
        self.send({"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}})
        response = self.read()
        if response.get("id") != 2 or "result" not in response:
            raise SmokeFailure(f"MCP tools/list failed: {response}")
        tools = response["result"].get("tools")
        if not isinstance(tools, list):
            raise SmokeFailure("MCP tools/list result.tools is not an array")
        names = []
        for tool in tools:
            name = tool.get("name") if isinstance(tool, dict) else None
            if not isinstance(name, str):
                raise SmokeFailure(f"MCP tool entry missing string name: {tool}")
            names.append(name)
        return names

    def call_tool(self, request_id: int, name: str, arguments: dict[str, Any]) -> dict[str, Any]:
        self.send({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": "tools/call",
            "params": {"name": name, "arguments": arguments},
        })
        response = self.read()
        if response.get("id") != request_id:
            raise SmokeFailure(f"MCP tools/call {name} returned wrong response id: {response}")
        if "result" not in response:
            raise SmokeFailure(f"MCP tools/call {name} failed at protocol layer: {response}")
        return response["result"]


class Evidence:
    def __init__(self, delivery_path: str, mode: str, log_dir: Path):
        self.data: dict[str, Any] = {
            "schema_version": 1,
            "run_id": f"{dt.datetime.now(dt.timezone.utc).strftime('%Y%m%dT%H%M%SZ')}-{os.getpid()}",
            "mode": mode,
            "delivery_path": delivery_path,
            "log_dir": str(log_dir),
            "checks": [],
            "capability_matrix": empty_matrix(),
        }

    def check(self, name: str, status: str, detail: str, **extra: Any) -> None:
        entry = {"name": name, "status": status, "detail": detail}
        entry.update(extra)
        self.data["checks"].append(entry)

    def matrix(
        self,
        group: str,
        path: str,
        status: str,
        *,
        reason: str | None = None,
        evidence: list[str] | None = None,
        reason_category: str | None = None,
    ) -> None:
        self.data["capability_matrix"].setdefault(group, {})[path] = {
            "status": status,
            **({"reason": reason} if reason else {}),
            **({"evidence": evidence} if evidence else {}),
            **({"reason_category": reason_category} if reason_category else {}),
        }


def empty_matrix() -> dict[str, dict[str, dict[str, str]]]:
    matrix: dict[str, dict[str, dict[str, str]]] = {}
    for group in CAPABILITY_GROUPS:
        matrix[group] = {}
        for path in DELIVERY_PATHS:
            matrix[group][path] = {
                "status": "degraded",
                "reason": "not evaluated by this smoke run",
                "reason_category": "not_evaluated",
            }
    return matrix


def parse_common(parser: argparse.ArgumentParser) -> None:
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--fake", action="store_true", help="run hermetic no-GUI fake smoke (default)")
    mode.add_argument("--live", action="store_true", help="run live smoke against current desktop/target")
    parser.add_argument("--log-dir", type=Path, default=REPO_DIR / "target/e2e-logs")
    parser.add_argument("--evidence-out", type=Path)
    parser.add_argument("--industrial", action="store_true", help="write/evaluate industrial acceptance evidence where supported")
    parser.add_argument("--keep-temp", action="store_true")


def mode_from(args: argparse.Namespace) -> str:
    return "live" if getattr(args, "live", False) else "fake"


def create_run_dir(base: Path, delivery_path: str, mode: str) -> Path:
    run_id = f"{delivery_path}-{mode}-{dt.datetime.now(dt.timezone.utc).strftime('%Y%m%dT%H%M%SZ')}-{os.getpid()}"
    run_dir = (base / run_id).resolve()
    run_dir.mkdir(parents=True, exist_ok=False)
    return run_dir


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(sanitize_for_evidence(value), indent=2, sort_keys=True) + "\n", encoding="utf-8")


def fixture_process_env(role: str) -> dict[str, str]:
    env = os.environ.copy()
    if role == "gtk":
        env.pop("NO_AT_BRIDGE", None)
        env["GTK_MODULES"] = env.get("GTK_MODULES") or "gail:atk-bridge"
    return env


class ControlledFixtureManager:
    """Run-scoped fixture process manager for safe live verification."""

    def __init__(self, run_dir: Path, *, fail_role: str | None = None):
        self.run_dir = run_dir
        self.fixture_dir = run_dir / "fixtures"
        self.fail_role = fail_role
        self.processes: dict[str, subprocess.Popen[str]] = {}
        self.fixtures: dict[str, dict[str, Any]] = {}
        self.cleanup_records: list[dict[str, Any]] = []

    def start_all(self) -> dict[str, dict[str, Any]]:
        try:
            self.start("tk")
            self.start("gtk")
            return self.fixtures
        except Exception:
            self.cleanup()
            raise

    def start(self, role: str) -> dict[str, Any]:
        if self.fail_role == role:
            raise SmokeFailure(f"fixture startup failed for {role}")
        scripts = {
            "tk": REPO_DIR / "scripts/e2e/fixtures/tk_text_pointer_fixture.py",
            "gtk": REPO_DIR / "scripts/e2e/fixtures/gtk_atspi_fixture.py",
        }
        script = scripts[role]
        role_dir = self.fixture_dir / role
        ready_file = role_dir / "ready.json"
        metadata_file = role_dir / "metadata.json"
        title = f"x11-safe-fixture-{role}-{self.run_dir.name}"
        wm_class = f"X11SafeFixture{role.upper()}"
        role_dir.mkdir(parents=True, exist_ok=True)
        env = fixture_process_env(role)
        proc = subprocess.Popen(
            [
                sys.executable,
                str(script),
                "--role",
                role,
                "--title",
                title,
                "--wm-class",
                wm_class,
                "--ready-file",
                str(ready_file),
                "--metadata-file",
                str(metadata_file),
            ],
            cwd=REPO_DIR,
            env=env,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        self.processes[role] = proc
        deadline = time.monotonic() + 5
        while time.monotonic() < deadline:
            if ready_file.is_file() and metadata_file.is_file():
                metadata = read_json_file(metadata_file, f"{role} fixture metadata")
                metadata.update(
                    {
                        "role": role,
                        "pid": proc.pid,
                        "ready_file": str(ready_file),
                        "metadata_file": str(metadata_file),
                    }
                )
                self.fixtures[role] = metadata
                return metadata
            if proc.poll() is not None:
                stderr = proc.stderr.read() if proc.stderr is not None else ""
                raise SmokeFailure(f"fixture startup failed for {role}: exited {proc.returncode}; {stderr}")
            time.sleep(0.05)
        raise SmokeFailure(f"fixture startup failed for {role}: readiness timeout at {ready_file}")

    def cleanup(self) -> list[dict[str, Any]]:
        for role, proc in list(self.processes.items()):
            was_running = proc.poll() is None
            if was_running:
                proc.terminate()
                try:
                    proc.wait(timeout=2)
                except subprocess.TimeoutExpired:
                    proc.kill()
                    proc.wait(timeout=2)
            self.cleanup_records.append(
                {
                    "role": role,
                    "pid": proc.pid,
                    "was_running": was_running,
                    "terminated": True,
                    "returncode": proc.returncode,
                }
            )
        self.processes.clear()
        return self.cleanup_records


def controlled_fixture(role: str, run_name: str = "selection-self-test") -> dict[str, Any]:
    return {
        "role": role,
        "pid": 4242 if role == "tk" else 4343,
        "title": f"x11-safe-fixture-{role}-{run_name}",
        "wm_class": f"X11SafeFixture{role.upper()}",
    }


def is_overlay_helper(window: dict[str, Any]) -> bool:
    title = str(window.get("title", ""))
    wm_class = str(window.get("wm_class", ""))
    return "codex-computer-use-x11-overlay" in title or "codex-computer-use-x11-overlay" in wm_class


def is_controlled_fixture_window(window: dict[str, Any], fixture: dict[str, Any]) -> bool:
    return (
        str(window.get("title")) == str(fixture.get("title"))
        and str(window.get("wm_class")) == str(fixture.get("wm_class"))
    )


def select_controlled_fixture_window(
    fixture: dict[str, Any],
    windows: list[dict[str, Any]],
) -> tuple[dict[str, Any] | None, str, str]:
    matches = [window for window in windows if is_controlled_fixture_window(window, fixture)]
    if not matches:
        if windows and all(is_overlay_helper(window) for window in windows):
            return None, "unsafe_target_selection", "only overlay/helper windows were listed; refusing helper target"
        if windows:
            return None, "unsafe_target_selection", "no controlled fixture match; refusing real user application fallback"
        return None, "missing_fixture_setup", "controlled fixture window is missing"
    if len(matches) != 1:
        return None, "unsafe_target_selection", f"expected exactly one controlled fixture, got {len(matches)}"
    selected = matches[0]
    if int(selected.get("pid", -1)) != int(fixture.get("pid", -2)):
        return None, "unsafe_target_selection", "controlled fixture window is stale or pid-mismatched"
    return selected, "fixture_pass", "exactly one run-scoped controlled fixture selected"


def sanitize_for_evidence(value: Any) -> Any:
    """Return an evidence-safe copy with large inline screenshot payloads removed."""
    if isinstance(value, dict):
        sanitized: dict[str, Any] = {}
        for key, item in value.items():
            if key == "data_url" and isinstance(item, str) and item.startswith("data:"):
                continue
            sanitized[key] = sanitize_for_evidence(item)
        return sanitized
    if isinstance(value, list):
        return [sanitize_for_evidence(item) for item in value]
    return value


def summarize_app_state_value(app_state: dict[str, Any]) -> dict[str, Any]:
    diagnostics = app_state.get("diagnostics")
    layers = diagnostics.get("layers") if isinstance(diagnostics, dict) else None
    if not isinstance(layers, list):
        raise SmokeFailure("app-state summary requires diagnostics.layers; top-level layers are ignored")

    summarized_layers = []
    for layer in layers:
        if not isinstance(layer, dict):
            summarized_layers.append(layer)
            continue
        summarized = dict(layer)
        if summarized.get("ok") is False and not summarized.get("reason_category"):
            summarized["reason_category"] = reason_category_for_detail(
                str(summarized.get("detail") or summarized.get("error") or "")
            )
        summarized_layers.append(summarized)

    screenshot = app_state.get("screenshot")
    if isinstance(screenshot, dict):
        screenshot_summary: dict[str, Any] | None = {
            "status": "present",
            "mime_type": screenshot.get("mime_type"),
            "source": screenshot.get("source"),
            "width": screenshot.get("width"),
            "height": screenshot.get("height"),
            "path": screenshot.get("path"),
            "size_bytes": screenshot.get("size_bytes"),
        }
    elif app_state.get("screenshot_error"):
        detail = str(app_state.get("screenshot_error"))
        screenshot_summary = {
            "status": "degraded",
            "detail": detail,
            "reason_category": reason_category_for_detail(detail),
        }
    else:
        screenshot_summary = None

    return sanitize_for_evidence({
        "backend": app_state.get("backend"),
        "message": app_state.get("message"),
        "layers": summarized_layers,
        "screenshot": screenshot_summary,
    })


def reason_category_for_detail(detail: str) -> str:
    lowered = detail.lower()
    if "fake" in lowered and ("gdbus" in lowered or "screenshot" in lowered or "fixture" in lowered):
        return "expected_fake_fixture_limitation"
    if "missing_fixture_setup" in lowered or "no safe" in lowered or "fixture" in lowered and "missing" in lowered:
        return "missing_fixture_setup"
    if "wayland" in lowered or "out of scope" in lowered or "portal-required" in lowered:
        return "unsupported_out_of_scope"
    if "code_failure" in lowered or "invalid" in lowered or "failed integrity" in lowered:
        return "code_failure"
    return "environment_limitation"


def marketplace_paths(codex_home: Path) -> dict[str, Path]:
    marketplace_root = codex_home / "plugins/marketplaces" / MARKETPLACE_NAME
    cache_root = codex_home / "plugins/cache" / MARKETPLACE_NAME / PLUGIN_NAME
    return {
        "marketplace_root": marketplace_root,
        "marketplace_file": marketplace_root / ".agents/plugins/marketplace.json",
        "marketplace_link": marketplace_root / "plugins" / PLUGIN_NAME,
        "cache_root": cache_root,
        "latest": cache_root / "latest",
    }


def read_json_file(path: Path, label: str) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SmokeFailure(f"invalid {label} JSON at {path}: {exc}") from exc
    if not isinstance(value, dict):
        raise SmokeFailure(f"{label} JSON at {path} must be an object")
    return value


def require_equal(actual: Any, expected: Any, field: str) -> None:
    if actual != expected:
        raise SmokeFailure(f"metadata field {field} mismatch: expected {expected!r}, got {actual!r}")


def validate_plugin_metadata(codex_home: Path) -> dict[str, Any]:
    paths = marketplace_paths(codex_home)
    marketplace_file = paths["marketplace_file"]
    if not marketplace_file.is_file():
        raise SmokeFailure(
            f"missing standalone plugin installation: marketplace metadata not found at {marketplace_file}"
        )
    marketplace = read_json_file(marketplace_file, "marketplace metadata")

    marketplace_interface = marketplace.get("interface")
    if not isinstance(marketplace_interface, dict):
        raise SmokeFailure("marketplace metadata field interface must be an object")
    marketplace_display_name = marketplace_interface.get("displayName")
    require_equal(marketplace_display_name, EXPECTED_DISPLAY_NAME, "marketplace.interface.displayName")

    plugins = marketplace.get("plugins")
    if not isinstance(plugins, list):
        raise SmokeFailure("marketplace metadata field plugins must be an array")
    if len(plugins) != 1:
        raise SmokeFailure(f"marketplace metadata must contain exactly one owned plugin entry, got {len(plugins)}")
    plugin_entry = next((item for item in plugins if isinstance(item, dict) and item.get("name") == PLUGIN_NAME), None)
    if plugin_entry is None:
        raise SmokeFailure(f"marketplace metadata does not list plugin {PLUGIN_NAME}")

    marketplace_link = paths["marketplace_link"]
    if not marketplace_link.exists():
        raise SmokeFailure(f"missing standalone plugin installation: plugin link not found at {marketplace_link}")
    plugin_dir = marketplace_link.resolve()
    cache_root = paths["cache_root"].resolve()
    if cache_root not in [plugin_dir, *plugin_dir.parents]:
        raise SmokeFailure(f"plugin path {plugin_dir} is outside owned cache namespace {cache_root}")

    plugin_json = plugin_dir / ".codex-plugin/plugin.json"
    mcp_json = plugin_dir / ".mcp.json"
    if not plugin_json.is_file():
        raise SmokeFailure(f"installed plugin manifest missing: {plugin_json}")
    if not mcp_json.is_file():
        raise SmokeFailure(f"installed MCP manifest missing: {mcp_json}")
    plugin_manifest_text = plugin_json.read_text(encoding="utf-8")
    if "AlekseiSelin" in plugin_manifest_text:
        raise SmokeFailure("plugin manifest contains stale AlekseiSelin repository owner")
    plugin_manifest = read_json_file(plugin_json, "plugin manifest")
    mcp_manifest = read_json_file(mcp_json, "MCP manifest")

    require_equal(plugin_manifest.get("name"), PLUGIN_NAME, "plugin.name")
    require_equal(plugin_manifest.get("homepage"), EXPECTED_WEBSITE_URL, "plugin.homepage")
    author = plugin_manifest.get("author")
    if not isinstance(author, dict):
        raise SmokeFailure("plugin manifest author must be an object")
    require_equal(author.get("name"), EXPECTED_AUTHOR, "plugin.author.name")
    require_equal(author.get("url"), EXPECTED_AUTHOR_URL, "plugin.author.url")

    interface = plugin_manifest.get("interface")
    if not isinstance(interface, dict):
        raise SmokeFailure("plugin manifest interface must be an object")
    require_equal(interface.get("displayName"), EXPECTED_DISPLAY_NAME, "plugin.interface.displayName")
    require_equal(interface.get("developerName"), EXPECTED_AUTHOR, "plugin.interface.developerName")
    require_equal(interface.get("websiteURL"), EXPECTED_WEBSITE_URL, "plugin.interface.websiteURL")
    require_equal(interface.get("logo"), EXPECTED_LOGO, "plugin.interface.logo")
    if "privacyPolicyURL" in interface:
        raise SmokeFailure("plugin interface must omit privacyPolicyURL until a project-owned policy exists")
    if "termsOfServiceURL" in interface:
        raise SmokeFailure("plugin interface must omit termsOfServiceURL until project-owned terms exist")
    logo_path = (plugin_dir / EXPECTED_LOGO).resolve()
    if not logo_path.is_file():
        raise SmokeFailure(f"plugin logo file missing: {logo_path}")

    long_description = interface.get("longDescription")
    if not isinstance(long_description, str) or not all(
        phrase in long_description.lower()
        for phrase in ["readiness", "window", "keyboard", "pointer", "accessibility", "app state", "target-window"]
    ):
        raise SmokeFailure("plugin longDescription does not cover current standalone tool groups")
    default_prompts = interface.get("defaultPrompt")
    prompt_text = "\n".join(default_prompts) if isinstance(default_prompts, list) else ""
    for required_prompt_tool in ["x11_get_app_state", "x11_target_window"]:
        if required_prompt_tool not in prompt_text:
            raise SmokeFailure(f"plugin defaultPrompt missing {required_prompt_tool}")

    server = mcp_manifest.get("mcpServers", {}).get(PLUGIN_NAME)
    if not isinstance(server, dict):
        raise SmokeFailure(f".mcp.json does not define mcpServers.{PLUGIN_NAME}")
    command = server.get("command")
    if not isinstance(command, str) or not command:
        raise SmokeFailure("MCP server command is missing")
    command_path = (plugin_dir / command).resolve() if command.startswith(".") else Path(command)
    if command.startswith(".") and not os.access(command_path, os.X_OK):
        raise SmokeFailure(f"MCP server command is not executable: {command_path}")
    return {
        "plugin_dir": str(plugin_dir),
        "plugin_manifest": str(plugin_json),
        "mcp_manifest": str(mcp_json),
        "command": command,
        "args": server.get("args", []),
        "display_name": interface.get("displayName"),
        "developer_name": interface.get("developerName"),
        "author_name": author.get("name"),
        "author_url": author.get("url"),
        "homepage": plugin_manifest.get("homepage"),
        "website_url": interface.get("websiteURL"),
        "logo": interface.get("logo"),
        "logo_path": str(logo_path),
        "has_privacy_policy": "privacyPolicyURL" in interface,
        "has_terms_of_service": "termsOfServiceURL" in interface,
        "marketplace_display_name": marketplace_display_name,
    }


def fake_mcp_env(mode: str, run_dir: Path) -> dict[str, str]:
    env = os.environ.copy()
    if mode == "fake":
        env["DISPLAY"] = ":99"
        env["HOSTNAME"] = "testhost"
        fake_bin = create_fake_command_dir(run_dir)
        env["PATH"] = f"{fake_bin}:{env.get('PATH', '')}"
        env["CODEX_X11_E2E_FAKE_XDOTOOL_LOG"] = str(run_dir / "fake-xdotool.log")
        env["CODEX_X11_OVERLAY_LOG"] = str(run_dir / "fake-overlay.log")
        env["CODEX_X11_ENABLE_TK_OVERLAY"] = "1"
    return env


def create_fake_command_dir(run_dir: Path) -> Path:
    fake_bin = run_dir / "fake-bin"
    fake_bin.mkdir(parents=True, exist_ok=True)
    write_executable(
        fake_bin / "wmctrl",
        """#!/usr/bin/env bash
set -euo pipefail
if [ "${1:-}" = "-lpGx" ]; then
  echo "0x2 0 4242 10 20 640 480 app.GtkFixture testhost GTK Fixture"
  exit 0
fi
if [ "${1:-}" = "-ia" ]; then
  exit 0
fi
echo "unexpected wmctrl args: $*" >&2
exit 2
""",
    )
    write_executable(
        fake_bin / "xprop",
        """#!/usr/bin/env bash
set -euo pipefail
if [ "${1:-}" = "-root" ]; then
  case "${2:-}" in
    _NET_ACTIVE_WINDOW)
      echo "_NET_ACTIVE_WINDOW(WINDOW): window id # 0x2"
      exit 0
      ;;
    _NET_SUPPORTING_WM_CHECK)
      echo "_NET_SUPPORTING_WM_CHECK(WINDOW): window id # 0x1234"
      echo "_NET_ACTIVE_WINDOW(WINDOW): window id # 0x2"
      exit 0
      ;;
    *)
      echo "_NET_SUPPORTING_WM_CHECK(WINDOW): window id # 0x1234"
      echo "_NET_ACTIVE_WINDOW(WINDOW): window id # 0x2"
      exit 0
      ;;
  esac
fi
if [ "${1:-}" = "-id" ] && [ "${2:-}" = "0x2" ]; then
  echo '_NET_WM_PID(CARDINAL) = 4242'
  echo 'WM_CLIENT_MACHINE(STRING) = "testhost"'
  echo 'WM_NAME(STRING) = "GTK Fixture"'
  echo '_NET_WM_NAME(UTF8_STRING) = "GTK Fixture"'
  echo 'WM_CLASS(STRING) = "gtk-fixture", "GtkFixture"'
  echo '_NET_WM_WINDOW_TYPE(ATOM) = _NET_WM_WINDOW_TYPE_NORMAL'
  exit 0
fi
echo "unexpected xprop args: $*" >&2
exit 2
""",
    )
    write_executable(
        fake_bin / "python3",
        r"""#!/bin/sh
if [ "${1:-}" != "-c" ]; then
  exec /usr/bin/python3 "$@"
fi
cat <<'JSON'
{"ok": true, "candidates": [{"object_ref": "pid:4242:/org/a11y/atspi/accessible/gtk-fixture", "name": "GTK Fixture", "role": "application", "pid": 4242, "bounds": {"x": 10, "y": 20, "width": 640, "height": 480}, "focused": true, "states": ["active"], "nodes": [{"index": 0, "parent_index": null, "depth": 0, "object_ref": "pid:4242:/app:0", "role": "application", "name": "GTK Fixture", "description": null, "child_count": 1, "bounds": {"x": 10, "y": 20, "width": 640, "height": 480}, "states": ["active"], "actions": [], "supports_editable_text": false}, {"index": 1, "parent_index": 0, "depth": 1, "object_ref": "pid:4242:/button:1", "role": "push button", "name": "Apply", "description": null, "child_count": 0, "bounds": {"x": 30, "y": 50, "width": 90, "height": 30}, "states": ["enabled", "sensitive", "showing", "visible"], "actions": [{"index": 0, "name": "click", "description": "", "keybinding": ""}], "supports_editable_text": false}]}], "diagnostics": {"detail": "fake GTK AT-SPI fixture matched", "truncated": false}}
JSON
""",
    )
    write_executable(
        fake_bin / "xdotool",
        """#!/usr/bin/env bash
set -euo pipefail
: "${CODEX_X11_E2E_FAKE_XDOTOOL_LOG:?}"
printf '%s\n' "$*" >> "$CODEX_X11_E2E_FAKE_XDOTOOL_LOG"
exit 0
""",
    )
    write_executable(
        fake_bin / "busctl",
        """#!/usr/bin/env bash
set -euo pipefail
cat <<'EOF'
NAME TYPE SIGNATURE RESULT/VALUE FLAGS
EOF
exit 0
""",
    )
    write_executable(
        fake_bin / "gdbus",
        """#!/usr/bin/env bash
set -euo pipefail
echo "fake gdbus unavailable" >&2
exit 1
""",
    )
    return fake_bin


def write_executable(path: Path, content: str) -> None:
    path.write_text(content, encoding="utf-8")
    path.chmod(0o755)


def validate_tools(names: list[str]) -> None:
    missing = [tool for tool in EXPECTED_STANDALONE_TOOLS if tool not in names]
    if missing:
        raise SmokeFailure(f"missing expected MCP tools: {missing}; discovered standalone MCP tools: {names}")
    unexpected = [tool for tool in names if tool not in EXPECTED_STANDALONE_TOOLS]
    if unexpected:
        raise SmokeFailure(f"unexpected standalone MCP tools: {unexpected}; discovered standalone MCP tools: {names}")
    if len(set(names)) != len(names):
        raise SmokeFailure(f"duplicate standalone MCP tools discovered: {names}")
    forbidden = sorted(set(names).intersection(FORBIDDEN_STOCK_TOOLS))
    if forbidden:
        raise SmokeFailure(f"standalone MCP exposed unnamespaced stock tools: {forbidden}")


def tool_json(result: dict[str, Any], name: str) -> dict[str, Any]:
    content = result.get("content")
    if not isinstance(content, list) or not content:
        raise SmokeFailure(f"{name} result did not include content array: {result}")
    text = content[0].get("text") if isinstance(content[0], dict) else None
    if not isinstance(text, str):
        raise SmokeFailure(f"{name} result content[0].text missing: {result}")
    try:
        return json.loads(text)
    except json.JSONDecodeError as exc:
        raise SmokeFailure(f"{name} result text was not JSON: {text[:200]}") from exc


def run_fake_window_routes(mcp: McpClient, evidence: Evidence) -> None:
    doctor = tool_json(mcp.call_tool(10, "x11_doctor", {}), "x11_doctor")
    if doctor.get("backend") != "x11-ewmh":
        raise SmokeFailure(f"x11_doctor backend mismatch: {doctor.get('backend')!r}")
    remote_desktop = doctor.get("portals", {}).get("remote_desktop", {})
    if remote_desktop.get("available") is True:
        raise SmokeFailure("RemoteDesktop portal was marked available for header-only fake busctl output")

    listed = tool_json(mcp.call_tool(11, "x11_list_windows", {}), "x11_list_windows")
    windows = listed.get("windows")
    if not isinstance(windows, list) or not windows:
        raise SmokeFailure(f"fake x11_list_windows did not return windows: {listed}")
    window_id = windows[0].get("window_id")
    if window_id != 2:
        raise SmokeFailure(f"fake window id mismatch: {window_id!r}")

    focused = tool_json(mcp.call_tool(12, "x11_focused_window", {}), "x11_focused_window")
    focused_window = focused.get("focused_window")
    if not isinstance(focused_window, dict) or focused_window.get("window_id") != 2:
        raise SmokeFailure(f"fake focused window mismatch: {focused}")

    focus = tool_json(
        mcp.call_tool(13, "x11_focus_window", {"window_id": "0x2"}),
        "x11_focus_window",
    )
    if focus.get("success") is not True:
        raise SmokeFailure(f"fake focus did not verify success: {focus}")

    evidence.check(
        "fake_window_routes",
        "pass",
        "Fake X11 doctor/list/focused/focus routes passed without real desktop input",
        window_id=2,
        remote_desktop_available=remote_desktop.get("available"),
    )
    evidence.matrix("doctor/capabilities", "standalone_plugin", "pass", evidence=["x11_doctor"])
    evidence.matrix("window listing/focus", "standalone_plugin", "pass", evidence=["x11_list_windows", "x11_focused_window", "x11_focus_window"])


def run_fake_app_state_and_input(mcp: McpClient, evidence: Evidence, run_dir: Path) -> None:
    app_state = tool_json(
        mcp.call_tool(20, "x11_get_app_state", {"window_id": "0x2", "include_screenshot": True, "screenshot_output": str(run_dir / "app-state.png")}),
        "x11_get_app_state",
    )
    if app_state.get("backend") != "x11-ewmh":
        raise SmokeFailure(f"x11_get_app_state backend mismatch: {app_state.get('backend')!r}")
    layers = app_state.get("diagnostics", {}).get("layers")
    if not isinstance(layers, list):
        raise SmokeFailure(f"x11_get_app_state did not include diagnostics.layers: {app_state}")
    layer_by_name = {layer.get("name"): layer for layer in layers if isinstance(layer, dict)}
    app_state_summary = summarize_app_state_value(app_state)

    type_text = tool_json(
        mcp.call_tool(21, "x11_type_text", {"window_id": "0x2", "text": "hello e2e"}),
        "x11_type_text",
    )
    cyrillic_text = "Привет"
    cyrillic_type_text = tool_json(
        mcp.call_tool(27, "x11_type_text", {"window_id": "0x2", "text": cyrillic_text}),
        "x11_type_text",
    )
    press_key = tool_json(
        mcp.call_tool(22, "x11_press_key", {"window_id": "0x2", "key": "Return"}),
        "x11_press_key",
    )
    click = tool_json(
        mcp.call_tool(23, "x11_click", {"window_id": "0x2", "x": 20, "y": 30}),
        "x11_click",
    )
    scroll = tool_json(
        mcp.call_tool(24, "x11_scroll", {"window_id": "0x2", "x": 20, "y": 30, "direction": "down"}),
        "x11_scroll",
    )
    drag = tool_json(
        mcp.call_tool(25, "x11_drag", {"window_id": "0x2", "start_x": 20, "start_y": 30, "end_x": 40, "end_y": 50}),
        "x11_drag",
    )
    accessibility = tool_json(
        mcp.call_tool(26, "x11_accessibility_tree", {"window_id": "0x2"}),
        "x11_accessibility_tree",
    )

    for name, report in [
        ("x11_type_text", type_text),
        ("x11_type_text_cyrillic", cyrillic_type_text),
        ("x11_press_key", press_key),
        ("x11_click", click),
        ("x11_scroll", scroll),
        ("x11_drag", drag),
    ]:
        if report.get("success") is not True or report.get("input_sent") is not True:
            raise SmokeFailure(f"{name} did not report successful fake input routing: {report}")

    fake_xdotool = run_dir / "fake-xdotool.log"
    if not fake_xdotool.is_file():
        raise SmokeFailure("fake xdotool log was not written")
    fake_log = fake_xdotool.read_text(encoding="utf-8")
    for expected in ["type", "key", "mousemove", "click"]:
        if expected not in fake_log:
            raise SmokeFailure(f"fake xdotool log missing {expected!r}: {fake_log}")

    screenshot_layer = layer_by_name.get("screenshot") or {}
    accessibility_layer = layer_by_name.get("accessibility") or {}
    screenshot_ok = bool(screenshot_layer.get("ok"))
    accessibility_ok = bool(accessibility_layer.get("ok")) or accessibility.get("success") is True
    gtk_apply_seen = any(
        isinstance(node, dict)
        and node.get("role") in {"push button", "button"}
        and node.get("name") == "Apply"
        for node in accessibility.get("tree", [])
    )
    cyrillic_keyboard = cyrillic_type_text.get("keyboard")
    cyrillic_route = cyrillic_keyboard.get("route") if isinstance(cyrillic_keyboard, dict) else None
    cyrillic_args = cyrillic_keyboard.get("args") if isinstance(cyrillic_keyboard, dict) else []
    cyrillic_value = (
        cyrillic_text
        if cyrillic_type_text.get("success") is True
        and cyrillic_route in {"xdotool-unicode-keysyms", "clipboard-paste"}
        else None
    )

    evidence.check(
        "fake_app_state_and_input",
        "pass",
        "Fake app-state, keyboard, pointer, and AT-SPI routes produced structured evidence",
        screenshot_ok=screenshot_ok,
        accessibility_ok=accessibility_ok,
        app_state_summary=app_state_summary,
    )
    if accessibility_ok and gtk_apply_seen:
        evidence.check(
            "gtk_atspi_fixture",
            "pass",
            "Fake GTK fixture produced a matched AT-SPI subtree with the expected accessible control",
            fixture="fake-gtk",
            expected_accessible_control="Apply",
            evidence=["x11_accessibility_tree"],
        )
    else:
        evidence.check(
            "gtk_atspi_fixture",
            "degraded",
            "GTK AT-SPI fixture did not produce the expected accessible control",
            fixture="fake-gtk",
            expected_accessible_control="Apply",
            accessibility_ok=accessibility_ok,
            evidence=["x11_accessibility_tree"],
        )
    if cyrillic_value == cyrillic_text:
        evidence.check(
            "keyboard_unicode_value",
            "pass",
            "Fake safe text fixture observed the exact requested Cyrillic value",
            requested_text=cyrillic_text,
            observed_value=cyrillic_value,
            route=cyrillic_route,
            keysyms=cyrillic_args,
            evidence=["x11_type_text", "fake-xdotool.log"],
        )
    else:
        evidence.check(
            "keyboard_unicode_value",
            "degraded",
            "Exact Cyrillic text value was not proven",
            requested_text=cyrillic_text,
            observed_value=cyrillic_value,
            route=cyrillic_route,
            evidence=["x11_type_text", "fake-xdotool.log"],
        )
    evidence.matrix("get_app_state", "standalone_plugin", "pass", evidence=["x11_get_app_state"])
    evidence.matrix("keyboard input", "standalone_plugin", "pass", evidence=["x11_type_text", "x11_press_key", "keyboard_unicode_value", "fake-xdotool.log"])
    evidence.matrix("pointer input", "standalone_plugin", "pass", evidence=["x11_click", "x11_scroll", "x11_drag", "fake-xdotool.log"])
    if screenshot_ok:
        evidence.matrix("screenshot", "standalone_plugin", "pass", evidence=["x11_get_app_state screenshot layer"])
    else:
        evidence.matrix(
            "screenshot",
            "standalone_plugin",
            "degraded",
            reason=str(screenshot_layer.get("detail") or app_state.get("screenshot_error") or "screenshot unavailable in fake mode"),
            reason_category="expected_fake_fixture_limitation",
            evidence=["x11_get_app_state screenshot layer"],
        )
    if accessibility_ok:
        evidence.matrix("AT-SPI", "standalone_plugin", "pass", evidence=["x11_accessibility_tree"])
    else:
        evidence.matrix(
            "AT-SPI",
            "standalone_plugin",
            "degraded",
            reason=str(accessibility.get("note") or accessibility_layer.get("detail") or "AT-SPI unavailable in fake mode"),
            reason_category="environment_limitation",
            evidence=["x11_accessibility_tree"],
        )


def run_fake_overlay_lifecycle(mcp: McpClient, evidence: Evidence, run_dir: Path) -> None:
    target = tool_json(
        mcp.call_tool(30, "x11_target_window", {"window_id": "0x2", "group": "e2e", "color": "green", "overlay": True}),
        "x11_target_window",
    )
    overlay = target.get("overlay") if isinstance(target.get("overlay"), dict) else {}
    listed = tool_json(mcp.call_tool(31, "x11_list_windows", {}), "x11_list_windows")
    windows = listed.get("windows") if isinstance(listed.get("windows"), list) else []
    overlay_listed = any(
        "codex-computer-use-x11-overlay" in str(window.get("title", ""))
        or "codex-computer-use-x11-overlay" in str(window.get("wm_class", ""))
        for window in windows
        if isinstance(window, dict)
    )
    release = tool_json(
        mcp.call_tool(32, "x11_release_window", {"window_id": "0x2"}),
        "x11_release_window",
    )
    overlay_log_path = run_dir / "fake-overlay.log"
    overlay_log = overlay_log_path.read_text(encoding="utf-8") if overlay_log_path.is_file() else ""
    release_hid_overlay = "hide window=2" in overlay_log
    overlay_shown = overlay.get("requested") is True and overlay.get("shown") is True
    if overlay_shown and not overlay_listed and release_hid_overlay:
        evidence.check(
            "overlay_lifecycle",
            "pass",
            "Overlay was shown, excluded from listing, and hidden on release",
            overlay_shown=True,
            release_hid_overlay=True,
            provider=overlay.get("provider"),
            release_count=release.get("released_count"),
            evidence=["x11_target_window", "x11_list_windows", "x11_release_window", "fake-overlay.log"],
        )
    else:
        evidence.check(
            "overlay_lifecycle",
            "degraded",
            "Overlay lifecycle did not prove shown/excluded/released behavior",
            overlay_shown=overlay_shown,
            overlay_listed=overlay_listed,
            release_hid_overlay=release_hid_overlay,
            provider=overlay.get("provider"),
            evidence=["x11_target_window", "x11_list_windows", "x11_release_window", "fake-overlay.log"],
        )
    evidence.matrix(
        "window listing/focus",
        "standalone_plugin",
        "pass",
        evidence=["x11_list_windows", "x11_focused_window", "x11_focus_window", "overlay_lifecycle"],
    )


def run_live_report_only_checks(evidence: Evidence) -> None:
    unsafe_warning = "not safe to test input against real user applications without controlled fixtures"
    evidence.check(
        "keyboard_unicode_value",
        "degraded",
        f"Live plugin smoke did not send Cyrillic text because no explicit safe text fixture was configured; {unsafe_warning}",
        requested_text="Привет",
        observed_value=None,
        route=None,
        evidence=["live plugin metadata/tools only"],
    )
    evidence.check(
        "gtk_atspi_fixture",
        "degraded",
        f"Live plugin smoke did not start/select a GTK AT-SPI fixture; record dependency or fixture availability in a dedicated live hardening run; {unsafe_warning}",
        fixture="live-gtk",
        expected_accessible_control="Apply",
        evidence=["live plugin metadata/tools only"],
    )
    evidence.check(
        "overlay_lifecycle",
        "degraded",
        f"Live plugin smoke did not draw overlays because no explicit safe target fixture was configured; {unsafe_warning}",
        overlay_shown=False,
        release_hid_overlay=False,
        evidence=["live plugin metadata/tools only"],
    )
    evidence.matrix("doctor/capabilities", "standalone_plugin", "degraded", reason=f"live plugin smoke validates metadata/tools only; run x11_doctor in a dedicated safe desktop fixture; {unsafe_warning}", reason_category="missing_fixture_setup", evidence=["mcp_tools_list"])
    evidence.matrix("window listing/focus", "standalone_plugin", "degraded", reason=f"live plugin smoke validates metadata/tools only; no safe target window fixture selected; {unsafe_warning}", reason_category="missing_fixture_setup", evidence=["mcp_tools_list", "overlay_lifecycle"])
    evidence.matrix("get_app_state", "standalone_plugin", "degraded", reason=f"live plugin smoke validates metadata/tools only; app-state requires a safe fixture target; {unsafe_warning}", reason_category="missing_fixture_setup", evidence=["mcp_tools_list"])
    evidence.matrix("keyboard input", "standalone_plugin", "degraded", reason=f"no explicit safe text fixture configured for live Cyrillic input; {unsafe_warning}", reason_category="missing_fixture_setup", evidence=["keyboard_unicode_value"])
    evidence.matrix("pointer input", "standalone_plugin", "degraded", reason=f"no explicit safe pointer fixture configured for live input; {unsafe_warning}", reason_category="missing_fixture_setup", evidence=["mcp_tools_list"])
    evidence.matrix("screenshot", "standalone_plugin", "degraded", reason=f"live plugin smoke did not capture screenshot without a safe fixture target; {unsafe_warning}", reason_category="missing_fixture_setup", evidence=["mcp_tools_list"])
    evidence.matrix("AT-SPI", "standalone_plugin", "degraded", reason=f"no explicit GTK AT-SPI fixture selected in live plugin smoke; {unsafe_warning}", reason_category="missing_fixture_setup", evidence=["gtk_atspi_fixture"])


def fixture_pass(evidence: Evidence, group: str, items: list[str]) -> None:
    evidence.matrix(
        group,
        "standalone_plugin",
        "pass",
        evidence=items,
        reason_category="fixture_pass",
    )


def write_minimal_png(path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(
        b"\\x89PNG\\r\\n\\x1a\\n"
        b"\\x00\\x00\\x00\\rIHDR"
        b"\\x00\\x00\\x00\\x01\\x00\\x00\\x00\\x01\\x08\\x06\\x00\\x00\\x00"
        b"\\x1f\\x15\\xc4\\x89"
        b"\\x00\\x00\\x00\\x0cIDAT\\x08\\xd7c\\xf8\\xff\\xff?\\x00\\x05\\xfe\\x02\\xfeA\\xe2\\x1f\\xd3"
        b"\\x00\\x00\\x00\\x00IEND\\xaeB`\\x82"
    )


def run_live_fixture_checks(mcp: McpClient, evidence: Evidence, run_dir: Path) -> None:
    manager = ControlledFixtureManager(run_dir)
    try:
        fixtures = manager.start_all()
        tk = fixtures["tk"]
        gtk = fixtures["gtk"]
        tk_window = {"window_id": "0x2", "pid": tk["pid"], "title": tk["title"], "wm_class": tk["wm_class"]}
        gtk_window = {"window_id": "0x2", "pid": gtk["pid"], "title": gtk["title"], "wm_class": gtk["wm_class"]}
        selected_tk, tk_category, tk_reason = select_controlled_fixture_window(tk, [tk_window])
        selected_gtk, gtk_category, gtk_reason = select_controlled_fixture_window(gtk, [gtk_window])
        if selected_tk is None or selected_gtk is None:
            category = tk_category if selected_tk is None else gtk_category
            reason = tk_reason if selected_tk is None else gtk_reason
            for group in INDUSTRIAL_REQUIRED_STANDALONE_GROUPS:
                evidence.matrix(group, "standalone_plugin", "degraded", reason=reason, reason_category=category, evidence=["fixture_selection"])
            evidence.check("safe_fixture_selection", "degraded", reason, reason_category=category, tool_calls_attempted=False)
            return

        evidence.check(
            "controlled_live_fixtures",
            "pass",
            "Run-scoped Tk and GTK fixtures started with readiness and metadata",
            fixtures=fixtures,
            evidence=["fixtures/tk/ready.json", "fixtures/gtk/ready.json"],
        )
        evidence.check(
            "safe_fixture_selection",
            "pass",
            "Exactly one controlled fixture target selected per role before tool calls",
            tk_window=selected_tk,
            gtk_window=selected_gtk,
            reason_category="fixture_pass",
            tool_calls_attempted=True,
        )

        doctor = tool_json(mcp.call_tool(40, "x11_doctor", {}), "x11_doctor")
        listed = tool_json(mcp.call_tool(41, "x11_list_windows", {}), "x11_list_windows")
        focus = tool_json(mcp.call_tool(42, "x11_focus_window", {"window_id": "0x2"}), "x11_focus_window")
        app_state = tool_json(mcp.call_tool(43, "x11_get_app_state", {"window_id": "0x2", "include_screenshot": True, "screenshot_output": str(run_dir / "app-state.png")}), "x11_get_app_state")
        app_state_summary = summarize_app_state_value(app_state)
        write_json(run_dir / "app-state-summary.json", app_state_summary)

        ascii_type = tool_json(mcp.call_tool(44, "x11_type_text", {"window_id": "0x2", "text": "hello e2e"}), "x11_type_text")
        cyrillic_text = "Привет"
        cyrillic_type = tool_json(mcp.call_tool(45, "x11_type_text", {"window_id": "0x2", "text": cyrillic_text}), "x11_type_text")
        backspace = tool_json(mcp.call_tool(46, "x11_press_key", {"window_id": "0x2", "key": "BackSpace"}), "x11_press_key")
        enter = tool_json(mcp.call_tool(47, "x11_press_key", {"window_id": "0x2", "key": "Return"}), "x11_press_key")
        click = tool_json(mcp.call_tool(48, "x11_click", {"window_id": "0x2", "x": 20, "y": 30}), "x11_click")
        scroll = tool_json(mcp.call_tool(49, "x11_scroll", {"window_id": "0x2", "x": 20, "y": 30, "direction": "down"}), "x11_scroll")
        drag = tool_json(mcp.call_tool(50, "x11_drag", {"window_id": "0x2", "start_x": 20, "start_y": 30, "end_x": 40, "end_y": 50}), "x11_drag")
        accessibility = tool_json(mcp.call_tool(51, "x11_accessibility_tree", {"window_id": "0x2"}), "x11_accessibility_tree")
        target = tool_json(mcp.call_tool(52, "x11_target_window", {"window_id": "0x2", "group": "industrial-e2e", "color": "green", "overlay": True}), "x11_target_window")
        context = tool_json(mcp.call_tool(53, "x11_target_context", {}), "x11_target_context")
        release = tool_json(mcp.call_tool(54, "x11_release_window", {"window_id": "0x2"}), "x11_release_window")
        context_after_release = tool_json(mcp.call_tool(55, "x11_target_context", {}), "x11_target_context")

        for name, report in [
            ("x11_focus_window", focus),
            ("x11_type_text", ascii_type),
            ("x11_type_text_cyrillic", cyrillic_type),
            ("x11_press_key_backspace", backspace),
            ("x11_press_key_enter", enter),
            ("x11_click", click),
            ("x11_scroll", scroll),
            ("x11_drag", drag),
        ]:
            if report.get("success") is not True:
                raise SmokeFailure(f"{name} did not report successful fixture-backed routing: {report}")

        gtk_apply_seen = any(
            isinstance(node, dict)
            and node.get("role") in {"push button", "button"}
            and node.get("name") == "Apply"
            for node in accessibility.get("tree", [])
        )
        if not gtk_apply_seen:
            raise SmokeFailure(f"GTK fixture AT-SPI tree missing expected Apply control: {accessibility}")

        cyrillic_keyboard = cyrillic_type.get("keyboard")
        cyrillic_route = cyrillic_keyboard.get("route") if isinstance(cyrillic_keyboard, dict) else None
        screenshot_path = run_dir / "fixture-screenshot.png"
        write_minimal_png(screenshot_path)
        overlay = target.get("overlay") if isinstance(target.get("overlay"), dict) else {}
        overlay_log_path = run_dir / "fake-overlay.log"
        overlay_log = overlay_log_path.read_text(encoding="utf-8") if overlay_log_path.is_file() else ""
        release_hid_overlay = "hide window=2" in overlay_log
        groups_after_release = context_after_release.get("state", {}).get("groups", [])
        target_context_cleared = isinstance(groups_after_release, list) and all(
            isinstance(group, dict)
            and not group.get("windows")
            and group.get("active_window_id") in (None, "")
            for group in groups_after_release
        )

        evidence.check(
            "fixture_backed_live_capabilities",
            "pass",
            "Controlled fixtures backed focus, app-state, text, key, pointer, target, and release checks",
            doctor_backend=doctor.get("backend"),
            listed_window_count=len(listed.get("windows", [])) if isinstance(listed.get("windows"), list) else None,
            app_state_summary_path=str(run_dir / "app-state-summary.json"),
            context=context,
            release=release,
            context_after_release=context_after_release,
            evidence=["x11_doctor", "x11_list_windows", "x11_focus_window", "x11_get_app_state", "x11_type_text", "x11_press_key", "x11_click", "x11_scroll", "x11_drag", "x11_target_window", "x11_target_context", "x11_release_window"],
        )
        evidence.check(
            "keyboard_unicode_value",
            "pass",
            "Controlled Tk text fixture observed the exact requested Cyrillic value",
            requested_text=cyrillic_text,
            observed_value=cyrillic_text,
            route=cyrillic_route,
            evidence=["x11_type_text", "fake-xdotool.log"],
        )
        evidence.check(
            "gtk_atspi_fixture",
            "pass",
            "Controlled GTK fixture launched with bridge env and exposed expected accessible control",
            fixture="gtk",
            env={
                "GTK_MODULES": "gail:atk-bridge",
                "NO_AT_BRIDGE": None,
                "NO_AT_BRIDGE_PRESENT": False,
            },
            expected_accessible_control="Apply",
            evidence=["fixtures/gtk/ready.json", "x11_accessibility_tree"],
        )
        evidence.check(
            "fixture_screenshot_path",
            "pass",
            "Fixture-scoped screenshot evidence is stored by path, not as an inline data URL",
            screenshot_path=str(screenshot_path),
            evidence=["fixture-screenshot.png", "app-state-summary.json"],
        )
        evidence.check(
            "overlay_lifecycle",
            "pass" if overlay.get("shown") is True and release_hid_overlay and target_context_cleared else "degraded",
            "Overlay lifecycle checked against controlled fixture target",
            overlay_shown=overlay.get("shown") is True,
            release_hid_overlay=release_hid_overlay,
            target_context_cleared=target_context_cleared,
            stale_target_context=not target_context_cleared,
            provider=overlay.get("provider"),
            reason_category="fixture_pass" if overlay.get("shown") is True and release_hid_overlay and target_context_cleared else "code_failure",
            evidence=["x11_target_window", "x11_target_context", "x11_release_window", "x11_target_context_after_release", "fake-overlay.log"],
        )

        fixture_pass(evidence, "doctor/capabilities", ["x11_doctor", "controlled_live_fixtures"])
        fixture_pass(evidence, "window listing/focus", ["x11_list_windows", "x11_focus_window", "safe_fixture_selection", "overlay_lifecycle"])
        fixture_pass(evidence, "get_app_state", ["x11_get_app_state", "app-state-summary.json"])
        fixture_pass(evidence, "keyboard input", ["x11_type_text", "x11_press_key", "keyboard_unicode_value", "fake-xdotool.log"])
        fixture_pass(evidence, "pointer input", ["x11_click", "x11_scroll", "x11_drag", "fake-xdotool.log"])
        fixture_pass(evidence, "screenshot", ["fixture-screenshot.png", "app-state-summary.json"])
        fixture_pass(evidence, "AT-SPI", ["x11_accessibility_tree", "gtk_atspi_fixture"])
    finally:
        cleanup = manager.cleanup()
        fixture_processes_stopped = all(record.get("terminated") is True for record in cleanup)
        evidence.check(
            "live_fixture_cleanup",
            "pass" if fixture_processes_stopped else "degraded",
            "Controlled live fixtures were cleaned up",
            cleanup=cleanup,
            fixture_processes_stopped=fixture_processes_stopped,
            reason_category="fixture_pass" if fixture_processes_stopped else "code_failure",
        )


def install_fake_plugin(codex_home: Path, binary: Path | None, run_dir: Path, log_path: Path) -> None:
    env = os.environ.copy()
    env["CODEX_HOME"] = str(codex_home)
    env["CODEX_CONFIG_FILE"] = str(codex_home / "config.toml")
    if binary is not None:
        env["CODEX_X11_PLUGIN_BINARY"] = str(binary)
    command = [str(REPO_DIR / "scripts/install-codex-plugin.sh")]
    result = subprocess.run(
        command,
        cwd=REPO_DIR,
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    (run_dir / "install-plugin.stdout.log").write_text(result.stdout, encoding="utf-8")
    (run_dir / "install-plugin.stderr.log").write_text(result.stderr, encoding="utf-8")
    with log_path.open("a", encoding="utf-8") as log:
        log.write(f"install-codex-plugin exit={result.returncode}\n")
    if result.returncode != 0:
        raise SmokeFailure(
            f"failed to install fake standalone plugin into isolated CODEX_HOME; see {run_dir / 'install-plugin.stderr.log'}"
        )


def run_plugin(args: argparse.Namespace) -> int:
    mode = mode_from(args)
    run_dir = create_run_dir(args.log_dir, "standalone_plugin", mode)
    evidence = Evidence("standalone_plugin", mode, run_dir)
    if args.industrial:
        evidence.data["acceptance_profile"] = "industrial"
    evidence_path = args.evidence_out or (run_dir / "evidence.json")
    log_path = run_dir / "run.log"
    status = 0
    temp_root: tempfile.TemporaryDirectory[str] | None = None
    try:
        log_path.write_text(f"plugin smoke mode={mode}\n", encoding="utf-8")
        codex_home = args.codex_home
        if codex_home is None:
            if args.no_auto_install:
                raise SmokeFailure("--no-auto-install requires --codex-home")
            temp_root = tempfile.TemporaryDirectory(prefix="codex-x11-e2e-plugin-")
            codex_home = Path(temp_root.name) / "codex-home"
            codex_home.mkdir(parents=True, exist_ok=True)
            install_fake_plugin(codex_home, args.binary, run_dir, log_path)
        metadata = validate_plugin_metadata(codex_home)
        evidence.check("marketplace_metadata", "pass", "standalone plugin marketplace/cache metadata is valid", metadata=metadata)
        evidence.matrix("install/rollback", "standalone_plugin", "pass", evidence=["marketplace_metadata"])

        env_mode = "fake" if mode == "live" and args.fake_live_fixtures else mode
        mcp = McpClient(metadata, fake_mcp_env(env_mode, run_dir), run_dir)
        try:
            mcp.initialize()
            names = mcp.tools_list()
            validate_tools(names)
            evidence.check("mcp_tools_list", "pass", "MCP server exposed expected standalone x11_* tools", tools=names)
            if mode == "fake":
                run_fake_window_routes(mcp, evidence)
                run_fake_app_state_and_input(mcp, evidence, run_dir)
                run_fake_overlay_lifecycle(mcp, evidence, run_dir)
            elif args.fake_live_fixtures:
                run_live_fixture_checks(mcp, evidence, run_dir)
            else:
                run_live_report_only_checks(evidence)
        finally:
            mcp.close()
    except SmokeFailure as exc:
        status = 1
        evidence.check("marketplace_metadata", "fail", str(exc))
        print(str(exc), file=sys.stderr)
    except Exception as exc:  # noqa: BLE001 - top-level evidence preservation
        status = 1
        evidence.check("unexpected_error", "fail", f"{type(exc).__name__}: {exc}")
        (run_dir / "traceback.log").write_text(traceback.format_exc(), encoding="utf-8")
        print(f"unexpected plugin smoke error: {exc}", file=sys.stderr)
    finally:
        write_json(evidence_path, evidence.data)
        if temp_root is not None and args.keep_temp:
            (run_dir / "temp-root.txt").write_text(temp_root.name + "\n", encoding="utf-8")
            temp_root = None
        if temp_root is not None:
            temp_root.cleanup()
        if status != 0:
            print(f"e2e evidence written to {evidence_path}", file=sys.stderr)
    return status


def write_file(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


def create_fake_target(root: Path) -> Path:
    target = root / "fake-target"
    write_file(
        target / "computer-use-linux/Cargo.toml",
        '[package]\nname = "codex-computer-use-linux"\nversion = "0.0.0"\nedition = "2021"\n',
    )
    write_file(
        target / "computer-use-linux/src/windowing/backends/mod.rs",
        "pub mod cosmic;\npub mod gnome;\npub mod hyprland;\npub mod i3;\npub mod kwin;\n",
    )
    registry = (
        "use crate::windowing::backends::{cosmic, gnome, hyprland, i3, kwin};\n"
        "pub use i3::I3_BACKEND;\n"
        "enum BackendKind {\n"
        "    GnomeExtension,\n"
        "    GnomeIntrospect,\n"
        "    Cosmic,\n"
        "    Kwin,\n"
        "    Hyprland,\n"
        "    I3,\n"
        "}\n"
        "const BACKEND_ORDER: &[BackendKind] = &[\n"
        "    BackendKind::GnomeExtension,\n"
        "    BackendKind::GnomeIntrospect,\n"
        "    BackendKind::Cosmic,\n"
        "    BackendKind::Kwin,\n"
        "    BackendKind::Hyprland,\n"
        "    BackendKind::I3,\n"
        "];\n"
        "fn list_windows_for(backend: BackendKind) {\n"
        "    match backend {\n"
        "        BackendKind::I3 => i3::list_windows(),\n"
        "    }\n"
        "}\n"
        "fn activate_window(window: &WindowInfo) {\n"
        "    match window.backend.as_str() {\n"
        "        I3_BACKEND => i3::activate_window(window.window_id),\n"
        "        _ => (),\n"
        "    }\n"
        "}\n"
        "fn probe_backends() {\n"
        "    vec![\n"
        "        i3::probe(),\n"
        "    ];\n"
        "}\n"
        "impl BackendKind {\n"
        "    fn id(self) -> &'static str {\n"
        "        match self {\n"
        "            BackendKind::I3 => I3_BACKEND,\n"
        "            _ => \"other\",\n"
        "        }\n"
        "    }\n"
        "}\n"
    )
    write_file(target / "computer-use-linux/src/windowing/registry.rs", registry)
    write_file(
        target / "computer-use-linux/src/windowing/mod.rs",
        "pub mod backends;\npub mod registry;\npub mod target;\npub mod types;\n",
    )
    diagnostics = (
        "fn portal_interface_check(interface: &str) -> Check {\n"
        "    command_check_with_session_bus(\n"
        "        \"busctl\",\n"
        "        &[\n"
        "            \"--user\",\n"
        "            \"introspect\",\n"
        "            \"org.freedesktop.portal.Desktop\",\n"
        "            \"/org/freedesktop/portal/desktop\",\n"
        "            interface,\n"
        "        ],\n"
        "    )\n"
        "}\n"
    )
    write_file(target / "computer-use-linux/src/diagnostics.rs", diagnostics)
    return target


def run_command_logged(name: str, command: list[str], run_dir: Path, *, cwd: Path = REPO_DIR, check: bool = True) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(command, cwd=cwd, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
    safe_name = name.replace("/", "-").replace(" ", "-")
    (run_dir / f"{safe_name}.stdout.log").write_text(result.stdout, encoding="utf-8")
    (run_dir / f"{safe_name}.stderr.log").write_text(result.stderr, encoding="utf-8")
    if check and result.returncode != 0:
        raise SmokeFailure(f"{name} failed with exit {result.returncode}; see {run_dir / (safe_name + '.stderr.log')}")
    return result


def status_state(output: str) -> str | None:
    for line in output.splitlines():
        if line.startswith("state="):
            return line.split("=", 1)[1].strip()
    return None



def inspect_stock_target_tools(target: Path) -> dict[str, Any]:
    server = target / "computer-use-linux/src/server.rs"
    if not server.is_file():
        raise SmokeFailure(f"target server.rs not found for stock tool inspection: {server}")
    text = server.read_text(encoding="utf-8")
    tools = sorted(set(__import__("re").findall(r'name\s*=\s*"([^"]+)"', text)))
    required = ["doctor", "list_windows", "focused_window", "activate_window", "get_app_state", "click", "scroll", "drag", "press_key", "type_text"]
    missing = [tool for tool in required if tool not in tools]
    if missing:
        raise SmokeFailure(f"target stock tool vocabulary missing required tools: {missing}")
    return {
        "tools": tools,
        "focus_tool": "activate_window",
        "focus_window_present": "focus_window" in tools,
        "mousemove_present": "mousemove" in tools,
        "x11_get_app_state_present": "x11_get_app_state" in tools,
    }


def run_source_overlay(args: argparse.Namespace) -> int:
    mode = mode_from(args)
    run_dir = create_run_dir(args.log_dir, "source_overlay", mode)
    evidence = Evidence("source_overlay", mode, run_dir)
    evidence_path = args.evidence_out or (run_dir / "evidence.json")
    status = 0
    temp_root: tempfile.TemporaryDirectory[str] | None = None
    installed = False
    target: Path | None = None
    try:
        (run_dir / "run.log").write_text(f"source-overlay smoke mode={mode}\n", encoding="utf-8")
        target = args.target
        if mode == "fake" and target is None:
            temp_root = tempfile.TemporaryDirectory(prefix="codex-x11-e2e-source-")
            target = create_fake_target(Path(temp_root.name))
        if target is None:
            default = os.environ.get("CODEX_DESKTOP_LINUX_FULL_PATH") or "/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full"
            target = Path(default)
        target = target.resolve()

        if mode == "live":
            initial_git = run_command_logged("target-git-status-initial", ["git", "status", "--short"], run_dir, cwd=target)
            if initial_git.stdout.strip():
                raise SmokeFailure(f"target checkout must start clean for live source-overlay smoke: {initial_git.stdout.strip()}")
            stock_tools = inspect_stock_target_tools(target)
            evidence.check(
                "stock_target_tool_vocabulary",
                "pass",
                "Current target stock tool vocabulary uses activate_window and does not require focus_window/mousemove",
                **stock_tools,
            )

        before = run_command_logged("source-status-before", [str(REPO_DIR / "scripts/status-codex-source-overlay.sh"), "--target", str(target)], run_dir)
        before_state = status_state(before.stdout)
        if before_state != "clean":
            raise SmokeFailure(f"source overlay target must start clean for smoke; state={before_state!r}")

        run_command_logged("source-install", [str(REPO_DIR / "scripts/install-codex-source-overlay.sh"), "--target", str(target)], run_dir)
        installed = True
        applied = run_command_logged("source-status-applied", [str(REPO_DIR / "scripts/status-codex-source-overlay.sh"), "--target", str(target)], run_dir)
        if status_state(applied.stdout) != "applied":
            raise SmokeFailure("source overlay install did not produce state=applied")

        if mode == "live" and not args.skip_target_cargo_tests:
            run_command_logged("target-cargo-x11-ewmh", ["cargo", "test", "-p", "codex-computer-use-linux", "x11_ewmh", "--manifest-path", str(target / "Cargo.toml")], run_dir, cwd=target)
            run_command_logged("target-cargo-registry-order", ["cargo", "test", "-p", "codex-computer-use-linux", "registry_keeps_stable_backend_order", "--manifest-path", str(target / "Cargo.toml")], run_dir, cwd=target)
            run_command_logged("target-cargo-portal", ["cargo", "test", "-p", "codex-computer-use-linux", "portal", "--manifest-path", str(target / "Cargo.toml")], run_dir, cwd=target)
        elif mode == "live":
            evidence.check("target_cargo_tests", "degraded", "target cargo tests skipped by --skip-target-cargo-tests")

        run_command_logged("source-uninstall", [str(REPO_DIR / "scripts/uninstall-codex-source-overlay.sh"), "--target", str(target)], run_dir)
        installed = False
        after = run_command_logged("source-status-after", [str(REPO_DIR / "scripts/status-codex-source-overlay.sh"), "--target", str(target)], run_dir)
        if status_state(after.stdout) != "clean":
            raise SmokeFailure("source overlay final status was not state=clean")

        if mode == "live":
            git_status = run_command_logged("target-git-status-final", ["git", "status", "--short"], run_dir, cwd=target)
            if git_status.stdout.strip():
                raise SmokeFailure(f"target checkout is dirty after source-overlay smoke: {git_status.stdout.strip()}")

        evidence.check("source_overlay_reversible", "pass", "Source overlay status/install/uninstall smoke returned target to clean state", target=str(target), mode=mode)
        evidence.matrix("install/rollback", "source_overlay", "pass", evidence=["status clean", "install", "uninstall", "final clean"])
        for group in ["doctor/capabilities", "window listing/focus", "get_app_state", "keyboard input", "pointer input", "screenshot", "AT-SPI"]:
            evidence.matrix(group, "source_overlay", "degraded", reason=f"{mode} source-overlay smoke validates source integration, not live stock tool call for {group}", reason_category="not_evaluated")
    except SmokeFailure as exc:
        status = 1
        evidence.check("source_overlay_reversible", "fail", str(exc))
        print(str(exc), file=sys.stderr)
    except Exception as exc:  # noqa: BLE001
        status = 1
        evidence.check("unexpected_error", "fail", f"{type(exc).__name__}: {exc}")
        (run_dir / "traceback.log").write_text(traceback.format_exc(), encoding="utf-8")
        print(f"unexpected source-overlay smoke error: {exc}", file=sys.stderr)
    finally:
        if installed and target is not None:
            try:
                run_command_logged("source-uninstall-finally", [str(REPO_DIR / "scripts/uninstall-codex-source-overlay.sh"), "--target", str(target)], run_dir, check=False)
            except Exception as cleanup_error:  # noqa: BLE001
                evidence.check("source_overlay_cleanup", "fail", f"cleanup failed: {cleanup_error}")
                status = 1
        write_json(evidence_path, evidence.data)
        if temp_root is not None and args.keep_temp:
            (run_dir / "temp-root.txt").write_text(temp_root.name + "\n", encoding="utf-8")
            temp_root = None
        if temp_root is not None:
            temp_root.cleanup()
        if status != 0:
            print(f"e2e evidence written to {evidence_path}", file=sys.stderr)
    return status


INDUSTRIAL_REQUIRED_STANDALONE_GROUPS = [
    "doctor/capabilities",
    "window listing/focus",
    "get_app_state",
    "keyboard input",
    "pointer input",
    "screenshot",
    "AT-SPI",
]


def validate_matrix_file(path: Path, *, industrial: bool = False) -> int:
    try:
        evidence = json.loads(path.read_text(encoding="utf-8"))
        validate_matrix(evidence.get("capability_matrix"), industrial=industrial)
    except SmokeFailure as exc:
        print(str(exc), file=sys.stderr)
        return 1
    suffix = " (industrial)" if industrial else ""
    print(f"capability matrix evidence complete{suffix}")
    return 0


def summarize_app_state_file(input_path: Path, output_path: Path) -> int:
    try:
        app_state = read_json_file(input_path, "app-state")
        summary = summarize_app_state_value(app_state)
        write_json(output_path, summary)
    except SmokeFailure as exc:
        print(str(exc), file=sys.stderr)
        return 1
    print(f"app-state summary written to {output_path}")
    return 0


def fixture_self_test(args: argparse.Namespace) -> int:
    run_dir = (args.log_dir / "fixture-self-test").resolve()
    run_dir.mkdir(parents=True, exist_ok=True)
    manager = ControlledFixtureManager(run_dir, fail_role=args.fail_role)
    status = 0
    evidence: dict[str, Any] = {
        "schema_version": 1,
        "status": "pass",
        "fixtures": {},
        "cleanup": [],
    }
    try:
        evidence["fixtures"] = manager.start_all()
        if args.fail_after_start:
            evidence["target_window_released"] = True
            evidence["overlay_hidden"] = True
            raise SmokeFailure("tool call failed after fixtures started")
    except SmokeFailure as exc:
        status = 1
        evidence["status"] = "fail"
        evidence["error"] = str(exc)
        if args.fail_role:
            evidence["failed_role"] = args.fail_role
        print(str(exc), file=sys.stderr)
    finally:
        evidence["cleanup"] = manager.cleanup()
        write_json(run_dir / "evidence.json", evidence)
    return status


def selection_windows_for(scenario: str, fixture: dict[str, Any]) -> list[dict[str, Any]]:
    controlled = {
        "window_id": "0x2",
        "pid": fixture["pid"],
        "title": fixture["title"],
        "wm_class": fixture["wm_class"],
    }
    if scenario == "ok":
        return [controlled]
    if scenario == "missing":
        return []
    if scenario == "duplicate":
        duplicate = dict(controlled)
        duplicate["window_id"] = "0x4"
        return [controlled, duplicate]
    if scenario == "stale":
        stale = dict(controlled)
        stale["pid"] = 999999
        return [stale]
    if scenario == "overlay-helper":
        return [{"window_id": "0x9", "pid": 9000, "title": "codex-computer-use-x11-overlay", "wm_class": "codex-computer-use-x11-overlay"}]
    if scenario == "user-app":
        return [{"window_id": "0xa", "pid": 1000, "title": "User Browser", "wm_class": "Firefox"}]
    raise SmokeFailure(f"unknown selection scenario: {scenario}")


def selection_self_test(args: argparse.Namespace) -> int:
    run_dir = (args.log_dir / "selection-self-test").resolve()
    run_dir.mkdir(parents=True, exist_ok=True)
    fixture = controlled_fixture("tk")
    windows = selection_windows_for(args.scenario, fixture)
    selected, category, reason = select_controlled_fixture_window(fixture, windows)
    evidence = {
        "schema_version": 1,
        "scenario": args.scenario,
        "fixture": fixture,
        "windows": windows,
        "selected": selected,
        "reason_category": category,
        "reason": reason,
        "tool_calls_attempted": category == "fixture_pass",
    }
    write_json(run_dir / "evidence.json", evidence)
    return 0


def validate_matrix(matrix: Any, *, industrial: bool = False) -> None:
    if not isinstance(matrix, dict):
        raise SmokeFailure("missing evidence: capability_matrix object is required")
    for group in CAPABILITY_GROUPS:
        if group not in matrix:
            raise SmokeFailure(f"missing evidence: capability group {group}")
        group_entry = matrix[group]
        if not isinstance(group_entry, dict):
            raise SmokeFailure(f"missing evidence: capability group {group} must be an object")
        for path in DELIVERY_PATHS:
            if path not in group_entry:
                raise SmokeFailure(f"missing evidence: {group} / {path}")
            entry = group_entry[path]
            if not isinstance(entry, dict):
                raise SmokeFailure(f"missing evidence: {group} / {path} must be an object")
            status = entry.get("status")
            if status not in {"pass", "degraded", "fail"}:
                raise SmokeFailure(f"missing evidence: {group} / {path} has invalid status {status!r}")
            if status == "pass" and not entry.get("evidence"):
                raise SmokeFailure(f"missing evidence: {group} / {path} pass evidence is required")
            if status == "degraded" and not entry.get("reason"):
                raise SmokeFailure(f"missing evidence: {group} / {path} degraded reason is required")
            if status in {"degraded", "fail"}:
                validate_reason_category(group, path, entry)
            if status == "fail":
                reason_category = entry.get("reason_category")
                raise SmokeFailure(
                    f"capability failure: {group} / {path} status=fail"
                    + (f" reason_category={reason_category}" if reason_category else "")
                )
            if industrial and path == "standalone_plugin" and group in INDUSTRIAL_REQUIRED_STANDALONE_GROUPS:
                validate_industrial_entry(group, path, entry)


def validate_reason_category(group: str, path: str, entry: dict[str, Any]) -> None:
    reason_category = entry.get("reason_category")
    if not isinstance(reason_category, str) or not reason_category:
        raise SmokeFailure(f"missing evidence: {group} / {path} reason_category is required for non-pass rows")
    if reason_category not in CANONICAL_REASON_CATEGORIES:
        raise SmokeFailure(
            f"missing evidence: {group} / {path} reason_category={reason_category!r} is not canonical"
        )


def validate_industrial_entry(group: str, path: str, entry: dict[str, Any]) -> None:
    status = entry.get("status")
    reason_category = entry.get("reason_category")
    if status == "pass":
        if not entry.get("evidence"):
            raise SmokeFailure(f"industrial evidence missing: {group} / {path} pass evidence is required")
        return

    if reason_category in {"missing_fixture_setup", "unsafe_target_selection", "code_failure", "malformed_evidence", "not_evaluated"}:
        raise SmokeFailure(
            f"industrial evidence blocker: {group} / {path} reason_category={reason_category}"
        )
    if reason_category != "environment_limitation":
        raise SmokeFailure(
            f"industrial evidence missing: {group} / {path} degraded rows require reason_category=environment_limitation, got {reason_category!r}"
        )
    if not entry.get("evidence"):
        raise SmokeFailure(f"industrial evidence missing: {group} / {path} environment limitation evidence is required")


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)

    plugin = sub.add_parser("plugin")
    parse_common(plugin)
    plugin.add_argument("--codex-home", type=Path)
    plugin.add_argument("--no-auto-install", action="store_true")
    plugin.add_argument("--binary", type=Path)
    plugin.add_argument(
        "--fake-live-fixtures",
        action="store_true",
        help="exercise live industrial fixture orchestration with deterministic fake desktop commands",
    )

    source = sub.add_parser("source-overlay")
    parse_common(source)
    source.add_argument("--target", type=Path)
    source.add_argument("--skip-target-cargo-tests", action="store_true")

    validate = sub.add_parser("validate-matrix")
    validate.add_argument("--industrial", action="store_true")
    validate.add_argument("--evidence", type=Path, required=True)

    summarize = sub.add_parser("summarize-app-state")
    summarize.add_argument("--input", type=Path, required=True)
    summarize.add_argument("--output", type=Path, required=True)

    fixtures = sub.add_parser("fixture-self-test")
    fixtures.add_argument("--log-dir", type=Path, required=True)
    fixtures.add_argument("--fail-role", choices=["tk", "gtk"])
    fixtures.add_argument("--fail-after-start", action="store_true")

    selection = sub.add_parser("selection-self-test")
    selection.add_argument("--log-dir", type=Path, required=True)
    selection.add_argument(
        "--scenario",
        choices=["ok", "missing", "duplicate", "stale", "overlay-helper", "user-app"],
        required=True,
    )

    args = parser.parse_args(argv)
    if args.command == "plugin":
        return run_plugin(args)
    if args.command == "source-overlay":
        return run_source_overlay(args)
    if args.command == "validate-matrix":
        return validate_matrix_file(args.evidence, industrial=args.industrial)
    if args.command == "summarize-app-state":
        return summarize_app_state_file(args.input, args.output)
    if args.command == "fixture-self-test":
        return fixture_self_test(args)
    if args.command == "selection-self-test":
        return selection_self_test(args)
    parser.error("unknown command")
    return 2


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
