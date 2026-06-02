# 0003 — Formalize project context entrypoints

## Status

Accepted

## Date

2026-05-30

## Context

ADR 0002 introduced constitution preflight, but lifecycle work also needs stable entrypoints for architecture state, glossary language, OpenSpec bridge documentation, and durable ADR history. The OpenSpec CLI does not read these files; Codex must use them as project context before acting.

## Decision

Formalize root project context entrypoints:

- `CONSTITUTION.md` contains persistent project rules, technology choices, verification rules, external-system policy, and secret handling.
- `ARCHITECTURE.md` is the current architecture snapshot for new chats and architecture-sensitive work.
- `CONTEXT.md` or `CONTEXT-MAP.md` holds glossary/domain language only.
- `adr/` stores durable decision rationale and supersession history.
- `openspec/README.md` and OpenSpec artifacts bridge lifecycle state and canonical behavior specs.

Secret values remain local-only in `.secrets.local.env` or the local environment and must not be printed, staged, committed, archived, or copied into tracked artifacts.

## Considered Options

1. **Separate root context entrypoints** (chosen)
   - Lets each file serve one purpose and keeps architecture rationale discoverable.

2. **Single omnibus project document**
   - Rejected because policy, glossary, architecture, and ADR history have different update rules.

3. **OpenSpec-only project context**
   - Rejected because OpenSpec changes archive over time and the CLI does not enforce local secret or Codex preflight rules.

## Consequences

- Codex reads `CONSTITUTION.md` before OpenSpec lifecycle actions.
- Architecture-sensitive work reads `ARCHITECTURE.md`, `adr/README.md`, and relevant ADRs.
- Glossary updates stay in `CONTEXT.md`/`CONTEXT-MAP.md`, not in architecture or policy files.
- ADR 0002 is superseded; its constitution preflight intent is preserved under this broader model.

## Evidence

- Root `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`.
