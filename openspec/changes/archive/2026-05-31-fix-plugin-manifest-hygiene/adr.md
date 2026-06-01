## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — Accepted; remains in force. This change keeps OpenSpec as lifecycle source of truth and uses project-local Codex workflow gates.
- `adr/0003-formalize-project-context-entrypoints.md` — Accepted; remains in force. `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and ADRs were read as Codex-layer context and are not treated as OpenSpec artifacts.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — Accepted; remains in force. `grill.md`, `design-review.md`, and TDD-style test planning are required before apply.
- `adr/0006-adopt-claude-artifact-review.md` — Accepted; remains in force. Claude review is optional auxiliary evidence and is disabled for this session per user instruction/session state.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — Accepted; remains in force. Safe local lifecycle checkpoint commits are allowed in auto mode; push/archive/destructive operations remain approval-gated.
- `adr/0008-adopt-x11-root-coordinate-model.md` — Accepted; remains in force. No coordinate, screenshot, or app-state behavior changes are included.
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — Accepted; remains in force. Standalone plugin tool names stay in the project-owned `x11_*` namespace.

Superseded ADRs considered for historical context:

- `adr/0002-adopt-project-constitution-preflight.md` — Superseded by ADR 0003.
- `adr/0004-adopt-mandatory-review-and-test-plan-gates.md` — Superseded by ADR 0005.

## Constitution / Architecture Rules Considered

- Use Rust 2021/Cargo and root Makefile verification (`make fmt`, `make check`, `make test`) for implementation changes.
- Run OpenSpec validation for changed OpenSpec artifacts.
- Keep real secrets out of tracked files, artifacts, diffs, logs, and chat; this change needs no external systems or secret variables.
- Prefer small visible Git checkpoint boundaries; safe lifecycle checkpoints may be automatic in session auto mode.
- Keep standalone plugin metadata under the existing standalone plugin/source architecture; do not alter runtime integration boundaries.

## Decisions Evaluated

- Whether to create a durable ADR for adding `*.bak.*` to `.gitignore` and correcting generated manifest metadata.
  - Decision: No durable ADR. The change is reversible repository hygiene and copy accuracy, not a hard-to-reverse architecture trade-off.
- Whether to revisit ADR 0009's standalone plugin namespace decision.
  - Decision: No. The change preserves the `x11_*` namespace and only updates user-facing metadata to match it.
- Whether to update `ARCHITECTURE.md`.
  - Decision: No. Runtime architecture, lifecycle architecture, and durable decision state do not change.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None.

## No ADR Needed

- No durable ADR is needed because the work is low-risk maintenance: remove accidental backup artifacts, ignore future timestamped backups, and align generated plugin manifest copy with existing repository/tool facts. It does not alter architecture, external-system policy, security model, coordinate model, or plugin/runtime boundaries.
