## Why

The standalone X11/EWMH plugin can list, focus, and type into verified windows, but a useful Computer Use baseline also needs pointer actions. Click, scroll, and drag must be guarded because X11 pointer injectors are global desktop injectors, not per-window isolated channels.

## What Changes

- Add standalone JSON CLI commands for safe pointer actions: `click`, `scroll`, and `drag`.
- For targeted pointer actions, resolve one current X11/EWMH window, require known bounds, validate global/root coordinates inside that window, focus and verify the exact active window, and only then invoke the pointer backend.
- Allow explicitly marked global/unverified pointer actions with `--global`; reports must state that the action was not window-isolated and still validate finite, bounded coordinates/amounts.
- Use standalone active-context `xdotool` pointer commands for this plugin stage; do not patch the Codex Desktop Linux target checkout and do not choose the RemoteDesktop portal on Cinnamon/X11 when strict introspection is unavailable.
- Expose matching MCP tools `x11_click`, `x11_scroll`, and `x11_drag` in the project-owned `x11_*` namespace.
- Record fake-command RED/GREEN evidence first, then live Cinnamon/X11 smoke or an explicit degraded reason.

## Capabilities

- New capability: `x11-pointer-actions` — safe standalone pointer click, scroll, and drag reports for X11/EWMH windows.
- Modified capability: `standalone-codex-mcp-plugin` — MCP `tools/list` includes `x11_click`, `x11_scroll`, and `x11_drag` after the verified keyboard tools, preserving deterministic project-owned order.
- Existing capability consumed: `x11-targeted-input-safety` — the same unique target resolution and focus verification safety boundary applies before targeted pointer injection.
- Existing capability consumed: `x11-window-listing` — pointer bounds validation uses the current `WindowInfo.bounds` model with signed X/Y and positive dimensions.

## Research refresh

Date: 2026-05-31 (Europe/Moscow session date).

- Project checkout `/home/as/ai-projects/codex-computer-use-x11`: on `main` at `0a47414` after scaffold checkpoint; working tree clean before creating this proposal. Existing standalone surfaces are `doctor`, `list-windows`, `focused-window`, `focus-window`, `type-text`, `press-key`, and MCP tools through `x11_press_key`.
- Target checkout `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`: on `main` at `1a6f343ee7ce597019a4c573259c2a9838376874`, working tree clean. Inspected `computer-use-linux/src/server.rs`, `remote_desktop.rs`, `abs_pointer.rs`, `windowing/target.rs`, `diagnostics.rs`, `atspi_tree.rs`, and `screenshot.rs`.
- Target repo finding: stock `server.rs` already exposes `click`, `scroll`, and `drag`; it prefers `abs_pointer` where available, then portal/ydotool fallbacks. It uses `resolve_click_target()` for element-targeted clicks and `focus_target_for_input()` for keyboard tools. There is still no stock `mousemove` tool requirement.
- Local Cinnamon/X11 smoke before planning: `DISPLAY=:0`, `XDG_SESSION_TYPE=x11`, `XDG_CURRENT_DESKTOP=X-Cinnamon`; `wmctrl`, `xprop`, `xdotool`, `ydotool`, and `xmessage` are installed; `/dev/uinput` is read/write; `/tmp/.ydotool_socket` is connectable; portal Screenshot exposes `Screenshot` version 2; strict RemoteDesktop introspection returns no concrete methods/properties and is therefore unavailable for pointer input.
- GitHub/reuse refresh with `gh repo view`: `agent-sh/computer-use-linux` remains MIT and directly relevant for target-style pointer/ydotool semantics; `tak-uukti/linux-computer-use` remains MIT and useful for X11/xdotool ideas; `BeckhamLabsLLC/linux-desktop-mcp` remains MIT and useful for backend-priority ideas; `MONTBRAIN/vadgr-computer-use` remains Apache-2.0 and useful as ideas-only unless attribution requirements are met; `joe223/sootie` reports `Other` license and remains copy-unsafe; `jordansissel/xdotool` remains BSD-3-Clause. `gh search repos "linux x11 computer use mcp xdotool"` did not surface a better new primary source in the top results. No external code will be copied in this change.
- `xdotool` capability refresh: local `xdotool` supports `mousemove`, `click`, `mousedown`, `mouseup`, and wheel buttons through `click 4/5/6/7`; these are active/global X11 actions, so target/focus/bounds validation must be the safety boundary.

## Impact

- Rust standalone crate under `src/` and integration tests under `tests/`.
- OpenSpec specs under `openspec/changes/add-x11-pointer-actions/specs/` and, on archive, canonical specs under `openspec/specs/`.
- Existing plugin installer scripts remain structurally unchanged, but installed/rebuilt binaries will expose the additional MCP tools.
- No source-overlay writes to `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`; target checkout is research/read-only for this change.
- No external credentials or `.secrets.local.env` values are required.
- Constitution/architecture constraints preserved: Rust 2021/Cargo, root `Makefile` checks, canonical backend id `x11-ewmh`, strict secret handling, OpenSpec validation, `make fmt`, `make check`, `make test`, and TDD RED/GREEN/REFACTOR slices.
