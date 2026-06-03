## Context Read

- Change artifacts: `proposal.md`, delta specs under `specs/`, `grill.md`, and `design.md` for `add-x11-target-window-groups-overlays`.
- Project context: `CONSTITUTION.md`, updated `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, and `adr/0008-adopt-x11-root-coordinate-model.md`.
- Current implementation: `src/cli.rs`, `src/mcp.rs`, `src/list_windows.rs`, `src/input.rs`, `src/pointer.rs`, `src/app_state.rs`, `src/coordinates.rs`, and relevant tests.
- Target checkout read-only files: Codex Desktop Linux `server.rs`, `windowing/types.rs`, `windowing/target.rs`, and `windowing/registry.rs`.
- External/standards context already refreshed in `proposal.md`: `linux-desktop-mcp` window groups/overlay behavior, freedesktop EWMH, X11 SHAPE, and GDK input-shape docs.

## Design Summary

- The design adds a `target_window` module for mutable target context while leaving existing input and app-state commands explicitly selector-driven.
- CLI target state is file-backed through an ignored local runtime path / `CODEX_X11_TARGET_STATE`; MCP target state is in-memory per stdio server process.
- Target groups use deterministic ids/colors/active targets and validate stale windows against fresh `list_windows` output.
- Overlay is a provider seam with production `NoOverlayProvider` and fake-provider tests; real visual overlay drawing is deferred.
- Listing safety filters or marks `codex-computer-use-x11` overlay/helper rows as internal metadata, not normal application targets.

## Question Loop

### Q1: Can the same X11 window safely be targeted in multiple groups at once?

Recommended answer: No for this stage. Retargeting the same X11 window into another group should move/update it into the new group rather than duplicating the same desktop window across groups.

Rationale: The public release command is `release-window --window-id <id>` and the specs do not introduce a group-qualified release selector. Allowing the same desktop window in multiple groups would make release semantics ambiguous and could leave stale overlay state. One-owner semantics are simpler, deterministic, and reversible later if a future use case needs multi-group membership.

Resolution: Updated `design.md` to specify that adding the same X11 window to another group moves it to the newly requested group.

## Design Findings

- **No constitution/architecture conflict found.** The design stays in Rust, keeps target checkout read-only, avoids secrets, and uses existing `x11-ewmh` and X11 root-coordinate decisions.
- **Implicit-target risk is controlled.** The design explicitly keeps saved targets out of existing input/app-state defaults, preserving targeted-input safety.
- **Overlay risk is controlled.** Real overlay drawing is deferred; provider failure is warning-only; future real providers must set project-owned metadata and avoid accepting focus.
- **State leak risk is bounded.** CLI state is local runtime JSON and validated against fresh listings; MCP state is process-scoped.
- **Verification is feasible.** Fake `PATH`/state-file tests can cover CLI behavior; MCP tests can cover process state; parser tests can cover owned overlay filtering without live X11; live smoke can prove degraded no-overlay behavior on Cinnamon/X11.

## Document Updates Applied

- Updated `openspec/changes/add-x11-target-window-groups-overlays/design.md` to resolve cross-group duplicate semantics: the same X11 window moves to the newly requested group instead of being duplicated across groups.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

No new durable ADR is required. The reviewed decisions are intentionally local/reversible: file-backed CLI state, in-memory MCP state, one-owner group membership, and a no-overlay production provider. A future change that introduces a real always-on overlay provider or implicit active-target defaults should reconsider an ADR because that would be more durable and surprising.

## Open Questions

None.
