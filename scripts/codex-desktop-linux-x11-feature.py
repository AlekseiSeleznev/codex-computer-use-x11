#!/usr/bin/env python3
"""Local Codex Desktop Linux x11-ewmh-computer-use feature installer.

This script intentionally records only non-secret filesystem state. It is a
local verification helper for the optional disabled-by-default Linux Feature
adapter, not an upstream default installer.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import stat
import subprocess
import sys
import time
from pathlib import Path
from typing import Any, Dict, List, Optional

FEATURE_ID = "x11-ewmh-computer-use"
PLUGIN_NAME = "codex-computer-use-x11"
INSTALL_OP = "install-codex-desktop-linux-x11-feature"
UNINSTALL_OP = "uninstall-codex-desktop-linux-x11-feature"
DEFAULT_TARGET = Path("/home/as/Документы/AI_PROJECTS/codex-desktop-linux")


def repo_root() -> Path:
    return Path(__file__).resolve().parents[1]


def now_id() -> str:
    return time.strftime("%Y%m%dT%H%M%SZ", time.gmtime())


def sha_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def mode_of(path: Path) -> Optional[str]:
    try:
        return oct(stat.S_IMODE(path.lstat().st_mode))
    except FileNotFoundError:
        return None


def snapshot(path: Path) -> Dict[str, Any]:
    if not path.exists() and not path.is_symlink():
        return {"exists": False}
    if path.is_file() or path.is_symlink():
        return {
            "exists": True,
            "kind": "file",
            "sha256": sha_file(path),
            "mode": mode_of(path),
            "size": path.stat().st_size,
        }
    if path.is_dir():
        h = hashlib.sha256()
        count = 0
        for item in sorted(path.rglob("*")):
            rel = item.relative_to(path).as_posix()
            if item.is_dir():
                h.update(f"dir\0{rel}\0{mode_of(item)}\n".encode())
            elif item.is_file() or item.is_symlink():
                h.update(f"file\0{rel}\0{mode_of(item)}\0".encode())
                if item.is_symlink():
                    h.update(os.readlink(item).encode())
                else:
                    h.update(item.read_bytes())
                h.update(b"\n")
                count += 1
        return {
            "exists": True,
            "kind": "dir",
            "sha256": h.hexdigest(),
            "mode": mode_of(path),
            "file_count": count,
        }
    return {"exists": True, "kind": "other", "mode": mode_of(path)}


def same_snapshot(a: Dict[str, Any], b: Dict[str, Any]) -> bool:
    keys = ["exists", "kind", "sha256", "mode", "size", "file_count"]
    return all(a.get(k) == b.get(k) for k in keys)


def copy_any(src: Path, dest: Path) -> None:
    if src.is_dir():
        dest.parent.mkdir(parents=True, exist_ok=True)
        if dest.exists():
            shutil.rmtree(dest)
        shutil.copytree(src, dest, symlinks=True)
    else:
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dest)


def remove_any(path: Path) -> None:
    if path.is_dir() and not path.is_symlink():
        shutil.rmtree(path)
    elif path.exists() or path.is_symlink():
        path.unlink()


def read_json(path: Path, default: Any) -> Any:
    try:
        return json.loads(path.read_text())
    except FileNotFoundError:
        return default
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Invalid JSON in {path}: {exc}")


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


def enable_feature_config(path: Path) -> None:
    data = read_json(path, {"enabled": []})
    if not isinstance(data, dict):
        data = {"enabled": []}
    enabled = data.get("enabled")
    if not isinstance(enabled, list):
        enabled = []
    if FEATURE_ID not in enabled:
        enabled.append(FEATURE_ID)
    data["enabled"] = enabled
    write_json(path, data)


def resolve_target(value: Optional[str]) -> Path:
    if value:
        return Path(value).expanduser().resolve()
    env = os.environ.get("CODEX_DESKTOP_LINUX_FULL_PATH", "").strip()
    if env:
        return Path(env).expanduser().resolve()
    return DEFAULT_TARGET


def resolve_install_dir(value: Optional[str], target: Path) -> Path:
    if value:
        return Path(value).expanduser().resolve()
    opt = Path("/opt/codex-desktop")
    if opt.exists():
        return opt
    return (target / "codex-app").resolve()


def resolve_manifest(value: Optional[str], install_dir: Path) -> Path:
    if value:
        return Path(value).expanduser().resolve()
    return install_dir / ".codex-x11-feature" / "install-manifest.json"


def adapter_source(source_root: Path) -> Path:
    p = source_root / "adapters/codex-desktop-linux/linux-features" / FEATURE_ID
    if not p.joinpath("feature.json").exists():
        raise SystemExit(f"Adapter scaffold not found: {p}")
    return p


class ManifestBuilder:
    def __init__(self, manifest_path: Path, dry_run: bool):
        self.manifest_path = manifest_path
        self.dry_run = dry_run
        self.backup_root = manifest_path.parent / "backups" / now_id()
        self.manifest: Dict[str, Any] = {
            "schema_version": 1,
            "operation": INSTALL_OP,
            "feature_id": FEATURE_ID,
            "plugin_name": PLUGIN_NAME,
            "created_at": now_id(),
            "dry_run": dry_run,
            "entries": [],
        }

    def add_entry(self, entry_id: str, path: Path, surface: str) -> Dict[str, Any]:
        before = snapshot(path)
        backup_path = None
        if before.get("exists") and not self.dry_run:
            backup_path = self.backup_root / entry_id
            copy_any(path, backup_path)
        entry = {
            "id": entry_id,
            "surface": surface,
            "path": str(path),
            "before": before,
            "backup_path": str(backup_path) if backup_path else None,
            "after": None,
            "completed": False,
            "installer_changed": False,
        }
        self.manifest["entries"].append(entry)
        self.write()
        return entry

    def complete(self, entry: Dict[str, Any]) -> None:
        after = snapshot(Path(entry["path"]))
        entry["after"] = after
        entry["completed"] = True
        entry["installer_changed"] = not same_snapshot(entry["before"], after)
        self.write()

    def write(self) -> None:
        if self.dry_run:
            return
        write_json(self.manifest_path, self.manifest)


def parse_common(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--target")
    parser.add_argument("--install-dir")
    parser.add_argument("--source", default=str(repo_root()))
    parser.add_argument("--binary")
    parser.add_argument("--manifest")
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--report-json", nargs="?", const="-", default=None)


def write_report(report: Dict[str, Any], report_json: Optional[str]) -> None:
    if report_json is None:
        if not report.get("success", True):
            print(json.dumps(report, indent=2, ensure_ascii=False), file=sys.stderr)
        return
    text = json.dumps(report, indent=2, ensure_ascii=False) + "\n"
    if report_json == "-":
        sys.stdout.write(text)
    else:
        Path(report_json).write_text(text)


def stage_plugin(target_feature_dir: Path, install_dir: Path, work_dir: Path, source: Path, binary: Optional[str]) -> None:
    env = os.environ.copy()
    env.update({
        "INSTALL_DIR": str(install_dir),
        "WORK_DIR": str(work_dir),
        "ARCH": "x86_64",
        "CODEX_UPSTREAM_APP_DIR": str(install_dir),
    })
    if binary:
        env["CODEX_X11_COMPUTER_USE_BINARY"] = str(Path(binary).expanduser().resolve())
        env.pop("CODEX_X11_COMPUTER_USE_SOURCE", None)
    else:
        env["CODEX_X11_COMPUTER_USE_SOURCE"] = str(source)
        env.pop("CODEX_X11_COMPUTER_USE_BINARY", None)
    subprocess.run(["bash", str(target_feature_dir / "stage.sh")], check=True, cwd=str(target_feature_dir), env=env)


def fake_patch_app(install_dir: Path) -> None:
    app = install_dir / "resources/app.asar"
    if app.exists():
        original = app.read_text(errors="replace")
        app.write_text(original + "\n# codex-computer-use-x11 fake patch\n")
    webview = install_dir / "content/webview/index.html"
    if webview.exists():
        original = webview.read_text(errors="replace")
        webview.write_text(original + "\n<!-- codex-computer-use-x11 fake patch -->\n")


def real_patch_app(target: Path, install_dir: Path, work_dir: Path) -> Dict[str, Any]:
    app = install_dir / "resources/app.asar"
    if not app.exists():
        return {"mode": "auto", "status": "skipped", "reason": "resources/app.asar missing"}
    patcher = target / "scripts/patch-linux-window-ui.js"
    if not patcher.exists():
        return {"mode": "auto", "status": "blocked", "reason": f"patcher missing: {patcher}"}
    if shutil.which("npx") is None or shutil.which("node") is None:
        return {"mode": "auto", "status": "blocked", "reason": "node/npx unavailable"}
    extracted = work_dir / "app-asar"
    report = work_dir / "patch-report.json"
    if extracted.exists():
        shutil.rmtree(extracted)
    subprocess.run(["npx", "--yes", "asar", "extract", str(app), str(extracted)], check=True)
    env = os.environ.copy()
    env["CODEX_LINUX_FEATURES_ROOT"] = str(target / "linux-features")
    env["CODEX_LINUX_FEATURES_CONFIG"] = str(target / "linux-features/features.json")
    subprocess.run(["node", str(patcher), "--report-json", str(report), str(extracted)], check=True, env=env)
    packed = work_dir / "app.asar"
    if packed.exists():
        packed.unlink()
    subprocess.run(["npx", "--yes", "asar", "pack", str(extracted), str(packed)], check=True)
    shutil.copy2(packed, app)
    webview_src = extracted / "content/webview"
    webview_dst = install_dir / "content/webview"
    if webview_src.exists():
        if webview_dst.exists():
            shutil.rmtree(webview_dst)
        shutil.copytree(webview_src, webview_dst)
    return {"mode": "auto", "status": "applied", "report": str(report)}


def install(args: argparse.Namespace) -> int:
    target = resolve_target(args.target)
    install_dir = resolve_install_dir(args.install_dir, target)
    source = Path(args.source).expanduser().resolve()
    manifest_path = resolve_manifest(args.manifest, install_dir)
    feature_src = adapter_source(source)
    target_feature_dir = target / "linux-features/local" / FEATURE_ID
    target_features_json = target / "linux-features/features.json"
    plugin_dir = install_dir / "resources/plugins/openai-bundled/plugins" / PLUGIN_NAME
    marketplace = install_dir / "resources/plugins/openai-bundled/.agents/plugins/marketplace.json"
    update_feature_dir = install_dir / "update-builder/linux-features/local" / FEATURE_ID
    update_features_json = install_dir / "update-builder/linux-features/features.json"
    app_asar = install_dir / "resources/app.asar"
    webview = install_dir / "content/webview"

    planned = [
        "target_feature_dir", "target_features_json", "plugin_dir", "marketplace"
    ]
    if (install_dir / "update-builder").exists():
        planned.extend(["update_builder_feature_dir", "update_builder_features_json"])
    if args.patch_mode != "skip" and app_asar.exists():
        planned.append("app_asar")
    if args.patch_mode != "skip" and webview.exists():
        planned.append("webview_dir")

    report: Dict[str, Any] = {
        "operation": INSTALL_OP,
        "success": True,
        "dry_run": args.dry_run,
        "feature_id": FEATURE_ID,
        "plugin_name": PLUGIN_NAME,
        "target": str(target),
        "install_dir": str(install_dir),
        "manifest": str(manifest_path),
        "patch_mode": args.patch_mode,
        "planned_surfaces": planned,
        "entries": [],
    }
    if args.dry_run:
        write_report(report, args.report_json)
        return 0

    mb = ManifestBuilder(manifest_path, False)
    mb.manifest.update({
        "target": str(target),
        "install_dir": str(install_dir),
        "source": str(source),
        "patch_mode": args.patch_mode,
    })

    work_dir = manifest_path.parent / "work" / now_id()
    work_dir.mkdir(parents=True, exist_ok=True)
    try:
        entries: Dict[str, Dict[str, Any]] = {}
        for entry_id, path, surface in [
            ("target_feature_dir", target_feature_dir, "target"),
            ("target_features_json", target_features_json, "target"),
            ("plugin_dir", plugin_dir, "install"),
            ("marketplace", marketplace, "install"),
        ]:
            entries[entry_id] = mb.add_entry(entry_id, path, surface)
        if (install_dir / "update-builder").exists():
            entries["update_builder_feature_dir"] = mb.add_entry("update_builder_feature_dir", update_feature_dir, "update-builder")
            entries["update_builder_features_json"] = mb.add_entry("update_builder_features_json", update_features_json, "update-builder")
        if args.patch_mode != "skip" and app_asar.exists():
            entries["app_asar"] = mb.add_entry("app_asar", app_asar, "app")
        if args.patch_mode != "skip" and webview.exists():
            entries["webview_dir"] = mb.add_entry("webview_dir", webview, "app")

        copy_any(feature_src, target_feature_dir)
        mb.complete(entries["target_feature_dir"])
        enable_feature_config(target_features_json)
        mb.complete(entries["target_features_json"])

        if "update_builder_feature_dir" in entries:
            copy_any(feature_src, update_feature_dir)
            mb.complete(entries["update_builder_feature_dir"])
            enable_feature_config(update_features_json)
            mb.complete(entries["update_builder_features_json"])

        stage_plugin(target_feature_dir, install_dir, work_dir, source, args.binary)
        mb.complete(entries["plugin_dir"])
        mb.complete(entries["marketplace"])

        if args.patch_mode == "fake":
            fake_patch_app(install_dir)
            report["app_patch"] = {"mode": "fake", "status": "applied", "test_only": True}
        elif args.patch_mode == "auto":
            result = real_patch_app(target, install_dir, work_dir)
            report["app_patch"] = result
            if result.get("status") == "blocked":
                raise RuntimeError(result.get("reason", "app patch blocked"))
        elif args.patch_mode == "skip":
            report["app_patch"] = {"mode": "skip", "status": "skipped"}
        if "app_asar" in entries:
            mb.complete(entries["app_asar"])
        if "webview_dir" in entries:
            mb.complete(entries["webview_dir"])
        report["entries"] = mb.manifest["entries"]
        write_report(report, args.report_json)
        return 0
    except Exception as exc:  # noqa: BLE001 - command-line structured failure
        report["success"] = False
        report["error"] = str(exc)
        report["entries"] = mb.manifest.get("entries", [])
        write_report(report, args.report_json)
        return 1


def uninstall(args: argparse.Namespace) -> int:
    target = resolve_target(args.target)
    install_dir = resolve_install_dir(args.install_dir, target)
    manifest_path = resolve_manifest(args.manifest, install_dir)
    if not manifest_path.exists():
        report = {
            "operation": UNINSTALL_OP,
            "success": True,
            "dry_run": args.dry_run,
            "manifest": str(manifest_path),
            "restored": [],
            "skipped": ["manifest_missing"],
            "blockers": [],
        }
        write_report(report, args.report_json)
        return 0

    manifest = read_json(manifest_path, {})
    blockers: List[Dict[str, Any]] = []
    restored: List[Dict[str, Any]] = []
    skipped: List[Any] = []
    entries = list(reversed(manifest.get("entries", [])))

    for entry in entries:
        if not entry.get("completed"):
            skipped.append({"id": entry.get("id"), "reason": "not_completed"})
            continue
        if not entry.get("installer_changed"):
            skipped.append({"id": entry.get("id"), "reason": "already_acceptable"})
            continue
        path = Path(entry["path"])
        current = snapshot(path)
        after = entry.get("after") or {"exists": False}
        if not same_snapshot(current, after):
            blockers.append({
                "id": entry.get("id"),
                "path": str(path),
                "reason": "drift",
                "expected_after": after,
                "current": current,
            })

    report: Dict[str, Any] = {
        "operation": UNINSTALL_OP,
        "success": not blockers,
        "dry_run": args.dry_run,
        "manifest": str(manifest_path),
        "restored": restored,
        "skipped": skipped,
        "blockers": blockers,
    }
    if blockers or args.dry_run:
        write_report(report, args.report_json)
        return 1 if blockers else 0

    for entry in entries:
        if not entry.get("completed") or not entry.get("installer_changed"):
            continue
        path = Path(entry["path"])
        before = entry.get("before") or {"exists": False}
        backup = Path(entry["backup_path"]) if entry.get("backup_path") else None
        if before.get("exists"):
            if backup is None or not backup.exists():
                blockers.append({"id": entry.get("id"), "path": str(path), "reason": "missing_backup"})
                continue
            remove_any(path)
            copy_any(backup, path)
            restored.append({"id": entry.get("id"), "path": str(path), "action": "restored_backup"})
        else:
            remove_any(path)
            restored.append({"id": entry.get("id"), "path": str(path), "action": "removed"})

    if blockers:
        report["success"] = False
        report["blockers"] = blockers
        write_report(report, args.report_json)
        return 1
    # Keep backups for audit but remove active manifest after successful uninstall.
    try:
        manifest_path.unlink()
    except FileNotFoundError:
        pass
    report["restored"] = restored
    report["success"] = True
    write_report(report, args.report_json)
    return 0


def main(argv: Optional[List[str]] = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    install_p = sub.add_parser("install")
    parse_common(install_p)
    install_p.add_argument("--patch-mode", choices=["auto", "skip", "fake"], default="auto")
    uninstall_p = sub.add_parser("uninstall")
    parse_common(uninstall_p)
    ns = parser.parse_args(argv)
    if ns.command == "install":
        return install(ns)
    if ns.command == "uninstall":
        return uninstall(ns)
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
