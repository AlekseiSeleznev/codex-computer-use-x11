## ADR Review

## Existing In-Force ADRs

- `adr/README.md` lists ADR 0001, 0003, 0005, 0006, and 0007 as in force, and ADR 0002/0004 as superseded. The numbered files for those earlier ADRs were not present in this checkout, so this review used `ARCHITECTURE.md` and `adr/README.md` as the available durable context.
- New `adr/0008-adopt-x11-root-coordinate-model.md` is created by this change and is in force once committed.

## Constitution / Architecture Rules Considered

- Rust 2021 and root Cargo/Makefile are the implementation stack.
- `CODEX_DESKTOP_LINUX_FULL_PATH` target checkout is read-only for this change.
- No secrets or real credential values are required or read.
- OpenSpec artifacts remain source of truth; durable architecture rationale belongs in top-level `adr/` and the current snapshot in `ARCHITECTURE.md`.
- `x11-ewmh` remains the canonical backend id and upstream `WindowInfo` remains the primary model.
- Screenshot provider capability is separate from input capability; portal RemoteDesktop absence must not hide working screenshot methods.

## Decisions Evaluated

- Use X11 root/global pixel coordinates for bounds, pointer points, and screenshot crop rectangles rather than window-client-local or provider-specific coordinates.
- Preserve upstream-compatible `WindowBounds` with signed optional x/y and unsigned dimensions.
- Keep `wmctrl -lpGx` as primary standalone bounds source while reporting `xwininfo` alternates and source disagreement.
- Use standalone `gdbus` `ScreenshotArea` only as a validated live-smoke provider boundary; future source overlay should reuse target `screenshot.rs`.
- Defer MCP/get_app_state exposure to backlog 09b to avoid adding file-writing MCP tools in this coordinate-model stage.

## New Durable ADRs Created

- `adr/0008-adopt-x11-root-coordinate-model.md` — Accepted. Captures the X11 root coordinate model, bounds provenance rule, and standalone screenshot crop provider boundary.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- `ARCHITECTURE.md` updated to include ADR 0008 in current in-force ADRs and to summarize the coordinate-model/provider-boundary rule.
- `adr/README.md` updated to include ADR 0008 in current state.

## No ADR Needed

- N/A. A durable ADR is needed because this decision is hard to reverse, surprising without live geometry context, and drives future pointer/screenshot/`get_app_state` integration.
