## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — in force for OpenSpec lifecycle and project-local overlay behavior.
- `adr/0003-formalize-project-context-entrypoints.md` — in force for root `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and local-secret boundaries.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — in force for mandatory grill/design-review and TDD planning.
- `adr/0006-adopt-claude-artifact-review.md` — in force; session has Claude review disabled.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — in force for automatic safe lifecycle checkpoints.
- `adr/0008-adopt-x11-root-coordinate-model.md` — in force but not directly changed.
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — in force; preserves `x11-ewmh`, standalone `x11_*` tool names, and backend/wrapper PR separation.
- `adr/0010-adopt-x11-provider-takeover-shim.md` — in force; forbids global masquerading as bundled `computer-use`.
- `adr/0011-adopt-rollback-first-install-manifest.md` — in force but not directly changed.
- Superseded historical ADRs `0002` and `0004` were considered only through their successors.

## Constitution / Architecture Rules Considered

- `CONSTITUTION.md` requires Rust/Cargo project verification through `make fmt`, `make check`, and `make test` for Rust changes; documentation-only changes still require relevant OpenSpec and test validation.
- Secret handling rules forbid reading or printing local secrets; this change needs no secrets.
- `ARCHITECTURE.md` records the current standalone plugin identity, namespaced `x11_*` tools, optional source overlay, and no global default behavior changes.
- `CONTEXT.md` now distinguishes `Linux Feature adapter` from `Backend flavor route`.

## Decisions Evaluated

- Whether to create a durable ADR choosing the backend flavor route: rejected for this change because the route is only a future evaluation path, not an accepted architecture.
- Whether to supersede ADR 0009 or ADR 0010: rejected because this change preserves their boundaries.
- Whether to update `ARCHITECTURE.md`: rejected because the current architecture snapshot does not change; only maintainer-facing adapter guidance is clarified.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None.

## No ADR Needed

- No durable ADR is needed because this change does not choose a hard-to-reverse runtime architecture, does not move behavior into `agent-sh/computer-use-linux`, and does not supersede existing standalone/upstream separation decisions. It only documents a future evaluation route and adds tests to keep the existing adapter handoff aligned with issue #389.
