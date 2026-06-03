## Why

The current provider-takeover rollout can leave visible `X11 Computer Use` UI state after the standalone plugin is uninstalled because the one-command installer changes multiple surfaces while rollback is split across lower-level commands and does not reliably restore every live asset mutation. Installer and uninstaller behavior must become symmetric, manifest-backed, and safe under partial failure so users can prove the system returned to bundled Computer Use mode.

## What Changes

- Add a one-command provider-takeover uninstaller that mirrors `install-x11-provider-takeover.sh` and removes the standalone plugin, source overlay, live asset patch, and owned metadata as one rollback workflow.
- Harden provider-takeover install so every source file and live webview asset mutation is backed up before write, recorded in a manifest/report, and either completed atomically or rolled back on failure.
- Harden provider-takeover uninstall so it restores source files and live assets from the recorded manifest, verifies drift/checksums before restore, removes owned metadata only after successful rollback, and reports exact blockers when safe restore is impossible.
- Add verification for install → uninstall round trips, live asset backup/restore, missing-manifest/drift safety, dry-run behavior, and standalone plugin cleanup.
- Update install/uninstall documentation to show the provider-takeover rollback path and restart/rebuild expectations.

## Capabilities

- Modify `codex-source-overlay-extension` to strengthen provider-takeover install/uninstall, live asset backup, rollback, and verification requirements.

## Impact

- Affected scripts: `scripts/install-x11-provider-takeover.sh`, `scripts/codex-source-overlay.py`, and a new `scripts/uninstall-x11-provider-takeover.sh` wrapper.
- Affected tests: source-overlay/provider-takeover script tests and plugin installer tests covering manifest-backed rollback and live asset restoration.
- Affected docs: README/INSTALL/docs install-uninstall guidance for provider takeover rollback and live asset restart behavior.
- Project constraints: keep changes in Rust/Bash/Python as already used by the repository; do not edit installed OpenSpec packages; never read or write secrets; preserve bundled `openai-bundled/computer-use` plugin state and unrelated Codex config; keep target checkout changes reversible and visible through `git status --short`.
