# Integration Contract

This document is the normative stage-01 contract for future `codex-computer-use-x11` integration. `README.md` is a summary; this file is the durable reference for future backend and source overlay work.

## Backend identity

The canonical backend id is `x11-ewmh`.

Future X11/EWMH windows mapped into Codex Computer Use Linux use:

```text
WindowInfo.backend = "x11-ewmh"
```

Do not use ambiguous backend ids such as `x11` or `cinnamon`. A window's `client_type` can describe X11/window-type details separately from backend identity.

## Window model

Upstream `WindowInfo` is the primary model for window consumers. Supported X11 fields should map into `WindowInfo` fields such as id, title, app id, wm class, pid, bounds, workspace, focus, hidden state, client type, backend, and terminal metadata when available.

X11-only provenance and reliability fields remain in a sidecar or report by default. Examples include raw command source, PID reliability, warnings, degraded observations, and raw id strings. Do not expand upstream `WindowInfo` for these fields without a later design/ADR decision.

Stage 01 documents this non-implemented sidecar sketch for future review:

```rust
pub struct WindowObservationMeta {
    pub window_id: u64,
    pub raw_id: Option<String>,
    pub source: String,
    pub pid_reliable: Option<bool>,
    pub warnings: Vec<String>,
    pub degraded: Vec<String>,
}
```

`WindowObservationMeta` is not implemented in stage 01. It records the intended sidecar/report boundary so future work can evaluate whether the shape is sufficient before expanding any upstream model.


## Reversible overlay scripts

`scripts/status-codex-source-overlay.sh`, `scripts/install-codex-source-overlay.sh`, and `scripts/uninstall-codex-source-overlay.sh` are the project-owned source-overlay controls. They accept `--target <path>` and otherwise resolve `CODEX_DESKTOP_LINUX_FULL_PATH` before the documented local default. Install owns only marker blocks labeled `BEGIN codex-computer-use-x11` / `END codex-computer-use-x11` plus the generated `computer-use-linux/src/windowing/backends/x11_ewmh.rs` file. Uninstall removes only that owned content.

Status values are:

- `state=clean`: no owned overlay content is present;
- `state=applied`: expected owned markers and generated backend are present;
- `state=drifted`: owned markers, generated backend content, target anchors, or unowned/native X11 content no longer match overlay expectations.

The overlay should be applied to the real target only for reversible smoke: status, install, target cargo tests, uninstall, and final clean target status.

## Source overlay fallback order

A future source overlay should register `x11-ewmh` as a late fallback after existing desktop-specific backends unless a later accepted ADR changes the strategy. The planned fallback order is after:

- GNOME extension
- GNOME introspect
- COSMIC
- KWin
- Hyprland
- i3

The generic X11/EWMH backend must not replace a more specific backend that can list or focus windows successfully.

## get_app_state source-overlay contract

Future target-repo integration should improve the existing stock
`get_app_state` path instead of adding a competing bundled tool shape.

The source overlay should make `x11-ewmh` feed the target repo's existing
windowing and target-resolution functions so stock `get_app_state` can populate
its established fields:

```text
window_context
window_error
screenshot
screenshot_error
accessibility_tree
accessibility_error
diagnostics
message
```

Screenshot capture in the target repo should reuse the existing target
`screenshot.rs` provider. AT-SPI tree extraction should reuse the target
`atspi_tree.rs` path and only add X11/EWMH correlation where needed to avoid
arbitrary app/window subtree selection. Target/source-overlay diagnostics may keep screenshot provider facts separate
from RemoteDesktop compatibility facts and should treat empty RemoteDesktop
introspection as unavailable. Standalone `doctor` readiness for this repository
does not require or degrade on RemoteDesktop input.

The standalone `x11_get_app_state` MCP tool is a project-owned validation
surface for this repository. It is not the desired stock tool name for the
bundled Codex Desktop Linux plugin.


## Target-window groups and overlay context

The standalone `target-window`, `target-context`, and `release-window` CLI commands plus `x11_target_window`, `x11_target_context`, and `x11_release_window` MCP tools are project-owned validation surfaces for session-scoped target context. They are useful UX and safety evidence, but they do not replace the target repo's existing `WindowTarget` and `resolve_window_target` concepts.

Future source-overlay work should treat target-window groups as optional context around the existing stock target-resolution/windowing path. It should not make saved targets implicit defaults for stock input or `get_app_state` without a later design/ADR decision, because targeted input safety depends on explicit selectors plus fresh focus/bounds verification.

Visual overlay providers are optional. Any future real X11 overlay provider must set project-owned class/name metadata such as `codex-computer-use-x11-overlay`, avoid accepting focus, avoid taskbar/pager pollution, and be filtered or marked internal by window listing so overlay/helper windows are never normal app targets.

## Target checkout path

The Codex Desktop Linux target checkout is machine-local. Use the durable variable name `CODEX_DESKTOP_LINUX_FULL_PATH` when future documentation, scripts, or tests need to refer to it. Concrete local paths are development-machine defaults only and are not portable requirements.

Stage 01 must not modify the target checkout.

## Command execution and testing

Standalone project code that eventually exercises external command behavior must use a command-runner seam or fake `PATH` fixture so tests can run without live `wmctrl`, `xprop`, or `xdotool` binaries.

Future source overlay code should follow the target repository style of thin `Command::new(...)` wrappers plus pure parser/normalizer fixture tests unless an explicit design/ADR accepts adding a dependency-injection runner to the target repository.

The stage-01 X11 id normalizer is numeric-only. Formatting for future `wmctrl`, `xprop`, or `xdotool` command boundaries remains separate from canonical parsing.

## License and reuse policy

The license posture is reference-first:

- Treat external projects as references and ideas unless a later task performs explicit license review for copying code.
- Do not copy GPL, AGPL, unlicensed, or unclear-license source code into this repository without a separate decision.
- Runtime invocation of external system tools is distinct from vendoring or copying their source code.
- Any future copied or vendored compatible code must include the required attribution, NOTICE, or license handling before merge.

No external project code is copied in stage 01.
