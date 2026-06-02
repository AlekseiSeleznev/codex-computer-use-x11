## TDD Strategy

Apply the project-local `tdd` skill with small vertical RED -> GREEN -> REFACTOR slices. Each behavior-changing slice starts with one failing test/check against an observable interface before production code. Parser tests are allowed because `wmctrl`/`xprop` output parsing is a system-boundary adapter; CLI/service tests must still prove the public `list-windows --json` behavior. Do not write a horizontal batch of all tests before all implementation.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1 | CLI recognizes `list-windows --json` and emits degraded JSON without `DISPLAY` | `cargo test list_windows_cli_degrades_without_display` fails because the command is unsupported or no listing report exists | Same test passes with JSON containing `project`, `version`, `backend`, empty `windows`, and `diagnostics` explaining no display | Keep CLI dispatch small; preserve existing `doctor --json` tests/behavior |
| 2 | `wmctrl -lpGx` parser maps one normal row into a WindowInfo-compatible object | `cargo test wmctrl_parser_maps_normal_window` fails because parser/module is absent | Same test passes for id, workspace, pid, bounds, title, `wm_class`, `app_id`, `backend`, `focused=false`, `hidden=false` | Parser is pure and independent from live X11 commands |
| 3 | Parser preserves titles with spaces, Cyrillic, emoji, and multibyte characters | `cargo test wmctrl_parser_preserves_unicode_title_remainder` fails on missing parser or split behavior | Same test passes using max-split parsing so title remainder is intact | Avoid title trimming beyond removing row terminator; no lossy byte handling |
| 4 | Geometry/id validation handles padded ids, negative coordinates, and invalid dimensions | `cargo test wmctrl_parser_validates_ids_and_geometry` fails because edge cases are not handled | Same test passes: padded/unpadded ids normalize, negative x/y are preserved, invalid/non-positive dimensions are degraded without unsigned wraparound | Keep canonical id parsing in `src/x11_id.rs`; do not duplicate hex parsing |
| 5 | `WM_CLASS` and PID reliability sidecar behavior is deterministic | `cargo test list_windows_reports_class_and_pid_reliability` fails because diagnostics are absent | Same test passes: raw class stored in diagnostics, `instance.Class` maps to `app_id`/`wm_class`, no-dot fallback is recorded, PID 0/2 or non-local host is unreliable | Keep primary window shape free of raw/provenance fields |
| 6 | Active-window focus is marked from `_NET_ACTIVE_WINDOW` | `cargo test list_windows_marks_focused_window_from_xprop` fails because active id lookup is absent | Same test passes with fake `xprop -root` output and exactly one matching window `focused=true` | Active id parser remains pure; lookup failure degrades instead of aborting listing |
| 7 | Missing `wmctrl` and command failures produce structured degraded JSON | `cargo test list_windows_degrades_when_wmctrl_missing_or_fails` fails because missing command is not represented | Same test passes with fake runner/PATH: `windows=[]`, status 0 for JSON report, diagnostics blockers/degraded reasons | Do not leak unrelated stderr or raw live titles into diagnostics |
| 8 | MVP does not perform unbounded per-window type/state enrichment | `cargo test list_windows_does_not_spawn_unbounded_xprop_per_window` fails if command calls are untracked or unbounded | Same test passes by proving no per-window `xprop -id` calls by default, or a documented bound if enrichment is implemented | If enrichment is added, keep bound/caching explicit and fixture-tested |
| 9 | Public CLI success path with fake commands returns valid JSON windows | `cargo test list_windows_cli_outputs_windows_with_fake_commands` fails until CLI/service/parser connect end-to-end | Same test passes with fake `wmctrl` and fake `xprop` producing a JSON object whose `windows[]` satisfy the spec | Keep serialization stable; do not require live X11 for automated tests |
| 10 | Live Cinnamon/X11 smoke after automated GREEN | `cargo run -- list-windows --json` manually/smoke-checked after `make test`; expected initial RED is not applicable because this is post-unit smoke evidence | Command exits 0 on the local X11 session and returns at least a valid JSON object; record only status/counts, not live titles | Live smoke must not replace automated unit/CLI tests and must not leak sensitive window titles |

## Mocking / Boundary Policy

