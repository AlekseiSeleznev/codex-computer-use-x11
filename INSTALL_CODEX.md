# Install codex-computer-use-x11 v0.1.3

This file is the short release install path. The full rollback-first guide lives in [`docs/install-uninstall.md`](docs/install-uninstall.md).

## Scope

`v0.1.3` supports the documented Linux Mint Cinnamon on X11 baseline with backend id `x11-ewmh`.

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

## Optional codex-desktop-linux adapter and local feature install

Prepared adapter contract for optional linux-features/x11-ewmh-computer-use integration in codex-desktop-linux. See [`docs/codex-desktop-linux-x11-ewmh-adapter.md`](docs/codex-desktop-linux-x11-ewmh-adapter.md). This is not the standalone user-local install path, does not claim upstream integration is merged, and does not require standalone users to write into `/opt` or `openai-bundled`. For manual no-release Codex Desktop Linux verification, use `scripts/install-codex-desktop-linux-x11-feature.sh` and the matching manifest-backed uninstaller documented below.

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
## Local Codex Desktop Linux feature install helper

For manual verification before a published release, this repository provides a local opt-in installer that applies the prepared Linux Feature adapter to a selected `codex-desktop-linux` checkout/install directory:

```bash
scripts/install-codex-desktop-linux-x11-feature.sh \
  --target "$CODEX_DESKTOP_LINUX_FULL_PATH" \
  --install-dir /opt/codex-desktop \
  --source "$PWD" \
  --patch-mode auto \
  --report-json /tmp/codex-x11-feature-install.json
```

For a source checkout that is not writable as the current user, run the command with appropriate permissions or choose a writable fixture/install directory. The script does not silently escalate privileges.

Useful local modes:

```bash
# Show the plan only, with no writes.
scripts/install-codex-desktop-linux-x11-feature.sh --dry-run --report-json -

# Stage from an already built binary.
scripts/install-codex-desktop-linux-x11-feature.sh \
  --target /path/to/codex-desktop-linux \
  --install-dir /path/to/codex-app \
  --binary /path/to/codex-computer-use-x11 \
  --patch-mode skip \
  --report-json -

# Fixture tests only: writes a fake marker instead of a real app.asar patch.
scripts/install-codex-desktop-linux-x11-feature.sh \
  --target /tmp/fake-codex-desktop-linux \
  --install-dir /tmp/fake-codex-app \
  --binary /tmp/codex-computer-use-x11 \
  --patch-mode fake \
  --report-json -
```

`--patch-mode auto` patches `resources/app.asar` only when the selected install directory has app assets and the target checkout has the expected patcher/tooling. `--patch-mode skip` stages the feature/plugin without touching app assets. `--patch-mode fake` is test-only evidence for temporary fixtures and must not be used as real live UI patch evidence.

The installer writes a non-secret rollback manifest by default at:

```text
<install-dir>/.codex-x11-feature/install-manifest.json
```

Rollback/uninstall uses that manifest and refuses to overwrite drifted files:

```bash
scripts/uninstall-codex-desktop-linux-x11-feature.sh \
  --install-dir /opt/codex-desktop \
  --report-json /tmp/codex-x11-feature-uninstall.json

# Preview rollback only.
scripts/uninstall-codex-desktop-linux-x11-feature.sh \
  --install-dir /opt/codex-desktop \
  --dry-run \
  --report-json -
```

The helper preserves the bundled `computer-use` plugin and marketplace entry. It only stages the separate `codex-computer-use-x11` plugin and local `x11-ewmh-computer-use` feature state.

