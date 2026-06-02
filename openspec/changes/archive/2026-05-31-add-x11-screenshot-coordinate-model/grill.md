## Context Read

- `openspec/changes/add-x11-screenshot-coordinate-model/proposal.md`
- `openspec/changes/add-x11-screenshot-coordinate-model/specs/x11-screenshot-coordinate-model/spec.md`
- `CONSTITUTION.md`
- `CONTEXT.md`
- `ARCHITECTURE.md`
- `adr/README.md` (no top-level numbered ADR files were present in this checkout before this change)
- `docs/integration-contract.md`
- `README.md`
- `backlog/00-research-reuse-map.md`
- `backlog/09-screenshot-coordinate-model.md`
- Source files: `src/list_windows.rs`, `src/focus.rs`, `src/pointer.rs`, `src/input.rs`, `src/doctor.rs`, `src/mcp.rs`
- Target checkout read-only files under `${CODEX_DESKTOP_LINUX_FULL_PATH:-/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full}`: `computer-use-linux/src/windowing/types.rs`, `windowing/target.rs`, `windowing/registry.rs`, `server.rs`, `diagnostics.rs`, `screenshot.rs`, `remote_desktop.rs`, `atspi_tree.rs`
- Live local probes on 2026-05-31: `wmctrl -lpGx`, `xprop -root _NET_ACTIVE_WINDOW`, `xwininfo -id <active>`, `xdotool getwindowgeometry --shell <active>`, `xrandr --listmonitors`, `xdpyinfo`, `gdbus introspect` for `org.gnome.Shell.Screenshot` and portal Screenshot
- External license refresh on 2026-05-31 via GitHub license API for the reference projects listed in `backlog/00-research-reuse-map.md`

## Plan Summary

- The change adds an explicit `x11_root_global_pixels` coordinate model for standalone X11/EWMH bounds and screenshot crop rectangles.
- The public CLI surface becomes `window-bounds --window-id <id> --json` for geometry/provenance and `screenshot-crop --window-id <id> ... --output <path> --json` for validated crop provider invocation.
- Upstream compatibility is preserved: `WindowBounds.x/y` are signed optional coordinates and `width/height` are positive `u32` values.
- `wmctrl -lpGx` remains the primary standalone listing source, but the report must surface alternate `xwininfo` geometry and frame/client uncertainty rather than silently claiming client-content bounds.
- Screenshot capture is a standalone smoke boundary only: future source overlay should reuse the target repo's existing screenshot provider rather than replacing it inside the X11 backend.

## Question Loop

No user-facing material questions were needed.

Repository evidence resolved the material uncertainties:

1. **Coordinate type compatibility**
   - Recommended answer: keep `x/y: Option<i32>` and `width/height: u32`.
   - Rationale: target `computer-use-linux/src/windowing/types.rs` already defines this exact shape, and existing standalone `src/list_windows.rs` mirrors it.
   - Resolution: accepted from repository context.

2. **Coordinate space**
   - Recommended answer: define all standalone bounds, pointer points, and screenshot crop rectangles as global/root X11 pixels.
   - Rationale: pointer actions already use global/root X11 coordinates; `xrandr`/`xdpyinfo` report one root coordinate space; target screenshot provider captures screen-level images.
   - Resolution: accepted from backlog, code, and live Cinnamon/X11 probes.

3. **`wmctrl` vs `xwininfo` disagreement**
   - Recommended answer: keep `wmctrl -lpGx` as the primary `WindowInfo` source for continuity, but include alternate source diagnostics and frame/client uncertainty.
   - Rationale: live Cinnamon/X11 probe on 2026-05-31 observed an active browser window where `wmctrl -lpGx` and `xwininfo`/`xdotool getwindowgeometry` disagreed on `x`; silently switching sources would change existing listing semantics and could break pointer safety assumptions.
   - Resolution: accepted as design constraint; spec already requires alternate diagnostics.

4. **Screenshot provider strategy**
   - Recommended answer: source overlay should reuse the existing target screenshot provider; standalone may call GNOME Shell-compatible `ScreenshotArea` through `gdbus` only as a smoke provider after crop validation.
   - Rationale: target `screenshot.rs` already attempts GNOME Shell DBus then portal screenshot, and live Cinnamon exposes `org.gnome.Shell.Screenshot` methods plus portal Screenshot version 2. Duplicating provider internals in the X11 backend would be unnecessary.
   - Resolution: accepted; spec names standalone DBus command as MAY/standalone smoke boundary.

## Resolved Terms

Updated `CONTEXT.md` with:

- `X11 root coordinates`
- `Crop rectangle`
- `Bounds provenance`

## Document Updates Applied

- Added glossary terms to `CONTEXT.md`.
- Proposal and specs already encode the resolved coordinate model, provider boundary, and geometry-provenance behavior.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- Durable ADR candidate: **Adopt X11 root coordinate model and screenshot crop boundary**.
  - Hard to reverse: future pointer, screenshot, and `get_app_state` behavior will rely on this coordinate space.
  - Surprising without context: `wmctrl`/`xwininfo` disagreement and frame/client ambiguity are non-obvious.
  - Real trade-off: choosing root coordinates + provenance diagnostics over client-local coordinates or silent xwininfo replacement.

## Open Questions

None.
