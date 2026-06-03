## Context Read

- `openspec/changes/align-issue-389-backend-flavor-guidance/proposal.md`
- `openspec/changes/align-issue-389-backend-flavor-guidance/specs/x11-release-adapter-handoff/spec.md`
- `openspec/specs/x11-release-adapter-handoff/spec.md`
- `CONSTITUTION.md`
- `CONTEXT.md`
- `ARCHITECTURE.md`
- `adr/README.md`
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md`
- `adr/0010-adopt-x11-provider-takeover-shim.md`
- `docs/codex-desktop-linux-x11-ewmh-adapter.md`
- `adapters/codex-desktop-linux/linux-features/x11-ewmh-computer-use/README.md`
- `adapters/codex-desktop-linux/linux-features/x11-ewmh-computer-use/feature.json`
- Current public GitHub issue #389 comments from `ilysenko/codex-desktop-linux`.

## Plan Summary

- The current adapter path already matches the issue #389 hard boundary: an optional disabled-by-default Linux Feature, not a core Computer Use rewrite and not a default bundled plugin.
- The new maintainer/commenter guidance introduces a second path to evaluate later: `agent-sh/computer-use-linux` selectable backend/flavor integration.
- This change should document the second path without moving implementation into that path or changing runtime behavior.
- The acceptance boundary is documentation/spec/test coverage that prevents future readers from treating the backend/flavor path as required scaffold behavior.

## Question Loop

- Question considered: Should this change pivot the scaffold from a separate plugin adapter to an `agent-sh/computer-use-linux` flavor implementation now?
  - Recommended answer: No.
  - Rationale: The maintainer comments keep the separate Linux Feature adapter acceptable if the X11 path stays separate; the backend/flavor route is explicitly conditional on fit and would require a separate upstream/backend evaluation. ADR 0009 also preserves standalone `x11_*` tools and separates backend vs wrapper PRs.
  - Resolution: Answered from repository context and issue #389 wording; no user question required.

- Question considered: Should the adapter scaffold add a runtime hook now because issue #389 mentions stage/runtime hooks?
  - Recommended answer: No, not unless upstream testing proves a runtime hook is necessary.
  - Rationale: The existing scaffold stages plugin resources and adds a narrow plugin gate descriptor. The requirement says existing hooks should cover the integration and asks for only narrow generic hooks if missing; adding an unused runtime hook would widen scope.
  - Resolution: Answered from scaffold behavior and maintainer constraint; no user question required.

- Question considered: Should this change alter release artifact layout, plugin names, or `x11_*` MCP tool names?
  - Recommended answer: No.
  - Rationale: Issue #389 and ADR 0009 both preserve namespaced `x11_*` tools and standalone source-of-truth identity; the current gap is documentation/decision alignment.
  - Resolution: Answered from ADR 0009 and existing specs; no user question required.

## Resolved Terms

- Added glossary term `Backend flavor route` to `CONTEXT.md` to distinguish a future selectable backend/flavor evaluation from the current Linux Feature adapter path.

## Document Updates Applied

- Added modified delta requirements for `x11-release-adapter-handoff` covering backend/flavor path documentation and scaffold README separation.
- Updated `CONTEXT.md` with the `Backend flavor route` glossary entry.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- No new durable ADR candidate. This change documents a future evaluation path but does not choose a hard-to-reverse architecture or supersede ADR 0009/0010.

## Open Questions

None.
