## Context Read

- `openspec/changes/align-issue-389-backend-flavor-guidance/proposal.md`
- `openspec/changes/align-issue-389-backend-flavor-guidance/specs/x11-release-adapter-handoff/spec.md`
- `openspec/changes/align-issue-389-backend-flavor-guidance/grill.md`
- `openspec/changes/align-issue-389-backend-flavor-guidance/design.md`
- `CONSTITUTION.md`
- `CONTEXT.md`
- `ARCHITECTURE.md`
- `adr/README.md`
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md`
- `adr/0010-adopt-x11-provider-takeover-shim.md`
- `docs/codex-desktop-linux-x11-ewmh-adapter.md`
- `adapters/codex-desktop-linux/linux-features/x11-ewmh-computer-use/README.md`
- `tests/packaging_docs.rs`

## Design Summary

- The design treats `agent-sh/computer-use-linux` backend/flavor integration as a future evaluation route, not current scaffold behavior.
- Implementation scope is documentation plus concept-level tests that lock maintainer-facing boundaries.
- Runtime binaries, MCP tool names, release bundle layout, and default feature state stay unchanged.
- The design preserves ADR 0009/0010 by keeping standalone identity and no global masquerading.

## Question Loop

- Question considered: Are documentation tests enough for this change, or is a runtime test required?
  - Recommended answer: Documentation tests are enough.
  - Rationale: The required behavior is contract clarity and future-path separation; no runtime interface changes. Existing release/scaffold tests already cover runtime staging safety.
  - Resolution: Answered from design scope and current tests; no user question required.

- Question considered: Does the phrase `backend flavor route` conflict with existing glossary terms like `Linux Feature adapter` or `Upstream target matrix`?
  - Recommended answer: No, because it is defined as a separate future route.
  - Rationale: The glossary now distinguishes a selectable backend/flavor evaluation from the copyable Linux Feature adapter, preventing overloaded use of `adapter`.
  - Resolution: Answered from `CONTEXT.md`; no user question required.

## Design Findings

- The design is intentionally narrow and does not require a durable ADR because it records external guidance, not a committed architecture pivot.
- Verification is feasible with existing Rust test style in `tests/packaging_docs.rs` plus OpenSpec validation.
- The future backend/flavor path must not be allowed to imply default feature enablement or core Computer Use modification; tests should assert those concepts explicitly.

## Document Updates Applied

None during design review.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

None. A durable ADR may be appropriate only in a later change if the project chooses to move behavior into `agent-sh/computer-use-linux` or supersede ADR 0009/0010 boundaries.

## Open Questions

None.
