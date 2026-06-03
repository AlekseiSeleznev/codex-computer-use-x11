## Why

The X11/EWMH baseline can now list windows, but targeted keyboard or pointer input is still unsafe until the project can identify the currently focused window and verify that a requested window actually became active after an activation attempt. This change implements the next backlog milestone by adding standalone focused-window and focus-window JSON commands that treat verified active-window identity—not command exit status—as the safety boundary.

## What Changes

- Add `focused-window --json` to report the current EWMH active window as a `WindowInfo`-shaped object when it can be matched to the current window list, with structured diagnostics when there is no active window or no match.
- Add `focus-window --window-id <id> --json` to activate a specific X11 window through an ordered backend attempt (`wmctrl -ia`, then `xdotool windowactivate --sync` fallback when needed) and verify the result through a fresh active-window lookup.
- Add a stable focus result JSON shape including `success`, `requested_window`, `focused_window`, `exact_window_focused`, `backend`, `error_code`, `note`, and diagnostics; focus-stealing prevention or an active-window mismatch returns `FocusNotVerified` rather than a false success.
- Reuse the existing shared X11 window-id normalizer so short hex, padded hex, and decimal CLI inputs resolve to the same `u64` identity.
- Extend tests and smoke evidence for active-window parsing, no-active/missing active behavior, activation verification, fallback, and unverified focus failures.
- No breaking change: existing `doctor --json` and `list-windows --json` shapes remain additive-compatible.

## Capabilities

- New capability: `x11-active-window-focus` with requirements for focused-window reporting, focus activation, verification, fallback diagnostics, and safety boundaries.
- Existing capability consumed: `x11-window-listing` provides the `WindowInfo`-shaped list and focused flag that `focused-window` and focus verification reuse.

## Impact

- Affected code: root Rust CLI and tests under `src/` and `tests/`; no source-overlay mutation in the integration target checkout.
- Affected commands: `codex-computer-use-x11 focused-window --json`, `codex-computer-use-x11 focus-window --window-id <id> --json`, plus unchanged `doctor --json` and `list-windows --json`.
- Required technologies and verification: Rust 2021, root `Makefile`, OpenSpec validation, `make fmt`, `make check`, `make test`; behavior-changing work follows RED/GREEN/REFACTOR slices.
- Architecture/ADR constraints: preserve `x11-ewmh` as the backend id, use OpenSpec artifacts as source of truth, do not change accepted ADR history, and keep local-secret boundaries intact.
- External systems/secrets: no external credentialed systems and no `.secrets.local.env` access are required. The local Codex Desktop Linux target checkout is read for compatibility research only.

## Research refresh (2026-05-30)

- Repository state: project root is on `main` with clean status before this change; target checkout at `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` is on `main` and was inspected read-only.
- Target repo files inspected: `computer-use-linux/src/windowing/types.rs`, `windowing/target.rs`, `windowing/registry.rs`, `windowing/backends/{gnome,kwin,cosmic,hyprland,i3}.rs`, and `server.rs` references for `list_windows`, `focused_window`, and stock `activate_window`.
- Local Cinnamon/X11 probes: `DISPLAY=:0`, `XDG_SESSION_TYPE=x11`, `XDG_CURRENT_DESKTOP=X-Cinnamon`; `wmctrl`, `xprop`, and `xdotool` are installed. `xprop -root _NET_ACTIVE_WINDOW` returned `0x6600004`, matching `xdotool getactivewindow` decimal `106954756`.
- External docs checked: freedesktop EWMH `_NET_ACTIVE_WINDOW` specifies the active-window property and allows window managers to refuse activation requests; xdotool man pages document `getactivewindow` and `windowactivate --sync`; wmctrl documentation confirms EWMH-based X11 window activation/listing.
- Ideas used: read active state through `_NET_ACTIVE_WINDOW`; treat activation command success as advisory; verify through a fresh active-window lookup; use fallback only when primary activation fails or remains unverified.
- Ideas rejected: direct `xdotool --window` targeted input as a safety boundary; Cinnamon/Muffin extension work in this stage; copying code from external projects.
- Risks/unknowns: Muffin focus-stealing prevention may legitimately reject some activations, so the user-visible result must explain `FocusNotVerified` and preserve safe degraded behavior.
