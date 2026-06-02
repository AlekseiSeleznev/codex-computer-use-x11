## TDD Strategy

Apply the project-local `tdd` skill with vertical RED -> GREEN -> REFACTOR slices. Each behavior starts with one failing public-interface test or command check, then the minimal production/harness change, then focused refactor while green. Tests should verify observable CLI/MCP/harness behavior through public surfaces such as `codex-computer-use-x11 screenshot-crop --json`, `scripts/e2e/codex-plugin-smoke.sh`, and `scripts/e2e/codex-x11-e2e.py validate-matrix`; they should not assert private implementation details except at pure helper boundaries where public command behavior would be too slow or unsafe.

Industrial live checks that require a real X11 desktop are not the first RED signal. First RED should use deterministic fake provider/fixture/matrix evidence. Live runs are required as final evidence when a safe Cinnamon/X11 session is available.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1. Resolve relative screenshot output path | `screenshot-crop --output relative/path --json` resolves path against cwd before provider call | Add/enable a CLI integration test in `tests/screenshot_coordinate_cli.rs` with fake `gdbus` asserting the provider receives an absolute cwd-resolved path; run `cargo test --test screenshot_coordinate_cli relative_output_path_is_resolved_before_provider_call` and expect failure because current code passes the relative path | Implement output path resolution/preflight; rerun the same test and expect pass | Keep path resolution isolated and reusable; no screenshot provider behavior changes beyond absolute path use |
| 2. Provider false/no file fails | `screenshot-crop` reports `success=false` when provider returns false and output is missing | Add/enable a fake `gdbus` test returning `(false, '<path>')` without creating a file; run `cargo test --test screenshot_coordinate_cli provider_false_missing_output_is_failure` and expect current `success=true`/wrong behavior failure | Implement provider status parsing/output verification; rerun and expect `success=false`, `error_code`, and no success claim | Error-code mapping is explicit and diagnostics stay sanitized |
| 3. Output integrity rejects empty/non-PNG files | `screenshot-crop` verifies readable non-empty PNG signature before success | Add/enable tests for empty output and text/non-PNG output; run `cargo test --test screenshot_coordinate_cli screenshot_output_integrity_rejects_invalid_files` and expect failure | Add file metadata and PNG signature verification; rerun and expect pass | Avoid duplicating file verification code; keep stdout free of image data |
| 4. Valid PNG remains success | `screenshot-crop` succeeds for provider success plus valid PNG file | Add/enable a positive fake provider test writing a tiny PNG fixture; run `cargo test --test screenshot_coordinate_cli valid_png_output_is_success` and expect failure until output verifier recognizes the positive path | Wire output verifier into success path; rerun and expect pass | Existing crop validation tests remain green |
| 5. Fixture lifecycle manager | Live harness can start/readiness-check/cleanup controlled fixtures without invoking unsafe tools | Add/enable deterministic fixture-manager unit/integration tests in `tests/e2e_harness_scripts.rs` or Python test path using fake processes/readiness files; run targeted test and expect missing fixture manager failure | Implement committed fixture scripts/helpers and cleanup traps; rerun targeted tests | Fixture manager remains small, deterministic, and stores run-scoped metadata under log dir |
| 6. Safe target selection allowlist | Harness refuses missing/ambiguous/non-fixture targets before input/capture calls | Add/enable fake evidence/runner tests with zero, duplicate, overlay, and real-app window listings; run targeted harness tests and expect current harness to select nothing or misclassify | Implement fixture allowlist and unsafe-target classification; rerun and expect no tool call and proper `missing_fixture_setup`/`unsafe_target_selection` reason | Selection logic is data-driven and reused by keyboard/pointer/screenshot/app-state/overlay checks |
| 7. Tk fixture backs keyboard/pointer/focus/target rows | `codex-plugin-smoke.sh --live` fixture path records pass rows only when controlled Tk fixture evidence matches | Add/enable a fake-live runner mode with simulated MCP responses and Tk event/value files; run `python3 scripts/e2e/codex-x11-e2e.py plugin --live --fake-live-fixtures ...` or test wrapper and expect degraded metadata-only rows | Implement live fixture orchestration and row evidence update; rerun fake-live test and expect keyboard/pointer/focus/target rows pass | Keep real live input disabled unless target ownership is proven; no fallback to user apps |
| 8. GTK bridge backs AT-SPI row | GTK fixture with bridge env is required for semantic AT-SPI pass | Add/enable fake-live GTK fixture tests proving env metadata and expected accessible node are required; run targeted harness test and expect no GTK pass today | Implement GTK fixture launch/selection/evidence parsing; rerun and expect AT-SPI pass when fake/live tree contains expected node | Tk no-match remains degraded; matcher thresholds unchanged |
| 9. Screenshot/app-state evidence is fixture-scoped and sanitized | Harness stores screenshots by path and omits full data URLs from ordinary evidence | Add/enable fake-live app-state JSON with data URL; run summary/matrix test and expect current output to include or mishandle data URL/layers | Implement sanitizer/file-path evidence for screenshot/app-state; rerun and expect metadata/path without base64 | Preserve enough layer metadata for diagnosis; never hide failures by sanitizing |
| 10. Overlay enabled lifecycle evidence | Overlay check records shown/release hide only for controlled fixture and excludes helper windows | Add/enable fake-live overlay MCP response and listing with helper window; run targeted harness test and expect current harness has no fixture-backed overlay pass | Implement overlay fixture check, release, and helper exclusion classification; rerun and expect pass/degraded as appropriate | Overlay degradation remains explicit and does not block target context lifecycle when provider intentionally unavailable |
| 11. Industrial matrix profile | `validate-matrix --industrial` rejects missing fixture setup and code failures but preserves legacy validation | Add/enable matrix validator fixture tests for `environment_limitation`, `missing_fixture_setup`, `code_failure`, malformed evidence, and legacy pass/degraded evidence; run `python3 scripts/e2e/codex-x11-e2e.py validate-matrix --industrial --evidence <fixture>` and expect unsupported flag/wrong acceptance failure | Implement schema/reason categories and industrial profile; rerun and expect expected pass/fail outcomes | Backward-compatible default validation remains green for existing fake evidence |
| 12. Documentation/release checklist | Docs distinguish metadata smoke, fake evidence, industrial live acceptance, safe fixtures, and screenshot paths | Add/enable docs grep/test in `tests/e2e_harness_scripts.rs`, `tests/final_dod.rs`, or docs tests expecting industrial commands and safety language; run targeted docs tests and expect failure | Update docs/release checklist/final DoD guidance; rerun targeted tests and expect pass | Docs use variable names only, no secrets, and no raw screenshot payload examples |
| 13. Safe live final evidence | Full safe live plugin smoke produces industrial evidence on current Cinnamon/X11 desktop when fixtures are available | After deterministic tests pass, run `scripts/e2e/codex-plugin-smoke.sh --live --industrial --log-dir target/e2e-logs/<run-id>` or the implemented equivalent; expected first live failure may be real environment limitation but must not target user apps | Fix only code/harness issues found by deterministic evidence first; rerun safe live command and expect fixture-backed rows pass or environment-degraded with concrete reasons | Capture evidence paths; cleanup fixtures; final git status remains clean except intended changes |

