# 0001 — Adopt Codex-native intent-driven OpenSpec overlay

## Status

Accepted

## Date

2026-05-30

## Context

The project needs a repeatable way for Codex to run intent-driven development with OpenSpec as the lifecycle engine. The OpenSpec CLI owns change state, validation, and archive mechanics, but it does not read project-local Codex prompts, skills, root constitution files, or local secret policy.

Without a Codex-native overlay, lifecycle rules, artifact gates, and repository-specific safety constraints would remain scattered across chat history or manual operator memory.

## Decision

Adopt a project-local Codex/OpenSpec overlay under `.codex/prompts` and `.codex/skills` as the Codex-facing workflow layer for intent-driven OpenSpec projects.

The overlay owns Codex prompts/skills for OpenSpec lifecycle actions, project-context preflight, quality gates, and Git discipline guidance. The OpenSpec CLI remains the source of truth for lifecycle state, artifact dependency ordering, spec validation, and archive mechanics.

## Considered Options

1. **Project-local Codex/OpenSpec overlay** (chosen)
   - Keeps workflow behavior versioned with the repository.
   - Lets Codex enforce project rules that the OpenSpec CLI cannot read directly.

2. **Rely on globally installed OpenSpec prompts only**
   - Rejected because project-specific gates and safety rules would not be durable in the repository.

3. **Use chat-only instructions**
   - Rejected because future sessions would lose lifecycle and safety context.

## Consequences

- Codex must read project context before lifecycle work and use overlay skills/prompts rather than ad-hoc workflow memory.
- Overlay files become part of the project's trusted development surface and must be validated after updates.
- OpenSpec artifacts remain distinct from Codex-layer project context.
- ADR 0007 later narrows this ADR only for ordinary safe local lifecycle checkpoint commits.

## Evidence

- Current root context: `CONSTITUTION.md`, `ARCHITECTURE.md`, and `adr/README.md`.
- OpenSpec overlay paths: `.codex/prompts/`, `.codex/skills/`.
