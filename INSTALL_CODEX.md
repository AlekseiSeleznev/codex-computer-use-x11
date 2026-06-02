# Install codex-computer-use-x11 v0.1.2

This file is the short release install path. The full rollback-first guide lives in [`docs/install-uninstall.md`](docs/install-uninstall.md).

## Scope

`v0.1.2` supports the documented Linux Mint Cinnamon on X11 baseline with backend id `x11-ewmh`.

Out of scope for this release:

- Cinnamon Wayland;
- RemoteDesktop/Wayland-required runtime paths;
- native `.deb`, `.rpm`, or AppImage packages;
- uncontrolled live input against arbitrary user applications.

## Install

Run from the repository root:

```bash
scripts/install-codex-plugin.sh --dry-run
scripts/install-codex-plugin.sh
```

The installer writes only owned user-local Codex paths under `CODEX_HOME`:

- `plugins/cache/codex-computer-use-x11/`;
- `plugins/marketplaces/codex-computer-use-x11/`;
- owned `config.toml` sections for `codex-computer-use-x11`.

It does not write `/opt`, `openai-bundled`, or the bundled `computer-use` cache.

Restart or refresh Codex after installation, then look for the `x11_*` tools.

## Verify

```bash
cargo run -- doctor --json
scripts/e2e/codex-plugin-smoke.sh --fake
scripts/e2e/codex-plugin-smoke.sh --live --industrial --fake-live-fixtures
```

For a fresh-machine confidence check, use an isolated `CODEX_HOME` first:

```bash
tmp="$(mktemp -d)"
CODEX_HOME="$tmp/codex-home" \
CODEX_CONFIG_FILE="$tmp/codex-home/config.toml" \
  scripts/install-codex-plugin.sh
```

## Uninstall

```bash
scripts/uninstall-codex-plugin.sh --dry-run
scripts/uninstall-codex-plugin.sh
```

The uninstaller removes only owned cache, marketplace, and config entries.

## Provider takeover rollback

If you installed the broader X11 provider takeover path, use its matching rollback instead of only uninstalling the standalone plugin:

```bash
scripts/uninstall-x11-provider-takeover.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH" --dry-run
scripts/uninstall-x11-provider-takeover.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH"
```

That rollback restores manifest-backed source/live asset backups, removes the standalone plugin state, and may report a safe blocker when an old live asset contains takeover markers without a restorable backup. Fully restart Codex Desktop after rollback.
