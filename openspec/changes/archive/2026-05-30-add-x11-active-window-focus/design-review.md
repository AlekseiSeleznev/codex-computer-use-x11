## Context Read

- Change artifacts: `proposal.md`, `specs/x11-active-window-focus/spec.md`, `grill.md`, and `design.md` for `add-x11-active-window-focus`.
- Project context: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`.
- Current implementation: `src/cli.rs`, `src/lib.rs`, `src/list_windows.rs`, `src/x11_id.rs`, `tests/list_windows_cli.rs`.
- Target compatibility research: target repo `windowing/types.rs`, `windowing/target.rs`, `windowing/registry.rs`, and stock tool registrations for `focused_window`/`activate_window`.
- Fresh docs/probes already captured in proposal/grill: EWMH `_NET_ACTIVE_WINDOW`, xdotool `windowactivate --sync`, wmctrl EWMH behavior, and local Cinnamon/X11 active-window agreement between `xprop` and `xdotool`.

## Design Summary

- The design adds `src/focus.rs` and CLI arms for `focused-window --json` and `focus-window --window-id <id> --json`.
- It reuses the existing `WindowInfo` list parser and `x11_id` normalizer instead of introducing a parallel window model.
- Focus success is gated by a fresh `_NET_ACTIVE_WINDOW` lookup after activation; `wmctrl -ia` is attempted before `xdotool windowactivate --sync` fallback.
- JSON diagnostics expose blockers, degraded reasons, and ordered activation attempts.
- The target checkout remains read-only; source overlay integration and targeted input are explicitly later work.

## Question Loop

### Q1: Could the focus result accidentally return stale `focused` flags from the pre-activation window listing?

- **Recommended answer:** Yes, unless the design explicitly normalizes the matched `focused_window` clone after the fresh active-window lookup.
- **Rationale:** `list_windows::report_from_system()` marks focus based on the active window observed during listing. A later successful activation can change active id without refreshing every listing row's `focused` field.
- **Resolution:** Updated `design.md` to require matching the verified active id back to the listing and normalizing the returned `focused_window.focused` flag from the verification result. No user question required.

### Q2: Should `focus-window` refresh the full `wmctrl` listing after every activation attempt?

- **Recommended answer:** Not for the MVP; reuse the current listing for identity metadata and use fresh `_NET_ACTIVE_WINDOW` for the authoritative focus fact.
- **Rationale:** The requested window identity should not change during a small activation window, and a full re-list after each attempt adds process churn. The correctness-critical fact is the active id, not a second copy of static metadata.
- **Resolution:** Keep design as current-listing plus fresh active-id verification. If future evidence shows metadata drift matters, a later change can add a fresh listing step.

### Q3: Does fallback to `xdotool` after an unverified `wmctrl` attempt risk double-focusing the wrong window?

- **Recommended answer:** The fallback is acceptable because it targets the same normalized requested id and is still followed by exact verification. It should be visible in diagnostics.
- **Rationale:** The fallback does not authorize input. It only gives the window manager a second activation mechanism, and the final `_NET_ACTIVE_WINDOW` comparison remains the safety boundary.
- **Resolution:** Existing spec/design already records ordered attempts and final verification. No user question required.

### Q4: Are invalid CLI arguments required to produce JSON?

- **Recommended answer:** No. Invalid command usage or invalid id syntax may use non-zero status plus stderr, matching existing unsupported CLI behavior. Once a supported JSON command has a valid id and can construct a report, focus failures should be JSON.
- **Rationale:** This preserves the existing CLI distinction between unsupported usage and runtime degraded reports.
- **Resolution:** Existing spec/design already allows invalid-id stderr before activation. No user question required.

## Design Findings

- **Resolved finding:** The initial design did not explicitly prevent stale `focused` flags in `focus-window` results. `design.md` now requires normalizing `focused_window.focused` from the fresh verification result.
- **Risk accepted:** A full post-activation listing refresh is deferred to avoid extra process churn; exact active-window id remains authoritative for MVP safety.
- **Verification feasibility:** Fake-command integration tests can deterministically cover parser, fallback, mismatch, invalid id, and no-activation-on-missing-window paths without moving real desktop focus.
- **Architecture coherence:** The design matches target repo semantics (`activate_window` followed by focused-window verification) and does not require a new durable ADR or target checkout edits.

## Document Updates Applied

- Updated `openspec/changes/add-x11-active-window-focus/design.md` to require normalized `focused_window.focused` after verification.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

No durable ADR candidate. The design is an expected standalone implementation of the already accepted `x11-ewmh` direction and existing focus-verification posture; it does not add a hard-to-reverse project architecture decision.

## Open Questions

None.
