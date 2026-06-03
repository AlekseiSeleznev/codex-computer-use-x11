## Context Read

- `proposal.md` and `specs/x11-release-adapter-handoff/spec.md`
- `CONSTITUTION.md` verification, secret-handling, local integration target, and checkpoint rules
- `CONTEXT.md` terms: Linux Feature adapter, rollback-first install, backup manifest, source overlay, rollback drift
- `ARCHITECTURE.md`
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md`
- `adr/0010-adopt-x11-provider-takeover-shim.md`
- `adr/0011-adopt-rollback-first-install-manifest.md`
- `docs/codex-desktop-linux-x11-ewmh-adapter.md`
- `openspec/specs/x11-release-adapter-handoff/spec.md`
- `adapters/codex-desktop-linux/linux-features/x11-ewmh-computer-use/{feature.json,README.md,stage.sh,patches.js,test.js}`
- Existing installer/uninstaller scripts: `scripts/install-codex-plugin.sh`, `scripts/uninstall-codex-plugin.sh`, `scripts/codex-source-overlay.py`, `scripts/install-x11-provider-takeover.sh`, `scripts/uninstall-x11-provider-takeover.sh`

## Plan Summary

- The requested install flow is local and opt-in: it prepares a selected Codex Desktop Linux checkout/live install for manual verification, not an upstream default and not a published release.
- The installer must automate the manual architecture already validated: copy the Linux Feature scaffold, enable `x11-ewmh-computer-use` locally, stage `codex-computer-use-x11`, preserve bundled `computer-use`, and apply the feature-owned app patch when requested.
- The uninstaller must use ADR 0011 rollback-first state rather than deleting guessed paths or restoring stale backups blindly.
- Dry-run/report-json and fake fixture tests are required because `/opt/codex-desktop` is root-owned and unsuitable as the primary test boundary.

## Question Loop

- None asked. Repository context and the user's explicit instruction resolved the key choice: implement local installer/uninstaller for the already accepted thin Linux Feature adapter architecture, without release publication and without default upstream enablement.

## Resolved Terms

- `Codex Desktop Linux feature install`: a local opt-in development install that stages the copyable `x11-ewmh-computer-use` adapter and plugin into a selected `codex-desktop-linux` checkout/install directory for manual verification.
- `Feature uninstaller`: a manifest-backed rollback command that restores only installer-owned target/local/live surfaces and reports drift blockers.

## Document Updates Applied

- Proposal and delta spec already constrain scope to local opt-in feature install/uninstall, rollback-first manifest, preservation of bundled `computer-use`, and no secrets.

## Document Updates Required Before Next Gate

- None.

## ADR Candidates

- None. ADR 0011 already covers rollback-first manifests; ADR 0009/0010 already cover standalone identity, `x11_*` namespacing, and non-masquerade behavior.

## Open Questions

None