## Mocking / Boundary Policy

- Mock/fake only external desktop boundaries: `gdbus`, `wmctrl`, `xprop`, `xdotool`, MCP stdio responses, fixture process readiness, and screenshot/app-state payload files.
- Do not mock internal Rust collaborators when a CLI test through fake commands can verify behavior.
- For live harness TDD, use fake-live evidence fixtures before real desktop runs. Real desktop checks are verification evidence, not the first RED signal.
- No tests may read `.secrets.local.env` or depend on private local paths beyond documented non-secret variables.
- Live input/pointer tests must use controlled fixture windows only. If a safe fixture cannot be resolved uniquely, the expected behavior is refusal/degraded/fail evidence, not fallback to a user app.

## Required Checks

Before marking apply complete:

```bash
cargo test --test screenshot_coordinate_cli
cargo test --test e2e_harness_scripts
cargo test --test get_app_state_cli
cargo test --test accessibility_tree_cli
cargo test --test target_window_cli
cargo test --test targeted_input_cli
cargo test --test pointer_actions_cli
python3 scripts/e2e/codex-x11-e2e.py validate-matrix --evidence <new fake/legacy evidence fixture>
python3 scripts/e2e/codex-x11-e2e.py validate-matrix --industrial --evidence <new industrial evidence fixture>
scripts/e2e/codex-plugin-smoke.sh --fake --log-dir target/e2e-logs/<run-id>/plugin-fake
scripts/e2e/codex-x11-e2e.py validate-matrix --evidence target/e2e-logs/<run-id>/plugin-fake/<run>/evidence.json
make fmt
make check
make test
openspec validate --all --strict
git status --short
```

