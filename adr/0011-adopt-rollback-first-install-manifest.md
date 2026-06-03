# 0011 — Adopt rollback-first install manifest contract

## Status

Accepted

## Date

2026-06-02

## Context

The standalone `codex-computer-use-x11` plugin, source overlay, provider takeover shim, optional live webview asset patching, and Cinnamon/X11 accessibility activation touch several local state surfaces. Some are user-owned under `$CODEX_HOME`, some are in a local Codex Desktop Linux checkout, some are root-owned live assets under the installed Codex Desktop app, and some are user desktop session settings such as gsettings and user activation environment variables.

A fresh install needs to activate all selected Cinnamon/X11 functionality, but uninstall must return the system to its pre-install state without deleting unrelated user settings or overwriting administrator/user changes made after install. Prior provider takeover work already used backup manifests for source/live assets, but the rollback contract was not yet a durable architecture rule across standalone plugin install, accessibility setup, source overlay, and live assets.

## Decision

Adopt a **rollback-first install manifest contract** for install, uninstall, and provider takeover work.

Every installer-owned mutation must record enough non-secret before-state before applying the mutation, then record completed after-state once the mutation succeeds. The manifest must distinguish:

- state the installer changed vs state that was already acceptable;
- planned entries vs completed entries so partial installs can roll back safely;
- before-state vs installer after-state;
- file bytes/checksums, ownership, mode, and backup path when file restoration is possible;
- setting or activation-environment values vs absence for supported non-secret keys.

Uninstall and rollback must restore only completed installer-owned changes. For file-like and setting-like state that could have been changed after installation, rollback must compare current state with recorded installer after-state before restoring before-state. If current state has drifted, rollback reports a blocker instead of blindly overwriting.

The manifest is a safety boundary, not a secret store. It must not contain credentials, private URLs, tokens, or broad environment dumps. It may record non-secret variable names and the explicitly supported accessibility values needed for rollback, such as `NO_AT_BRIDGE`, `GTK_MODULES`, and `QT_ACCESSIBILITY`.

## Considered Options

1. **Rollback-first manifest across all install surfaces** (chosen)
   - Provides a single safety model for standalone plugin paths, accessibility setup, source overlay, provider takeover, and live assets.
   - Supports partial install rollback and idempotent uninstall.
   - Makes drift/blockers observable rather than silently destructive.

2. **Ad hoc uninstaller per surface**
   - Simpler for a single script.
   - Rejected because the combined fresh install crosses ownership and privilege boundaries, and ad hoc deletion cannot reliably distinguish installer-owned changes from user/admin changes.

3. **Always restore recorded before-state even after drift**
   - Maximizes automatic cleanup.
   - Rejected because it could overwrite changes made by Codex Desktop updates, package managers, administrators, or the user after install.

4. **Never mutate accessibility/session settings automatically**
   - Minimizes installer responsibility.
   - Rejected as insufficient for the requested fresh install goal because disabling `NO_AT_BRIDGE=1` and enabling toolkit accessibility may be required for AT-SPI tree extraction to work after install.

## Consequences

- Fresh install becomes safer but more stateful: each installer surface needs manifest read/write and report-json coverage.
- Uninstall can be idempotent and partial-install safe, but some rollback attempts will intentionally stop on drift and require manual review.
- Tests must verify manifest contents, changed-vs-already-present classification, dry-run behavior, partial install behavior, and drift blockers.
- Provider takeover remains governed by ADR 0010 for settings/provider identity and bundled fallback; this ADR adds the cross-surface rollback safety contract.
- The architecture snapshot must treat backup manifests and drift detection as part of the delivery boundary, not just implementation detail.

## Evidence

- OpenSpec change: `openspec/changes/harden-fresh-install-atspi-rollback/`.
- Design review finding: `openspec/changes/harden-fresh-install-atspi-rollback/design-review.md` recommends the rollback-first manifest contract as a durable ADR.
- Existing provider takeover manifest/rollback code: `scripts/codex-source-overlay.py`.
