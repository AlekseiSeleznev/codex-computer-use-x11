## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — Accepted; remains in force for OpenSpec/source-of-truth workflow.
- `adr/0003-formalize-project-context-entrypoints.md` — Accepted; remains in force for `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, ADR, and local-secret boundaries.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — Accepted; remains in force for mandatory grill/design-review and TDD apply discipline.
- `adr/0006-adopt-claude-artifact-review.md` — Accepted; remains in force but session Claude review is disabled locally.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — Accepted; remains in force for safe lifecycle checkpoint commits.
- `adr/0008-adopt-x11-root-coordinate-model.md` — Accepted; remains in force; this change does not alter root-coordinate or targeted-input safety boundaries.
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — Accepted; remains in force; this change improves install/doctor/rollback evidence for the Cinnamon/X11 baseline.
- `adr/0010-adopt-x11-provider-takeover-shim.md` — Accepted; remains in force; this change preserves localized takeover shim, standalone identity, and bundled rollback.
- `adr/0011-adopt-rollback-first-install-manifest.md` — Accepted; created by this change to make rollback-first manifests the cross-surface install/uninstall safety contract.

Superseded ADRs considered:

- `adr/0002-adopt-project-constitution-preflight.md` — Superseded by ADR 0003; historical context only.
- `adr/0004-adopt-mandatory-review-and-test-plan-gates.md` — Superseded by ADR 0005; historical context only.

## Constitution / Architecture Rules Considered

- `CONSTITUTION.md`: Rust 2021/Cargo stack, `make fmt`, `make check`, `make test`, OpenSpec validation, no secret printing/staging, local target selected by `CODEX_DESKTOP_LINUX_FULL_PATH` or documented local default.
- `CONTEXT.md`: standalone plugin, source overlay, overlay drift, AT-SPI window correlation, accessibility tree, app state, e2e harness, controlled fixture, rollback-first install, and backup manifest terms.
- `ARCHITECTURE.md`: standalone plugin remains primary runtime delivery; source overlay is optional integration staging; lifecycle and rollback boundaries must remain explicit.
- ADR 0010: provider takeover must not globally masquerade as bundled `computer-use` or rewrite bundled ownership.

## Decisions Evaluated

- Whether doctor AT-SPI readiness is an implementation detail or architecture decision: treated as implementation/design because it reuses existing report fields and collector boundaries rather than changing system architecture.
- Whether the rollback manifest contract is durable architecture: yes. It applies across multiple delivery surfaces, changes the safety boundary for install/uninstall, and intentionally blocks drifted rollback rather than forcing restoration.
- Whether ADR 0010 should be superseded: no. The new rollback-first manifest contract complements ADR 0010 and keeps its takeover boundaries intact.
- Whether accessibility setup needs a durable ADR: no separate ADR. It is scoped by the rollback-first manifest contract and the existing Cinnamon/X11 baseline; no Orca/screen-reader decision is introduced.

## New Durable ADRs Created

- `adr/0011-adopt-rollback-first-install-manifest.md` — Accepted; captures the rollback-first install manifest contract across standalone plugin, accessibility setup, source overlay, provider takeover, and live assets.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- Updated `ARCHITECTURE.md` to list ADR 0011 as in force and to summarize the rollback-first install manifest rule.
- Updated `adr/README.md` current-state list to include ADR 0011.

## No ADR Needed

- N/A. A durable ADR was created.
