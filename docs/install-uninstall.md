# Install/uninstall guide

This guide covers the two v1 delivery paths for `codex-computer-use-x11`:

1. the **standalone user-local Codex MCP plugin**; and
2. the **reversible source overlay** for a local Codex Desktop Linux target checkout.

Run fake/dry-run checks before mutating real user or target state. Do not read or paste `.secrets.local.env`; this project needs variable names such as `CODEX_DESKTOP_LINUX_FULL_PATH`, not secret values.

## Standalone user-local Codex MCP plugin

The standalone path installs only project-owned plugin state under `CODEX_HOME` (default `~/.codex`). It does not write `/opt`, `openai-bundled`, or the bundled `computer-use` cache. It exposes namespaced `x11_*` tools so it does not collide with the bundled Codex `computer-use` plugin.

Preview the install:

```bash
scripts/install-codex-plugin.sh --dry-run
```

Preview fresh Cinnamon/X11 accessibility activation as JSON without mutating
user state:

```bash
scripts/install-codex-plugin.sh --activate-accessibility --dry-run --report-json
```

Install user-locally:

```bash
scripts/install-codex-plugin.sh
```

Run deterministic smoke evidence without touching the real desktop:

```bash
scripts/e2e/codex-plugin-smoke.sh --fake
```

Optional live plugin smoke validates the current user-local plugin state. It should not send keyboard or pointer input unless a future safe target is explicitly added:

```bash
scripts/e2e/codex-plugin-smoke.sh --live
```

Preview rollback:

```bash
scripts/uninstall-codex-plugin.sh --dry-run
```

Preview manifest-backed rollback as JSON:

```bash
scripts/uninstall-codex-plugin.sh --dry-run --report-json
```

Rollback:

```bash
scripts/uninstall-codex-plugin.sh
```

The installer can write a rollback-first backup manifest under `CODEX_HOME`
for plugin and accessibility activation state. The uninstaller removes only
owned `codex-computer-use-x11` cache, marketplace, and config sections, restores
only installer-owned accessibility changes, and reports drift/blockers instead
of overwriting unrelated user settings. It preserves unrelated marketplaces,
`openai-bundled` plugin caches, and user config.

## Reversible source overlay

The reversible source overlay applies owned marker blocks and a generated `computer-use-linux/src/windowing/backends/x11_ewmh.rs` file into a local Codex Desktop Linux checkout. It is staging evidence, not a long-lived fork. Always inspect status first, run target checks while applied, uninstall, and verify the target checkout is clean.

Choose the target with `--target`, `CODEX_DESKTOP_LINUX_FULL_PATH`, or the documented local development default.

Inspect target status:

```bash
scripts/status-codex-source-overlay.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH"
git -C "$CODEX_DESKTOP_LINUX_FULL_PATH" status --short
```

Apply the overlay:

```bash
scripts/install-codex-source-overlay.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH"
```

Run target checks while the overlay is applied:

```bash
cargo test -p codex-computer-use-linux x11_ewmh --manifest-path "$CODEX_DESKTOP_LINUX_FULL_PATH/Cargo.toml"
```

Remove the overlay and confirm clean state:

```bash
scripts/uninstall-codex-source-overlay.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH"
scripts/status-codex-source-overlay.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH"
git -C "$CODEX_DESKTOP_LINUX_FULL_PATH" status --short
```

Run deterministic source-overlay smoke without mutating the real target:

```bash
scripts/e2e/codex-source-overlay-smoke.sh --fake
```

Optional live source-overlay smoke may be used only when the real target checkout starts clean. The smoke attempts uninstall before exit, but you must still inspect final target status.

```bash
scripts/e2e/codex-source-overlay-smoke.sh --live --target "$CODEX_DESKTOP_LINUX_FULL_PATH"
```


## X11 provider takeover rollout and rollback

The provider-takeover path is broader than the standalone plugin: it can install the standalone plugin, patch the Codex Desktop Linux target checkout, and optionally patch live `computer-use-settings-*.js` webview assets. Use the matching one-command rollback when you used the one-command installer.

Preview provider takeover install:

```bash
scripts/install-x11-provider-takeover.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH" --dry-run
```

Preview provider takeover install with aggregate JSON:

```bash
scripts/install-x11-provider-takeover.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH" --dry-run --report-json /tmp/x11-provider-install-report.json
```

Install provider takeover:

```bash
scripts/install-x11-provider-takeover.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH"
```

The installer writes manifest-backed backups for owned source-overlay files and any live assets it mutates. Backup metadata records original and installed checksums plus file metadata when available. If a later install step fails, the installer reports the failure and the source/live transaction attempts to restore writes from the current transaction instead of claiming success.

Preview provider takeover rollback:

```bash
scripts/uninstall-x11-provider-takeover.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH" --dry-run
```

Preview provider takeover rollback with aggregate JSON:

```bash
scripts/uninstall-x11-provider-takeover.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH" --dry-run --report-json /tmp/x11-provider-uninstall-report.json
```

Rollback provider takeover:

```bash
scripts/uninstall-x11-provider-takeover.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH"
scripts/status-codex-source-overlay.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH"
git -C "$CODEX_DESKTOP_LINUX_FULL_PATH" status --short
```

The provider-takeover uninstaller restores target source and live assets from manifest-backed backups, removes the standalone `codex-computer-use-x11` plugin state, verifies live settings assets no longer contain owned takeover markers, and reports a safe blocker instead of blindly deleting marker strings when a manifest or backup is missing. After rollback, fully restart Codex Desktop so Electron reloads bundled Computer Use settings assets.

## Live-safe verification checklist

When live Cinnamon/X11 validation is available, record non-secret evidence for
the selected delivery path:

```text
x11_doctor
x11_get_app_state include_screenshot=true
x11_accessibility_tree
provider takeover marker
full uninstall restore
```

Use controlled fixtures for `x11_accessibility_tree` and app-state checks. Store
screenshot evidence by path or summary only; do not paste inline screenshot data
URLs. If live assets were patched, verify the provider takeover marker is present
before rollback and absent after full uninstall restore. If any layer is
unavailable, record the exact limitation instead of claiming a pass.

## Status and drift

`state=clean` means no owned overlay content is present. `state=applied` means expected owned markers and generated backend content are present. `state=drifted` means owned marker blocks, generated backend content, anchors, or metadata no longer match expectations.

If drift is reported, stop before reinstalling. Inspect target `git status --short`, compare owned marker blocks, and do not overwrite unowned target code or native X11 backend files blindly.
