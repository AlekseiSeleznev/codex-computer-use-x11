## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — in force through the architecture snapshot; this change follows the project-local OpenSpec/Codex overlay lifecycle.
- `adr/0003-formalize-project-context-entrypoints.md` — in force; this change reads and preserves `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and local-secret boundaries.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — in force; this change completed grill/design-review and will use vertical docs-check TDD slices.
- `adr/0006-adopt-claude-artifact-review.md` — in force as optional review architecture; session state has Claude review disabled per user request, so skipped review reports are non-blocking.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — in force; safe lifecycle checkpoint commits are automatic, while archive/push require explicit approval already provided by the user request.
- `adr/0008-adopt-x11-root-coordinate-model.md` — in force; documentation must preserve X11 root/global coordinates for bounds, pointer points, and screenshot crop rectangles.

## Constitution / Architecture Rules Considered

- Required stack remains Rust 2021/Cargo with root `Makefile` verification commands (`make fmt`, `make check`, `make test`).
- OpenSpec remains the source of truth and must validate changed artifacts before apply/archive.
- Git-tracked files must not contain real secrets; `.secrets.local.env` is not read or committed.
- Source overlay work must respect `CODEX_DESKTOP_LINUX_FULL_PATH` as a variable name and treat the local target path as machine-specific.
- `ARCHITECTURE.md` already captures the standalone/source-overlay split, optional Claude review/session controls, and ADR 0008 coordinate model.
- `CONTEXT.md` terms added during grill (`Upstream target matrix`, `Runtime command dependency`, `Release checklist`) are glossary terms only, not architecture rules.

## Decisions Evaluated

- **Documentation topology:** Use README plus focused docs under `docs/`. This is a documentation organization decision, easy to revise, and not a durable architecture boundary.
- **Docs-check test approach:** Add behavior-focused integration tests for stable public doc contracts. This follows ADR 0005/TDD discipline but does not introduce a new architectural pattern.
- **Upstream target matrix:** Document the existing split between backend/windowing lineage and Codex Desktop wrapper/integration lineage. This applies current architecture; it does not create a new integration boundary.
- **License posture:** Document existing reference-first/no-copy policy and runtime-command distinction. This is compliance documentation for the current change, not a new hard-to-reverse reuse decision.
- **NOTICE file:** Rejected for this change because no external source code or bundled asset is copied/vendored.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None. `ARCHITECTURE.md` does not need an update because runtime boundaries and durable decisions are unchanged.

## No ADR Needed

- No durable ADR is needed. The change adds documentation and public-interface docs checks that apply existing architecture, source-overlay/plugin contracts, license/reuse policy, and ADR 0008 coordinate wording. The decisions are reversible documentation/test organization choices rather than hard-to-reverse architecture trade-offs.
