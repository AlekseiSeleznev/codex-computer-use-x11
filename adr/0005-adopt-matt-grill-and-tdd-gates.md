# 0005 — Adopt Matt grill and TDD gates

## Status

Accepted

## Date

2026-05-30

## Context

ADR 0004 established that review and test planning are mandatory, but it did not define a sufficiently sharp method for resolving ambiguity or controlling implementation order. The project needs a repeatable way to stress-test plans and require behavior-first implementation evidence.

## Decision

Adopt Matt `grill-with-docs` gates and canonical TDD discipline in the intent-driven OpenSpec lifecycle:

- `grill.md` runs after proposal/specs and before design.
- `design-review.md` runs after design and before ADR/test planning/tasks.
- The grill process reads repository context first, asks one material question at a time only when context cannot answer it, and reaches `Open Questions: None` or records an explicit override.
- `test-plan.md` records vertical RED -> GREEN -> REFACTOR slices.
- Apply follows one behavior-changing TDD slice at a time and records RED/GREEN evidence before marking tasks complete.

## Considered Options

1. **Matt grill plus vertical TDD gates** (chosen)
   - Forces ambiguity resolution before design and implementation.
   - Keeps tests tied to observable behavior rather than speculative horizontal batches.

2. **Generic design review only**
   - Rejected because it does not require one-question-at-a-time ambiguity resolution or glossary/context updates.

3. **Tests after implementation**
   - Rejected because it weakens the safety value of the test plan.

## Consequences

- Every intent-driven change has `grill.md`, `design-review.md`, `adr.md`, and `test-plan.md` before tasks/apply.
- Behavior-changing apply work uses TDD unless the test plan records an explicit approved exception.
- ADR 0004 is superseded; its mandatory gate intent is preserved and sharpened.

## Evidence

- `.codex/skills/grill-with-docs/SKILL.md`.
- `.codex/skills/tdd/SKILL.md`.
- OpenSpec artifact sequence in active changes.
