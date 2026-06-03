## Context Read

- User intent: after uninstalling the standalone plugin, the Computer Use settings UI still displayed `X11 Computer Use`; manual cleanup showed provider takeover source overlay and a patched live webview asset remained.
- Existing specs: `openspec/specs/codex-source-overlay-extension/spec.md`, `openspec/specs/codex-computer-use-provider-takeover/spec.md`, `openspec/specs/codex-computer-use-settings-ui/spec.md`, and `openspec/specs/x11-packaging-docs-upstreaming/spec.md`.
- Existing scripts: `scripts/install-x11-provider-takeover.sh`, `scripts/install-codex-plugin.sh`, `scripts/uninstall-codex-plugin.sh`, `scripts/codex-source-overlay.py`, source-overlay shell wrappers, and provider takeover overlay patch files under `scripts/overlays/codex-desktop-linux-full/provider-takeover/`.
- Existing tests: `tests/source_overlay_scripts.rs`, `tests/plugin_installer.rs`, and e2e plugin/source-overlay smoke boundaries.
- Project context: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and ADRs 0008-0010.

## Question Loop

No user-facing question required.

Repository evidence already answers the material uncertainty: the one-command installer can touch three surfaces (standalone plugin state, target source overlay, live webview assets), while rollback was split and did not provide a symmetric one-command restore. The correct change is to make install transaction/manifest-backed and provide a matching provider-takeover uninstaller.

## Findings

- **Hidden surface risk:** `scripts/uninstall-codex-plugin.sh` correctly removes only owned `$CODEX_HOME` standalone plugin state, but a user naturally expects rollback after `install-x11-provider-takeover.sh` to remove every visible provider takeover surface. A separate provider-takeover uninstaller is necessary.
- **Live asset risk:** live `computer-use-settings-*.js` assets may be root-owned and already loaded by Electron. If install patches or a rebuild installs a patched asset, source overlay rollback alone does not restore the running app's settings bundle.
- **Manifest completeness risk:** current provider manifest can have `live_asset_backups: []` even when later visible UI state exists. Future install must record every live asset write it performs and its restore metadata; uninstall must refuse unsafe blind deletion when markers remain without backups.
- **Partial failure risk:** installing plugin, source overlay, and live assets in sequence can fail mid-run. The installer should either leave a clearly failed transaction report and roll back completed current-transaction writes, or refuse to claim success.
- **Drift safety:** rollback must verify current files still represent owned provider takeover content before overwriting them with backups. If a user or upstream changed the file after install, rollback should stop with a blocker instead of destroying unrelated work.
- **No architecture conflict:** The change preserves ADR 0010's localized provider-takeover shim and standalone plugin identity. It does not change X11 coordinate/input architecture, external systems, or secret handling.

## Decisions

- Add `scripts/uninstall-x11-provider-takeover.sh` as the operator-facing rollback counterpart to `scripts/install-x11-provider-takeover.sh`.
- Keep lower-level source-overlay rollback in `scripts/codex-source-overlay.py`, but harden it to use transaction/manifest metadata and live asset restore checks.
- Treat missing manifest/backups as a safe blocker when owned markers are present; do not implement best-effort blind regex deletion for normal rollback.
- Allow no-op success only when plugin state is absent, provider overlay status is clean, and live assets contain no owned takeover marker/string.

## Document Updates Applied

- Proposal scopes the one-command rollback wrapper, install backup hardening, safe uninstall, tests, and docs.
- Delta spec strengthens `codex-source-overlay-extension` requirements for live asset backups, transaction failure rollback, one-command rollback, drift refusal, absent takeover no-op, and missing manifest blockers.

## Open Questions

None.
