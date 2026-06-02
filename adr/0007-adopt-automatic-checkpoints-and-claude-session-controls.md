# 0007 — Adopt automatic checkpoints and Claude session controls

## Status

Accepted

## Date

2026-05-30

## Context

The OpenSpec lifecycle creates many coherent planning and implementation checkpoints. Requiring manual approval for every safe local lifecycle commit slows work and conflicts with the repository's preference for small durable checkpoints. At the same time, risky operations such as push, merge, archive, destructive Git actions, and hard-gate bypasses still require explicit approval. Claude artifact review also needs per-session enable/disable controls that do not modify tracked policy.

## Decision

Adopt scoped automatic local checkpoint commits for safe OpenSpec lifecycle artifacts and coherent implementation groups when session git discipline is `auto`. Keep explicit approval requirements for push, merge, pull request creation, archive, destructive Git operations, dirty unrelated work, and hard-gate bypasses.

Adopt ignored non-secret session state under `.codex/session/openspec-session.json` for per-session Claude review controls and other session-scoped workflow settings. Session state may disable or overlay Claude review stages without storing credentials.

This narrows ADR 0001 only for ordinary safe local lifecycle checkpoint commits; ADR 0001 remains otherwise in force.

## Considered Options

1. **Automatic safe local lifecycle checkpoints plus explicit approval for risky operations** (chosen)
   - Preserves granular history and avoids repeated low-risk prompts.
   - Keeps high-risk Git and lifecycle operations under user control.

2. **Manual approval for every commit**
   - Rejected because it makes long OpenSpec workflows unnecessarily brittle.

3. **Fully automatic archive/push/merge**
   - Rejected because those operations cross hard safety and collaboration boundaries.

4. **Tracked session controls**
   - Rejected because per-session state should not change repository policy or contain local/auth data.

## Consequences

- Codex can commit safe lifecycle artifacts automatically in `auto` mode after showing status and affected paths.
- Archive, push, merge, PR creation, destructive operations, dirty unrelated work, and gate bypasses still need explicit approval.
- Claude review can be disabled for a run without editing tracked reviewer configuration.

## Evidence

- `scripts/openspec-session-state`.
- `scripts/openspec-git-checkpoint`.
- `.gitignore` rules for `.codex/session/`.
- Current `CONSTITUTION.md` Git discipline rules.
