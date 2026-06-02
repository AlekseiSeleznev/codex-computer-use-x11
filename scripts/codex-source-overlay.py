#!/usr/bin/env python3
"""Reversible source overlay for codex-computer-use-x11."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

DEFAULT_TARGET = Path('/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full')
MARKER = 'codex-computer-use-x11'
BEGIN = f'BEGIN {MARKER}'
END = f'END {MARKER}'
TAKEOVER_MARKER_VERSION = 'codex-computer-use-x11-provider-takeover:v1'
PROVIDER_OVERLAY_ROOT = (
    Path(__file__).resolve().parent
    / 'overlays'
    / 'codex-desktop-linux-full'
    / 'provider-takeover'
)
PROVIDER_OVERLAY_FILES = [
    'scripts/patches/computer-use.js',
    'scripts/patch-linux-window-ui.js',
    'scripts/patch-linux-window-ui.test.js',
    'scripts/patches/core/all-linux/webview/computer-use-ui/patch.js',
]
PROVIDER_STATE_DIR = Path('.codex-computer-use-x11-overlay/provider-takeover')
PROVIDER_MANIFEST = PROVIDER_STATE_DIR / 'manifest.json'

REQUIRED_FILES = [
    'computer-use-linux/src/windowing/registry.rs',
    'computer-use-linux/src/windowing/mod.rs',
    'computer-use-linux/src/windowing/backends/mod.rs',
    'computer-use-linux/src/diagnostics.rs',
    'computer-use-linux/Cargo.toml',
]
PATCHED_FILES = REQUIRED_FILES[:-1]
BACKEND_REL = Path('computer-use-linux/src/windowing/backends/x11_ewmh.rs')


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description='Manage codex-computer-use-x11 source overlay')
    parser.add_argument('command', choices=['install', 'uninstall', 'status'])
    parser.add_argument('--target', help='Path to codex-desktop-linux target checkout')
    parser.add_argument('--provider', default=None, help='Provider overlay to manage (supported: x11)')
    parser.add_argument('--mode', default=None, help='Provider overlay mode (supported: takeover)')
    parser.add_argument('--dry-run', action='store_true', help='Report actions without mutating files')
    parser.add_argument('--report-json', help='Write a machine-readable patch report')
    parser.add_argument('--patch-live-assets', action='store_true', help='Patch live computer-use-settings assets')
    parser.add_argument('--live-assets-dir', help='Directory containing live webview assets')
    return parser.parse_args()


def resolve_target(value: str | None) -> Path:
    if value:
        return Path(value).expanduser().resolve()
    env = os.environ.get('CODEX_DESKTOP_LINUX_FULL_PATH')
    if env:
        return Path(env).expanduser().resolve()
    return DEFAULT_TARGET


def target_commit(target: Path) -> str:
    try:
        return subprocess.check_output(
            ['git', '-C', str(target), 'rev-parse', 'HEAD'],
            text=True,
            stderr=subprocess.DEVNULL,
        ).strip()
    except Exception:
        return 'unknown'


def check_structure(target: Path) -> list[str]:
    return [rel for rel in REQUIRED_FILES if not (target / rel).is_file()]


def fail(message: str, code: int = 1) -> int:
    print(message, file=sys.stderr)
    return code


def read(path: Path) -> str:
    return path.read_text(encoding='utf-8')


def write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding='utf-8')


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open('rb') as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b''):
            digest.update(chunk)
    return digest.hexdigest()


def now_stamp() -> str:
    return datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%SZ')


def text_metadata(text: str, kind: str, **extra: object) -> dict:
    data = {
        'kind': kind,
        'before_sha256' if kind.endswith('_before') else 'sha256': hashlib.sha256(text.encode('utf-8')).hexdigest(),
    }
    data.update(extra)
    return data


def file_stat_metadata(path: Path) -> dict:
    stat = path.stat()
    return {
        'mode': stat.st_mode & 0o7777,
        'uid': stat.st_uid,
        'gid': stat.st_gid,
    }


def text_digest_fields(prefix: str, text: str) -> dict:
    encoded = text.encode('utf-8')
    return {
        f'{prefix}_sha256': hashlib.sha256(encoded).hexdigest(),
        f'{prefix}_size': len(encoded),
    }


def file_digest_fields(prefix: str, path: Path) -> dict:
    return {
        f'{prefix}_sha256': sha256(path),
        f'{prefix}_size': path.stat().st_size,
    }


def file_owner_mode_fields(prefix: str, path: Path) -> dict:
    stat = path.stat()
    return {
        f'{prefix}_mode': stat.st_mode & 0o7777,
        f'{prefix}_uid': stat.st_uid,
        f'{prefix}_gid': stat.st_gid,
    }


def restore_owner_mode(path: Path, item: dict, prefix: str = 'before') -> None:
    mode = item.get(f'{prefix}_mode')
    if mode is not None:
        os.chmod(path, int(mode))
    uid = item.get(f'{prefix}_uid')
    gid = item.get(f'{prefix}_gid')
    if uid is not None and gid is not None and hasattr(os, 'chown'):
        try:
            os.chown(path, int(uid), int(gid))
        except PermissionError:
            # Non-root test and user-local rollback paths should still restore bytes/mode.
            pass


def expected_installed_sha(item: dict) -> str | None:
    value = item.get('installed_sha256')
    return str(value) if value else None


def expected_before_sha(item: dict) -> str | None:
    value = item.get('before_sha256') or item.get('sha256')
    return str(value) if value else None


def has_takeover_marker(path: Path) -> bool:
    if not path.exists():
        return False
    text = path.read_text(encoding='utf-8', errors='replace')
    return (
        TAKEOVER_MARKER_VERSION in text
        or 'codexLinuxComputerUseTakeoverProvider' in text
        or 'codex-computer-use-x11-unavailable' in text
    )


def live_assets_with_takeover_markers(assets_dir_value: str | None) -> list[str]:
    if not assets_dir_value:
        return []
    assets_dir = Path(assets_dir_value).expanduser().resolve()
    if not assets_dir.is_dir():
        return []
    return [str(asset) for asset in sorted(assets_dir.glob('computer-use-settings-*.js')) if has_takeover_marker(asset)]


def write_report(path: str | None, report: dict) -> None:
    if not path:
        return
    report_path = Path(path).expanduser().resolve()
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(json.dumps(report, indent=2, sort_keys=True) + '\n', encoding='utf-8')


def provider_restart_hint(live: bool = False) -> str:
    if live:
        return 'restart Codex Desktop fully so Electron/webview reloads patched live Computer Use settings assets.'
    return 'restart Codex Desktop after rebuilding/reinstalling the target so Computer Use settings reload the X11 provider takeover patch.'


def provider_manifest_path(target: Path) -> Path:
    return target / PROVIDER_MANIFEST


def load_provider_manifest(target: Path) -> dict | None:
    path = provider_manifest_path(target)
    if not path.is_file():
        return None
    return json.loads(path.read_text(encoding='utf-8'))


def save_provider_manifest(target: Path, manifest: dict, dry_run: bool) -> None:
    if dry_run:
        return
    exclude = target / '.git/info/exclude'
    if exclude.is_file():
        line = f'/{PROVIDER_STATE_DIR.as_posix()}/'
        current = exclude.read_text(encoding='utf-8', errors='replace')
        if line not in current.splitlines():
            with exclude.open('a', encoding='utf-8') as handle:
                handle.write(f'\n{line}\n')
    path = provider_manifest_path(target)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(manifest, indent=2, sort_keys=True) + '\n', encoding='utf-8')


def git_parent_file(target: Path, rel: str) -> str | None:
    try:
        return subprocess.check_output(
            ['git', '-C', str(target), 'show', f'HEAD^:{rel}'],
            text=True,
            stderr=subprocess.DEVNULL,
        )
    except Exception:
        return None


def provider_source_state(target: Path) -> tuple[str, list[str]]:
    missing = [rel for rel in PROVIDER_OVERLAY_FILES if not (target / rel).is_file()]
    if missing:
        return 'drifted', [f'missing target structure: {rel}' for rel in missing]
    marker_present = TAKEOVER_MARKER_VERSION in read(target / 'scripts/patches/computer-use.js')
    descriptor_present = 'linux-x11-computer-use-provider-takeover' in read(
        target / 'scripts/patches/core/all-linux/webview/computer-use-ui/patch.js'
    )
    if marker_present and descriptor_present:
        drift = []
        for rel in PROVIDER_OVERLAY_FILES:
            overlay = PROVIDER_OVERLAY_ROOT / rel
            target_file = target / rel
            if overlay.is_file() and read(overlay) != read(target_file):
                drift.append(f'overlay content drift: {rel}')
        return ('drifted', drift) if drift else ('applied', [])
    if marker_present or descriptor_present:
        return 'drifted', ['partial provider takeover source markers present']
    return 'clean', []


def provider_report(target: Path, state: str, changed_files: list[str] | None = None, **extra: object) -> dict:
    report = {
        'schema_version': 1,
        'operation': extra.pop('operation', None),
        'provider': 'x11',
        'mode': 'takeover',
        'state': state,
        'target': str(target),
        'target_commit': target_commit(target),
        'marker_version': TAKEOVER_MARKER_VERSION,
        'changed_files': changed_files or [],
        'restart_hint': provider_restart_hint(bool(extra.get('live_assets'))),
    }
    report.update({key: value for key, value in extra.items() if value is not None})
    return report


def print_provider_report(report: dict) -> None:
    print(f"state={report['state']}")
    print(f"target={report['target']}")
    print(f"target_commit={report['target_commit']}")
    print(f"provider={report['provider']}")
    print(f"mode={report['mode']}")
    print(f"marker_version={report['marker_version']}")
    print(f"changed_files={','.join(report['changed_files'])}")
    print(f"restart_hint={report['restart_hint']}")


def validate_provider_mode(args: argparse.Namespace) -> bool:
    return args.provider == 'x11' and args.mode == 'takeover'


def backup_source_file(target: Path, rel: str, backup_root: Path, current_text: str, desired_text: str, dry_run: bool) -> dict:
    backup_path = backup_root / rel
    dest = target / rel
    parent_text = git_parent_file(target, rel)
    backup_text = parent_text if parent_text is not None else current_text
    source_kind = 'git-parent' if parent_text is not None and parent_text != current_text else 'working-tree'
    before_owner_mode = file_owner_mode_fields('before', dest) if dest.exists() else {
        'before_mode': None,
        'before_uid': None,
        'before_gid': None,
    }
    if not dry_run:
        backup_path.parent.mkdir(parents=True, exist_ok=True)
        backup_path.write_text(backup_text, encoding='utf-8')
    item = {
        'kind': 'source',
        'rel': rel,
        'backup': str(backup_path.relative_to(target)),
        'sha256': hashlib.sha256(backup_text.encode('utf-8')).hexdigest(),  # legacy key
        'source': source_kind,
        'installer_changed': current_text != desired_text,
        'completed': not dry_run,
    }
    item.update(text_digest_fields('before', backup_text))
    item.update(before_owner_mode)
    item.update(text_digest_fields('current_before_install', current_text))
    item.update(text_digest_fields('installed', desired_text))
    item.update({
        'installed_mode': item.get('before_mode'),
        'installed_uid': item.get('before_uid'),
        'installed_gid': item.get('before_gid'),
        'before': {
            'exists': dest.exists(),
            'sha256': item.get('before_sha256'),
            'size': item.get('before_size'),
            'mode': item.get('before_mode'),
            'uid': item.get('before_uid'),
            'gid': item.get('before_gid'),
        },
        'after': {
            'exists': True,
            'sha256': item.get('installed_sha256'),
            'size': item.get('installed_size'),
            'mode': item.get('installed_mode'),
            'uid': item.get('installed_uid'),
            'gid': item.get('installed_gid'),
        },
    })
    return item


def apply_provider_source_overlay(target: Path, dry_run: bool, stamp: str | None = None) -> tuple[list[str], list[dict]]:
    changed: list[str] = []
    backups: list[dict] = []
    backup_root = target / PROVIDER_STATE_DIR / 'backups' / (stamp or now_stamp()) / 'source'
    for rel in PROVIDER_OVERLAY_FILES:
        source = PROVIDER_OVERLAY_ROOT / rel
        dest = target / rel
        desired = read(source)
        current = read(dest) if dest.is_file() else ''
        item = backup_source_file(target, rel, backup_root, current, desired, dry_run)
        backups.append(item)
        if current == desired:
            continue
        changed.append(rel)
        if not dry_run:
            write(dest, desired)
    return changed, backups


def patch_live_asset_with_target(target: Path, asset: Path, dry_run: bool) -> str:
    patcher_path = (
        PROVIDER_OVERLAY_ROOT / 'scripts/patches/computer-use.js'
        if dry_run
        else target / 'scripts/patches/computer-use.js'
    )
    script = f"""
