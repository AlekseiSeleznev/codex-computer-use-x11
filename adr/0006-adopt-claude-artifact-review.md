# 0006 — Adopt Claude artifact review

## Status

Accepted

## Date

2026-05-30

## Context

Some OpenSpec artifacts benefit from an auxiliary reviewer pass that can critique proposal/spec/design/test artifacts without replacing Codex judgment, mandatory grill gates, ADR review, TDD, or OpenSpec validation. The reviewer must not store credentials in tracked files and must be controllable per session.

## Decision

Adopt optional Claude Code print-mode artifact review as an auxiliary Codex-overlay reviewer gate. The project keeps non-secret stage/model configuration in `.codex/openspec-claude-review.json`, writes structured review reports under `openspec/changes/<change>/reviews/` when enabled, and treats those reports as advisory evidence for later gates.

Claude review does not replace OpenSpec artifacts, grill/design-review, ADR review, test-plan, tasks, TDD evidence, or validation. Credentials and authentication remain external local state.

## Considered Options

1. **Optional configured artifact reviewer** (chosen)
   - Adds another review perspective while preserving Codex/OpenSpec authority.
   - Can be disabled per session.

2. **Mandatory external review for every change**
   - Rejected because availability, budget, and local authentication may vary.

3. **No auxiliary reviewer support**
   - Rejected because structured review reports can improve artifact quality when available.

## Consequences

- Review reports are auxiliary artifacts and must not be treated as lifecycle replacements.
- Claude credentials are never stored in tracked config/reports.
- Session controls can disable review when the user requests it or when budget/availability blocks progress.

## Evidence

- `.codex/openspec-claude-review.json`.
- `scripts/openspec-claude-review`.
- Session review controls documented by ADR 0007.
