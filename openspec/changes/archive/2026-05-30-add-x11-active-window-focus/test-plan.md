## TDD Strategy

Apply the project-local `tdd` skill with vertical public-interface slices. Each behavior starts with one failing test/check, then the minimum production code needed for GREEN, then refactor only while the focused slice and surrounding checks are GREEN. CLI tests use the built binary and fake boundary commands on `PATH`; pure parser tests are allowed only for the `_NET_ACTIVE_WINDOW` parser because it is a deterministic system-boundary parser. No production focus behavior should be implemented before its RED evidence is recorded here.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1 | Parse `_NET_ACTIVE_WINDOW` as active id vs explicit no-active/missing/invalid state | Add one parser test in `src/focus.rs` for `window id # 0x123456`, `0x0`, missing property, and invalid text; run `cargo test focus::tests::active_window_parser_classifies_states`. Expected RED: unresolved module/function or failing assertions. | Implement typed active-window parser; same command passes. | Parser is small, deterministic, and reused by CLI/listing paths without duplicating id parsing. |
| 2 | `focused-window --json` reports matched focused window through the public CLI | Add one integration test using fake `wmctrl` and `xprop`; run `cargo test focused_window_cli_reports_matched_active_window`. Expected RED: unsupported command or missing JSON shape. | Add CLI arm and focused-window report using current listing plus active-window match; same command passes. | Keep JSON additive and reuse `WindowInfo`; no activation code yet. |
| 3 | `focused-window --json` degrades for no-active and active-not-in-list | Add one or two focused CLI tests for `0x0` and unmatched active id; run the focused test names. Expected RED: missing degradation/null behavior. | Extend focused-window reporting diagnostics; tests pass. | Avoid turning no-active into a hard blocker when JSON can be emitted. |
| 4 | `focus-window --window-id <id> --json` rejects invalid ids and missing windows before activation | Add integration tests with fake commands and activation log; run `cargo test focus_window_cli_rejects_invalid_id_before_activation focus_window_cli_reports_window_not_found_without_activation`. Expected RED: unsupported command or activation attempted. | Add CLI argument parsing, id normalization, target resolution, JSON `WindowNotFound`, and invalid-id stderr; tests pass. | Keep unsupported usage behavior compatible with existing CLI. |
| 5 | `focus-window` succeeds only after `wmctrl -ia` plus fresh active-window verification | Add fake-command integration test where `wmctrl -ia` updates active id and `xprop` verifies it; run `cargo test focus_window_cli_verifies_wmctrl_activation`. Expected RED: missing activation behavior. | Implement `wmctrl -ia 0x<id>` attempt, verification helper, success JSON, and diagnostics; test passes. | Keep activation and verification helpers separated enough to test through CLI without mocking internal code. |
| 6 | `FocusNotVerified` when activation exits success but active id mismatches | Add fake-command integration test where activation commands do not update the active id; run `cargo test focus_window_cli_reports_focus_not_verified_on_mismatch`. Expected RED: command incorrectly succeeds or no error code. | Return non-zero with JSON `success:false`, `error_code:"FocusNotVerified"`, and matched last focused window when available; test passes. | Do not add aggressive retries beyond bounded design; maintain safe failure. |
| 7 | Fallback from failed `wmctrl` to `xdotool windowactivate --sync` still verifies focus | Add fake-command integration test where `wmctrl -ia` fails, `xdotool` updates active id, and `xprop` verifies; run `cargo test focus_window_cli_falls_back_to_xdotool`. Expected RED: fallback missing. | Add ordered fallback attempt and diagnostics; test passes. | Attempts remain ordered and machine-readable; final verification remains shared. |
| 8 | Full project and live smoke checks | Run `make fmt`, `make check`, `make test`, `cargo run -- focused-window --json`, and a cautious `cargo run -- focus-window --window-id <current-active-id> --json`. Expected RED only if previous slices missed integration issues. | All checks pass or any live focus refusal is recorded as `FocusNotVerified` evidence with exact command output. | No refactor if it changes public JSON shape without spec update. |

## Mocking / Boundary Policy

- Use fake executable scripts in temporary directories for `wmctrl`, `xprop`, and `xdotool` to simulate the X11 boundary deterministically through the public CLI.
- Do not mock internal Rust collaborators; tests should launch `CARGO_BIN_EXE_codex-computer-use-x11` or exercise the pure active-window parser directly.
- Fake commands may use temp files to model active-window state and activation logs.
- Live smoke is supplementary because it can move real desktop focus and the window manager may legitimately refuse activation.

