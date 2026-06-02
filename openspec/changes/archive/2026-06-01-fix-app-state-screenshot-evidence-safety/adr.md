## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — accepted and in force for OpenSpec/Codex overlay lifecycle.
- `adr/0003-formalize-project-context-entrypoints.md` — accepted and in force for `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and local-secret boundaries.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — accepted and in force for mandatory grill/design-review and TDD apply gates.
- `adr/0006-adopt-claude-artifact-review.md` — accepted and in force, with current session review disabled by session state.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — accepted and in force for scoped automatic lifecycle checkpoints when permitted.
- `adr/0008-adopt-x11-root-coordinate-model.md` — accepted and in force. This change must preserve X11 root/global coordinates and path-oriented screenshot crop evidence.
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — accepted and in force. This change remains inside the Cinnamon/X11 `x11-ewmh` baseline and preserves verified input/fixture/degraded evidence safety.
- `adr/0010-adopt-x11-provider-takeover-shim.md` — accepted and in force. This change does not alter standalone plugin identity, bundled rollback, provider takeover, or `x11_*` MCP tool names.

## Constitution / Architecture Rules Considered

- Rust 2021/root Cargo/Makefile verification remains required: `make fmt`, `make check`, and `make test`.
- OpenSpec validation remains required: `openspec validate --all --strict`.
- Secret handling remains unchanged: do not read, print, stage, commit, archive, or copy `.secrets.local.env` or real secrets.
- Scope remains Cinnamon/X11 baseline only; Wayland and portal-required runtime paths remain out of scope.
- `ARCHITECTURE.md` currently says ADR 0008 defines root/global screenshot/window context composition and ADR 0009 defines Cinnamon/X11 v1 baseline. This change refines implementation evidence safety without changing the snapshot.
- Existing docs/evidence guidance already prefers screenshot files/paths over huge inline data URLs for release evidence.

## Decisions Evaluated

- **Default app-state screenshot behavior:** Change default JSON from inline data URL to path-oriented PNG artifact metadata. Chosen because machine-readable evidence must be safe by default and ADR 0008 already favors path-oriented screenshot evidence.
- **Screenshot capture default:** Keep screenshot capture enabled by default when requested by existing CLI/MCP defaults, but write to generated/caller-supplied paths. Chosen to avoid a larger compatibility change while removing inline payloads.
- **Inline compatibility:** Allow retaining inline mode only behind explicit unsafe opt-in. This is an implementation compatibility choice, not a durable architecture shift.
- **Fixture identity:** Rename/rework real-live fixture titles/classes to neutral run-scoped values and prove ownership through metadata. Chosen to avoid filters that exclude project-owned/overlay-looking `Codex` windows.
- **Durable ADR need:** Rejected creating a new durable ADR because this change does not supersede ADR 0008/0009/0010 and does not introduce a new architecture direction.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None required. `ARCHITECTURE.md` remains accurate: ADR 0008 and ADR 0009 already cover screenshot evidence safety and Cinnamon/X11 pass/degraded boundaries.

## No ADR Needed

- No new durable ADR is needed because this change is a bugfix/hardening implementation within existing durable decisions: ADR 0008's coordinate/path-oriented screenshot posture, ADR 0009's Cinnamon/X11 evidence/degraded baseline, and ADR 0010's standalone identity/provider boundary remain intact.
