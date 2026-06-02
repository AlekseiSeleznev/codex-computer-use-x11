## TDD Strategy

Use the project-local `tdd` skill with vertical slices. Each slice starts with one observable behavior test through the compiled CLI or MCP stdio server, confirms a RED failure, implements the smallest GREEN path, then refactors only while green. External X11 commands are simulated with fake executables in a temporary `PATH` so tests do not move or click the real desktop. Live Cinnamon/X11 smoke runs only after fake CLI/MCP tests pass and only against a disposable test window when available.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1 | CLI targeted click rejects an out-of-bounds point before focus/input | `cargo test --test pointer_actions_cli pointer_click_refuses_out_of_bounds_before_focus` fails because `click` is unsupported | Same test passes; JSON has `PointOutsideTargetBounds`, `input_sent=false`, and command log is empty | Bounds validation helper is shared by click/scroll/drag |
| 2 | CLI targeted click sends active-context xdotool after bounds + focus verification | `cargo test --test pointer_actions_cli pointer_click_invokes_xdotool_after_verified_focus` fails because no pointer pipeline/backend exists | Same test passes; log shows focus then `xdotool mousemove --sync ... click --repeat ...`, no `--window` | Report shape is stable and does not duplicate keyboard-only fields |
| 3 | CLI targeted pointer action blocks when focus verification mismatches | `cargo test --test pointer_actions_cli pointer_click_does_not_invoke_xdotool_when_focus_unverified` fails until pointer actions call focus verification | Same test passes; focus attempt may be logged, `xdotool` is absent, `error_code=FocusNotVerified` | Failure builders are shared across pointer actions |
| 4 | CLI scroll maps direction/amount to bounded wheel commands | `cargo test --test pointer_actions_cli pointer_scroll_maps_down_to_wheel_button_and_clamps_amount` fails until scroll command exists | Same test passes; `down` maps to button `5`, amount clamps to finite limit, input sent after verification | Direction parsing stays explicit and tested |
| 5 | CLI drag emits finite down/move/up and refuses huge drags | `cargo test --test pointer_actions_cli pointer_drag_refuses_huge_distance_without_xdotool` fails until drag validation exists | Same test passes; huge drag returns `DragDistanceTooLarge` and no backend command | Drag sequence construction stays isolated from safety gates |
| 6 | CLI explicit global mode is marked unverified and no-target without global is refused | `cargo test --test pointer_actions_cli pointer_global_click_is_explicitly_unverified` fails until `--global` mode exists | Same test passes; missing target without `--global` is `MissingTarget`, explicit global click can run with `verification_mode=global_unverified` | Global mode cannot accidentally set `targeted=true` |
| 7 | MCP tool list and pointer calls wrap pointer reports | `cargo test --test mcp_server mcp_server_lists_x11_tools` fails because only six tools are listed | `cargo test --test mcp_server` passes; list includes nine tools and `x11_click` missing target returns a JSON `MissingTarget` tool error | MCP validation delegates to pointer report builders |
| 8 | Full project verification and live/degraded smoke | `make test` or live smoke fails before all wiring is complete | `openspec validate add-x11-pointer-actions --strict`, `make fmt`, `make check`, `make test`, and safe live/degraded smoke pass or record blocker | No uncommitted target checkout changes or staged secrets |

## Mocking / Boundary Policy

- Use fake executable scripts in a temporary `PATH` for `wmctrl`, `xprop`, and `xdotool` to verify observable CLI behavior without live X11 side effects.
- Do not mock internal Rust collaborators; tests run the compiled binary through public CLI/MCP interfaces.
- Parser/validation helper unit tests may be added only if a pure helper becomes non-trivial; acceptance remains CLI/MCP behavior.
- Live smoke may use real `wmctrl`, `xprop`, `xdotool`, and `xmessage` only after fake tests are green and only with a disposable test window.

## Required Checks

- `openspec validate add-x11-pointer-actions --strict`
- `make fmt`
- `make check`
- `make test`
- Focused fake-command tests:
  - `cargo test --test pointer_actions_cli`
  - `cargo test --test mcp_server`
- Live/degraded smoke:
  - Create or identify a disposable X11 test window.
  - Run one targeted click, one targeted scroll, and one small targeted drag inside its reported bounds, or record exact degraded reason if a safe disposable target is unavailable.
  - Run one refusal smoke for stale target, out-of-bounds point, or missing target.
