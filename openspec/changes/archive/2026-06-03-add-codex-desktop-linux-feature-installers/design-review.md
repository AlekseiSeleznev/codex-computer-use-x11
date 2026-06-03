## Context Read

- `proposal.md`, delta spec, `grill.md`, and `design.md`
- `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md`
- `adr/0010-adopt-x11-provider-takeover-shim.md`
- `adr/0011-adopt-rollback-first-install-manifest.md`
- `docs/codex-desktop-linux-x11-ewmh-adapter.md`
- Adapter scaffold `feature.json`, `stage.sh`, `patches.js`, and `test.js`
- Existing installer/provider/source-overlay scripts and tests

## Design Summary

- The design adds local opt-in install/uninstall wrappers, not upstream default behavior.
- Plugin staging delegates to the adapter scaffold `stage.sh`, keeping the standalone plugin and bundled `computer-use` separated.
- Rollback is manifest-driven with before/after checksums and drift blockers.
- App asset patching is explicit, optional in tests, and report-driven rather than hidden global doctor/core behavior.

## Question Loop

- None asked. The design choices are directly constrained by maintainer feedback, ADR 0011, and the user's current goal to manually verify without a release.

## Design Findings

- **Resolved risk: root-owned installs.** The design intentionally avoids automatic sudo. This preserves auditability and makes permission failures visible; the user can run the script with privileges when targeting `/opt/codex-desktop`.
- **Resolved risk: fake patch mode misuse.** The design marks fake patching as fixture/test-only. Documentation and report fields must label it as fake so it cannot be confused with real `app.asar` patch evidence.
- **Resolved risk: marketplace drift.** Restoring whole marketplace files is acceptable only with after-checksum drift checks. If the app/updater changed marketplace content after install, uninstall must block.
- **Resolved risk: scaffold divergence.** Delegating staging to `stage.sh` avoids duplicate plugin bundle semantics; tests should prove the wrapper preserves `computer-use` through the stage hook.

## Document Updates Applied

- None required. Proposal/spec/design already include opt-in scope, non-masquerade, rollback-first, dry-run/report-json, and drift-blocker behavior.

## Document Updates Required Before Next Gate

- None.

## ADR Candidates

- None. This is an implementation of ADR 0011 and existing adapter decisions, not a new durable architecture choice.

## Open Questions

None