## Required Checks

- `openspec validate add-x11-active-window-focus --type change --strict` before apply and before archive.
- Per-slice RED and GREEN `cargo test <test-name>` commands.
- Final Rust checks: `make fmt`, `make check`, `make test`.
- JSON smoke: `cargo run -- focused-window --json` emits valid JSON; `cargo run -- focus-window --window-id <current-active-id> --json` either verifies focus or returns structured `FocusNotVerified`.
- Git status clean or only expected archive/sync changes before archive/commit/push.

## Evidence Log

- Slice 1 RED (2026-05-30): `cargo test focus::tests::active_window_parser_classifies_states` failed with unresolved `parse_active_window_xprop_state`/`ActiveWindowState` in `src/focus.rs` before production parser existed.
- Slice 1 GREEN (2026-05-30): `cargo test focus::tests::active_window_parser_classifies_states` passed after adding typed parser in `src/focus.rs` and exposing `pub mod focus`.

- Slice 2 RED (2026-05-30): `cargo test focused_window_cli_reports_matched_active_window` failed because `focused-window --json` was unsupported and exited non-zero.
- Slice 2 GREEN (2026-05-30): `cargo test focused_window_cli_reports_matched_active_window` passed after adding `focused-window --json`, `FocusedWindowReport`, and reusing listing/active-window matching.
- Slice 3 RED (2026-05-30): `cargo test focused_window_cli_degrades` failed for the no-active case because `0x0` did not produce the required `no active X11 window` degradation.
- Slice 3 GREEN (2026-05-30): `cargo test focused_window_cli_degrades` passed after classifying `ActiveWindowState::NoActive` in listing/focused diagnostics.

- Slice 4 RED (2026-05-30): `cargo test focus_window_cli_` failed for invalid id/missing window because `focus-window --window-id <id> --json` was unsupported and did not return `WindowNotFound` JSON.
- Slice 4 GREEN (2026-05-30): `cargo test focus_window_cli_` passed after adding focus-window CLI parsing, invalid-id stderr handling, current-listing target resolution, and JSON `WindowNotFound` without activation attempts.

- Slice 5 RED (2026-05-30): `cargo test focus_window_cli_verifies_wmctrl_activation` failed because found-window focus still returned non-success and no activation verification existed.
- Slice 5 GREEN (2026-05-30): `cargo test focus_window_cli_verifies_wmctrl_activation` passed after parsing decimal ids, running `wmctrl -ia 0xa`, verifying with fresh `xprop`, returning `success:true`, and normalizing `focused_window.focused`.

- Slice 6 RED (2026-05-30): `cargo test focus_window_cli_reports_focus_not_verified_on_mismatch` failed because focus mismatch JSON lacked a degraded reason explaining the observed active id vs requested id.
- Slice 6 GREEN (2026-05-30): `cargo test focus_window_cli_reports_focus_not_verified_on_mismatch` passed after adding mismatch diagnostics while preserving non-zero JSON `FocusNotVerified` behavior.

- Slice 7 RED (2026-05-30): `cargo test focus_window_cli_falls_back_to_xdotool` failed because `focus-window` did not attempt `xdotool windowactivate --sync` after `wmctrl -ia` failed.
- Slice 7 GREEN (2026-05-30): `cargo test focus_window_cli_falls_back_to_xdotool` passed after adding ordered `xdotool windowactivate --sync <decimal-id>` fallback with the same fresh active-window verification boundary.

- Slice 8 GREEN/final checks (2026-05-30): `cargo fmt`; `make fmt`; `make check`; `make test` passed with 40 lib tests, 2 doctor CLI tests, 8 focus CLI tests, 3 list-windows CLI tests, and doc tests.
- Slice 8 OpenSpec (2026-05-30): `openspec validate add-x11-active-window-focus --type change --strict` passed.
- Slice 8 live JSON smoke (2026-05-30): `cargo run -- focused-window --json` emitted valid JSON and matched current active Codex window id `65011716`; `cargo run -- focus-window --window-id 65011716 --json` returned `success:true`, `exact_window_focused:true`, and a `wmctrl -ia 0x3e00004` activation attempt.
- Slice 8 target checkout guard (2026-05-30): `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` git status was clean after read-only research and live smoke.
- Apply preflight (2026-05-30): `git status --short` was clean before apply, `openspec validate add-x11-active-window-focus --type change --strict` passed, and `openspec instructions apply --change add-x11-active-window-focus --json` was read.

## TDD Exceptions

None.
