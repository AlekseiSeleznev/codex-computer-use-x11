## TDD Strategy

Use the project-local `tdd` skill with vertical slices: one observable behavior test/check fails, then minimal production code makes it pass, then refactor only while green. Public interfaces are CLI JSON commands and MCP JSON-RPC/tool calls. Parser-only listing safety can use fixture/unit tests because it is a pure boundary parser. No production behavior is to be marked complete without RED/GREEN evidence recorded here or in the apply report.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1. Save/release target via CLI | `target-window --window-id ... --json`, `target-context --json`, `release-window --window-id ... --json` using fake `wmctrl`/`xprop` and `CODEX_X11_TARGET_STATE` | Add one CLI integration test in `tests/target_window_cli.rs`; run `cargo test --test target_window_cli saves_and_releases_target_window -- --nocapture`; expect unsupported command or missing JSON fields | Same command passes: target saved in state, context shows it, release removes it | Keep tests through binary public interface; no internal mocks except fake external commands/state path |
| 2. Refuse ambiguous target | CLI target save with duplicate title selectors | Add one CLI integration test; run `cargo test --test target_window_cli refuses_ambiguous_title_without_saving_state -- --nocapture`; expect command currently unsupported or state incorrectly saved | Test passes: non-zero status, JSON `success=false`, ambiguity detail, no state saved | Reuse `input::resolve_target`; do not duplicate resolver rules |
| 3. Stale target validation | CLI `target-context --json` after saved window disappears from fake listing | Add one CLI integration test; run `cargo test --test target_window_cli marks_vanished_target_stale -- --nocapture`; expect stale target still reported active or command unsupported | Test passes: vanished target appears in stale diagnostics and is removed/cleared from active state | Validation must run before context reports current state |
| 4. Group idempotence and move semantics | CLI target same/different windows/groups | Add one CLI integration test; run `cargo test --test target_window_cli groups_are_idempotent_and_move_existing_window -- --nocapture`; expect duplicates or unsupported command | Test passes: same window not duplicated; retargeting to a different group moves it; active target updates deterministically | Keep group id normalization simple and deterministic |
| 5. Overlay warning boundary | CLI `target-window --overlay --json` with production no-overlay provider | Add one CLI integration test; run `cargo test --test target_window_cli overlay_failure_is_warning_and_target_is_saved -- --nocapture`; expect unsupported command or target failure | Test passes: command succeeds, target saved, `overlay.requested=true`, `shown=false`, warning present | Overlay must not become a hard dependency |
| 6. Owned overlay listing safety | `list-windows --json` parser/CLI with fake internal overlay row | Add one listing parser/CLI test; run `cargo test --test list_windows_cli excludes_project_owned_overlay_windows -- --nocapture`; expect overlay row appears as normal window | Test passes: overlay/helper row absent from primary `windows` and present/marked in diagnostics | Keep primary `WindowInfo` upstream-compatible; internal markers stay in diagnostics |
| 7. MCP target tools and process state | `codex-computer-use-x11 mcp` JSON-RPC `tools/list`, `x11_target_window`, `x11_target_context`, `x11_release_window` | Add one MCP integration test; run `cargo test --test mcp_server mcp_server_tracks_target_window_context -- --nocapture`; expect tools missing or state not retained | Test passes: new tools listed in deterministic order; target saved/context/release works in one MCP process | Refactor MCP state passing without changing existing tool results |
| 8. Docs and live/degraded smoke | README/integration docs plus live Cinnamon/X11 command smoke | Add docs assertions or grep checks as needed; run `cargo test docs_include_target_window_guidance -- --nocapture` or equivalent; expect missing docs | Docs test/check passes; live smoke: `cargo run -- target-context --json`, target a current window if available, overlay unsupported warning is clear | Live smoke supplements tests, never replaces RED/GREEN tests |

## Mocking / Boundary Policy

- Use fake `PATH` scripts for `wmctrl` and `xprop`, matching existing CLI tests.
- Use `CODEX_X11_TARGET_STATE` pointing at a temp file for CLI state isolation.
- Do not mock `input::resolve_target`; exercise it through public CLI/MCP behavior.
- Parser tests may call pure parser functions directly for owned-overlay row handling.
- Production overlay provider is `NoOverlayProvider`; tests may use a fake provider only around the overlay boundary or assert public no-overlay CLI JSON.
- Do not invoke real `xdotool`, focus, input, screenshot, or AT-SPI in unit/CLI RED/GREEN tests for this change.

## Required Checks

- `openspec validate add-x11-target-window-groups-overlays --strict`
- Focused RED/GREEN commands listed per slice, recorded with expected failure and pass.
- `make fmt`
- `make check`
- `make test`
- Live/degraded Cinnamon/X11 smoke after automated tests are green:
  - `cargo run -- target-context --json`
  - If `list-windows --json` returns a safe current application window, run `cargo run -- target-window --window-id <id> --overlay --json` and verify target saved with overlay warning or success.
  - `cargo run -- release-window --all --json`
- Confirm target checkout remains clean: `git -C "$CODEX_DESKTOP_LINUX_FULL_PATH" status --short` or documented default path equivalent.
- Confirm no secret/local state files are staged: `git status --short` and no `.secrets.local.env` tracked.

## Evidence Log

