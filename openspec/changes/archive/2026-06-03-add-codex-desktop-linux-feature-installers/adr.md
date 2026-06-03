## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — accepted; remains in force.
- `adr/0003-formalize-project-context-entrypoints.md` — accepted; remains in force.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — accepted; remains in force.
- `adr/0006-adopt-claude-artifact-review.md` — accepted; remains in force, but session review is disabled by prior user instruction.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — accepted; remains in force.
- `adr/0008-adopt-x11-root-coordinate-model.md` — accepted; remains in force but not directly changed.
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — accepted; remains in force and constrains standalone `x11_*` identity and source-overlay/upstream separation.
- `adr/0010-adopt-x11-provider-takeover-shim.md` — accepted; remains in force and forbids global `computer-use` masquerade.
- `adr/0011-adopt-rollback-first-install-manifest.md` — accepted; remains in force and directly governs installer/uninstaller behavior.

## Constitution / Architecture Rules Considered

- Use OpenSpec artifacts as lifecycle source of truth and checkpoint after each artifact/apply group.
- Keep real secrets out of tracked files, reports, manifests, and output.
- Prefer Bash helper entrypoints under `scripts/`; Rust project checks apply when Rust changes are made.
- Local integration target defaults to `CODEX_DESKTOP_LINUX_FULL_PATH` or the documented local `codex-desktop-linux` path.
- `ARCHITECTURE.md` requires standalone plugin identity, rollback-first manifests, and no global core Computer Use rewrite for this adapter path.

## Decisions Evaluated

- Add local installer/uninstaller wrappers for the optional Linux Feature adapter: accepted as implementation work because it automates the already accepted adapter architecture for manual verification.
- Use a Python helper behind shell entrypoints: accepted as an implementation choice for JSON reports and manifest safety; no durable ADR needed.
- Delegate plugin staging to adapter `stage.sh`: accepted to avoid divergence from the copyable upstream scaffold.
- Do not add automatic sudo escalation: accepted to preserve auditability and testability; permission failures remain explicit.
- Restore whole backed-up files/directories only after after-state drift checks: accepted as direct ADR 0011 implementation.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None. The current architecture already includes the optional adapter, standalone identity, provider-shim/non-masquerade boundary, and rollback-first manifest rule.

## No ADR Needed

- No new durable ADR is needed because the change implements ADR 0011's rollback-first manifest contract and ADR 0009/0010's adapter identity boundaries rather than introducing a new architecture decision.
