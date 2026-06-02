## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — in force; this change follows OpenSpec as source of truth and keeps project-local Codex workflow artifacts in Git.
- `adr/0003-formalize-project-context-entrypoints.md` — in force; root `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and `adr/` were read before lifecycle actions.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — in force; `grill.md`, `design-review.md`, and `test-plan.md` are mandatory and TDD is required for apply.
- `adr/0006-adopt-claude-artifact-review.md` — in force but session review is disabled by local session state per user `claude off`; no global review config change.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — in force; safe lifecycle checkpoints are automatic and have been used for each artifact.

## Constitution / Architecture Rules Considered

- Rust 2021/Cargo root crate remains the implementation stack.
- `make fmt`, `make check`, `make test`, and OpenSpec validation are required before completion.
- No real secrets are needed or read; `.secrets.local.env` remains untouched.
- Canonical backend id remains `x11-ewmh`.
- Source overlay target checkout `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` is research/read-only for this change.
- Targeted input must use focus verification as the safety boundary; no global/unverified input fallback is added to safe commands.

## Decisions Evaluated

- **Standalone plugin keyboard tools vs source overlay now:** choose standalone tools because backlog/00 defers source overlay and target `server.rs` already has a similar safety pattern for future integration.
- **Active-context `xdotool` vs `xdotool --window`:** choose active-context after verified focus because direct-window events use XSendEvent and are not a reliable safety boundary.
- **Global/unverified development input fallback:** rejected for this capability; missing target or verification failure must refuse input.
- **Durable ADR need:** rejected because this change applies already-recorded architecture rather than introducing a hard-to-reverse architecture change.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None. `ARCHITECTURE.md` already describes active-window focus verification and safe targeted input invariants at the project level; no snapshot change is required.

## No ADR Needed

- No durable ADR is needed because the key decision is an implementation-stage application of the existing verify-before-inject invariant and standalone-before-source-overlay strategy. The trade-offs are captured in this change's `design.md` and `design-review.md`, and no current in-force ADR is changed or superseded.
