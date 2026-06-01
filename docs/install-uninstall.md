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

Rollback:

```bash
scripts/uninstall-codex-plugin.sh
```

The uninstaller removes only owned `codex-computer-use-x11` cache, marketplace, and config sections. It preserves unrelated marketplaces, `openai-bundled` plugin caches, and user config.

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

## Status and drift

`state=clean` means no owned overlay content is present. `state=applied` means expected owned markers and generated backend content are present. `state=drifted` means owned marker blocks, generated backend content, anchors, or metadata no longer match expectations.

If drift is reported, stop before reinstalling. Inspect target `git status --short`, compare owned marker blocks, and do not overwrite unowned target code or native X11 backend files blindly.
