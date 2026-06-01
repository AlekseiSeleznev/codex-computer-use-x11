## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — listed as in force by `ARCHITECTURE.md` / `adr/README.md`; body file is absent in this checkout. This change follows OpenSpec as source of truth and keeps project-local lifecycle artifacts in Git.
- `adr/0003-formalize-project-context-entrypoints.md` — listed as in force; body file is absent. Root `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and `adr/README.md` were read before lifecycle actions.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — listed as in force; body file is absent. `grill.md`, `design-review.md`, and `test-plan.md` remain mandatory and TDD is required for apply.
- `adr/0006-adopt-claude-artifact-review.md` — listed as in force; body file is absent. Session review is disabled by local session state per user `claude off`; no global review config change.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — listed as in force; body file is absent. Safe lifecycle checkpoints are automatic and have been used for each completed artifact.

## Constitution / Architecture Rules Considered

- Rust 2021/Cargo root crate remains the implementation stack; Python GI is only a bounded local desktop collector boundary analogous to existing external command backends.
- `make fmt`, `make check`, `make test`, and OpenSpec validation are required before completion.
- No real secrets are needed or read; `.secrets.local.env` remains untouched.
- Canonical backend id remains `x11-ewmh`.
- Source overlay target checkout `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` is research/read-only for this change.
- AT-SPI accessibility trees are semantic read context, not a replacement for focus verification as the input safety boundary.
- Standalone plugin keeps project-owned `x11_*` MCP names and does not replace bundled Computer Use stock tool names.

## Decisions Evaluated

- **Standalone CLI/MCP now vs source overlay now:** choose standalone `accessibility-tree` / `x11_accessibility_tree` because backlog ordering keeps the target checkout read-only until source-overlay work resumes.
- **Rust `atspi` dependency now vs Python GI collector boundary:** choose Python GI collector boundary for the standalone stage to keep the crate lightweight and command-testable. The pure matcher/report contract remains portable to target Rust `atspi` later.
- **PID-only matching vs multi-signal confidence:** choose multi-signal scoring because browser and terminal windows often violate naive PID assumptions, and current listing records PID reliability separately.
- **Best-effort low-confidence tree vs empty ambiguous/degraded result:** choose safe refusal with diagnostics because a wrong semantic tree could mislead later actions.
- **Window-id-only interface vs broad target selectors:** choose concrete `window_id` for this stage to avoid duplicating target selector ambiguity in a semantic read primitive.
- **Durable ADR need:** rejected because the choices are local, reversible, and apply existing standalone/degraded-reporting architecture rather than changing project-wide direction.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None. `ARCHITECTURE.md` already captures the relevant lifecycle, standalone plugin, source overlay, no-secrets, and safe verification boundaries. This change does not alter the current architecture snapshot.

## No ADR Needed

- No durable ADR is needed because this change is an incremental standalone capability with reversible implementation details. The material trade-offs are captured in `design.md` and `design-review.md`; no in-force architecture decision is changed or superseded.
