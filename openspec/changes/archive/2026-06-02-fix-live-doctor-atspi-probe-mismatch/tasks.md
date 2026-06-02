## 1. Doctor AT-SPI probe TDD slice

- [x] 1.1 RED: add `tests/doctor_cli.rs` public CLI coverage for `doctor --json` consuming a fake collector output shaped like successful `accessibility-tree` collector JSON, expecting `tree_available=true`, positive `candidate_count`, `match_outcome=tree_available`, and no `atspi_tree_extraction_unavailable` degraded reason.
- [x] 1.2 GREEN: update the doctor/accessibility probe path so the bounded doctor probe treats `ok=true` plus candidates the same way the accessibility-tree collector path does, without changing the public JSON shape except corrected values.
- [x] 1.3 REFACTOR: keep bridge-disabled short-circuit and hung-command timeout behavior intact; run focused doctor CLI tests.
- [x] 1.4 Evidence: record RED/GREEN/REFACTOR commands and outcomes in `test-plan.md` Evidence Log.

## 2. Live-safe comparison and verification

- [x] 2.1 Run a live-safe focused-window comparison when X11 is available: `doctor --json` versus `accessibility-tree --window-id <focused> --json` with no screenshots/input and with `NO_AT_BRIDGE` neutralized for the non-disabled branch.
- [x] 2.2 If live comparison is unavailable or AT-SPI is genuinely unavailable, record exact limitation; otherwise require doctor to avoid `collector_unavailable` when accessibility-tree succeeds.
- [x] 2.3 Run `openspec validate fix-live-doctor-atspi-probe-mismatch --type change --strict`.
- [x] 2.4 Run `make fmt`.
- [x] 2.5 Run `make check`.
- [x] 2.6 Run `make test`.
- [x] 2.7 Ensure `.secrets.local.env` and real secret values were not read, printed, staged, or committed.
- [x] 2.8 Show final `git status --short`, checkpoint the coherent apply group, and stop before archive.
