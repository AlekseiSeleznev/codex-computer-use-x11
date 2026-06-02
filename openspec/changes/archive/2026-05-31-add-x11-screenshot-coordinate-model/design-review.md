## Context Read

- `openspec/changes/add-x11-screenshot-coordinate-model/proposal.md`
- `openspec/changes/add-x11-screenshot-coordinate-model/specs/x11-screenshot-coordinate-model/spec.md`
- `openspec/changes/add-x11-screenshot-coordinate-model/grill.md`
- `openspec/changes/add-x11-screenshot-coordinate-model/design.md`
- `CONSTITUTION.md`
- `CONTEXT.md`
- `ARCHITECTURE.md`
- `adr/README.md` (no numbered ADR files present before this change)
- `docs/integration-contract.md`
- `README.md`
- Source files: `src/list_windows.rs`, `src/pointer.rs`, `src/input.rs`, `src/focus.rs`, `src/doctor.rs`, `src/mcp.rs`
- Target files: `computer-use-linux/src/windowing/types.rs`, `server.rs`, `diagnostics.rs`, `screenshot.rs`, `remote_desktop.rs`
- Live research notes captured in `design.md`

## Design Summary

- The design uses a single X11 root/global pixel coordinate model for bounds, pointer points, and screenshot crop rectangles.
- It keeps `wmctrl -lpGx` as primary `WindowInfo` bounds source and surfaces `xwininfo` as alternate diagnostics when available.
- It validates crops before provider invocation, requires positive dimensions, refuses targeted crops outside target bounds, and clamps valid crops to known root screen geometry.
- It uses GNOME Shell-compatible DBus `ScreenshotArea` through `gdbus` only as a standalone live-smoke provider; future source overlay should reuse target `screenshot.rs`.
- It requires a durable ADR because the coordinate/provider boundary is a long-lived architecture rule.

## Question Loop

No user-facing material questions were needed.

Repository and live evidence resolved the review points:

1. **Should screenshot crop coordinates be window-local for convenience?**
   - Recommended answer: no; keep explicit global/root X11 coordinates and default to full target bounds when no crop is supplied.
   - Rationale: pointer commands already use global/root points, target screenshot providers operate in screen coordinates, and window-local offsets would require a separate conversion convention that could hide frame/client ambiguity.
   - Resolution: accepted from specs/design context.

2. **Should xwininfo replace wmctrl when they disagree?**
   - Recommended answer: no; report alternate provenance and disagreement, but keep primary `WindowInfo.bounds` stable.
   - Rationale: existing listing/focus/pointer behavior uses the `wmctrl` listing. Silent source replacement would make reports harder to compare across commands.
   - Resolution: accepted. The tasks must implement source-disagreement diagnostics.

3. **Can standalone screenshot capture leak sensitive screen data?**
   - Recommended answer: require an explicit output path and never serialize pixels/data URLs by default.
   - Rationale: screenshot files may contain private local desktop content. JSON metadata is enough for smoke evidence; tests can use fake providers.
   - Resolution: accepted; design/spec already state this.

4. **Should this add MCP tools now?**
   - Recommended answer: no, not for backlog 09. Add CLI behavior and reusable library functions; leave MCP/get_app_state exposure to backlog 09b unless a later task explicitly needs it.
   - Rationale: the existing standalone plugin spec currently lists prior tool surfaces, and `get_app_state` integration is a separate next backlog stage. Adding MCP screenshot file-write tools now would enlarge the security/review surface.
   - Resolution: accepted; no spec modification for MCP is required.

## Design Findings

- **Finding: README is stale.** It still describes stage 01 as having no live backend and its standalone MCP tool list omits pointer and AT-SPI tools from prior completed stages. This change should update README as part of docs work.
- **Finding: ADR files referenced by `ARCHITECTURE.md` are absent in this checkout.** The ADR review can still create `adr/0008-adopt-x11-root-coordinate-model.md`, but it should not rewrite history for missing prior ADRs. It should update `adr/README.md` and `ARCHITECTURE.md` to add the new ADR while preserving existing references.
- **Finding: source overlay must remain read-only.** All target research is inspection only; tasks must not modify `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`.
- **Finding: validation needs both parser-level and CLI-level tests.** Negative monitor offsets are unlikely to exist on the current live setup, so parser/validator tests must cover them with fixtures.

## Document Updates Applied

- No proposal/spec/design changes were required; the design already covers the findings.
- This design-review adds an explicit decision not to add MCP screenshot/bounds tools in backlog 09.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- Create `adr/0008-adopt-x11-root-coordinate-model.md` for the durable coordinate/provider boundary.

## Open Questions

None.
