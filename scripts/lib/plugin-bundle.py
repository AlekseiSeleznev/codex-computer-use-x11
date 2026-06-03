#!/usr/bin/env python3
"""Write the codex-computer-use-x11 Codex plugin bundle layout."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import pathlib
import shutil
import stat
from typing import Any

PLUGIN_NAME = "codex-computer-use-x11"
DISPLAY_NAME = "X11 Computer Use"
SHORT_DESCRIPTION = "Standalone x11_* tools for Linux X11/EWMH"
SOURCE_REPO_URL = "https://github.com/AlekseiSeleznev/codex-computer-use-x11"
AUTHOR = {"name": "AlekseiSeleznev", "url": "https://github.com/AlekseiSeleznev"}
MCP_MANIFEST = {
    "mcpServers": {
        PLUGIN_NAME: {
            "command": "./bin/codex-computer-use-x11",
            "args": ["mcp"],
            "cwd": ".",
        }
    }
}


def plugin_manifest(version: str) -> dict[str, Any]:
    return {
        "name": PLUGIN_NAME,
        "version": version,
        "description": "Standalone X11/EWMH Computer Use MCP tools for Codex.",
        "author": AUTHOR,
        "homepage": SOURCE_REPO_URL,
        "license": "MIT",
        "keywords": ["computer-use", "linux", "x11", "ewmh", "mcp"],
        "mcpServers": "./.mcp.json",
        "interface": {
            "displayName": DISPLAY_NAME,
            "shortDescription": SHORT_DESCRIPTION,
            "longDescription": "Provides standalone x11_* readiness diagnostics, window listing/focus, keyboard input, pointer actions, accessibility tree, app state, and target-window context tools for validating the codex-computer-use-x11 backend without replacing the bundled Computer Use plugin.",
            "developerName": "AlekseiSeleznev",
            "category": "Productivity",
            "websiteURL": SOURCE_REPO_URL,
            "logo": "./assets/app-icon.png",
            "defaultPrompt": [
                "Check whether standalone X11 Computer Use is ready with x11_doctor",
                "List X11 windows with x11_list_windows and inspect app state with x11_get_app_state",
                "Save a target context with x11_target_window before using verified x11_type_text, x11_click, x11_scroll, or x11_drag",
            ],
            "brandColor": "#1E293B",
            "screenshots": [],
        },
    }


def sha256_file(path: pathlib.Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def write_json(path: pathlib.Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")


def copy_executable(source: pathlib.Path, dest: pathlib.Path) -> None:
    if not source.is_file():
        raise SystemExit(f"binary source does not exist: {source}")
    if not os.access(source, os.X_OK):
        raise SystemExit(f"binary source is not executable: {source}")
    dest.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source, dest)
    mode = dest.stat().st_mode
    dest.chmod(mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)


def release_metadata(version: str, artifact: str | None, binary_sha256: str) -> dict[str, Any]:
    return {
        "plugin_name": PLUGIN_NAME,
        "version": version,
        "command": "./bin/codex-computer-use-x11",
        "args": ["mcp"],
        "cwd": ".",
        "display_name": DISPLAY_NAME,
        "short_description": SHORT_DESCRIPTION,
        "baseline": "x11-ewmh / Cinnamon X11",
        "source_repo_url": SOURCE_REPO_URL,
        "release_url_pattern": f"{SOURCE_REPO_URL}/releases/download/v{{version}}/{{artifact}}",
        "artifact": artifact,
        "sha256": binary_sha256,
        "sha256_scope": "bin/codex-computer-use-x11",
        "artifact_sha256_sidecar": f"{artifact}.sha256" if artifact else None,
    }


def write_bundle(args: argparse.Namespace) -> None:
    repo_dir = pathlib.Path(args.repo_dir).resolve()
    dest = pathlib.Path(args.dest).resolve()
    binary = pathlib.Path(args.binary).resolve()
    if dest.exists():
        shutil.rmtree(dest)
    (dest / ".codex-plugin").mkdir(parents=True)
    (dest / "bin").mkdir(parents=True)
    (dest / "assets").mkdir(parents=True)

    target_binary = dest / "bin" / "codex-computer-use-x11"
    copy_executable(binary, target_binary)

    icon = repo_dir / "assets" / "app-icon.png"
    if not icon.is_file():
        raise SystemExit(f"missing icon asset: {icon}")
    shutil.copy2(icon, dest / "assets" / "app-icon.png")

    write_json(dest / ".codex-plugin" / "plugin.json", plugin_manifest(args.version))
    write_json(dest / ".mcp.json", MCP_MANIFEST)
    if args.release_metadata:
        write_json(
            dest / "RELEASE-METADATA.json",
            release_metadata(args.version, args.artifact, sha256_file(target_binary)),
        )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-dir", required=True)
    parser.add_argument("--dest", required=True)
    parser.add_argument("--binary", required=True)
    parser.add_argument("--version", required=True)
    parser.add_argument("--release-metadata", action="store_true")
    parser.add_argument("--artifact")
    args = parser.parse_args()
    write_bundle(args)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