- Slice 1 RED: `cargo test --test target_window_cli saves_and_releases_target_window -- --nocapture` -> failed as expected before production code because `target-window` was unsupported (`unsupported command; try --help`).
- Slice 1 GREEN: `cargo test --test target_window_cli saves_and_releases_target_window -- --nocapture` -> passed after adding `src/target_window.rs`, CLI parsing, file-backed state, and save/context/release JSON reports.

- Slice 2 RED: baseline worktree at pre-implementation commit `a54e04e` with `tests/target_window_cli.rs::refuses_ambiguous_title_without_saving_state` -> failed as expected because `target-window` was unsupported.
- Slice 2 GREEN: `cargo test --test target_window_cli refuses_ambiguous_title_without_saving_state -- --nocapture` -> passed; ambiguous title returns JSON `success=false`, `error_code=AmbiguousTarget`, candidates are reported, and state remains empty.
- Slice 3 RED: baseline worktree at `a54e04e` with `tests/target_window_cli.rs::marks_vanished_target_stale` -> failed as expected because target save/context was unsupported.
- Slice 3 GREEN: `cargo test --test target_window_cli marks_vanished_target_stale -- --nocapture` -> passed; vanished target is reported in `diagnostics.stale_removed` and active state is cleared.
- Slice 4 RED: baseline worktree at `a54e04e` with `tests/target_window_cli.rs::groups_are_idempotent_and_move_existing_window` -> failed as expected because target save was unsupported.
- Slice 4 GREEN: `cargo test --test target_window_cli groups_are_idempotent_and_move_existing_window -- --nocapture` -> passed; duplicate targets are not created, active target updates, and retargeting a window moves it to the new group.
- Slice 5 RED: baseline worktree at `a54e04e` with `tests/target_window_cli.rs::overlay_failure_is_warning_and_target_is_saved` -> failed as expected because target save/overlay was unsupported.
- Slice 5 GREEN: `cargo test --test target_window_cli overlay_failure_is_warning_and_target_is_saved -- --nocapture` -> passed; production no-overlay provider returns a warning while target save succeeds.
- Slice 6 RED: `cargo test --test list_windows_cli excludes_project_owned_overlay_windows -- --nocapture` -> failed as expected because the overlay/helper row appeared as a normal primary window (`windows` length 2 instead of 1).
- Slice 6 GREEN: `cargo test --test list_windows_cli excludes_project_owned_overlay_windows -- --nocapture` -> passed after adding owned/internal metadata and filtering project-owned overlay/helper rows from primary windows.
- Refactor/check evidence: `cargo test --test target_window_cli -- --nocapture` -> 5 passed; `cargo test --test list_windows_cli excludes_project_owned_overlay_windows -- --nocapture` -> passed.
- Slice 7 RED: baseline worktree at `6cc690d` with `tests/mcp_server.rs::mcp_server_tracks_target_window_context` -> failed as expected because `x11_target_window` was absent from `tools/list`.
- Slice 7 GREEN: `cargo test --test mcp_server mcp_server_tracks_target_window_context -- --nocapture` -> passed after adding per-process MCP target state and `x11_target_window`/`x11_target_context`/`x11_release_window` tools.
- Slice 7 malformed RED: baseline worktree at `6cc690d` with `tests/mcp_server.rs::mcp_target_window_rejects_malformed_arguments` -> failed as expected because target-window tool/error handling was absent.
- Slice 7 malformed GREEN: `cargo test --test mcp_server mcp_target_window_rejects_malformed_arguments -- --nocapture` -> passed; malformed `window_id` and invalid color return MCP tool errors and context remains empty.
- Refactor/check evidence: `cargo test --test mcp_server -- --nocapture` -> 11 passed, including existing MCP regressions and new target tools.
- Slice 8 docs RED: `grep -q "target-context" README.md` -> failed as expected before docs update because target-window CLI guidance was absent.
- Slice 8 docs GREEN: `grep -q "target-context" README.md`, `grep -q "x11_target_window" README.md`, and `grep -q "Target-window groups" docs/integration-contract.md` -> passed after README/integration-contract updates.
- Verification check: initial `make fmt` failed on `tests/mcp_server.rs` formatting; ran `cargo fmt` and reran checks.
- Verification GREEN: `make fmt`, `make check`, `make test`, and `openspec validate add-x11-target-window-groups-overlays --strict` -> all passed. `make test` included 41 unit tests and integration suites for accessibility, doctor, focus, app-state, listing, MCP, plugin installer, pointer actions, screenshot coordinates, target-window CLI, and targeted input.
- Live/degraded Cinnamon/X11 smoke GREEN: with isolated `CODEX_X11_TARGET_STATE`, `cargo run --quiet -- target-context --json` returned `success=true`, empty groups, and diagnostics ok; selected live window `0x4400009`; `cargo run --quiet -- target-window --window-id 0x4400009 --group live-smoke --color green --overlay --json` returned `success=true` with `overlay.requested=true`, `shown=false`, provider `no-overlay`, and clear warning; `cargo run --quiet -- release-window --all --json` returned `success=true`, `released_count=1`, remaining targets 0.
- Safety check: target checkout `git -C "$CODEX_DESKTOP_LINUX_FULL_PATH" status --short` returned clean; `git ls-files .secrets.local.env` returned no tracked file.

Required format per slice:

- Slice N RED: `<command>` -> expected failure summary.
- Slice N GREEN: `<command>` -> pass summary.
- Refactor/check evidence: command(s) and result.

## TDD Exceptions

None.
