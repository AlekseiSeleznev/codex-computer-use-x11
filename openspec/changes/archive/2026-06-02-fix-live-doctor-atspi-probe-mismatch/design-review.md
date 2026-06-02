## Context Read

- `openspec/changes/fix-live-doctor-atspi-probe-mismatch/proposal.md`
- `openspec/changes/fix-live-doctor-atspi-probe-mismatch/specs/doctor-cli/spec.md`
- `openspec/changes/fix-live-doctor-atspi-probe-mismatch/specs/x11-atspi-window-correlation/spec.md`
- `openspec/changes/fix-live-doctor-atspi-probe-mismatch/grill.md`
- `openspec/changes/fix-live-doctor-atspi-probe-mismatch/design.md`
- `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, ADR 0009, ADR 0011.
- Relevant code/tests: `src/doctor.rs`, `src/accessibility.rs`, `tests/doctor_cli.rs`, `tests/accessibility_tree_cli.rs`.

## Design Summary

- The design keeps doctor as a read-only ambient readiness probe, not a target-specific correlation command.
- The key implementation claim is that doctor and accessibility-tree must share the same collector success contract for `ok=true` plus candidates.
- Bridge-disabled `NO_AT_BRIDGE=1` remains an explicit degraded branch and does not run the probe.
- Tests must protect both fake deterministic behavior and live-safe consistency when X11/AT-SPI are available.

## Question Loop

1. **Could the fix accidentally make doctor claim target-window AT-SPI success from ambient candidates?**
   - **Recommended answer**: Avoid that by wording/code that says `tree_available`, not `target_matched`, unless a controlled fixture pass is explicitly observed.
   - **Rationale**: Existing glossary separates `Accessibility tree` and `AT-SPI window correlation`; doctor is not selecting arbitrary subtrees.
   - **Resolution**: No spec/design update required; current text already distinguishes ambient tree availability from target correlation.

2. **Could timeout handling be removed to make live doctor pass?**
   - **Recommended answer**: No. Keep bounded doctor probes; fix divergent success parsing or timeout wrapper behavior while preserving hung-command tests.
   - **Rationale**: Doctor must remain suitable for smoke tests and not hang indefinitely on desktop integration probes.
   - **Resolution**: Test plan must include existing timeout regression plus the new positive probe regression.

3. **Does this require updating `CONTEXT.md` or `ARCHITECTURE.md`?**
   - **Recommended answer**: No. This is an implementation correction under existing terms and architecture.
   - **Rationale**: The architecture already says AT-SPI is a thin boundary with degraded diagnostics; no new durable concept or boundary is introduced.
   - **Resolution**: No context/architecture update required.

## Design Findings

- The design is consistent with the non-invasive doctor contract: no secrets, screenshots, input, target checkout writes, or external credentials.
- The design preserves ADR 0009 by keeping AT-SPI degraded evidence truthful without making it required for X11 window/input readiness.
- The design should add a regression that fails before implementation at a public boundary, not only a unit-level mapping assertion.
- The implementation should expose or preserve enough internal detail in tests to distinguish `collector timed out`, `collector ok=false`, and `collector parse failed`; otherwise future live mismatches will be hard to triage.

## Document Updates Applied

None. The specs and design already cover the review findings.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

None. No new hard-to-reverse architecture decision was identified.

## Open Questions

None.