- Mock or fake only system boundaries: `wmctrl`, `xprop`, `PATH`, `DISPLAY`, hostname/client-machine facts, and command exit status.
- Do not mock internal parser or mapping functions; test them directly with fixtures where they are pure boundary adapters.
- The standalone crate may use a `CommandRunner` trait or fake `PATH` scripts for command behavior. This does not authorize adding a dependency-injection runner to the Codex Desktop Linux target repo.
- Do not require live X11, real window titles, or the machine-local target checkout for automated tests.
- External project code remains ideas-only unless a later implementation explicitly copies compatible licensed code with attribution.

## Required Checks

- `openspec validate add-x11-window-listing --strict` or the project-supported equivalent OpenSpec validation command.
- `make fmt`
- `make check`
- `make test`
- `cargo run -- doctor --json` still emits valid JSON and preserves bootstrap field compatibility.
- `cargo run -- list-windows --json` live smoke on the local Cinnamon/X11 session after automated checks are green; record status and window count only.
- `git status --short` before and after apply/verification; ensure no local secret files or target-checkout files are staged.

## Evidence Log

- Slice 1 CLI degraded no-DISPLAY:
  - RED: `cargo test list_windows_cli_degrades_without_display` failed because `list-windows --json` was unsupported and exited non-zero.
  - GREEN: `cargo test list_windows_cli_degrades_without_display` passed after adding CLI dispatch and degraded `WindowListReport` JSON.
- Slices 2-4 `wmctrl -lpGx` parser:
  - RED: `cargo test wmctrl_parser_maps_normal_window` failed to compile with missing `parse_wmctrl_lpgx`; the compile failure also covered the newly-added Unicode and geometry parser tests because the parser did not exist.
  - GREEN: `cargo test wmctrl_parser_maps_normal_window`, `cargo test wmctrl_parser_preserves_unicode_title_remainder`, and `cargo test wmctrl_parser_validates_ids_and_geometry` passed after adding pure parser, max-split title handling, canonical id normalization, negative coordinate preservation, and invalid-dimension degradation.
- Slice 5 sidecar class/PID metadata:
  - RED: class/PID sidecar assertions were added before report/metadata plumbing was complete; the parser test set could not compile/run until the missing listing functions were added.
  - GREEN: `cargo test list_windows_reports_class_and_pid_reliability` passed with raw class diagnostics, deterministic `instance.Class` / no-dot fallback mapping, and unreliable PID handling.
- Slice 6 focus from `_NET_ACTIVE_WINDOW`:
  - RED: `cargo test list_windows_marks_focused_window_from_xprop` failed to compile with missing `parse_active_window_xprop` and `mark_focused_window`.
  - GREEN: `cargo test list_windows_marks_focused_window_from_xprop` passed after adding active-id parsing and focus marking.
- Slice 7 active lookup degradation and bounded enrichment:
  - RED: `cargo test list_windows_degrades_when_wmctrl_missing_or_fails` failed to compile with missing `WindowListProbeFacts` and `report_from_probe`; the active-lookup and enrichment-boundary tests were present but blocked by the same missing report seam.
  - GREEN: `cargo test list_windows_degrades_when_wmctrl_missing_or_fails`, `cargo test list_windows_degrades_when_active_lookup_fails`, and `cargo test list_windows_does_not_spawn_unbounded_xprop_per_window` passed after adding report seam, command diagnostics, active lookup degradation, and disabled-by-default per-window enrichment.
- Slice 8 public fake-command CLI:
  - GREEN: `cargo test list_windows_cli_outputs_windows_with_fake_commands` passed with fake `wmctrl`/`xprop` commands, two serialized windows, focused-window id, and `xprop_id_calls=0`.
- Slice 9 doctor compatibility and unsupported usage:
  - GREEN: `cargo test doctor_cli_success_json`, `cargo test doctor_cli_arguments`, and `cargo test list_windows_cli_rejects_unsupported_usage` passed.
- Full automated verification:
  - `cargo test` passed: 38 unit tests, 2 `doctor_cli` tests, and 3 `list_windows_cli` tests.
  - Initial `make fmt` reported formatting diffs; after `cargo fmt`, `make fmt`, `make check`, and `make test` all passed.
  - `cargo run -- doctor --json` produced valid JSON with `project=codex-computer-use-x11`, `version=0.1.0`, `backend=x11-ewmh`, boolean readiness, and checks array.
  - `cargo run -- list-windows --json` live smoke produced valid JSON on local Cinnamon/X11 with `windows=15`, `diagnostics.ok=true`, `focused_window=65011716`, and `xprop_id_calls=0`; live titles were not recorded.
  - `openspec validate add-x11-window-listing --strict` passed.

## TDD Exceptions

None.
