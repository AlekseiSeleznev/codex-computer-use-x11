## Why

The standalone X11/EWMH plugin can now list, focus, inspect, screenshot, and act on windows, but agents still lack an explicit session concept for “the window(s) I am working with now.” Target-window groups and optional visual overlays make multi-window automation safer and clearer while keeping core v1 Computer Use capabilities usable when visual overlays are unavailable.

## What Changes

- Add session-scoped target-window state that can save, inspect, and release one or more resolved X11/EWMH windows without treating ambiguous selectors as valid targets.
- Add window groups with active-window tracking so multi-window tasks can organize related targets and switch/release targets deterministically.
- Add stale target detection that compares saved target ids against the current X11 listing and reports/removes vanished targets instead of reusing stale state.
- Add an optional overlay boundary that can request colored borders for targeted windows but reports warnings/degraded status when the provider is unsupported or fails.
- Mark or exclude project-owned overlay/helper windows from normal `list-windows` targeting so Codex never treats its own visual indicators as application targets.
- Extend the standalone MCP surface with project-owned `x11_*` target-context tools that wrap the same behavior without introducing unnamespaced stock tools.
- Document that this is v1.5 UX support: useful for safety and clarity, but not a blocker for already-required v1 Computer Use capability evidence.

## Capabilities

- New capability: `x11-target-window-groups-overlays` — session-scoped target-window state, window groups, stale detection, optional overlay warnings, and owned-overlay exclusion/marking.
- Modified capability: `standalone-codex-mcp-plugin` — expose deterministic `x11_target_window`, `x11_release_window`, and `x11_target_context` tools alongside existing standalone `x11_*` tools.
- Modified capability: `x11-window-listing` — ensure project-owned overlay/helper windows are excluded from normal application targets or clearly marked as internal metadata.
- Existing capabilities consumed: `x11-window-listing`, `x11-targeted-input-safety`, `x11-screenshot-coordinate-model`, and `x11-get-app-state-integration`.

## Research refresh

Date: 2026-05-31.

Sources checked:

- Project context and backlog: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, `adr/0008-adopt-x11-root-coordinate-model.md`, `backlog/00-research-reuse-map.md`, and `backlog/10-window-targeting-groups-overlays.md`.
- Current project checkout `/home/as/ai-projects/codex-computer-use-x11`: branch `main`, scaffold checkpoint `db07b28`, clean before proposal; reviewed `src/cli.rs`, `src/mcp.rs`, `src/list_windows.rs`, `src/input.rs`, `src/pointer.rs`, `src/app_state.rs`, `src/coordinates.rs`, `README.md`, `docs/integration-contract.md`, and existing tests.
- Current target checkout `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`: branch `main`, commit `1a6f343ee7ce597019a4c573259c2a9838376874`, clean status; reviewed `computer-use-linux/src/server.rs`, `windowing/types.rs`, `windowing/target.rs`, `windowing/registry.rs`, `screenshot.rs`, `remote_desktop.rs`, and `diagnostics.rs`.
- Target repo findings: stock `WindowTarget` already includes `window_id`, `pid`, terminal selectors, `app_id`, `wm_class`, and `title`; stock tools already include `activate_window`, `get_app_state`, `screenshot`, `click`, `scroll`, and `drag`. Future source overlay should feed target state through existing target-resolution concepts, not invent a competing bundled tool namespace.
- External/reuse refresh: cloned `BeckhamLabsLLC/linux-desktop-mcp` at `eaf67ca` (MIT) and reviewed `window_manager.py`, `handlers.py`, `overlay.py`, `tool_definitions.py`, and tests. Accepted ideas: session-scoped window groups, active window per group, release-all, stale validation, overlay failures as warnings, and a no-overlay fallback. Rejected direct source copy and generated ids tied to AT-SPI-only abstractions.
- Standards/docs refresh: reviewed freedesktop EWMH window-type and pager/taskbar guidance (<https://specifications.freedesktop.org/wm-spec/latest/>), X11 SHAPE/input-shape references (<https://www.x.org/releases/X11R7.5/doc/Xext/shape.pdf>), and GTK/GDK input-shape docs (<https://docs.gtk.org/gdk3/method.Window.input_shape_combine_region.html>) for overlay constraints.
- Local live probes: Cinnamon/X11 session (`DISPLAY=:0`, `XDG_SESSION_TYPE=x11`, `XDG_CURRENT_DESKTOP=X-Cinnamon`) has `wmctrl`, `xprop`, `xdotool`, `gdbus`, `python3`, GTK3 GI, and Cairo available. `wmctrl -lpGx` lists application and desktop windows; active window properties confirm EWMH window-type/state facts are available through `xprop`.

Ideas accepted:

- Keep target/window group state session-scoped and deterministic. For one-shot CLI tests, state may be file-backed through an explicit local state path, while MCP should keep state in the stdio server process.
- Reuse existing standalone `WindowTarget` selector semantics and `WindowInfo` shape; do not create a second target vocabulary.
- Treat visual overlays as optional. A target save/release operation must succeed or fail based on target resolution/state rules, not on whether a border can be drawn.
- Use `x11-ewmh` root-coordinate bounds from the listing/coordinate model for overlay requests.
- Identify project-owned overlays by explicit class/name metadata such as `codex-computer-use-x11-overlay` and never expose them as normal application targets.

Ideas rejected or deferred:

- Do not modify the Codex Desktop Linux target checkout in this stage.
- Do not add a GTK/Rust GUI dependency or long-running overlay daemon unless design evidence proves the no-overlay boundary is insufficient; a clear unsupported/warning result satisfies this stage when visual support is absent.
- Do not copy source from `linux-desktop-mcp`; use it as MIT-licensed reference only.
- Do not make target-window groups a v1 DoD blocker for core doctor/list/focus/input/app-state capabilities.

Risks / unknowns:

- Real X11 overlay windows can intercept focus or input if click-through/input shape fails; therefore overlay provider failures and unsupported states must be warnings, and listing must exclude owned overlays.
- CLI persistence can leak stale target ids if not validated against a fresh listing; stale detection must run before reporting saved state as current.
- Multi-window grouping is useful UX but can become confusing if ambiguous title/class selectors pick arbitrary windows; selectors must reuse current ambiguity behavior.

## Impact

- Rust standalone crate under `src/`, especially CLI dispatch, MCP server state handling, window listing metadata, a new target-window/group module, and tests.
- README and integration contract documentation for target-window groups, overlay degraded behavior, and future target-repo guidance.
- OpenSpec specs under `openspec/changes/add-x11-target-window-groups-overlays/specs/` and canonical specs after archive.
- Verification: `openspec validate add-x11-target-window-groups-overlays --strict`, `make fmt`, `make check`, `make test`, focused fake-command CLI/MCP tests, and live/degraded Cinnamon/X11 smoke.
- No external credentials are needed; `.secrets.local.env` is not read. GitHub push uses configured Git credentials without printing secrets. The target checkout is inspected read-only and must remain unmodified.
