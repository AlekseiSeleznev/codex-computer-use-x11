# 0002 — Adopt project constitution preflight

## Status

Superseded by ADR 0003

## Date

2026-05-30

## Context

Early lifecycle work needed a durable place for project rules that Codex must read before OpenSpec workflows. OpenSpec changes and specs are archived over time, but repository-wide rules such as secret handling, required verification commands, and external-system policy must persist outside individual changes.

## Decision

Adopt a root project constitution preflight: before substantial OpenSpec or implementation work, Codex reads the root project constitution and follows its rules for technology, verification, external systems, and secret handling.

## Considered Options

1. **Root project constitution preflight** (chosen at the time)
   - Makes repository-wide rules durable and discoverable.
   - Keeps secret policy outside change artifacts.

2. **Put all project policy in each OpenSpec change**
   - Rejected because policy would drift and archive with each change.

3. **Use only `AGENTS.md` or chat instructions**
   - Rejected because the OpenSpec workflow needed a project-owned, Git-tracked rule source.

## Consequences

- Codex checks constitution rules before OpenSpec lifecycle work.
- Secret values are excluded from tracked artifacts and outputs.
- This decision's intent is carried forward and broadened by ADR 0003, which formalizes the complete project-context entrypoint model.

## Supersession

ADR 0003 supersedes this ADR by formalizing `CONSTITUTION.md`, `ARCHITECTURE.md`, the OpenSpec bridge, ADR-derived architecture snapshots, and local-secret boundaries as a broader context model.

## Evidence

- Current root `CONSTITUTION.md` preserves the preflight rule.
- `adr/README.md` records this ADR as superseded by ADR 0003.
