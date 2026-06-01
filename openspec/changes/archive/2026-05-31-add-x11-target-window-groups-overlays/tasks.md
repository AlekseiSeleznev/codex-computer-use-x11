## 1. CLI target-window state TDD

- [x] 1.1 RED: Add `tests/target_window_cli.rs::saves_and_releases_target_window` using fake `wmctrl`/`xprop` and `CODEX_X11_TARGET_STATE`; record the failing `cargo test --test target_window_cli saves_and_releases_target_window -- --nocapture` evidence in `test-plan.md`.
- [x] 1.2 GREEN: Add `src/target_window.rs`, CLI parsing for `target-window`, `target-context`, and `release-window`, file-backed CLI state, JSON reports, and minimal save/context/release behavior until slice 1 passes.
- [x] 1.3 RED/GREEN: Add ambiguity refusal test for `target-window --title` with duplicate matches, implement strict reuse of `input::resolve_target()`, and record evidence.
- [x] 1.4 RED/GREEN: Add stale validation test for vanished saved targets, implement stale removal/active-target clearing before context reports, and record evidence.
- [x] 1.5 RED/GREEN: Add group idempotence and one-owner move-semantics test, implement deterministic group ids/colors/active windows, and record evidence.

## 2. Overlay boundary and listing safety TDD

- [x] 2.1 RED: Add CLI/public behavior test proving `--overlay` warning does not fail target save when the production overlay provider is unsupported; record evidence.
- [x] 2.2 GREEN: Implement overlay report fields and `NoOverlayProvider` behavior so target save succeeds with `overlay.requested=true`, `overlay.shown=false`, and a warning.
- [x] 2.3 RED: Add parser/CLI test proving `codex-computer-use-x11-overlay` / helper rows are excluded or marked internal in `list-windows --json`; record evidence.
- [x] 2.4 GREEN: Extend `WindowMetadata` and `parse_wmctrl_lpgx()` to mark/exclude project-owned internal overlay/helper rows without adding X11-only fields to primary `WindowInfo`.

## 3. MCP target tools TDD

- [x] 3.1 RED: Extend `tests/mcp_server.rs` with `mcp_server_tracks_target_window_context` proving `tools/list` includes `x11_target_window`, `x11_release_window`, and `x11_target_context`, and that target state persists within one MCP process; record evidence.
- [x] 3.2 GREEN: Refactor `src/mcp.rs` to carry mutable per-process target state, add target tool schemas and tool calls, and preserve existing MCP tool outputs.
- [x] 3.3 RED/GREEN: Add malformed target-window argument MCP test and implement tool-error handling without saving target state.

## 4. Documentation and integration guidance

- [x] 4.1 Update `README.md` with CLI examples for `target-window`, `target-context`, `release-window`, overlay degraded behavior, and MCP tool names.
- [x] 4.2 Update `docs/integration-contract.md` with source-overlay guidance: target groups are standalone context UX; future target integration should use existing stock target-resolution/windowing concepts.
- [x] 4.3 Add or update docs checks/tests if feasible, then record evidence in `test-plan.md`.

## 5. Verification, live smoke, and safety checks

- [x] 5.1 Run `make fmt`, `make check`, and `make test`; fix issues and record final evidence.
- [x] 5.2 Run `openspec validate add-x11-target-window-groups-overlays --strict` and record evidence.
- [x] 5.3 Run live/degraded Cinnamon/X11 smoke: `cargo run -- target-context --json`, target one current safe window with `--overlay --json` when available, and `cargo run -- release-window --all --json`; record overlay warning/success evidence.
- [x] 5.4 Verify target checkout remains read-only/clean and no local secret/session state files are staged; record `git status --short` evidence.
- [x] 5.5 Mark tasks complete only after matching RED/GREEN/check evidence is present in `test-plan.md`.
