## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — listed as in force by `ARCHITECTURE.md` / `adr/README.md`; body file is absent in this checkout. This change follows OpenSpec as source of truth and keeps project-local lifecycle artifacts in Git.
- `adr/0003-formalize-project-context-entrypoints.md` — listed as in force; body file is absent. Root `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and `adr/README.md` were read before lifecycle actions.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — listed as in force; body file is absent. `grill.md`, `design-review.md`, and `test-plan.md` remain mandatory and TDD is required for apply.
- `adr/0006-adopt-claude-artifact-review.md` — listed as in force; body file is absent. Session review is disabled by local session state per user `claude off`; no global review config change.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — listed as in force; body file is absent. Safe lifecycle checkpoints are automatic and have been used for each completed artifact.

## Constitution / Architecture Rules Considered

- Rust 2021/Cargo root crate remains the implementation stack.
- `make fmt`, `make check`, `make test`, and OpenSpec validation are required before completion.
- No real secrets are needed or read; `.secrets.local.env` remains untouched.
- Canonical backend id remains `x11-ewmh`.
- Source overlay target checkout `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` is research/read-only for this change.
- Targeted input uses focus verification as the safety boundary; explicit global pointer mode must be marked `global_unverified` and not reported as window-isolated targeting.
- Standalone plugin keeps project-owned `x11_*` MCP names and does not replace bundled Computer Use stock tool names.

## Decisions Evaluated

- **Standalone pointer tools vs source overlay now:** choose standalone tools because backlog/00 defers source overlay and target `server.rs` already has stock pointer tool semantics for later integration.
- **`xdotool` standalone backend vs importing target `abs_pointer`:** choose `xdotool` for the standalone plugin because it is command-testable through fake `PATH`, reversible, and avoids copying target internals.
- **Targeted-only pointer actions vs explicit global mode:** allow global mode only with explicit marker and degraded reporting because backlog/07b asks for global/unverified reporting; missing target without that marker remains a safe refusal.
- **Bounds validation before focus:** accepted to avoid focus side effects when geometry is already unsafe.
- **Durable ADR need:** rejected because this change applies existing standalone-before-source-overlay and verify-before-inject architecture rather than introducing a hard-to-reverse project-wide decision.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None. `ARCHITECTURE.md` already describes the relevant lifecycle, standalone plugin, source overlay, and safe targeted input boundaries. No durable architecture snapshot change is required.

## No ADR Needed

- No durable ADR is needed because the selected approach is local to the standalone plugin, reversible, and expected by the backlog sequence. The material trade-offs are captured in `design.md` and `design-review.md`; no in-force architecture decision is changed or superseded.
