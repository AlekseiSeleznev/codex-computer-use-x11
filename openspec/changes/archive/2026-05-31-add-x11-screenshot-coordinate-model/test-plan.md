## TDD Strategy

Use the project-local `tdd` skill with vertical RED → GREEN → REFACTOR slices. Tests will exercise public CLI behavior and report JSON shapes through `env!("CARGO_BIN_EXE_codex-computer-use-x11")` with fake `PATH` command fixtures, plus pure parser/validator unit tests only for boundary-heavy coordinate math that cannot be reliably exercised on the live desktop. Production code must not be written for a slice before the RED failure is observed.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1 | `window-bounds --window-id <id> --json` reports upstream-compatible signed bounds and coordinate metadata | Add one CLI test in `tests/screenshot_coordinate_cli.rs` with fake `wmctrl` negative x fixture; run `cargo test --test screenshot_coordinate_cli window_bounds_reports_signed_root_coordinates -- --nocapture`; expected: unsupported command or missing fields failure | Same command passes with JSON `success=true`, `bounds.x=-1280`, `coordinate_model.space=x11_root_global_pixels`, and provenance diagnostics | Shared JSON writing and window lookup should avoid duplicating existing CLI patterns |
| 2 | `window-bounds` surfaces `xwininfo` alternate bounds disagreement without replacing primary bounds | Add one CLI test with fake `wmctrl` x=3840 and fake `xwininfo` x=1920; run `cargo test --test screenshot_coordinate_cli window_bounds_reports_xwininfo_disagreement -- --nocapture`; expected: missing alternate diagnostics failure | Same command passes with primary `bounds.x=3840`, alternate source `x=1920`, `bounds_agree=false`, degraded reason | Parser should be pure and fixture-testable; command absence remains degraded, not fatal |
| 3 | Crop validation rejects invalid/outside targeted rectangles before provider invocation | Add one CLI test where fake provider logs calls but requested crop is outside target; run `cargo test --test screenshot_coordinate_cli screenshot_crop_refuses_outside_target_before_provider -- --nocapture`; expected: command unsupported or provider invoked incorrectly | Same command passes with non-zero exit, `error_code=CropOutsideTargetBounds`, `screenshot_invoked=false`, and empty provider log | Crop validation should be reusable by future get_app_state integration; no screenshot side effects before validation |
| 4 | `screenshot-crop` defaults to full target bounds and invokes GNOME Shell-compatible `ScreenshotArea` with exact validated crop | Add one CLI test with fake `gdbus` logging arguments and touching output; run `cargo test --test screenshot_coordinate_cli screenshot_crop_invokes_gdbus_with_validated_rect -- --nocapture`; expected: command unsupported/provider call missing | Same command passes with `screenshot_invoked=true`, provider `gnome_shell_screenshot_area`, output path, no data URL, and log containing `ScreenshotArea 10 20 800 600 false <path>` | Provider invocation should stay isolated behind a small boundary; no real screenshot in tests |
| 5 | Screen geometry parser supports negative monitor offsets and clamped crop intersections | Add one unit test for `xrandr --listmonitors` negative offset fixture and crop clamp; run `cargo test coordinates::tests::xrandr_negative_offsets_define_root_geometry -- --nocapture`; expected: module/test missing | Same command passes with root geometry spanning negative x/y and crop clamp diagnostics | Keep coordinate math independent from command execution |
| 6 | Documentation and ADR snapshot reflect coordinate model and command usage | Add/update docs checks through `rg`/`cargo test` assertions as appropriate; run `openspec validate add-x11-screenshot-coordinate-model --strict --no-interactive`; expected during RED: docs text missing or tasks incomplete | Validation passes and README mentions `window-bounds`, `screenshot-crop`, root coordinates, and no pixel/data-url output by default | Docs should be concise and not include live private window titles or screenshot data |

## Mocking / Boundary Policy

- Fake only external system commands (`wmctrl`, `xprop`, `xwininfo`, `xrandr`, `xdpyinfo`, `gdbus`) through temporary `PATH` fixtures.
- Do not mock internal collaborators. Keep parser/validator functions deterministic and unit-test them directly only where the public CLI cannot create the required live geometry fixture.
- Live Cinnamon/X11 smoke runs only after automated tests are green and must avoid printing screenshot bytes or private window titles in committed artifacts.
- The target checkout remains read-only.

## Required Checks

Before apply complete:

- `openspec validate add-x11-screenshot-coordinate-model --strict --no-interactive`
- `make fmt`
- `make check`
- `make test`
- Focused RED/GREEN commands recorded in this evidence log
- Live smoke for `window-bounds` and `screenshot-crop` on a temporary output file when a safe active window is available; delete the output file after checking existence

Before archive:

- `openspec validate --all --strict --no-interactive`
- `make fmt`
- `make check`
- `make test`
- Confirm `git status --short` has no unrelated dirty state and no secret files are staged

## Evidence Log

- Slice 1 RED: `cargo test --test screenshot_coordinate_cli window_bounds_reports_signed_root_coordinates -- --nocapture` failed as expected with `unsupported command; try --help` before `window-bounds` existed.
- Slice 1 GREEN: same command passed after adding `src/coordinates.rs`, CLI parsing, usage text, and signed root-coordinate bounds report.
- Slice 2 RED: `cargo test --test screenshot_coordinate_cli window_bounds_reports_xwininfo_disagreement -- --nocapture` failed because `diagnostics.bounds_agree` was `null` and no alternate source existed.
- Slice 2 GREEN: same command passed after adding optional `xwininfo -id` parsing and provenance diagnostics.
- Slice 3 RED: `cargo test --test screenshot_coordinate_cli screenshot_crop_refuses_outside_target_before_provider -- --nocapture` failed with unsupported command before `screenshot-crop` existed.
- Slice 3 GREEN: same command passed after crop parsing/validation refused `CropOutsideTargetBounds` before provider invocation.
- Slice 4 RED: `cargo test --test screenshot_coordinate_cli screenshot_crop_invokes_gdbus_with_validated_rect -- --nocapture` failed because screenshot provider invocation was not implemented and returned non-zero.
- Slice 4 GREEN: same command passed after adding the `gdbus` `org.gnome.Shell.Screenshot.ScreenshotArea` boundary and metadata-only success report.
- Slice 5 RED: `cargo test coordinates::tests::xrandr_negative_offsets_define_root_geometry -- --nocapture` failed to compile because `parse_xrandr_listmonitors_geometry` and `clamp_crop_to_screen` did not exist.
- Slice 5 GREEN: same command passed after adding negative-offset xrandr parsing, `ScreenGeometry`, `xdpyinfo` fallback parsing, and crop-to-screen clamping.
- Focused regression: `cargo test --test screenshot_coordinate_cli -- --nocapture` passed (4 tests).
- Full verification: `openspec validate add-x11-screenshot-coordinate-model --strict --no-interactive` passed; `make fmt && make check && make test` passed.
- Live Cinnamon/X11 smoke: active window id `0x6600004`; `window-bounds` returned `success=true`, `coordinate_space=x11_root_global_pixels`, primary bounds `{x:3840,y:0,width:1920,height:1040}`, `bounds_agree=false`, one alternate source, and one degraded reason; `screenshot-crop` returned `success=true`, `provider=gnome_shell_screenshot_area`, `screenshot_invoked=true`, output file existed with non-zero size, and the report had no `data_url`.

## TDD Exceptions

None.
