## TDD Strategy

Use project-local `tdd` discipline: one public CLI behavior at a time, RED before production code, minimal GREEN, then REFACTOR while green. The primary interface is `codex-computer-use-x11 doctor --json` executed through existing fake desktop command fixtures in `tests/doctor_cli.rs`. The supporting live check is a non-invasive comparison between `doctor --json` and `accessibility-tree --window-id <focused> --json` when X11/focused-window/AT-SPI are available.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1. `NO_AT_BRIDGE=1` does not short-circuit valid collector success | `doctor --json` with fake X11/AT-SPI bus, `NO_AT_BRIDGE=1`, and fake collector output containing valid candidates/tree | Update `tests/doctor_cli.rs::doctor_atspi_probe_preserves_bridge_disabled_state` into `doctor_atspi_probe_reports_tree_available_even_with_no_at_bridge_when_collector_succeeds`; run `cargo test --test doctor_cli doctor_atspi_probe_reports_tree_available_even_with_no_at_bridge_when_collector_succeeds -- --nocapture`. Expected RED: collector is not run and/or report has `tree_available=false`, `diagnostic_state=atspi_gtk_bridge_disabled_by_environment`, or `match_outcome` missing/collector unavailable. | Same command passes with collector log proving it ran, `tree_available=true`, `match_outcome=tree_available`, `diagnostic_state=tree_extraction_available`, no `atspi_tree_extraction_unavailable` degraded reason, and bridge env still records `NO_AT_BRIDGE=1`. | Keep assertions at CLI JSON boundary; do not assert private function names. Preserve timeout bounds. |
| 2. `env -u NO_AT_BRIDGE` accepts valid collector output | `doctor --json` with fake valid collector output and `NO_AT_BRIDGE` absent | Add/strengthen `tests/doctor_cli.rs` test such as `doctor_atspi_probe_reports_tree_available_without_no_at_bridge_when_collector_succeeds`; run `cargo test --test doctor_cli doctor_atspi_probe_reports_tree_available_without_no_at_bridge_when_collector_succeeds -- --nocapture`. Expected RED only if current cleared-env path still reports `collector_unavailable` for valid collector output; if already GREEN, record as regression guard. | Same command passes with `tree_available=true`, `candidate_count>0`, `match_outcome=tree_available`, and no tree-unavailable degraded reason. | Reuse helper fixtures; avoid duplicating large collector fixture beyond what demonstrates behavior. |
| 3. True collector degradation remains degraded | `doctor --json` with AT-SPI bus reachable but fake collector invalid/unavailable/timed-out or empty | Add `tests/doctor_cli.rs` test such as `doctor_atspi_probe_degrades_when_collector_output_invalid_even_with_no_at_bridge`; run `cargo test --test doctor_cli doctor_atspi_probe_degrades_when_collector_output_invalid_even_with_no_at_bridge -- --nocapture`. Expected RED before the slice 1 production change because the collector would not run under `NO_AT_BRIDGE=1`; after slice 1 this serves as a guard that invalid output is still degraded. | Same command passes with `tree_available=false`, a degraded diagnostic/match outcome for invalid/unavailable/no-tree/timeout, and no fabricated `match_outcome=tree_available`. | Preserve existing `doctor_live_probe_times_out_hung_desktop_commands` and large-output regression behavior. |
| 4. Existing bounded probe regressions still pass | Existing doctor CLI timeout and large-output tests | Run `cargo test --test doctor_cli doctor_atspi_probe_uses_accessibility_tree_success_contract doctor_live_probe_times_out_hung_desktop_commands -- --nocapture` after slices 1-3. Expected failure would indicate the new env behavior regressed prior v0.1.1 fixes. | Both tests pass; full `cargo test --test doctor_cli -- --nocapture` passes. | Refactor only after full doctor CLI tests are green. |
| 5. Live-safe doctor/accessibility comparison | Built CLI on current X11 session, no screenshots/input | If a focused window is available, run `./target/debug/codex-computer-use-x11 focused-window --json`, then `./target/debug/codex-computer-use-x11 doctor --json`, and `./target/debug/codex-computer-use-x11 accessibility-tree --window-id <focused> --json`. Expected pre-fix mismatch may show accessibility success while doctor degrades. If live AT-SPI unavailable, record limitation. | If `accessibility-tree` succeeds with non-empty tree, doctor reports `tree_available=true` and does not report `collector_unavailable`/`atspi_tree_extraction_unavailable`. If live layer is unavailable, fake regressions remain authoritative and limitation is recorded. | Do not inject input, click, screenshot, or use uncontrolled windows as pass fixtures beyond non-invasive focused-window read/comparison. |

## Mocking / Boundary Policy

- Fake only OS/desktop command boundaries: `wmctrl`, `xprop`, `xdotool`, `ydotool`, `busctl`, `gdbus`, and `python3` collector execution.
- Do not mock internal Rust functions in `src/doctor.rs` or `src/accessibility.rs`.
- Use fixture JSON shaped like the real AT-SPI collector output consumed by `accessibility-tree`.
- Keep `NO_AT_BRIDGE`, `GTK_MODULES`, and similar env facts sanitized; do not read `.secrets.local.env`.
- Use timeouts/fake commands to prove true degraded conditions without requiring live X11.

## Required Checks