Live evidence when safe and available:

```bash
scripts/e2e/codex-plugin-smoke.sh --live --industrial --log-dir target/e2e-logs/<run-id>/plugin-live
scripts/e2e/codex-x11-e2e.py validate-matrix --industrial --evidence target/e2e-logs/<run-id>/plugin-live/<run>/evidence.json
```

If the final CLI uses a different industrial flag/profile name, update these commands in this test plan during apply before marking tasks complete.

## Evidence Log

Apply evidence captured so far:

- Slice 1.1 RED: `cargo test --test screenshot_coordinate_cli screenshot_crop_resolves_relative_output_path_before_provider_call -- --nocapture` failed because `output_path` remained `relative/crop.png` and provider received the relative path.
- Slice 1.1/1.2 GREEN: same command passed after cwd-relative output path resolution and parent preflight were implemented.
- Slice 1.3 RED: `cargo test --test screenshot_coordinate_cli screenshot_crop_provider_false_without_output_is_failure -- --nocapture` failed because provider `(false, ...)` still produced `success=true`.
- Slice 1.3/1.4 GREEN: same command passed after provider false handling returned `ScreenshotOutputMissing` with `screenshot_invoked=true`.
- Slice 1.5 RED: `cargo test --test screenshot_coordinate_cli screenshot_crop_rejects_ -- --nocapture` failed because empty and non-PNG output files still produced `success=true`.
- Slice 1.5/1.6 GREEN: same command passed after readable non-empty PNG signature verification was added.
- Slice 1.7 GREEN: `cargo test --test screenshot_coordinate_cli -- --nocapture` passed after the positive provider fixture wrote a valid PNG.
- Slice 1.8 REFACTOR/GREEN: `cargo fmt` and `cargo test --test screenshot_coordinate_cli -- --nocapture` passed; screenshot output preflight/postflight helpers are isolated in `src/coordinates.rs`.
- Slice 11 RED: `cargo test --test e2e_harness_scripts industrial_matrix_validator_rejects_missing_fixture_setup_and_code_failure -- --nocapture` failed because `validate-matrix` did not support `--industrial`.
- Slice 11 GREEN: same test passed after adding the industrial validation profile, lower-case `pass`/`degraded`/`fail` handling, reason-category enforcement, and metadata-only live `missing_fixture_setup` classification.
- Compatibility GREEN: `cargo test --test e2e_harness_scripts -- --nocapture` passed with 11 tests, preserving default matrix validation and fake smoke behavior.
- Slice 12 RED: `cargo test --test e2e_harness_scripts docs_cover_industrial_live_verification_and_safe_evidence -- --nocapture` failed because docs did not mention `--industrial` and industrial reason categories.
- Slice 12 GREEN: same docs test passed after updating `docs/e2e-harness.md`, `docs/troubleshooting.md`, and `docs/release-checklist.md`. Full `cargo test --test e2e_harness_scripts -- --nocapture` passed with 12 tests.
- Slice 5 RED: `cargo test --test e2e_harness_scripts controlled_fixture_manager_creates_metadata_and_cleanup_records -- --nocapture` failed because `fixture-self-test` was not a supported harness command.
- Slice 5 GREEN: `cargo test --test e2e_harness_scripts controlled_fixture_manager -- --nocapture` passed after adding deterministic Tk/GTK fixture scripts, run-scoped metadata/readiness files, process ids, and cleanup records.
- Slice 5 cleanup RED: `cargo test --test e2e_harness_scripts controlled_fixture_manager_cleans_up_after_tool_failure -- --nocapture` failed because `--fail-after-start` was unsupported.
- Slice 5 cleanup GREEN: `cargo test --test e2e_harness_scripts controlled_fixture_manager -- --nocapture` passed with startup-failure and tool-failure cleanup evidence, including target-window release and overlay-hide markers.
- Slice 6 RED: `cargo test --test e2e_harness_scripts safe_fixture_selection_blocks_unsafe_targets_before_tool_calls -- --nocapture` failed because `selection-self-test` was not a supported harness command.
- Slice 6 GREEN: same test passed after adding exact run-scoped fixture selection, missing/duplicate/stale/overlay-helper/user-app refusal categories, and `tool_calls_attempted=false` evidence for unsafe scenarios.
- Slices 7-10 RED: `cargo test --test e2e_harness_scripts plugin_smoke_live_industrial_fake_fixtures_records_fixture_backed_rows -- --nocapture` failed because `codex-plugin-smoke.sh --live --industrial --fake-live-fixtures` was unsupported.
- Slices 7-10 GREEN: same test passed after adding deterministic fake-live industrial fixture orchestration for Tk input/pointer/focus/target rows, GTK bridge AT-SPI evidence, fixture-scoped screenshot/app-state path evidence, overlay lifecycle evidence, cleanup records, and `validate-matrix --industrial` pass coverage.
- Final targeted verification GREEN: `cargo test --test screenshot_coordinate_cli -- --nocapture`, `cargo test --test e2e_harness_scripts -- --nocapture`, `cargo test --test get_app_state_cli -- --nocapture`, `cargo test --test accessibility_tree_cli -- --nocapture`, `cargo test --test target_window_cli -- --nocapture`, `cargo test --test targeted_input_cli -- --nocapture`, and `cargo test --test pointer_actions_cli -- --nocapture` all passed.
- Final fake/default matrix GREEN: `scripts/e2e/codex-plugin-smoke.sh --fake --log-dir target/e2e-logs/apply-harden-x11-industrial-20260601T133011Z/plugin-fake` passed and `python3 scripts/e2e/codex-x11-e2e.py validate-matrix --evidence target/e2e-logs/apply-harden-x11-industrial-20260601T133011Z/plugin-fake/standalone_plugin-fake-20260601T133011Z-553032/evidence.json` passed.
- Final industrial fake-live controlled-fixture GREEN: `scripts/e2e/codex-plugin-smoke.sh --live --industrial --fake-live-fixtures --log-dir target/e2e-logs/apply-harden-x11-industrial-20260601T133011Z/plugin-live-fake-fixtures` passed on Cinnamon/X11 without user-app targeting, wrote cleanup evidence, and `python3 scripts/e2e/codex-x11-e2e.py validate-matrix --industrial --evidence target/e2e-logs/apply-harden-x11-industrial-20260601T133011Z/plugin-live-fake-fixtures/standalone_plugin-live-20260601T133016Z-553468/evidence.json` passed.
- Final harness helper GREEN: `python3 scripts/e2e/codex-x11-e2e.py fixture-self-test --log-dir target/e2e-logs/apply-harden-x11-industrial-20260601T133011Z/fixture-self-test`, `selection-self-test --scenario ok`, and `selection-self-test --scenario duplicate` passed.
- Final project verification GREEN: `make fmt`, `make check`, `make test`, and `openspec validate --all --strict` passed; OpenSpec reported 19 passed, 0 failed.
- Final git safety GREEN: `git status --short` was clean before final task/test-plan evidence update; no `.secrets.local.env` or uncontrolled screenshots/log payloads were staged.

Initial planning evidence:

- Retest source evidence: `target/e2e-logs/full-x11-retest-20260601T123839Z/report.md`.
- Relative crop issue evidence: `target/e2e-logs/full-x11-retest-20260601T123839Z/live-mcp/screenshot-and-bounds.log` and `live-mcp/screenshot-crop-absolute.log`.
- Existing metadata-only live smoke evidence: `target/e2e-logs/full-x11-retest-20260601T123839Z/plugin-live/**/evidence.json`.
- Safe manual fixture pass evidence from retest: `live-mcp/fixture-content.txt`, `live-mcp/gtk-fixture-ready.json`, and `live-mcp/overlay-enabled-cli.log`.

## TDD Exceptions

None.
