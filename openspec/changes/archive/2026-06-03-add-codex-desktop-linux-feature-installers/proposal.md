## Why

The manual local Codex Desktop Linux install proved the optional `x11-ewmh-computer-use` Linux Feature shape, but repeating it by hand is fragile: it touches a target checkout, a live app install, plugin marketplace metadata, update-builder feature state, and patched app assets. We need installer and uninstaller scripts that apply the same new architecture safely, without publishing a release yet, so the user can verify it locally.

## What Changes

- Add tracked Codex Desktop Linux feature installer/uninstaller entrypoints for the optional `x11-ewmh-computer-use` adapter.
- Make the installer stage the copyable Linux Feature scaffold, enable it locally, stage `codex-computer-use-x11`, preserve bundled `computer-use`, and optionally patch live app assets through a rollback-first manifest.
- Make the uninstaller restore only installer-owned changes from the manifest and stop on drift instead of overwriting user/admin or update changes.
- Add dry-run/report-json behavior and fake-install tests so the flow can be checked without touching `/opt/codex-desktop`.
- Document the local manual verification flow for a fresh release as if it were already included in Codex Desktop Linux, while keeping the upstream feature disabled by default.

## Capabilities

- Modify `x11-release-adapter-handoff` with Codex Desktop Linux live/local feature installer and rollback uninstaller requirements.

## Impact

- Affected files: new scripts under `scripts/`, tests under `tests/`, documentation under `docs/` or `INSTALL_CODEX.md`, and the `x11-release-adapter-handoff` spec.
- Target systems: local `codex-desktop-linux` checkout selected by `--target` or `CODEX_DESKTOP_LINUX_FULL_PATH`, and optional local Codex Desktop install selected by `--install-dir`.
- Architecture constraints: keep the adapter fully opt-in/disabled by default, keep `codex-computer-use-x11` as a separate namespaced plugin, avoid core Computer Use/global doctor rewrites, and obey ADR 0011 rollback-first manifests.
- Secret handling: no secrets are needed; scripts and reports must not read or record credentials, tokens, private URLs, or local secret files.