- `openspec validate fix-doctor-atspi-v011-runtime-mismatch --type change --strict`
- Slice 1 focused command: `cargo test --test doctor_cli doctor_atspi_probe_reports_tree_available_even_with_no_at_bridge_when_collector_succeeds -- --nocapture`
- Slice 2 focused command: `cargo test --test doctor_cli doctor_atspi_probe_reports_tree_available_without_no_at_bridge_when_collector_succeeds -- --nocapture`
- Slice 3 focused command: `cargo test --test doctor_cli doctor_atspi_probe_degrades_when_collector_output_invalid_even_with_no_at_bridge -- --nocapture`
- Regression command: `cargo test --test doctor_cli doctor_atspi_probe_uses_accessibility_tree_success_contract -- --nocapture`
- Regression command: `cargo test --test doctor_cli doctor_live_probe_times_out_hung_desktop_commands -- --nocapture`
- Full focused suite: `cargo test --test doctor_cli -- --nocapture`
- `make fmt`
- `make check`
- `make test`
- Live-safe doctor/accessibility comparison when X11 and a focused window are available; otherwise record exact limitation.
- Final `git status --short --untracked-files=all`; ensure no local secret files are staged or printed.

## Evidence Log

- Slice 1 — `NO_AT_BRIDGE=1` does not short-circuit valid collector success
  - RED command: `cargo test --test doctor_cli doctor_atspi_probe_reports_tree_available_even_with_no_at_bridge_when_collector_succeeds -- --nocapture`
  - RED result: failed as expected before production change. The report had `tree_available=false` because doctor skipped the collector under `NO_AT_BRIDGE=1`.
  - GREEN change: `src/doctor.rs` now runs `accessibility::atspi_probe_from_system()` whenever the AT-SPI bus is reachable instead of skipping it because `NO_AT_BRIDGE=1` is present.
  - GREEN command: `cargo test --test doctor_cli doctor_atspi_probe_reports_tree_available_even_with_no_at_bridge_when_collector_succeeds -- --nocapture`
  - GREEN result: passed; collector log proves the collector ran, `tree_available=true`, `match_outcome=tree_available`, and `diagnostic_state=tree_extraction_available` while bridge env still records `NO_AT_BRIDGE=1`.

- Slice 2 — `env -u NO_AT_BRIDGE` accepts valid collector output
  - Guard command: `cargo test --test doctor_cli doctor_atspi_probe_reports_tree_available_without_no_at_bridge_when_collector_succeeds -- --nocapture`
  - Result: passed. The absent-env valid collector path already reported `tree_available=true`, `candidate_count=1`, and `match_outcome=tree_available`; no additional production change was needed.

- Slice 3 — true collector degradation remains degraded
  - Guard command: `cargo test --test doctor_cli doctor_atspi_probe_degrades_when_collector_output_invalid_even_with_no_at_bridge -- --nocapture`
  - Result: passed after slice 1. The test proves doctor runs the bounded collector under `NO_AT_BRIDGE=1`, but invalid collector output remains `tree_available=false`, `match_outcome=collector_unavailable`, and bridge-disabled degraded.

- Regression checks
  - `cargo test --test doctor_cli doctor_atspi_probe_uses_accessibility_tree_success_contract -- --nocapture`: passed.
  - `cargo test --test doctor_cli doctor_live_probe_times_out_hung_desktop_commands -- --nocapture`: passed.
  - `cargo test --test doctor_cli -- --nocapture`: passed, 9/9 tests.


- Post-install default-timeout regression
  - Installed fixed build initially still reported `tree_available=false`, `diagnostic_state=atspi_gtk_bridge_disabled_by_environment`, `match_outcome=collector_unavailable` because the default bounded AT-SPI probe timed out before the real collector completed.
  - RED command: `cargo test --test doctor_cli doctor_atspi_probe_default_timeout_allows_slow_valid_collector -- --nocapture` failed as expected with a slow valid fake collector and default timeout.
  - GREEN change: increased the default AT-SPI collector timeout in `src/accessibility.rs` from 2s to 8s while preserving the `CODEX_X11_COMMAND_TIMEOUT_MS` override.
  - GREEN command: `cargo test --test doctor_cli doctor_atspi_probe_default_timeout_allows_slow_valid_collector -- --nocapture` passed.
  - Regression command: `cargo test --test doctor_cli -- --nocapture` passed, 10/10 tests.

- Full verification
  - `openspec validate fix-doctor-atspi-v011-runtime-mismatch --type change --strict`: passed.
  - `make fmt`: passed.
  - `make check`: passed.
  - First `make test`: failed on pre-existing overlay/doc verification blockers (`scripts/check-overlay` still referenced removed `README.ru.md`). Fixed by removing that stale loop target.
  - Second `make test`: failed on pre-existing placeholder links in `.codex/skills/grill-with-docs/CONTEXT-FORMAT.md`. Fixed by changing placeholder Markdown links to plain code paths.
  - Final `make test`: passed.
  - Final `scripts/check-overlay`: passed after overlay-tooling/doc fixes.

- Live-safe comparison
  - Focused window: `113246212`, title `Codex`, wm_class `codex-desktop`, pid `1673409`.
  - `./target/debug/codex-computer-use-x11 doctor --json`: passed with `atspi_bus_available=true`, `tree_available=true`, `diagnostic_state=tree_extraction_available`, `match_outcome=tree_available`, `candidate_count=67`; degraded reasons only included RemoteDesktop portal unavailability.
  - `./target/debug/codex-computer-use-x11 accessibility-tree --window-id 113246212 --json`: passed with `success=true`, `correlation.status=matched`, `confidence=high`, `candidate_count=67`, and `tree_nodes=12`.

- Secret safety
  - `.secrets.local.env` was not read or printed. No secret values were staged.

## TDD Exceptions

None.
