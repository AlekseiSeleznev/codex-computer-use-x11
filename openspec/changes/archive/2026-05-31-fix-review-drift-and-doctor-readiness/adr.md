# ADR Review — fix-review-drift-and-doctor-readiness

## Existing In-force ADRs Considered

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — referenced as the overlay architecture basis; currently missing and must be restored as tracked context.
- `adr/0003-formalize-project-context-entrypoints.md` — referenced for `CONSTITUTION.md`, `ARCHITECTURE.md`, OpenSpec bridge, and local-secret boundaries; currently missing and must be restored.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — referenced for mandatory grill/TDD gates; currently missing and must be restored.
- `adr/0006-adopt-claude-artifact-review.md` — referenced for optional Claude artifact review; currently missing and must be restored.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — referenced for automatic checkpoint/session review controls; currently missing and must be restored.
- `adr/0008-adopt-x11-root-coordinate-model.md` — tracked and remains in force.
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — tracked and remains in force.

## Superseded Historical ADRs Considered

- `adr/0002-adopt-project-constitution-preflight.md` — referenced as superseded by ADR 0003; currently missing and must be restored as historical context.
- `adr/0004-adopt-mandatory-review-and-test-plan-gates.md` — referenced as superseded by ADR 0005; currently missing and must be restored as historical context.

## Grill and Design-Review Findings Considered

- Restore referenced ADR files instead of deleting architecture/index references.
- Redact environment-derived ydotool socket paths in serialized diagnostics while preserving useful labels and the public `/tmp/.ydotool_socket` fallback.
- Compute focus readiness from verified X11/EWMH prerequisites; do not leave implemented v1 window focus stale/false.
- Keep strict clippy as review-remediation evidence rather than a new default `Makefile` gate.
- Replace release checklist commands that only work before archive with durable post-archive validation commands.
- Avoid broad Markdown link checking; target the specific illustrative-link drift.

## Decisions Evaluated by This Change

### Restore ADR traceability

- **Options:** remove missing ADR references; create one new ADR that summarizes all older decisions; restore/reconstruct the referenced ADR files.
- **Decision:** restore/reconstruct the referenced top-level ADR files 0001-0007.
- **Rationale:** `ARCHITECTURE.md` and `adr/README.md` already define these as the durable history. Restoring files repairs traceability without changing accepted architecture.

### Redact ydotool socket diagnostics

- **Options:** serialize raw socket paths; drop candidate details entirely; serialize stable labels for private env-derived paths.
- **Decision:** serialize stable labels and keep connectability booleans/details.
- **Rationale:** preserves diagnostic utility and follows no-secret/no-private-local-value posture.

### Refresh doctor readiness/capabilities

- **Options:** keep bootstrap false/planned placeholders; set broad success booleans unconditionally; derive capability facts from finalized v1 prerequisites.
- **Decision:** derive focus/window readiness from EWMH prerequisites and move implemented v1 capabilities from `planned` to `implemented`.
- **Rationale:** keeps additive shape while reporting the implemented state truthfully.

### Clippy gate scope

- **Options:** ignore strict clippy warnings; add clippy to Makefile; clean warnings while treating clippy as explicit remediation evidence.
- **Decision:** clean warnings without changing the default verification policy.
- **Rationale:** avoids a constitution-level gate change in a review-remediation change.

## Durable ADR Files Created or Restored Here

This change is expected to add tracked files:

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md`
- `adr/0002-adopt-project-constitution-preflight.md`
- `adr/0003-formalize-project-context-entrypoints.md`
- `adr/0004-adopt-mandatory-review-and-test-plan-gates.md`
- `adr/0005-adopt-matt-grill-and-tdd-gates.md`
- `adr/0006-adopt-claude-artifact-review.md`
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md`

These files are traceability restorations for decisions already referenced by the current architecture snapshot/index, not new architecture decisions introduced by this remediation.

## ADRs Superseded by This Change

None. This change must not rewrite or supersede ADR 0008 or ADR 0009. Supersession relationships already described by the index are restored as historical ADR metadata: ADR 0003 supersedes ADR 0002, and ADR 0005 supersedes ADR 0004.

## Rationale for No New Durable ADR

No new durable ADR is required because the remediation does not alter the system model, backend identity, source-overlay strategy, checkpoint policy, Claude-review policy, coordinate model, or final Cinnamon/X11 v1 baseline. It restores missing durable records and fixes implementation/documentation drift against already-accepted decisions.

## Architecture Snapshot Updates Required

No new architecture snapshot decision is required. `ARCHITECTURE.md` may remain semantically unchanged after referenced ADR files are restored. If apply discovers a referenced ADR cannot be restored consistently, apply must stop before changing architecture semantics and ask for a decision.

## Open Questions

None.
