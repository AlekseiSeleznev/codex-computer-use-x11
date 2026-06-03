## Context Read

- Root project rules and context: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`.
- Change artifacts: `openspec/changes/add-x11-active-window-focus/proposal.md`, `openspec/changes/add-x11-active-window-focus/specs/x11-active-window-focus/spec.md`.
- Backlog/research context: `backlog/00-research-reuse-map.md`, `backlog/04-active-window-focus.md`.
- Current implementation: `src/cli.rs`, `src/list_windows.rs`, `src/x11_id.rs`, `tests/list_windows_cli.rs`, `tests/doctor_cli.rs`.
- Target compatibility research, read-only: `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/computer-use-linux/src/windowing/types.rs`, `windowing/target.rs`, `windowing/registry.rs`, `windowing/backends/*.rs`, and `server.rs` tool registration for `list_windows`, `focused_window`, and `activate_window`.
- External docs checked 2026-05-30: freedesktop EWMH `_NET_ACTIVE_WINDOW`; xdotool man pages for `getactivewindow` and `windowactivate --sync`; wmctrl EWMH command documentation.
- Local probe evidence: Cinnamon/X11 session with `wmctrl`, `xprop`, and `xdotool` installed; `xprop -root _NET_ACTIVE_WINDOW` returned `0x6600004` and `xdotool getactivewindow` returned matching decimal `106954756`.

## Plan Summary

- Add a read-only `focused-window --json` command that matches `_NET_ACTIVE_WINDOW` against the current `wmctrl -lpGx` listing and reports a `WindowInfo`-shaped `focused_window` or structured degradation.
- Add `focus-window --window-id <id> --json`, resolving ids through the existing shared normalizer and activating only windows present in the current listing.
- Treat activation command exit status as advisory; success requires a fresh active-window lookup matching the requested id.
- Record machine-readable `FocusNotVerified` when Muffin/Mutter or another X11 window manager refuses or fails to complete activation, so future input remains gated.
- Keep this stage standalone: no target checkout mutation, no Cinnamon extension, no direct targeted input behavior.

## Question Loop

### Q1: Should `xprop -root _NET_ACTIVE_WINDOW` or `xdotool getactivewindow` be the canonical active-window source?

- **Recommended answer:** Use `_NET_ACTIVE_WINDOW` via `xprop` as the canonical source and keep `xdotool getactivewindow` as research/smoke corroboration or future fallback only if needed.
- **Rationale:** Existing `list-windows` already parses `_NET_ACTIVE_WINDOW`; EWMH defines that property as the window-manager-reported active window; local probes show it matches `xdotool getactivewindow` on the current Cinnamon/X11 session.
- **Resolution:** Answered by repository context and local probes. No user question required.

### Q2: Is an activation command's zero exit status enough to mark focus safe?

- **Recommended answer:** No. `focus-window` must perform a fresh active-window lookup and compare ids after the activation attempt.
- **Rationale:** EWMH allows a window manager to refuse an activation request; the target repo's `focus_window_target` also activates then waits for a fresh focused-window query before reporting exact focus.
- **Resolution:** Captured in the spec as `FocusNotVerified` behavior. No user question required.

### Q3: Should fallback from `wmctrl -ia` to `xdotool windowactivate --sync` be in scope?

- **Recommended answer:** Yes, but only as an activation attempt fallback; it must not weaken final verification.
- **Rationale:** The backlog explicitly proposes this fallback order, xdotool documents `windowactivate --sync`, and local tools are available. Fallback is low-risk because success still depends on `_NET_ACTIVE_WINDOW` matching the requested id.
- **Resolution:** Captured in the spec and left for design to define attempt ordering and diagnostics. No user question required.

### Q4: Should this change expose or bless direct `xdotool --window` input as safe targeted input?

- **Recommended answer:** No. This change only establishes focus verification; later input stages may consume verified focus evidence but must still perform their own authorization/backend checks.
- **Rationale:** The backlog warns that direct window-targeted X11 synthetic events may be ignored and must not become the safety boundary. The project architecture separates windowing facts from future input behavior.
- **Resolution:** Captured in the spec's targeted-input safety boundary requirement. No user question required.

### Q5: Is a durable ADR required for this stage?

- **Recommended answer:** No new durable ADR is required unless design review discovers a hard-to-reverse architecture decision. The focus-verification rule is important, but it is the natural continuation of existing `x11-ewmh` architecture and backlog constraints rather than a surprising project-wide trade-off.
- **Rationale:** Existing ADRs already require OpenSpec source of truth, Matt grill/TDD gates, and automatic checkpoint discipline. This stage does not choose a new platform architecture or mutate the target integration repo.
- **Resolution:** Record in `adr.md` that no durable ADR is needed if design remains within this scope. No user question required.

## Resolved Terms

- `Active window` — the window-manager-reported top-level window currently active for user interaction.
- `Focus verification` — comparing a requested target window to a freshly observed active window before targeted input can treat the target as safe.
- `FocusNotVerified` — machine-readable safe failure state when activation did not prove the requested window became active.

`CONTEXT.md` was updated inline with these glossary terms.

## Document Updates Applied

- Updated `CONTEXT.md` with `Active window`, `Focus verification`, and `FocusNotVerified` glossary entries.
- Confirmed the proposal/spec scope already includes research refresh, verified-focus safety, fallback diagnostics, and no target checkout mutation.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- No durable ADR candidate at pre-design. Re-evaluate during `adr.md` after design review.

## Open Questions

None.
