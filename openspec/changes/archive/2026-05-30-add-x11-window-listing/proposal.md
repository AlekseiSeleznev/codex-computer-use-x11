## Why

The standalone `codex-computer-use-x11` CLI currently reports X11/EWMH readiness through `doctor --json`, but it cannot yet enumerate real X11 windows for Codex targeting. This change adds the first observable window backend capability: a `list-windows --json` command that maps `wmctrl -lpGx` output into the upstream-compatible `WindowInfo` shape while keeping X11-only reliability diagnostics in a sidecar.

## What Changes

- Add a public `codex-computer-use-x11 list-windows --json` command for X11/EWMH window listing.
- Add pure parsing and normalization for `wmctrl -lpGx` rows, reusing the existing canonical X11 window-id parser.
- Map window rows to the target repo's `WindowInfo`-compatible primary fields: `window_id`, `title`, `app_id`, `wm_class`, `pid`, `bounds`, `workspace`, `focused`, `hidden`, `client_type`, and `backend`.
- Add sidecar/report metadata for raw ids, command source, PID reliability, degraded reasons, and any optional per-window type/state lookup facts without expanding upstream `WindowInfo`.
- Support graceful degraded JSON when `DISPLAY` or `wmctrl` is unavailable instead of panicking.
- Keep `_NET_WM_WINDOW_TYPE` / `_NET_WM_STATE_HIDDEN` lookup bounded or optional so the first listing command does not introduce unconditional slow N+1 process spawning.
- No **BREAKING** changes are intended for the existing `doctor --json` bootstrap surface.

## Capabilities

- New capability: `x11-window-listing` — specifies `list-windows --json`, `wmctrl -lpGx` parsing, upstream-compatible `WindowInfo` output, sidecar diagnostics, and degraded/no-display behavior.
- Existing capability consumed but not modified by default: `x11-integration-contract` — keeps `x11-ewmh` as the canonical backend id, upstream `WindowInfo` as the primary model, and X11-only metadata in sidecar/report fields.
- Existing capability consumed but not modified by default: `doctor-cli` — remains the readiness/capability report and may advertise the new capability additively after implementation.

## Impact

- Code: root Rust crate under `src/`, especially CLI dispatch, a new X11 window-listing module, shared id parsing, and command-runner/fake PATH seams for tests.
- Tests: parser fixtures for `wmctrl -lpGx`, CLI JSON tests using fake commands/PATH, and live Cinnamon/X11 smoke evidence after unit tests pass.
- APIs: adds `list-windows --json`; preserves `doctor --json` field compatibility.
- Dependencies: no required new Rust dependency for the MVP; native `x11rb` remains a later fallback candidate if shelling out to `wmctrl` proves insufficient.
- External systems/secrets: none. The command reads local desktop state only and must not require `.secrets.local.env` or modify the Codex Desktop Linux target checkout.
- Constitution/architecture constraints: Rust 2021, root Cargo/Makefile verification, `x11-ewmh` backend id, Codex-first/Cinnamon-X11-first/generic-EWMH posture, upstream-compatible `WindowInfo`, automatic safe checkpoints, and TDD apply slices.

## Research Refresh

Date: 2026-05-30.

- Project state checked: `main` branch, clean status, active change `add-x11-window-listing`, existing specs `doctor-cli`, `project-bootstrap`, and `x11-integration-contract`.
- Target repo checked: `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` on `main` with clean status. Current `computer-use-linux/src/windowing/types.rs` confirms `WindowInfo` / `WindowBounds`; `registry.rs` confirms backend ordering and descriptors; `target.rs` confirms exact-focus verification; `server.rs` confirms stock tools include `list_windows`, `focused_window`, `activate_window`, `click`, `scroll`, `drag`, `press_key`, and `type_text` but no stock standalone `mousemove` tool; `diagnostics.rs` confirms readiness vocabulary and existing portal/input/window reports.
- Local X11 facts checked without recording titles: `wmctrl`, `xprop`, `xdotool`, and `ydotool` are installed; `wmctrl -lpGx` returned 15 rows; `xprop -root _NET_ACTIVE_WINDOW` returned an active X11 id; sampled `_NET_WM_WINDOW_TYPE` was `_NET_WM_WINDOW_TYPE_NORMAL`.
- External references refreshed:
  - `tak-uukti/linux-computer-use` (MIT) still demonstrates an X11-only AT-SPI + `xdotool` + `wmctrl` + `scrot` approach and a `list_windows` tool; useful as ideas/reference, not copied.
  - `joe223/sootie` now has an explicit dual MIT/Apache-2.0 license and documents Linux X11-oriented helpers including `xprop`, `wmctrl`, `xdotool`, AT-SPI bindings, and screenshot utilities; useful as Rust/backend inspiration, not copied.
  - `wimi321/linux-computer-use-skill` (MIT) documents X11-focused desktop-control support using `python-xlib`, `pyautogui`, `mss`, `Pillow`, and `psutil`; useful for native X11/display mapping ideas, not copied.
  - `BeckhamLabsLLC/linux-desktop-mcp` (MIT) documents window targeting/window overlays and X11 support; useful later for target groups/overlays, out of MVP scope.
  - freedesktop.org `wmctrl`/EWMH docs remain the normative source for EWMH/NetWM behavior such as `_NET_CLIENT_LIST`, `_NET_ACTIVE_WINDOW`, and `_NET_WM_WINDOW_TYPE`.
- Reuse policy: external projects remain ideas-only for this change unless a later implementation explicitly copies compatible MIT/Apache code with attribution. GPL/no-license code and command source code are not copied; invoking installed commands is acceptable.
- Planning adjustment from fresh research: implement `wmctrl -lpGx` as the MVP listing source with optional/bounded `xprop` enrichment, not native `x11rb`, and keep `client_type`/`hidden` uncertainty explicit instead of pretending `wmctrl` alone provides all EWMH state.
