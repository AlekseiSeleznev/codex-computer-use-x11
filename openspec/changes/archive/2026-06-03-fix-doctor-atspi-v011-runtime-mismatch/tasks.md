## 1. TDD slice: NO_AT_BRIDGE valid collector success

- [x] 1.1 RED: update `tests/doctor_cli.rs` public CLI coverage so `doctor --json` with `NO_AT_BRIDGE=1` and a valid fake collector expects `tree_available=true`, `match_outcome=tree_available`, `diagnostic_state=tree_extraction_available`, sanitized bridge env, and evidence that the collector ran.
- [x] 1.2 RED: run `cargo test --test doctor_cli doctor_atspi_probe_reports_tree_available_even_with_no_at_bridge_when_collector_succeeds -- --nocapture` and record the expected failure in `test-plan.md`.
- [x] 1.3 GREEN: update `src/doctor.rs` so the bounded AT-SPI collector runs whenever the AT-SPI bus is reachable, even when `NO_AT_BRIDGE=1` is present.
- [x] 1.4 GREEN: rerun the slice 1 command and record the passing result in `test-plan.md`.
- [x] 1.5 REFACTOR: keep bridge env reporting sanitized and keep JSON shape additive/corrective only.

## 2. TDD slice: env-u valid collector success

- [x] 2.1 RED/GUARD: add or strengthen `tests/doctor_cli.rs` coverage for `NO_AT_BRIDGE` absent with valid fake collector output, expecting `tree_available=true`, `candidate_count>0`, and no `collector_unavailable` or tree-unavailable degraded reason.
- [x] 2.2 Run `cargo test --test doctor_cli doctor_atspi_probe_reports_tree_available_without_no_at_bridge_when_collector_succeeds -- --nocapture` and record whether it was RED or already GREEN in `test-plan.md`.
- [x] 2.3 GREEN: make the minimal production adjustment if the absent-env path still reports collector unavailable for valid collector output.
- [x] 2.4 Rerun the slice 2 command and record passing evidence.

## 3. TDD slice: true collector degradation remains degraded

- [x] 3.1 RED: add `tests/doctor_cli.rs` coverage for reachable AT-SPI bus with invalid/unavailable/no-tree collector output under `NO_AT_BRIDGE=1`, expecting `tree_available=false` and a degraded diagnostic rather than fabricated `tree_available`.
- [x] 3.2 Run `cargo test --test doctor_cli doctor_atspi_probe_degrades_when_collector_output_invalid_even_with_no_at_bridge -- --nocapture` and record RED evidence.
- [x] 3.3 GREEN: adjust collector/probe classification only if needed so invalid/unavailable/no-tree/timeout remains degraded.
- [x] 3.4 Rerun the slice 3 command and record passing evidence.

## 4. Regression checks and refactor

- [x] 4.1 Run `cargo test --test doctor_cli doctor_atspi_probe_uses_accessibility_tree_success_contract -- --nocapture` and record result.
- [x] 4.2 Run `cargo test --test doctor_cli doctor_live_probe_times_out_hung_desktop_commands -- --nocapture` and record result.
- [x] 4.3 Run `cargo test --test doctor_cli -- --nocapture` and record result.
- [x] 4.4 Refactor while green only; do not introduce internal mocks or target-scoped doctor behavior.

## 5. Full verification and checkpoint

- [x] 5.1 Run `openspec validate fix-doctor-atspi-v011-runtime-mismatch --type change --strict`.
- [x] 5.2 Run `make fmt`.
- [x] 5.3 Run `make check`.
- [x] 5.4 Run `make test`.
- [x] 5.5 Run a live-safe `doctor --json` / `accessibility-tree --window-id <focused> --json` comparison when X11 and a focused window are available; otherwise record the exact limitation in `test-plan.md`.
- [x] 5.6 Confirm `.secrets.local.env` was not read, printed, staged, or committed.
- [x] 5.7 Show `git status --short --untracked-files=all`, checkpoint implementation/test-plan evidence, push if appropriate, and stop before archive unless the user separately confirms archive.


## 6. Post-install default-timeout regression

- [x] 6.1 RED: add `tests/doctor_cli.rs::doctor_atspi_probe_default_timeout_allows_slow_valid_collector` proving default bounded doctor AT-SPI probe must allow a slow-but-valid collector result comparable to `accessibility-tree`.
- [x] 6.2 RED: run `cargo test --test doctor_cli doctor_atspi_probe_default_timeout_allows_slow_valid_collector -- --nocapture` and record failure with `tree_available=false`, `match_outcome=collector_unavailable`.
- [x] 6.3 GREEN: increase the default AT-SPI collector timeout in `src/accessibility.rs` while preserving `CODEX_X11_COMMAND_TIMEOUT_MS` override and hung-probe tests.
- [x] 6.4 GREEN: rerun the slow default-timeout test and full `doctor_cli` suite.
- [x] 6.5 Reinstall current binary into Codex and verify installed `doctor --json` reports `tree_available=true`.
