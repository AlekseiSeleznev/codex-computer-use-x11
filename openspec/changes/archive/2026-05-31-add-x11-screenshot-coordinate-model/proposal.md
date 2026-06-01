## Why

X11 pointer input, screenshot crops, and future `get_app_state` composition all depend on one explicit coordinate model; without it, root-screen coordinates, multi-monitor offsets, frame/client bounds, and crop rectangles can be mixed accidentally. This change makes screenshot and bounds behavior testable in the standalone `x11-ewmh` path while preserving the upstream `WindowBounds` contract and existing Codex screenshot provider strategy.

## What Changes

- Add a `window-bounds --window-id <id> --json` public CLI command that reports the selected window's bounds, global/root X11 coordinate model, provenance, and frame/client uncertainty diagnostics.
- Add crop-rectangle validation and an optional `screenshot-crop --window-id <id> ... --output <path> --json` standalone CLI path that validates a global/root X11 crop before calling a GNOME Shell-compatible DBus screenshot-area provider.
- Preserve the existing `WindowBounds` shape (`x/y: Option<i32>`, `width/height: u32`) and ensure known negative coordinates remain signed rather than becoming unsigned or sentinel values.
- Refresh docs and diagnostics language so screenshot availability is based on DBus provider methods (`org.gnome.Shell.Screenshot` / portal Screenshot) and so `wmctrl` frame/client uncertainty is visible.
- Add a durable ADR for the X11 root coordinate and screenshot-crop boundary if the ADR review confirms it is a durable architecture decision.

## Capabilities

- Add new capability: `x11-screenshot-coordinate-model`.
- Modify existing capability: `standalone-codex-mcp-plugin` only for documentation/tool-surface consistency if this change adds standalone MCP wrapping for the new public JSON behavior; otherwise keep MCP unchanged and defer `get_app_state` composition to backlog 09b.
- Reuse existing capabilities without modification where already covered: `doctor-cli` strict screenshot facts and `x11-integration-contract` upstream `WindowBounds` model.

## Impact

- Rust CLI/library code under `src/` for coordinate parsing, bounds reporting, crop validation, and screenshot-area provider invocation.
- CLI help text, README/docs, and tests under `tests/` using fake command `PATH` fixtures for `wmctrl`, `xprop`, `xwininfo`, `xrandr`/`xdpyinfo`, and `gdbus` before any live smoke.
- OpenSpec artifacts and canonical specs for the new screenshot/coordinate behavior.
- Optional architecture snapshot/ADR updates if the per-change ADR gate confirms a durable coordinate-model decision.
- No real credentials are needed; `.secrets.local.env` is not read. The target checkout referenced by `CODEX_DESKTOP_LINUX_FULL_PATH` is inspected read-only for compatibility and is not modified by this change.
