## 1. Apply Preflight and Guardrails

- [x] 1.1 Re-read `proposal.md`, `specs/x11-window-listing/spec.md`, `grill.md`, `design.md`, `design-review.md`, `adr.md`, `test-plan.md`, root `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and `adr/README.md` before implementation.
- [x] 1.2 Run `git branch --show-current` and `git status --short`; stop on dirty unrelated state or if not on the approved apply branch/context.
- [x] 1.3 Confirm the apply scope is standalone crate files only and does not modify `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` or require `.secrets.local.env`.
- [x] 1.4 Confirm each behavior task follows the `test-plan.md` RED -> GREEN -> REFACTOR order and record evidence in `test-plan.md` as slices complete.

## 2. CLI Degraded Entry Slice

- [x] 2.1 RED: add one public CLI test for `codex-computer-use-x11 list-windows --json` without `DISPLAY`; run `cargo test list_windows_cli_degrades_without_display` and record the expected unsupported-command/missing-report failure.
- [x] 2.2 GREEN: add minimal CLI dispatch/report plumbing so the no-display command exits 0 with valid degraded JSON containing `project`, `version`, `backend`, empty `windows`, and `diagnostics`.
- [x] 2.3 GREEN: run `cargo test list_windows_cli_degrades_without_display` and record passing evidence.
- [x] 2.4 REFACTOR: keep `doctor --json` behavior intact, update usage/help text, and run existing doctor CLI tests or the closest available `cargo test doctor` filter.

## 3. wmctrl Parser Slices

- [x] 3.1 RED: add a pure parser fixture test for one normal `wmctrl -lpGx` application row; run `cargo test wmctrl_parser_maps_normal_window` and record the failure.
- [x] 3.2 GREEN: implement the minimal `wmctrl -lpGx` parser and primary window mapping for id, workspace, pid, bounds, class/title, backend, focused, and hidden defaults.
- [x] 3.3 GREEN: run `cargo test wmctrl_parser_maps_normal_window` and record passing evidence.
- [x] 3.4 RED: add a parser test proving titles with spaces, Cyrillic, emoji, and multibyte characters are preserved as the title remainder; run `cargo test wmctrl_parser_preserves_unicode_title_remainder` and record the failure.
- [x] 3.5 GREEN: update parser splitting to preserve the title remainder and pass the Unicode/whitespace test.
- [x] 3.6 RED: add parser tests for padded/unpadded id normalization, negative coordinates, and invalid/non-positive dimensions; run `cargo test wmctrl_parser_validates_ids_and_geometry` and record the failure.
- [x] 3.7 GREEN: reuse `src/x11_id.rs` normalization, preserve signed coordinates, and degrade invalid dimensions without unsigned wraparound; run the geometry/id tests and record passing evidence.
- [x] 3.8 REFACTOR: keep parser errors structured and isolated from command execution; run the relevant parser test group.

## 4. Metadata, WM_CLASS, and PID Reliability Slices

- [x] 4.1 RED: add tests for raw `WM_CLASS` sidecar diagnostics, `instance.Class` mapping, no-dot fallback mapping, and PID reliability for PID `0`, PID `2`, local host, and non-local host; run `cargo test list_windows_reports_class_and_pid_reliability` and record the failure.
- [x] 4.2 GREEN: implement diagnostics sidecar metadata and deterministic class/PID reliability mapping without adding raw/provenance fields to primary window objects.
- [x] 4.3 GREEN: run `cargo test list_windows_reports_class_and_pid_reliability` and record passing evidence.
- [x] 4.4 REFACTOR: ensure raw ids/class/source/warnings remain under diagnostics, not in primary `windows[]`; add or update a serialization assertion if needed.

## 5. Focus and Enrichment Boundary Slices

- [x] 5.1 RED: add a pure active-window parser/service test with fake `xprop -root _NET_ACTIVE_WINDOW`; run `cargo test list_windows_marks_focused_window_from_xprop` and record the failure.
- [x] 5.2 GREEN: implement active-id parsing and focus marking so exactly the normalized matching window has `focused=true`.
- [x] 5.3 GREEN: run `cargo test list_windows_marks_focused_window_from_xprop` and record passing evidence.
- [x] 5.4 RED: add a test that active-window lookup failure leaves listing JSON valid and records diagnostics; run the relevant focus-degraded test and record the failure.
- [x] 5.5 GREEN: implement degraded active-window diagnostics without aborting window listing.
- [x] 5.6 RED: add a command-count or fake-runner test proving the MVP does not spawn unbounded per-window `xprop -id` calls; run `cargo test list_windows_does_not_spawn_unbounded_xprop_per_window` and record the failure.
- [x] 5.7 GREEN: keep per-window type/state enrichment disabled by default or bounded/cached with diagnostics; run the enrichment-boundary test and record passing evidence.

## 6. Command Boundary and Degraded Report Slices

- [x] 6.1 RED: add fake command/PATH or command-runner tests for missing `wmctrl`, failed `wmctrl`, and no `DISPLAY`; run `cargo test list_windows_degrades_when_wmctrl_missing_or_fails` and record the failure.
- [x] 6.2 GREEN: implement system command probing, missing-tool detection, command failure handling, and structured diagnostics with `windows=[]` when listing cannot proceed.
- [x] 6.3 GREEN: run `cargo test list_windows_degrades_when_wmctrl_missing_or_fails` and record passing evidence.
- [x] 6.4 REFACTOR: ensure diagnostics do not include unrelated sensitive local data or live window titles beyond actual synthetic fixture content.

## 7. End-to-End CLI and Compatibility Slices

- [x] 7.1 RED: add an end-to-end CLI test using fake `wmctrl` and fake `xprop` output; run `cargo test list_windows_cli_outputs_windows_with_fake_commands` and record the failure.
- [x] 7.2 GREEN: wire CLI, report builder, command seam, parsers, serializers, and diagnostics so fake-command CLI output satisfies the spec.
- [x] 7.3 GREEN: run `cargo test list_windows_cli_outputs_windows_with_fake_commands` and record passing evidence.
- [x] 7.4 RED: add or update a compatibility test proving `doctor --json` still emits valid JSON and preserves existing bootstrap fields; run the relevant doctor compatibility test and record the failure if it fails.
- [x] 7.5 GREEN: fix any CLI refactor regressions in `doctor --json`; run the doctor compatibility test and record passing evidence.
- [x] 7.6 REFACTOR: update README/help text only as needed to document `list-windows --json` without claiming source-overlay integration.

## 8. Verification and Apply Completion

- [x] 8.1 Run `cargo test` and record the result.
- [x] 8.2 Run `make fmt` and record the result.
- [x] 8.3 Run `make check` and record the result.
- [x] 8.4 Run `make test` and record the result.
- [x] 8.5 Run `cargo run -- doctor --json` and confirm stdout is valid JSON with preserved `doctor-cli` fields; record the result without secret values.
- [x] 8.6 Run `cargo run -- list-windows --json` on the local Cinnamon/X11 session after automated checks are green; record exit status, JSON validity, and window count only, not live titles.
- [x] 8.7 Run `openspec validate add-x11-window-listing --strict` or the project-supported equivalent OpenSpec validation command and record the result.
- [x] 8.8 Run `git status --short` and confirm no local secret files or target-checkout files are staged or modified.
- [x] 8.9 Ensure `test-plan.md` Evidence Log contains RED/GREEN evidence for each completed behavior slice before marking implementation tasks complete.
