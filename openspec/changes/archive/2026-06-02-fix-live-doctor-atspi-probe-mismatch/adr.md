## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — Accepted; remains in force; lifecycle/checkpoint discipline applies.
- `adr/0003-formalize-project-context-entrypoints.md` — Accepted; remains in force; `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and ADRs were read before design/apply.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — Accepted; remains in force; grill/design-review and TDD are mandatory for this behavior-changing fix.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — Accepted; remains in force; safe lifecycle checkpoints are automatic in this session.
- `adr/0008-adopt-x11-root-coordinate-model.md` — Accepted; remains in force; no coordinate behavior is changed.
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — Accepted; remains in force; AT-SPI may degrade, but diagnostics must be truthful and evidence-backed.
- `adr/0010-adopt-x11-provider-takeover-shim.md` — Accepted; remains in force; not directly affected.
- `adr/0011-adopt-rollback-first-install-manifest.md` — Accepted; remains in force; not directly affected except that `NO_AT_BRIDGE` semantics remain presence-based.

## Constitution / Architecture Rules Considered

- Rust 2021/Cargo/Makefile verification is required for implementation.
- `doctor --json` behavior must be validated as machine-readable JSON before claiming completion.
- Doctor must remain non-invasive: no secret file access, screenshots, input injection, target checkout mutation, or external credentials.
- AT-SPI diagnostics must report degraded states rather than fabricating successful target subtrees.
- Architecture snapshot keeps standalone plugin as primary runtime delivery with thin AT-SPI command/script boundary.

## Decisions Evaluated

- Whether to make doctor target-scoped: rejected because existing specs define doctor as an ambient readiness/capability surface and `accessibility-tree` as target-scoped correlation.
- Whether to remove bounded AT-SPI collection timeout: rejected because doctor must remain safe for smoke tests and hung desktop probes.
- Whether to create a new durable ADR for sharing collector semantics: rejected because this corrects implementation under existing ADR 0009 and existing specs; no new hard-to-reverse architecture boundary is introduced.

## New Durable ADRs Created

- None

## Superseded ADRs

- None

## Architecture Snapshot Updates

- None. The current architecture already covers doctor/capability diagnostics, AT-SPI degraded behavior, and the standalone Rust runtime boundary.

## No ADR Needed

- No durable ADR is needed because this change fixes a mismatch between two existing AT-SPI collector consumers without changing architecture, supported baseline scope, delivery path, or rollback/provider boundaries.
