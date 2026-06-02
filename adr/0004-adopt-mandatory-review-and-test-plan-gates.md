# 0004 — Adopt mandatory review and test-plan gates

## Status

Superseded by ADR 0005

## Date

2026-05-30

## Context

Intent-driven changes were at risk of moving from proposal/specs directly into implementation without explicit review or test planning. That made it too easy to miss unresolved questions, design contradictions, or untested behavior.

## Decision

Require review and test-plan gates in the OpenSpec lifecycle before implementation. Changes must include review evidence and a test plan before tasks/apply proceed.

## Considered Options

1. **Mandatory review and test-plan gates** (chosen at the time)
   - Makes implementation readiness explicit.
   - Creates a place to capture verification expectations before code changes.

2. **Optional review/test planning**
   - Rejected because high-risk changes could skip the very gates meant to reduce ambiguity.

3. **Rely only on post-implementation verification**
   - Rejected because it discovers design uncertainty too late.

## Consequences

- Planning artifacts must include review and test expectations before apply.
- Apply is blocked when review/test-plan artifacts are missing.
- ADR 0005 supersedes this ADR by specifying Matt `grill-with-docs` and canonical TDD gates.

## Supersession

ADR 0005 supersedes this ADR. The mandatory gate intent is carried forward as `grill.md`, `design-review.md`, and strict TDD evidence.

## Evidence

- Current OpenSpec lifecycle lists `grill`, `design-review`, `adr`, `test-plan`, and `tasks` before apply.
