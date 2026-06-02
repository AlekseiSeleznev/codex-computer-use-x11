## TDD Strategy

Use vertical RED -> GREEN -> REFACTOR slices through public CLI behavior first. The primary regression target is `codex-computer-use-x11 doctor --json`; the supporting comparison target is `codex-computer-use-x11 accessibility-tree --window-id ... --json`. Fake command boundaries are allowed for desktop tools and `python3` collector execution because X11/AT-SPI are external system boundaries; internal doctor/accessibility functions should not be mocked.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1. Doctor accepts accessibility-tree collector success contract | `doctor --json` with fake X11/AT-SPI commands and fake collector JSON equivalent to accessibility-tree success | Add/strengthen `tests/doctor_cli.rs` test, then run `cargo test --test doctor_cli doctor_atspi_probe_uses_accessibility_tree_success_contract -- --nocapture`; expected failure before fix: doctor reports `collector_unavailable`, `tree_available=false`, or missing candidate count for an `ok=true` candidate collector | Same command passes with `tree_available=true`, positive candidate count, `match_outcome=tree_available`, and no `atspi_tree_extraction_unavailable` degraded reason | Keep test at CLI boundary; do not assert private function names |
| 2. Bounded doctor probe preserves hung collector safety | Existing timeout public behavior for `doctor --json` with hung fake commands | Run `cargo test --test doctor_cli doctor_live_probe_times_out_hung_desktop_commands -- --nocapture`; if it fails after slice 1, RED confirms timeout regression | Same command passes; doctor still emits JSON quickly and marks unavailable facts rather than hanging | Timeout remains configurable through existing env var; no unbounded doctor collector |
| 3. Live-safe doctor/accessibility comparison | Local built CLI on live X11 when a focused window is available | Before/after evidence: `NO_AT_BRIDGE= ./target/debug/codex-computer-use-x11 doctor --json` and `NO_AT_BRIDGE= ./target/debug/codex-computer-use-x11 accessibility-tree --window-id <focused> --json`; pre-fix expected mismatch is doctor unavailable while accessibility succeeds | After fix, if accessibility-tree succeeds with a non-empty tree, doctor reports tree available and does not report collector unavailable; if live AT-SPI is unavailable, record limitation without blocking fake regression | Do not take screenshots or inject input; sanitize outputs in chat; no secret file access |

## Mocking / Boundary Policy

- Fake `wmctrl`, `xprop`, `gdbus`, `busctl`, `ydotool`, and `python3` only as OS/desktop command boundaries.
- Do not mock Rust functions inside `src/doctor.rs` or `src/accessibility.rs`.
- Use fixture JSON shaped like the real `ATSPI_COLLECTOR_SCRIPT` output.
- Preserve `NO_AT_BRIDGE=1` bridge-disabled short-circuit coverage.

## Required Checks

- `openspec validate fix-live-doctor-atspi-probe-mismatch --type change --strict`
- `cargo test --test doctor_cli doctor_atspi_probe_uses_accessibility_tree_success_contract -- --nocapture`
- `cargo test --test doctor_cli doctor_live_probe_times_out_hung_desktop_commands -- --nocapture`
- `cargo test --test doctor_cli -- --nocapture`
- `make fmt`
- `make check`
- `make test`
- Live-safe CLI comparison when X11/focused window is available; otherwise record exact unavailable layer.
- Final `git status --short` and secret safety metadata check; do not read `.secrets.local.env`.

## Evidence Log

- Slice 1 — Doctor accepts accessibility-tree collector success contract
  - RED command: `cargo test --test doctor_cli doctor_atspi_probe_uses_accessibility_tree_success_contract -- --nocapture`
  - RED result: failed as expected before production changes. After clearing inherited `NO_AT_BRIDGE`, doctor emitted `tree_available=false`, `match_outcome=collector_unavailable`, `candidate_count=null`, and `atspi_tree_extraction_unavailable` for a large `ok=true` fake collector output.
  - GREEN command: `cargo test --test doctor_cli doctor_atspi_probe_uses_accessibility_tree_success_contract -- --nocapture`
  - GREEN result: passed after making bounded AT-SPI collection drain stdout/stderr while polling the child process, preventing large valid collector output from blocking the child and timing out.

- Slice 2 — Bounded doctor probe preserves hung collector safety
  - Regression command: `cargo test --test doctor_cli doctor_live_probe_times_out_hung_desktop_commands -- --nocapture`
  - Result: passed; hung fake desktop commands remain bounded and doctor still emits JSON without hanging.
  - Refactor/check command: `cargo test --test doctor_cli -- --nocapture`
  - Result: passed 8/8 doctor CLI tests.

- Slice 3 — Live-safe doctor/accessibility comparison
  - Commands: `NO_AT_BRIDGE= ./target/debug/codex-computer-use-x11 focused-window --json`, `NO_AT_BRIDGE= CODEX_X11_COMMAND_TIMEOUT_MS=8000 ./target/debug/codex-computer-use-x11 doctor --json`, and `NO_AT_BRIDGE= ./target/debug/codex-computer-use-x11 accessibility-tree --window-id <focused> --json`.
  - Result: passed on focused window `111149060`; doctor reported `tree_available=true`, `diagnostic_state=tree_extraction_available`, `match_outcome=tree_available`, `candidate_count=66`, and no `atspi_tree_extraction_unavailable`; accessibility-tree reported `success=true`, high-confidence match, and 12 tree nodes.


- Final verification commands
  - OpenSpec validation: `openspec validate fix-live-doctor-atspi-probe-mismatch --type change --strict` passed.
  - Formatting: initial `make fmt` failed on formatting diffs in `src/accessibility.rs` and `tests/doctor_cli.rs`; `cargo fmt` was applied.
  - Formatting recheck: `make fmt` passed.
  - Build/check: `make check` passed.
  - Full test suite: `make test` passed, including 8/8 doctor CLI tests and the full project suite.
  - Secret safety: `.secrets.local.env` was not read or printed; tracked metadata check only.

## TDD Exceptions

None.