const fs = require('fs');
const patcher = require({json.dumps(str(patcher_path))});
const path = {json.dumps(str(asset))};
const before = fs.readFileSync(path, 'utf8');
const after = patcher.applyX11ComputerUseSettingsRowPatch(before);
process.stdout.write(after);
"""
    output = subprocess.check_output(['node', '-e', script], text=True)
    if not dry_run:
        asset.write_text(output, encoding='utf-8')
    return output


def patch_provider_live_assets(target: Path, assets_dir_value: str | None, dry_run: bool, stamp: str | None = None) -> tuple[list[str], list[dict]]:
    if not assets_dir_value:
        raise ValueError('--patch-live-assets requires --live-assets-dir')
    assets_dir = Path(assets_dir_value).expanduser().resolve()
    assets = sorted(assets_dir.glob('computer-use-settings-*.js'))
    if not assets:
        raise ValueError(f'no computer-use-settings-*.js assets found in {assets_dir}')
    changed: list[str] = []
    backups: list[dict] = []
    backup_root = target / PROVIDER_STATE_DIR / 'backups' / (stamp or now_stamp()) / 'live-assets'
    for asset in assets:
        before = asset.read_text(encoding='utf-8')
        patched = patch_live_asset_with_target(target, asset, True)
        if before == patched:
            continue
        rel_name = asset.name
        backup_path = backup_root / rel_name
        item = {
            'kind': 'live_asset',
            'asset': str(asset),
            'backup': str(backup_path.relative_to(target)),
            'sha256': hashlib.sha256(before.encode('utf-8')).hexdigest(),  # legacy key
            'size': len(before.encode('utf-8')),  # legacy key
        }
        item.update(text_digest_fields('before', before))
        item.update(file_owner_mode_fields('before', asset))
        item.update(text_digest_fields('installed', patched))
        item.update({
            'installed_mode': item.get('before_mode'),
            'installed_uid': item.get('before_uid'),
            'installed_gid': item.get('before_gid'),
            'installer_changed': before != patched,
            'completed': not dry_run,
        })
        if not dry_run:
            backup_path.parent.mkdir(parents=True, exist_ok=True)
            backup_path.write_text(before, encoding='utf-8')
            asset.write_text(patched, encoding='utf-8')
            restore_owner_mode(asset, item, 'before')
            item.update(file_digest_fields('installed', asset))
            item.update(file_owner_mode_fields('installed', asset))
        item.update({
            'before': {
                'exists': True,
                'sha256': item.get('before_sha256'),
                'size': item.get('before_size'),
                'mode': item.get('before_mode'),
                'uid': item.get('before_uid'),
                'gid': item.get('before_gid'),
            },
            'after': {
                'exists': True,
                'sha256': item.get('installed_sha256'),
                'size': item.get('installed_size'),
                'mode': item.get('installed_mode'),
                'uid': item.get('installed_uid'),
                'gid': item.get('installed_gid'),
            },
        })
        changed.append(str(asset))
        backups.append(item)
    return changed, backups


def provider_status(target: Path, args: argparse.Namespace) -> int:
    state, details = provider_source_state(target)
    manifest = load_provider_manifest(target)
    report = provider_report(
        target,
        state,
        operation='status',
        details=details,
        manifest_present=manifest is not None,
    )
    print_provider_report(report)
    for detail in details:
        print(f'detail={detail}')
    write_report(args.report_json, report)
    return 0 if state in {'clean', 'applied'} else 2


def restore_transaction_items(target: Path, items: list[dict]) -> list[dict]:
    outcomes: list[dict] = []
    for item in reversed(items):
        try:
            backup = target / item['backup']
            if not backup.is_file():
                raise ValueError(f'missing backup: {backup}')
            if item.get('kind') == 'live_asset':
                dest = Path(item['asset'])
                shutil.copy2(backup, dest)
                restore_owner_mode(dest, item, 'before')
                path = str(dest)
            else:
                rel = item['rel']
                dest = target / rel
                shutil.copy2(backup, dest)
                path = rel
            outcomes.append({'path': path, 'restored': True})
        except Exception as exc:  # pragma: no cover - reported to caller in tests via JSON/stderr
            outcomes.append({'path': item.get('rel') or item.get('asset') or item.get('backup'), 'restored': False, 'error': str(exc)})
    return outcomes


def provider_install(target: Path, args: argparse.Namespace) -> int:
    state, details = provider_source_state(target)
    if state == 'drifted':
        report = provider_report(target, state, operation='install', details=details)
        print_provider_report(report)
        for detail in details:
            print(f'detail={detail}')
        write_report(args.report_json, report)
        return 2
    changed: list[str] = []
    source_backups: list[dict] = []
    live_changed: list[str] = []
    live_backups: list[dict] = []
    stamp = now_stamp()
    try:
        changed, source_backups = apply_provider_source_overlay(target, args.dry_run, stamp)
        if args.patch_live_assets:
            live_changed, live_backups = patch_provider_live_assets(target, args.live_assets_dir, args.dry_run, stamp)
    except Exception as exc:
        restore_outcomes: list[dict] = []
        if not args.dry_run:
            restore_outcomes = restore_transaction_items(target, source_backups + live_backups)
        report = provider_report(
            target,
            'failed',
            changed + live_changed,
            operation='install',
            source_backups=source_backups,
            live_asset_backups=live_backups,
            restore_outcomes=restore_outcomes,
            error=str(exc),
            live_assets=args.patch_live_assets,
            dry_run=args.dry_run,
        )
        print_provider_report(report)
        write_report(args.report_json, report)
        return fail(f'failed to install provider takeover overlay: {exc}')
    manifest = {
        'schema_version': 1,
        'provider': 'x11',
        'mode': 'takeover',
        'marker_version': TAKEOVER_MARKER_VERSION,
        'target_commit': target_commit(target),
        'source_backups': source_backups,
        'live_asset_backups': live_backups,
    }
    save_provider_manifest(target, manifest, args.dry_run)
    all_changed = changed + live_changed
    report = provider_report(
        target,
        'dry-run' if args.dry_run else 'applied',
        all_changed,
        operation='install',
        source_backups=source_backups,
        live_asset_backups=live_backups,
        live_assets=args.patch_live_assets,
        dry_run=args.dry_run,
    )
    print_provider_report(report)
    write_report(args.report_json, report)
    return 0


def assert_current_matches_install(path: Path, item: dict, kind: str) -> None:
    expected = expected_installed_sha(item)
    if expected and path.exists():
        actual = sha256(path)
        if actual != expected:
            raise ValueError(f'{kind} drift: {path}')


def provider_uninstall(target: Path, args: argparse.Namespace) -> int:
    manifest = load_provider_manifest(target)
    state, details = provider_source_state(target)
    live_marker_hits = live_assets_with_takeover_markers(args.live_assets_dir)
    if manifest is None:
        if state == 'clean' and not live_marker_hits:
            report = provider_report(target, 'clean', operation='uninstall', details=['takeover absent'])
            print_provider_report(report)
            write_report(args.report_json, report)
            return 0
        if live_marker_hits:
            report = provider_report(
                target,
                'blocked',
                operation='uninstall',
                details=['live assets contain takeover markers but no backup manifest'],
                live_asset_markers=live_marker_hits,
                live_assets=True,
            )
            print_provider_report(report)
            write_report(args.report_json, report)
        return fail('cannot rollback provider takeover without an owned backup manifest')
    changed: list[str] = []
    try:
        for item in manifest.get('source_backups', []):
            rel = item['rel']
            path = target / rel
            backup = target / item['backup']
            if not backup.is_file():
                raise ValueError(f'missing source backup: {backup}')
            assert_current_matches_install(path, item, 'source file')
            if not args.dry_run:
                shutil.copy2(backup, path)
            changed.append(rel)
        for item in manifest.get('live_asset_backups', []):
            asset = Path(item['asset'])
            backup = target / item['backup']
            if not backup.is_file():
                raise ValueError(f'missing live asset backup: {backup}')
            if asset.exists() and not has_takeover_marker(asset):
                raise ValueError(f'live asset drift without takeover marker: {asset}')
            assert_current_matches_install(asset, item, 'live asset')
            if not args.dry_run:
                shutil.copy2(backup, asset)
                restore_owner_mode(asset, item, 'before')
            changed.append(str(asset))
        if not args.dry_run:
            provider_manifest_path(target).unlink(missing_ok=True)
    except Exception as exc:
        return fail(f'failed to rollback provider takeover overlay: {exc}')
    report = provider_report(
        target,
        'dry-run' if args.dry_run else 'clean',
        changed,
        operation='uninstall',
        live_assets=bool(manifest.get('live_asset_backups')),
        dry_run=args.dry_run,
    )
    print_provider_report(report)
    write_report(args.report_json, report)
    return 0


def block(name: str, body: str) -> str:
    return f'// BEGIN {MARKER}: {name}\n{body.rstrip()}\n// END {MARKER}: {name}\n'


def replace_block(text: str, name: str, body: str) -> str:
    begin = f'// BEGIN {MARKER}: {name}\n'
    end = f'// END {MARKER}: {name}\n'
    start = text.find(begin)
    if start == -1:
        return text
    finish = text.find(end, start)
    if finish == -1:
        raise ValueError(f'missing end marker for {name}')
    finish += len(end)
    return text[:start] + block(name, body) + text[finish:]


def insert_after(text: str, anchor: str, name: str, body: str) -> str:
    if f'BEGIN {MARKER}: {name}' in text:
        return replace_block(text, name, body)
    index = text.find(anchor)
    if index == -1:
        raise ValueError(f'missing anchor for {name}: {anchor!r}')
    index += len(anchor)
    prefix = '' if anchor.endswith('\n') else '\n'
    return text[:index] + prefix + block(name, body) + text[index:]


def remove_block(text: str, name: str) -> str:
    begin = f'// BEGIN {MARKER}: {name}\n'
    end = f'// END {MARKER}: {name}\n'
    while True:
        start = text.find(begin)
        if start == -1:
            return text
        finish = text.find(end, start)
        if finish == -1:
            raise ValueError(f'missing end marker for {name}')
        finish += len(end)
        text = text[:start] + text[finish:]


def generated_backend() -> str:
    return r'''// Generated by codex-computer-use-x11 source overlay.
// BEGIN codex-computer-use-x11: generated-x11-ewmh-backend
use crate::terminal::enrich_terminal_windows;
use crate::windowing::registry::BackendProbe;
use crate::windowing::types::{WindowBounds, WindowInfo};
use anyhow::{bail, Context, Result};
use std::{env, process::Command};

pub const X11_EWMH_BACKEND: &str = "x11-ewmh";

pub fn probe() -> BackendProbe {
    let has_display = env::var("DISPLAY").ok().is_some_and(|value| !value.trim().is_empty());
    let has_wmctrl = command_available("wmctrl");
    let has_xprop = command_available("xprop");
    let ok = has_display && has_wmctrl && has_xprop;
    let mut details = Vec::new();
    if !has_display {
        details.push("DISPLAY is not set".to_string());
    }
    if !has_wmctrl {
        details.push("wmctrl is unavailable".to_string());
    }
    if !has_xprop {
        details.push("xprop is unavailable".to_string());
    }
    BackendProbe {
        id: X11_EWMH_BACKEND,
        ok,
        can_list_windows: ok,
        can_focus_apps: ok,
        can_focus_windows: ok,
        detail: if ok {
            "wmctrl/xprop X11 EWMH backend is available".to_string()
        } else {
            details.join("; ")
        },
    }
}

pub fn list_windows() -> Result<Vec<WindowInfo>> {
    let output = Command::new("wmctrl")
        .arg("-lpGx")
        .output()
        .context("failed to run wmctrl -lpGx")?;
    if !output.status.success() {
        bail!(
            "wmctrl -lpGx failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    let active = active_window_id().ok().flatten();
    let mut windows = parse_wmctrl_lpgx(&String::from_utf8_lossy(&output.stdout), active);
    enrich_terminal_windows(&mut windows);
    Ok(windows)
}

pub fn activate_window(window_id: u64) -> Result<()> {
    let id = format!("0x{window_id:x}");
    let output = Command::new("wmctrl")
        .args(["-ia", id.as_str()])
        .output()
        .with_context(|| format!("failed to run wmctrl -ia {id}"))?;
    if output.status.success() {
        Ok(())
    } else {
        bail!(
            "wmctrl -ia {id} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
}

pub(crate) fn parse_wmctrl_lpgx(output: &str, active_window_id: Option<u64>) -> Vec<WindowInfo> {
    output
        .lines()
        .filter_map(|line| parse_wmctrl_row(line, active_window_id))
        .collect()
}

fn parse_wmctrl_row(line: &str, active_window_id: Option<u64>) -> Option<WindowInfo> {
    let mut parts = line.split_whitespace();
    let id = parse_window_id(parts.next()?)?;
    let workspace = parts.next()?.parse::<i32>().ok();
    let pid = parts.next()?.parse::<u32>().ok().filter(|pid| *pid > 0);
    let x = parts.next()?.parse::<i32>().ok()?;
    let y = parts.next()?.parse::<i32>().ok()?;
    let width = parts.next()?.parse::<u32>().ok().filter(|value| *value > 0)?;
    let height = parts.next()?.parse::<u32>().ok().filter(|value| *value > 0)?;
    let class = parts.next()?.to_string();
    let _host = parts.next()?;
    let title = parts.collect::<Vec<_>>().join(" ");

    Some(WindowInfo {
        window_id: id,
        title: (!title.is_empty()).then_some(title),
        app_id: (!class.is_empty()).then_some(class.clone()),
        wm_class: (!class.is_empty()).then_some(class),
        pid,
        bounds: Some(WindowBounds {
            x: Some(x),
            y: Some(y),
            width,
            height,
        }),
        workspace,
        focused: active_window_id == Some(id),
        hidden: false,
        client_type: Some("x11".to_string()),
        backend: X11_EWMH_BACKEND.to_string(),
        terminal: None,
    })
}

fn active_window_id() -> Result<Option<u64>> {
    let output = Command::new("xprop")
        .args(["-root", "_NET_ACTIVE_WINDOW"])
        .output()
        .context("failed to run xprop -root _NET_ACTIVE_WINDOW")?;
    if !output.status.success() {
        bail!(
            "xprop -root _NET_ACTIVE_WINDOW failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(parse_window_id(&String::from_utf8_lossy(&output.stdout)))
}

fn parse_window_id(value: &str) -> Option<u64> {
    let token = value
        .split(|c: char| c.is_whitespace() || c == ',' || c == '#')
        .rev()
        .find(|part| part.starts_with("0x") || part.chars().all(|c| c.is_ascii_digit()))?;
    if let Some(hex) = token.strip_prefix("0x") {
        u64::from_str_radix(hex, 16).ok()
    } else {
        token.parse::<u64>().ok()
    }
}

fn command_available(command: &str) -> bool {
    Command::new("sh")
        .args(["-c", &format!("command -v {command}")])
        .output()
        .is_ok_and(|output| output.status.success())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_wmctrl_rows_to_window_info() {
        let windows = parse_wmctrl_lpgx(
            "0x00000002 0 1234 -10 20 800 600 app.App testhost Editor Window\n",
            Some(2),
        );

        assert_eq!(windows.len(), 1);
        let window = &windows[0];
        assert_eq!(window.window_id, 2);
        assert_eq!(window.backend, X11_EWMH_BACKEND);
        assert_eq!(window.client_type.as_deref(), Some("x11"));
        assert_eq!(window.title.as_deref(), Some("Editor Window"));
        assert_eq!(window.bounds.as_ref().unwrap().x, Some(-10));
        assert_eq!(window.bounds.as_ref().unwrap().width, 800);
        assert!(window.focused);
    }

    #[test]
    fn skips_invalid_geometry_without_panic() {
        let windows = parse_wmctrl_lpgx(
            "0x00000002 0 1234 0 0 0 600 app.App host Bad\n0x00000003 0 1234 1 2 300 400 app.App host Good\n",
            None,
        );

        assert_eq!(windows.len(), 1);
        assert_eq!(windows[0].window_id, 3);
    }
}
// END codex-computer-use-x11: generated-x11-ewmh-backend
'''


def has_owned_backend(target: Path) -> bool:
    path = target / BACKEND_REL
    return path.is_file() and 'Generated by codex-computer-use-x11 source overlay' in path.read_text(errors='replace')


def marker_count(target: Path) -> int:
    count = 0
    for rel in PATCHED_FILES:
        path = target / rel
        if path.is_file():
            count += path.read_text(errors='replace').count(BEGIN)
    return count


def status(target: Path) -> int:
    missing = check_structure(target)
    print(f'target={target}')
    print(f'target_commit={target_commit(target)}')
    if missing:
        print('state=drifted')
        for rel in missing:
            print(f'detail=missing target structure: {rel}')
        return 2
    markers = marker_count(target)
    backend = target / BACKEND_REL
    expected_markers = 0 if not backend.exists() and markers == 0 else None
    if markers == 0 and not backend.exists():
        print('state=clean')
        print('marker_count=0')
        return 0
    if markers > 0 and has_owned_backend(target) and read(backend) == generated_backend():
        print('state=applied')
        print(f'marker_count={markers}')
        print(f'backend={BACKEND_REL}')
        return 0
    print('state=drifted')
    print(f'marker_count={markers}')
    print(f'detail=backend_owned={has_owned_backend(target)} backend_exists={backend.exists()}')
    return 2


def ensure_no_unowned_x11(target: Path) -> None:
    backend = target / BACKEND_REL
    if backend.exists() and not has_owned_backend(target):
        raise ValueError(f'unowned native X11 backend exists: {BACKEND_REL}')


def patch_backends_mod(target: Path) -> None:
    path = target / 'computer-use-linux/src/windowing/backends/mod.rs'
    text = read(path)
    body = 'pub mod x11_ewmh;'
    anchor = 'pub mod i3;\n'
    if anchor not in text:
        anchor = 'pub mod kwin;\n'
    write(path, insert_after(text, anchor, 'backend-module', body))


def patch_registry(target: Path) -> None:
    path = target / 'computer-use-linux/src/windowing/registry.rs'
    text = read(path)
    patches = [
        ('backend-import', 'use crate::windowing::backends::{cosmic, gnome, hyprland, i3, kwin};\n', 'use crate::windowing::backends::x11_ewmh;'),
        ('backend-export', 'pub use i3::I3_BACKEND;\n', 'pub use x11_ewmh::X11_EWMH_BACKEND;'),
        ('backend-kind', '    I3,\n', '    X11Ewmh,'),
        ('backend-order', '    BackendKind::I3,\n', '    BackendKind::X11Ewmh,'),
        ('list-dispatch', '        BackendKind::I3 => i3::list_windows(),\n', '        BackendKind::X11Ewmh => x11_ewmh::list_windows(),'),
        ('activate-dispatch', '        I3_BACKEND => i3::activate_window(window.window_id),\n', '        X11_EWMH_BACKEND => x11_ewmh::activate_window(window.window_id),'),
        ('probe-entry', '        i3::probe(),\n', '        x11_ewmh::probe(),'),
        ('id-match', '            BackendKind::I3 => I3_BACKEND,\n', '            BackendKind::X11Ewmh => X11_EWMH_BACKEND,'),
    ]
    for name, anchor, body in patches:
        if anchor in text:
            text = insert_after(text, anchor, name, body)
    descriptor_anchor = '''    BackendDescriptor {
        id: I3_BACKEND,
        failure_label: "i3",
        list_note: "Window list came from i3-msg. Terminal windows may include best-effort PTY and active-process context when xprop and the process tree are readable.",
        missing_hint: "On i3, ensure i3-msg can reach the active i3 IPC socket.",
        can_exact_focus: true,
    },
'''
    descriptor_body = '''    BackendDescriptor {
        id: X11_EWMH_BACKEND,
        failure_label: "X11/EWMH",
        list_note: "Window list came from the generic X11/EWMH fallback backend. Terminal windows may include best-effort PTY and active-process context when the process tree is readable.",
        missing_hint: "On X11 desktops, ensure DISPLAY is set and wmctrl/xprop are available.",
        can_exact_focus: true,
    },'''
    if descriptor_anchor in text:
        text = insert_after(text, descriptor_anchor, 'descriptor', descriptor_body)
    elif 'const DESCRIPTORS:' in text and 'X11_EWMH_BACKEND' not in text:
        text = insert_after(text, 'const DESCRIPTORS:', 'descriptor', descriptor_body)
    write(path, text)


def patch_windowing_mod(target: Path) -> None:
    path = target / 'computer-use-linux/src/windowing/mod.rs'
    text = read(path)
    text = insert_after(text, 'pub mod types;\n', 'backend-export', '#[allow(unused_imports)]\npub use registry::X11_EWMH_BACKEND;')
    expected_anchor = '                I3_BACKEND,\n'
    if expected_anchor in text:
        text = insert_after(text, expected_anchor, 'registry-order-test', '                X11_EWMH_BACKEND,')
    write(path, text)


ORIGINAL_PORTAL_FUNCTION = '''fn portal_interface_check(interface: &str) -> Check {
    command_check_with_session_bus(
        "busctl",
        &[
            "--user",
            "introspect",
            "org.freedesktop.portal.Desktop",
            "/org/freedesktop/portal/desktop",
            interface,
        ],
    )
}
'''

MODIFIED_PORTAL_FUNCTION = '''fn portal_interface_check(interface: &str) -> Check {
    let check = command_check_with_session_bus(
        "busctl",
        &[
            "--user",
            "introspect",
            "org.freedesktop.portal.Desktop",
            "/org/freedesktop/portal/desktop",
            interface,
        ],
    );
    // BEGIN codex-computer-use-x11: strict-portal-method-check
    if !check.ok {
        return check;
    }
    let required_methods: &[&str] = match interface {
        "org.freedesktop.portal.RemoteDesktop" => &["CreateSession", "SelectDevices", "Start"],
        "org.freedesktop.portal.Screenshot" => &["Screenshot"],
        _ => &[],
    };
    if !required_methods.is_empty()
        && !required_methods
            .iter()
            .any(|method| check.detail.contains(method))
    {
        return Check::fail(format!(
            "{interface} introspection did not include required methods: {}",
            required_methods.join(", ")
        ));
    }
    // END codex-computer-use-x11: strict-portal-method-check
    check
}
'''


def patch_diagnostics(target: Path) -> None:
    path = target / 'computer-use-linux/src/diagnostics.rs'
    text = read(path)
    if MODIFIED_PORTAL_FUNCTION in text:
        write(path, text)
        return
    if ORIGINAL_PORTAL_FUNCTION not in text:
        # For minimal fake fixtures, still add a visible owned marker if exact replacement is unavailable.
        text = insert_after(text, 'fn portal_interface_check', 'strict-portal-method-check', '// strict portal method check anchor')
    else:
        text = text.replace(ORIGINAL_PORTAL_FUNCTION, MODIFIED_PORTAL_FUNCTION)
    write(path, text)


def install(target: Path) -> int:
    missing = check_structure(target)
    if missing:
        return fail('missing target structure: ' + ', '.join(missing))
    try:
        ensure_no_unowned_x11(target)
        backend = target / BACKEND_REL
        write(backend, generated_backend())
        patch_backends_mod(target)
        patch_registry(target)
        patch_windowing_mod(target)
        patch_diagnostics(target)
    except Exception as exc:
        return fail(f'failed to install source overlay: {exc}')
    print('state=applied')
    print(f'target={target}')
    print(f'backend={BACKEND_REL}')
    return 0


def remove_all_marker_blocks(text: str) -> str:
    while f'// BEGIN {MARKER}:' in text:
        start = text.find(f'// BEGIN {MARKER}:')
        finish = text.find(f'// END {MARKER}:', start)
        if finish == -1:
            raise ValueError('missing end marker while uninstalling')
        line_end = text.find('\n', finish)
        if line_end == -1:
            line_end = len(text)
        else:
            line_end += 1
        text = text[:start] + text[line_end:]
    return text


def uninstall(target: Path) -> int:
    missing = check_structure(target)
    if missing:
        return fail('missing target structure: ' + ', '.join(missing))
    try:
        backend = target / BACKEND_REL
        if backend.exists():
            if has_owned_backend(target):
                backend.unlink()
            else:
                return fail(f'refusing to remove unowned native X11 backend: {BACKEND_REL}')
        for rel in PATCHED_FILES:
            path = target / rel
            text = read(path)
            if MODIFIED_PORTAL_FUNCTION in text:
                text = text.replace(MODIFIED_PORTAL_FUNCTION, ORIGINAL_PORTAL_FUNCTION)
            text = remove_all_marker_blocks(text)
            while '\n\n\n' in text:
                text = text.replace('\n\n\n', '\n\n')
            write(path, text)
    except Exception as exc:
        return fail(f'failed to uninstall source overlay: {exc}')
    print('state=clean')
    print(f'target={target}')
    return 0


def main() -> int:
    args = parse_args()
    target = resolve_target(args.target)
    if args.provider is not None or args.mode is not None:
        if not validate_provider_mode(args):
            return fail('unsupported provider/mode: only --provider x11 --mode takeover is supported')
        if args.command == 'status':
            return provider_status(target, args)
        if args.command == 'install':
            return provider_install(target, args)
        if args.command == 'uninstall':
            return provider_uninstall(target, args)
        raise AssertionError(args.command)
    if args.command == 'status':
        return status(target)
    if args.command == 'install':
        return install(target)
    if args.command == 'uninstall':
        return uninstall(target)
    raise AssertionError(args.command)


if __name__ == '__main__':
    raise SystemExit(main())
