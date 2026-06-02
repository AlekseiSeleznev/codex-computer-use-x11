## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — Accepted; remains in force for OpenSpec/Codex overlay workflow.
- `adr/0003-formalize-project-context-entrypoints.md` — Accepted; remains in force for `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and ADR preflight.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — Accepted; remains in force for `grill.md`, `design-review.md`, and TDD apply discipline.
- `adr/0006-adopt-claude-artifact-review.md` — Accepted; remains in force; session state has Claude review disabled for this run.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — Accepted; remains in force for safe local lifecycle checkpoints, while push was explicitly approved by the user.
- `adr/0008-adopt-x11-root-coordinate-model.md` — Accepted; remains in force but is not directly changed by this AT-SPI doctor fix.
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — Accepted; remains in force. The change preserves safe degraded AT-SPI semantics and the `x11-ewmh` baseline.
- `adr/0010-adopt-x11-provider-takeover-shim.md` — Accepted; remains in force but is not directly changed by this doctor runtime fix.
- `adr/0011-adopt-rollback-first-install-manifest.md` — Accepted; remains in force. The change preserves sanitized `NO_AT_BRIDGE`, `GTK_MODULES`, and accessibility setup facts but does not alter installer rollback state.

Superseded ADRs considered for history only:

- `adr/0002-adopt-project-constitution-preflight.md` — Superseded by ADR 0003.
- `adr/0004-adopt-mandatory-review-and-test-plan-gates.md` — Superseded by ADR 0005.

## Constitution / Architecture Rules Considered

- `CONSTITUTION.md` requires Rust 2021/Cargo, root `Makefile` verification via `make fmt`, `make check`, and `make test`, and machine-readable `doctor --json` validation before marking related tasks complete.
- `CONSTITUTION.md` and `ARCHITECTURE.md` require no real secrets in tracked files or outputs; this change does not require `.secrets.local.env`.
- `ARCHITECTURE.md` defines the standalone plugin, `x11-ewmh` backend identity, AT-SPI as a thin desktop boundary, and degraded diagnostics on absence or ambiguity.
- ADR 0009 requires AT-SPI correlation to remain confidence-scored and degraded on absence/ambiguity rather than returning arbitrary subtrees.
- ADR 0011 requires supported accessibility environment values such as `NO_AT_BRIDGE`, `GTK_MODULES`, and `QT_ACCESSIBILITY` to be treated as non-secret rollback/setup facts; this change preserves those facts as diagnostics.

## Decisions Evaluated

- **Run doctor's bounded collector even when `NO_AT_BRIDGE=1` is present.** Chosen for this change because observed collector success must override an environment-only prediction in the doctor readiness surface.
- **Keep presence-based `NO_AT_BRIDGE=1` as a hard doctor short-circuit.** Rejected because it reproduces the v0.1.1 bug: doctor degrades while `accessibility-tree` proves tree extraction works.
- **Make doctor require a target window to prove AT-SPI availability.** Rejected because it would change the zero-argument doctor/MCP readiness contract and conflate ambient tree extraction availability with target-specific correlation.
- **Create a durable ADR for collector-success precedence over bridge-env hints.** Rejected because this is a localized corrective bugfix under existing AT-SPI/readiness architecture, not a hard-to-reverse or broad architecture decision.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None. `ARCHITECTURE.md` already describes doctor/accessibility as runtime diagnostics with degraded AT-SPI semantics; no durable architecture snapshot change is needed.

## No ADR Needed

- No durable ADR is needed because the change corrects an implementation/spec mismatch in doctor AT-SPI probing. It preserves the existing architecture: doctor remains non-invasive, AT-SPI remains optional semantic enrichment with safe degraded outcomes, `NO_AT_BRIDGE` remains sanitized diagnostic/setup context, and `accessibility-tree` remains the target-specific correlation path.
