## Why

Targeted keyboard input is unsafe on X11 unless the target window is first focused and a fresh active-window lookup proves that the requested X11 window owns focus. The standalone MCP plugin currently exposes doctor/list/focus tools but not safe `x11_type_text` or `x11_press_key`, so Codex cannot yet validate the keyboard-input portion of the Cinnamon/X11 Computer Use baseline through the standalone feedback loop.

## What Changes

- Add a standalone safe targeted keyboard pipeline: resolve a listed X11 window, activate it, verify exact active-window focus, then call the selected keyboard injector.
- Expose `x11_type_text` and `x11_press_key` MCP tools, and matching CLI JSON commands, that refuse to call input commands when focus is unverified, missing, stale, or ambiguous.
- Use active-context `xdotool type/key --clearmodifiers` for the standalone plugin keyboard backend; do **not** use `xdotool --window` as the safety boundary.
- Keep global/unverified keyboard injection out of scope for this change except for explicit diagnostic text that explains global injectors are not window-isolated.
- Record RED/GREEN evidence for fake-command TDD slices and live Cinnamon/X11 smoke or an explicit degraded reason for Cyrillic/non-BMP text behavior.

## Capabilities

- New capability: `x11-targeted-input-safety` — verified-focus-gated standalone keyboard input for X11/EWMH windows.
- Modified capability: `standalone-codex-mcp-plugin` — MCP `tools/list` includes `x11_type_text` and `x11_press_key` after the focus tool, while preserving the existing project-owned `x11_*` namespace.
- Existing capability consumed: `x11-active-window-focus` — `focus-window`/`x11_focus_window` success is the prerequisite safety evidence for targeted keyboard input.

## Research refresh

Date: 2026-05-31 (Europe/Moscow session date).

- Project checkout `/home/as/ai-projects/codex-computer-use-x11`: on `main` at `6e96be9763f7702ccfeec957b6b0c52282ebe7e1` before this change; working tree clean before scaffold. Existing standalone surfaces are `doctor`, `list-windows`, `focused-window`, `focus-window`, and MCP `x11_doctor`, `x11_list_windows`, `x11_focused_window`, `x11_focus_window`.
- Target checkout `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`: on `main` at `1a6f343ee7ce597019a4c573259c2a9838376874`, working tree clean. Inspected `computer-use-linux/src/windowing/{types.rs,registry.rs,target.rs}`, `server.rs`, `diagnostics.rs`, `remote_desktop.rs`, and `screenshot.rs`.
- Target repo finding: stock keyboard tools are `type_text` and `press_key`, with optional `window_id`, `pid`, `app_id`, `wm_class`, `title`, `tty`, `terminal_pid`, `terminal_command`, and `terminal_cwd` selectors. `server.rs::focus_target_for_input()` already refuses input when focus verification fails; it falls back to ydotool after Wayland/KDE portal paths as appropriate. No stock `focus_window` or `mousemove` tool is required for this standalone change.
- Live Cinnamon/X11 smoke before planning: `doctor --json` sees `XDG_SESSION_TYPE=x11`, `XDG_CURRENT_DESKTOP=X-Cinnamon`, `wmctrl`, `xprop`, `xdotool`, and `ydotool` installed; RemoteDesktop portal is unavailable/incomplete; `/dev/uinput` and ydotool socket are available; `list-windows --json` returns windows and `focused-window --json` identifies the Codex window.
- GitHub/reuse refresh with `gh repo view`: `agent-sh/computer-use-linux` remains MIT and directly relevant for target-style ydotool/windowing semantics; `tak-uukti/linux-computer-use` remains MIT and useful for AT-SPI + xdotool ideas; `BeckhamLabsLLC/linux-desktop-mcp` remains MIT and useful for input backend priority ideas; `wimi321/linux-computer-use-skill` remains MIT; `MONTBRAIN/vadgr-computer-use` remains Apache-2.0; `Touchpoint-Labs/Touchpoint` remains MIT; `jordansissel/xdotool` remains BSD-3-Clause; `ReimuNotMoe/ydotool` remains AGPL-3.0. No code will be copied from those projects in this change.
- `xdotool` manpage refresh: `type --window`/`key --window` uses XSendEvent and many apps ignore those events; current-window typing uses XTEST. This confirms the design rule: activate/focus/verify first, then type/key into the active context rather than treating `--window` direct events as safe targeted input.
- `xdotool` manpage also warns unusual symbols under non-US keybindings may send the wrong character. This change must record Cyrillic/non-BMP behavior as evidence or a degraded limitation rather than promising full Unicode correctness without proof.

## Impact

- Rust standalone crate under `src/` and integration tests under `tests/`.
- OpenSpec specs under `openspec/changes/add-x11-targeted-input-safety/specs/` and, on archive, canonical specs under `openspec/specs/`.
- Existing plugin installer scripts remain structurally unchanged, but their installed binary will expose the new MCP tools after rebuild/reinstall.
- No source-overlay writes to `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`; target checkout is research/read-only for this change.
- No external credentials or `.secrets.local.env` values are required.
- Constitution/architecture constraints preserved: Rust 2021/Cargo, root `Makefile` checks, canonical backend id `x11-ewmh`, strict secret handling, OpenSpec validation, `make fmt`, `make check`, `make test`, and TDD RED/GREEN/REFACTOR slices.