- Git status check for project and target checkout; ensure `.secrets.local.env` and target files are not staged or modified.

## Evidence Log

- Slice 1 RED: `cargo test --test pointer_actions_cli pointer_click_refuses_out_of_bounds_before_focus` failed because `click` was unsupported and wrote `unsupported command; try --help` to stderr.
- Slice 1 GREEN: same command passed after adding `src/pointer.rs`, public target resolution reuse, `click` CLI parsing, target bounds validation, and JSON `PointOutsideTargetBounds` failure with no focus/input log.
- Slice 2 RED: `cargo test --test pointer_actions_cli pointer_click_invokes_xdotool_after_verified_focus` failed because the report did not yet reach a successful pointer backend invocation.
- Slice 2 GREEN: same command passed after adding exact focus verification and active-context `xdotool mousemove --sync 50 60 click --repeat 2 1` with `used_direct_window=false`.
- Slice 3 RED: `cargo test --test pointer_actions_cli pointer_click_does_not_invoke_xdotool_when_focus_unverified` first failed because the assertion incorrectly treated `xdotool windowactivate --sync` focus fallback as pointer input; the test was corrected to forbid `xdotool mousemove` pointer commands.
- Slice 3 GREEN: corrected command passed; focus mismatch returns `FocusNotVerified`, `input_sent=false`, and no pointer `xdotool mousemove` command.
- Slice 4 RED: `cargo test --test pointer_actions_cli pointer_scroll_maps_down_to_wheel_button_and_clamps_amount` failed because `scroll` was unsupported.
- Slice 4 GREEN: same command passed after adding scroll CLI/report/backend; `down` maps to wheel button `5` and amount `99` clamps to `20`.
- Slice 5 RED: `cargo test --test pointer_actions_cli pointer_drag_refuses_huge_distance_without_xdotool` failed because `drag` was unsupported.
- Slice 5 GREEN: same command passed after adding drag CLI/report validation; drag delta `5000` returns `DragDistanceTooLarge` before focus/input. Regression `pointer_drag_invokes_finite_down_move_up_after_verified_focus` also passed, proving finite `mousedown`/`mousemove`/`mouseup` command sequencing.
- Slice 6 RED: `cargo test --test pointer_actions_cli pointer_global_click_is_explicitly_unverified` failed because explicit global mode still returned failure.
- Slice 6 GREEN: same command passed after implementing global pointer execution with `targeted=false`, `verification_mode=global_unverified`, and degraded `not window-isolated` diagnostics.
- CLI pointer group GREEN: `cargo test --test pointer_actions_cli` passed (7 tests).
- Slice 7 RED: `cargo test --test mcp_server mcp_server_lists_x11_tools` failed because `tools/list` still exposed only six tools and omitted `x11_click`, `x11_scroll`, and `x11_drag`.
- Slice 7 GREEN: `cargo test --test mcp_server mcp_server_lists_x11_tools` and `cargo test --test mcp_server mcp_targeted_input_tools_refuse_missing_target` passed after adding pointer tool definitions, schemas, runtime parsing, and report-builder wrappers. Full `cargo test --test mcp_server` passed (5 tests).

- Verification GREEN: `openspec validate add-x11-pointer-actions --strict` passed.
- Verification GREEN: `make fmt`, `make check`, and `make test` passed. Full `make test` included 40 unit tests plus integration tests: doctor CLI (2), focus CLI (8), list-windows CLI (3), MCP server (5), plugin installer (5), pointer actions CLI (7), targeted input CLI (6), and doc tests.
- Live smoke GREEN: launched disposable `xterm` titled `CodexPointerSmoke`; resolved window `119537678` with bounds `1930,192,484,134`; targeted `click` at `2172,259`, `scroll down` at `2172,259`, and small `drag` `2167,254 -> 2177,264` all returned `success=true` / `input_sent=true`; out-of-bounds click at `2424,259` returned `PointOutsideTargetBounds` with `input_sent=false`.
- Git/target safety GREEN: project and target checkout `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` had clean `git status --short` before final verification checkpoint; no `.secrets.local.env` or target files were modified or staged.

## TDD Exceptions

None
