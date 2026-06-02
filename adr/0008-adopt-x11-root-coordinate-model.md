# 0008 — Adopt X11 root coordinate model for bounds and screenshot crops

## Status

Accepted

## Date

2026-05-31

## Context

`codex-computer-use-x11` is adding screenshot and crop behavior after earlier X11 window listing, focus, targeted keyboard/pointer input, and AT-SPI correlation stages. Those behaviors must agree on what an `(x, y)` point or rectangle means.

The target Codex Desktop Linux `WindowBounds` model uses optional signed x/y positions and unsigned width/height. Current standalone X11 listing uses `wmctrl -lpGx` as primary window geometry, while live Cinnamon/X11 research showed `wmctrl`, `xwininfo`, and `xdotool getwindowgeometry` can disagree for the same active window. Screenshot providers in the target repo capture screen/global images, not application-client-local images.

## Decision

Use **X11 root/global pixel coordinates** as the canonical coordinate space for X11/EWMH window bounds, pointer points, screenshot crop rectangles, and future `get_app_state` screenshot/window-context composition.

Preserve the upstream-compatible bounds shape:

- known `x`/`y` coordinates are signed values (`Some(i32)` / JSON number), including negative monitor offsets;
- unknown `x`/`y` coordinates are absent/null;
- `width`/`height` are positive unsigned dimensions.

Keep `wmctrl -lpGx` as the standalone primary bounds source for consistency with existing window listing and pointer safety checks, but record bounds provenance and optional alternate source diagnostics such as `xwininfo`. When sources disagree, report the disagreement instead of silently replacing primary bounds.

For standalone screenshot crop smoke tests, validate crop rectangles in X11 root coordinates before provider invocation and call a GNOME Shell-compatible `ScreenshotArea` provider only after validation. Future source overlay work should reuse the target repo's existing screenshot provider rather than making the X11 backend own screenshot capture.

## Considered Options

1. **X11 root/global coordinates with provenance diagnostics** (chosen)
   - Aligns pointer, screenshot, and target provider expectations.
   - Preserves upstream `WindowBounds` shape.
   - Makes frame/client and source disagreement visible.

2. **Window-client-local coordinates by default**
   - Convenient for some UI operations.
   - Rejected because X11 client vs frame extents are not reliably available from current shell-out sources and would hide titlebar/decorator ambiguity.

3. **Switch primary bounds from `wmctrl` to `xwininfo` when available**
   - Could match some observed live geometry better.
   - Rejected for this stage because it would silently change existing window-listing and pointer behavior; source disagreement should first be observable.

4. **Provider-specific coordinates**
   - Rejected because it would require callers and future `get_app_state` logic to know which screenshot/input provider supplied a coordinate.

## Consequences

- Pointer actions, screenshot crops, and future `get_app_state` composition can share one coordinate vocabulary.
- Reports must include bounds provenance and frame/client uncertainty when geometry source details matter.
- Tests must cover negative coordinates and source disagreement with fixtures because the current live setup may not expose negative monitor offsets.
- Standalone screenshot crop reports should not serialize screenshot pixels/data URLs by default; output paths are caller-provided.
- Future source overlay code must map this model into the existing target screenshot provider instead of creating a parallel screenshot stack inside the X11 backend.

## Evidence

- OpenSpec change: `openspec/changes/add-x11-screenshot-coordinate-model/`.
- Target source inspected read-only: `computer-use-linux/src/windowing/types.rs`, `screenshot.rs`, `remote_desktop.rs`, `server.rs`, `diagnostics.rs`.
- Live Cinnamon/X11 probes on 2026-05-31 confirmed GNOME Shell-compatible screenshot DBus methods and portal Screenshot version 2.
- Live Cinnamon/X11 probes on 2026-05-31 observed a geometry-source disagreement between `wmctrl -lpGx` and `xwininfo`/`xdotool getwindowgeometry` for one active browser window.
