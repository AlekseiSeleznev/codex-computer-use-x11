## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — in force; this change follows the OpenSpec lifecycle and does not alter overlay architecture.
- `adr/0003-formalize-project-context-entrypoints.md` — in force; root `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and ADR context were read and remain outside OpenSpec artifacts.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — in force; grill/design-review were completed and test planning will use TDD-style verification slices even though the apply work is documentation-only.
- `adr/0006-adopt-claude-artifact-review.md` — in force; Claude review helper was invoked and recorded skipped reports according to current effective configuration.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — in force; safe lifecycle checkpoints are being created automatically under session `auto` mode.

## Constitution / Architecture Rules Considered

- `CONSTITUTION.md` requires OpenSpec validation for changed artifacts, safe checkpoint discipline, no secret leakage, and Rust checks only when Rust changes are made.
- `ARCHITECTURE.md` identifies OpenSpec CLI as lifecycle engine and root context files as persistent project context; this change preserves that boundary.
- No external system or `.secrets.local.env` access is needed.

## Decisions Evaluated

- Edit canonical spec `## Purpose` prose directly during apply instead of relying on archive sync to update metadata prose.
- Keep the change metadata-only, avoiding Rust/runtime/source-overlay edits.
- Verify with OpenSpec validation and text checks for placeholder removal.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None. `ARCHITECTURE.md` does not change because spec Purpose metadata cleanup does not alter architecture, lifecycle, or durable decisions.

## No ADR Needed

- No durable ADR is needed because this change is easy to reverse, unsurprising, and does not involve a hard-to-reverse architectural trade-off. It only replaces placeholder spec metadata with accurate prose and adds narrow metadata regression requirements.
