# Tasks — fix-review-drift-and-doctor-readiness

## Preconditions

- [x] Proposal/specs/grill/design/design-review/adr/test-plan are complete and checkpointed.
- [x] Claude review is disabled for this run per user request.
- [x] Constitution/architecture/ADR index were read before planning.
- [x] Git status was clean before implementation planning.

## Implementation

- [x] T1 ADR traceability test-first slice
  - [x] Add or update final DoD/architecture tests to fail when `ARCHITECTURE.md` or `adr/README.md` references a missing top-level ADR path.
  - [x] Record RED evidence naming missing ADR paths.
  - [x] Add restored tracked ADR files 0001-0007 with status, context, decision, consequences, and supersession notes matching current architecture/index.
  - [x] Run focused final DoD/ADR tests and `scripts/validate-final-dod.py` for GREEN.

- [x] T2 Doctor capability/focus test-first slice
  - [x] Update focused doctor tests for implemented finalized-v1 capability names, no stale `x11-ewmh-windowing` planned placeholder, complete-fixture focus readiness true, and degraded/no-display readiness false.
  - [x] Record RED evidence against current stale doctor output.
  - [x] Update `src/doctor.rs` capability lists and focus readiness computation.
  - [x] Run focused doctor unit/integration tests for GREEN.

- [x] T3 Doctor ydotool privacy test-first slice
  - [x] Add a focused test that proves raw `YDOTOOL_SOCKET` and `XDG_RUNTIME_DIR` candidate paths are not serialized and stable labels are present.
  - [x] Record RED evidence against current live gather/serialization behavior.
  - [x] Refactor live ydotool candidate gathering to separate real probe path from serialized label.
  - [x] Run focused privacy tests for GREEN.

- [x] T4 Documentation drift test-first slice
  - [x] Update docs tests to reject archived active-change validation in the release checklist and broken illustrative local Markdown links in `CONTEXT-FORMAT.md`.
  - [x] Record RED evidence against current docs.
  - [x] Update `README.md`, `docs/release-checklist.md`, and `.codex/skills/grill-with-docs/CONTEXT-FORMAT.md`.
  - [x] Run focused docs tests for GREEN.

- [x] T5 Strict clippy cleanup slice
  - [x] Run strict clippy and record current RED warning summary.
  - [x] Apply local refactors or narrow helper-specific allows for current warnings.
  - [x] Run strict clippy for GREEN.

## Verification

- [x] Run `make fmt`.
- [x] Run `make check`.
- [x] Run `make test`.
- [x] Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] Run `openspec validate fix-review-drift-and-doctor-readiness --type change --strict`.
- [x] Run `openspec validate --all --strict`.
- [x] Run `scripts/check-overlay`.
- [x] Run `scripts/validate-final-dod.py`.
- [x] Confirm `git status --short` has only intended tracked changes before the final checkpoint.

## Safety / Handoff

- [x] Confirm no real secrets or private local values were added to tracked files or outputs.
- [x] Commit implementation and verification evidence in coherent checkpoint(s).
- [x] Stop before archive unless the user explicitly approves archive.
