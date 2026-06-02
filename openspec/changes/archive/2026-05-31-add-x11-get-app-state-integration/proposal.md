## Why

`get_app_state` is the first Computer Use state read that should compose window context, screenshot state, AT-SPI context, and diagnostics into one safe response. The standalone X11/EWMH plugin now has those pieces separately, but callers still need one command/MCP tool that resolves X11 selectors without returning random windows or arbitrary accessibility trees.

## What Changes

- Add a standalone `get-app-state --json` CLI command that accepts the current X11 target selectors (`window_id`, `title`, `wm_class`, `pid`) and composes window context, screenshot capture, AT-SPI correlation, diagnostics, and a short message.
- Add a project-owned MCP tool `x11_get_app_state` that wraps the same behavior without introducing a competing stock `get_app_state` tool name.
- Preserve target-repo compatibility by matching the existing `GetAppStateOutput` concepts (`window_context`, `window_error`, `screenshot`, `screenshot_error`, `accessibility_tree`, `accessibility_error`, `diagnostics`) while keeping standalone-only details under project-owned diagnostics.
- Ensure ambiguous or missing target selectors never produce arbitrary `window_context` or arbitrary AT-SPI subtrees; screenshot and diagnostics still return when those layers are usable.
- Refresh doctor/app-state diagnostics so Cinnamon/X11 screenshot readiness is based on DBus methods, strict RemoteDesktop availability remains false for empty introspection tables, and selected input backend facts remain visible.
- Document source-overlay guidance: future target integration should register `x11-ewmh` into the existing windowing path and improve stock `get_app_state`, not create a parallel bundled tool shape.

## Capabilities

- New capability: `x11-get-app-state-integration` — compose X11/EWMH window context, screenshot capture, AT-SPI correlation, and diagnostics into a standalone app-state command/report.
- Modified capability: `standalone-codex-mcp-plugin` — add deterministic `x11_get_app_state` to the project-owned standalone MCP tool surface.
- Modified capability: `doctor-cli` — ensure strict portal/screenshot/input diagnostics feed the composed app-state report without over-reporting RemoteDesktop or under-reporting Cinnamon screenshot methods.
- Existing capabilities consumed: `x11-window-listing`, `x11-atspi-window-correlation`, `x11-screenshot-coordinate-model`, and `x11-targeted-input-safety`.

## Research refresh

Date: 2026-05-31.

Sources checked:

- Project context and backlog: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, `adr/0008-adopt-x11-root-coordinate-model.md`, `backlog/00-research-reuse-map.md`, and `backlog/09b-get-app-state-integration.md`.
- Current project checkout `/home/as/ai-projects/codex-computer-use-x11`: branch `main`, scaffold checkpoint `5466ead`, clean before proposal; reviewed `src/cli.rs`, `src/list_windows.rs`, `src/accessibility.rs`, `src/coordinates.rs`, `src/doctor.rs`, `src/input.rs`, `src/mcp.rs`, existing tests, README, and `docs/integration-contract.md`.
- Current target checkout `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`: branch `main`, commit `1a6f343ee7ce597019a4c573259c2a9838376874`, clean status; reviewed `computer-use-linux/src/server.rs`, `windowing/types.rs`, `windowing/target.rs`, `windowing/registry.rs`, `atspi_tree.rs`, `screenshot.rs`, `remote_desktop.rs`, and `diagnostics.rs`.
- Target repo findings: stock `get_app_state` already has `window_context`, `window_error`, screenshot, AT-SPI tree/error, diagnostics, and target selectors; target integration should make `x11-ewmh` available through `list_windows()` / target resolution rather than inventing a new stock response shape.
- Local live probes: Cinnamon/X11 session (`DISPLAY=:0`, `XDG_SESSION_TYPE=x11`, `XDG_CURRENT_DESKTOP=X-Cinnamon`) has `wmctrl`, `xprop`, `xdotool`, `gdbus`, `busctl`, `python3`, `xrandr`, and `xdpyinfo`; `wmctrl` currently lists windows; `org.gnome.Shell.Screenshot` exposes `Screenshot`, `ScreenshotWindow`, and `ScreenshotArea`; portal Screenshot exposes `Screenshot` with version 2; `RemoteDesktop` introspection returns an empty table and is unavailable; `org.a11y.Bus.GetAddress` is callable but standalone doctor currently under-reports AT-SPI because live probe collection is incomplete.
- External/reuse refresh: GitHub metadata/web search checked `agent-sh/computer-use-linux`, `tak-uukti/linux-computer-use`, `BeckhamLabsLLC/linux-desktop-mcp`, `Touchpoint-Labs/Touchpoint`, `MONTBRAIN/vadgr-computer-use`, `wimi321/linux-computer-use-skill`, `joe223/sootie`, `iFurySt/open-codex-computer-use`, `nashaofu/xcap`, and screenshot/X11 references surfaced by search. No new source supersedes the local target repo as primary guidance.

Ideas accepted:

- Reuse target `GetAppStateOutput` concepts and existing standalone reports instead of creating a novel response vocabulary.
- Screenshot capture can be a standalone GNOME Shell-compatible smoke boundary, but source-overlay work should reuse target `screenshot.rs`.
- AT-SPI matching must reuse the existing correlation confidence model; ambiguous/unavailable AT-SPI remains a degraded layer while window/screenshot stay usable.
- Strict portal diagnostics must treat empty RemoteDesktop introspection as unavailable and report Screenshot separately from input readiness.

Ideas rejected or deferred:

- Do not modify the target checkout in this stage; source-overlay integration remains future work unless a later change explicitly targets it.
- Do not add unnamespaced stock MCP tools; standalone plugin tools remain `x11_*`.
- Do not copy external project source code; all external sources are ideas/reference only for this change.
- Do not add a Cinnamon extension or replace existing GNOME/COSMIC/KWin/Hyprland/i3 target backends.

Risks / unknowns:

- AT-SPI availability varies by application and local accessibility settings; app-state must preserve degraded `accessibility_error` without losing window/screenshot data.
- Full screenshot data can be large; tests should use tiny fake PNGs, and live smoke should capture only enough evidence to verify shape.
- Window-target ambiguity by title/class must remain explicit and non-random even when screenshot capture succeeds.

## Impact

- Rust standalone crate under `src/`, especially CLI dispatch, new app-state composition module, doctor live probes, MCP tool definitions/call handling, and tests.
- README and integration contract documentation for `get-app-state` / `x11_get_app_state` and future target integration guidance.
- OpenSpec specs under `openspec/changes/add-x11-get-app-state-integration/specs/` and canonical specs after archive.
- Verification: `openspec validate add-x11-get-app-state-integration --strict`, `make fmt`, `make check`, `make test`, focused fake-command CLI/MCP tests, and live/degraded Cinnamon/X11 smoke.
- No external credentials are needed; `.secrets.local.env` is not read. The target checkout is inspected read-only and must remain unmodified.
