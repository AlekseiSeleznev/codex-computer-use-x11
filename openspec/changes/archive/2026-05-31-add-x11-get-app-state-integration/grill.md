## Context Read

- `CONSTITUTION.md` — Rust 2021/Cargo, root `Makefile` checks, no secrets, target checkout read-only unless a change explicitly targets writes, OpenSpec validation required.
- `CONTEXT.md` — existing `x11-ewmh`, standalone plugin, source overlay, X11 root coordinate, bounds provenance, and AT-SPI terminology.
- `ARCHITECTURE.md`, `adr/README.md`, `adr/0008-adopt-x11-root-coordinate-model.md` — current lifecycle gates, root-coordinate model, screenshot provider boundary, and source-overlay guidance.
- `backlog/00-research-reuse-map.md` and `backlog/09b-get-app-state-integration.md` — stage scope, fresh-research requirement, TDD slices, and acceptance checks.
- Change artifacts: `proposal.md`, `specs/x11-get-app-state-integration/spec.md`, `specs/standalone-codex-mcp-plugin/spec.md`, `specs/doctor-cli/spec.md`.
- Current standalone code: `src/cli.rs`, `src/mcp.rs`, `src/list_windows.rs`, `src/input.rs`, `src/accessibility.rs`, `src/coordinates.rs`, `src/doctor.rs`, and current tests.
- Current target checkout (read-only): `computer-use-linux/src/server.rs`, `windowing/types.rs`, `windowing/target.rs`, `windowing/registry.rs`, `atspi_tree.rs`, `screenshot.rs`, `remote_desktop.rs`, `diagnostics.rs`.
- Project docs: `README.md`, `docs/integration-contract.md`.

## Plan Summary

- Add one standalone composed state surface: CLI `get-app-state --json` and MCP `x11_get_app_state`.
- Keep target compatibility by using stock target concepts (`window_context`, `window_error`, `screenshot`, `screenshot_error`, `accessibility_tree`, `accessibility_error`, `diagnostics`, `message`) instead of inventing a new Computer Use response vocabulary.
- Keep standalone plugin namespaced (`x11_get_app_state`) and document that future source overlay should improve target stock `get_app_state` through `x11-ewmh` windowing rather than add a competing stock tool.
- Preserve layer-degraded behavior: ambiguous/missing target, screenshot failure, or AT-SPI ambiguity should be visible per layer while other layers remain usable.
- Use fake-command TDD first; live Cinnamon/X11 smoke is evidence after unit/integration tests and may record degraded AT-SPI if local accessibility remains unavailable.

## Question Loop

### Q1: Should standalone `x11_get_app_state` return screenshot data URLs or only screenshot metadata?

- **Recommended answer:** Return target-compatible screenshot data (`mime_type`, `data_url`, `source`, `width`, `height`) when screenshot capture is requested, with `--no-screenshot` / `include_screenshot=false` to avoid large responses.
- **Rationale:** The target stock `GetAppStateOutput` returns a screenshot capture object, and backlog 09b is specifically about converging window context, screenshot, AT-SPI, and diagnostics. The prior `screenshot-crop` metadata-only rule was for crop files, not full app-state screenshots.
- **Resolution from repository context:** Adopt recommended answer. No user question needed because target `server.rs` already establishes the expected stock shape and the new spec includes opt-out behavior.

### Q2: Should app-state fail the whole report when target resolution or AT-SPI matching fails?

- **Recommended answer:** No. Return JSON with per-layer errors while preserving usable screenshot, diagnostics, and any successfully resolved layer.
- **Rationale:** Target `get_app_state` already separates `window_error`, `screenshot_error`, and `accessibility_error`. Existing standalone accessibility specs require ambiguity to avoid arbitrary subtree selection, not whole-report failure.
- **Resolution from repository context:** Adopt recommended answer. This is now named `Layer-degraded app state` in `CONTEXT.md`.

### Q3: Should this change patch `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`?

- **Recommended answer:** No. Inspect it read-only and document source-overlay guidance only.
- **Rationale:** README and integration contract say target remains read-only unless a later OpenSpec change explicitly modifies it. Backlog 09b accepts standalone `x11_get_app_state` or documented deferral, while target integration should map into existing stock `get_app_state` later.
- **Resolution from repository context:** Adopt recommended answer. No target checkout writes in this change.

## Resolved Terms

- `App state` — added to `CONTEXT.md` as the composed Computer Use read model for window, screenshot, accessibility, diagnostics, and message.
- `Layer-degraded app state` — added to `CONTEXT.md` for responses where one layer degrades but the report remains useful and explicit.

## Document Updates Applied

- Updated `CONTEXT.md` with `App state` and `Layer-degraded app state` glossary entries.
- The existing proposal/specs already reflect the grill decisions: target-compatible fields, namespaced standalone MCP tool, per-layer errors, and read-only target checkout.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- No new durable ADR is required at the pre-design gate. The decision to keep target checkout read-only and reuse target `get_app_state` concepts follows existing architecture/integration-contract rules; the root-coordinate/screenshot boundary is already covered by ADR 0008.

## Open Questions

None.
