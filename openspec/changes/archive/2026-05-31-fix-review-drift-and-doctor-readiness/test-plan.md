# Test Plan — fix-review-drift-and-doctor-readiness

## TDD Policy

Use vertical RED -> GREEN -> REFACTOR slices. Each slice starts with one public-interface behavior test or command check, observes RED where practical, implements the minimum fix, then records GREEN evidence. Documentation-only behavior uses focused Rust integration tests, validator commands, or stable text checks as the public interface.

## Slice 1 — ADR reference traceability

### Behavior

Every top-level ADR path referenced by `ARCHITECTURE.md` or `adr/README.md` exists as a tracked file, including superseded historical ADRs.

### RED

- Add/adjust a `tests/final_dod.rs` public behavior test or validator fixture that scans ADR references and fails on the current missing `adr/0001...0007` paths.
- Run: `cargo test --test final_dod final_dod_docs_record_adr_and_architecture_snapshot -- --nocapture` or a more focused new test name.
- Expected RED before implementation: failure naming at least one missing ADR path.

### GREEN

- Add restored ADR files 0001-0007 and/or validation logic as required.
- Run the focused test and `scripts/validate-final-dod.py`.

### REFACTOR

- Keep parsing narrow to top-level ADR references in root architecture/index docs.

## Slice 2 — Doctor capabilities and focus readiness

### Behavior

`doctor --json` reports finalized v1 X11/EWMH windowing/focus capabilities as implemented, does not keep stale `x11-ewmh-windowing` in planned, and sets focus readiness true in complete X11/EWMH fixture conditions while staying false/degraded when prerequisites are missing.

### RED

- Update `src/doctor.rs` unit tests and/or `tests/doctor_cli.rs` integration tests to expect:
  - `capabilities.implemented` contains finalized v1 windowing/focus names;
  - `capabilities.planned` is an array without `x11-ewmh-windowing`;
  - complete fixture has `can_focus_windows=true` and appropriate `can_focus_apps` semantics;
  - no-display/missing-tool cases remain false.
- Run focused tests such as `cargo test doctor_focus -- --nocapture` and `cargo test --test doctor_cli doctor_cli_success_json -- --nocapture`.
- Expected RED before implementation: assertions fail on stale planned placeholder and false focus booleans.

### GREEN

- Update `report_from_probe()` and `readiness_report()` minimally.
- Re-run focused tests.

### REFACTOR

- Extract implemented capability strings if duplication appears, while preserving serialized JSON shape.

## Slice 3 — Doctor ydotool privacy

### Behavior

Live system fact gathering may probe real ydotool socket paths internally but serialized doctor JSON uses labels for `YDOTOOL_SOCKET` and `XDG_RUNTIME_DIR` derived candidates and never exposes private path values.

### RED

- Add a test that sets fake env-derived ydotool socket paths and verifies serialized JSON omits raw private values while including stable labels.
- Run focused doctor privacy tests.
- Expected RED before implementation: serialized JSON contains the real env-derived path or lacks labels.

### GREEN

- Separate live probe paths from serialized labels in `gather_system_facts()`/ydotool candidate construction.
- Re-run focused privacy tests.

### REFACTOR

- Keep existing fixture-friendly `ProbeFacts.ydotool_candidates` shape unless a larger public API change is necessary.

## Slice 4 — Documentation drift tests

### Behavior

README and release checklist describe the current standalone plugin and reversible source-overlay posture, the release checklist uses post-archive-valid validation commands, and illustrative skill-template paths are not broken local Markdown links.

### RED

- Update `tests/packaging_docs.rs` to reject the stale archived-change validation command and to assert the illustrative context-format paths are not Markdown links unless the target files exist.
- Run focused docs tests.
- Expected RED before implementation: release checklist still contains the archived active-change command; context-format examples still use Markdown links to absent files.

### GREEN

- Edit `README.md`, `docs/release-checklist.md`, and `.codex/skills/grill-with-docs/CONTEXT-FORMAT.md`.
- Run focused docs tests.

### REFACTOR

- Keep helper assertions readable and avoid a broad repository-wide link checker.

## Slice 5 — Strict clippy cleanup

### Behavior

Current repository code passes strict clippy used as review-remediation evidence.

### RED

- Run: `cargo clippy --all-targets --all-features -- -D warnings`.
- Expected RED before cleanup: current known warnings (`unnecessary_sort_by`, `trim_split_whitespace`, `field_reassign_with_default`, `too_many_arguments`, identical/obfuscated if-else) fail the command.

### GREEN

- Apply local refactors or narrow helper-specific allows.
- Re-run strict clippy.

### REFACTOR

- Run `make fmt` after edits and keep clippy allows narrow.

## Full Verification After Slices

Run before claiming implementation complete:

- `make fmt`
- `make check`
- `make test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `openspec validate fix-review-drift-and-doctor-readiness --type change --strict`
- `openspec validate --all --strict`
- `scripts/check-overlay`
- `scripts/validate-final-dod.py`
- `git status --short`

## Evidence Log

- **Slice 1 RED:** `cargo test --test final_dod architecture_and_adr_index_reference_only_tracked_adr_files -- --nocapture` failed, naming missing ADR files 0001-0007. After adding validator logic, `scripts/validate-final-dod.py` failed with missing referenced ADR files.
- **Slice 1 GREEN:** After restoring ADR files 0001-0007, `cargo test --test final_dod architecture_and_adr_index_reference_only_tracked_adr_files -- --nocapture` passed and `scripts/validate-final-dod.py` reported final DoD complete.
- **Slice 2 RED:** Focused doctor tests failed because complete fixture focus booleans were false, implemented capabilities only contained `doctor-json`/`doctor-capability-detection`, and CLI JSON still planned `x11-ewmh-windowing`.
- **Slice 2 GREEN:** `cargo test doctor_focus_booleans_track_verified_x11_window_focus -- --nocapture`, `cargo test doctor_capabilities_reflect_finalized_v1_windowing -- --nocapture`, and `cargo test --test doctor_cli doctor_cli_success_json -- --nocapture` passed after refreshing readiness/capability facts.
- **Slice 3 RED:** `cargo test --test doctor_cli doctor_cli_redacts_env_derived_ydotool_socket_paths -- --nocapture` failed because serialized JSON contained the raw private `YDOTOOL_SOCKET` path.
- **Slice 3 GREEN:** The same privacy test passed after live ydotool probing separated real local paths from serialized candidate labels.
- **Slice 4 RED:** Focused `packaging_docs` tests failed on stale README source-overlay wording, archived active-change release validation, and Markdown links to missing illustrative context files.
- **Slice 4 GREEN:** Focused README/release/context-format documentation tests passed after docs updates.
- **Slice 5 RED:** `cargo clippy --all-targets --all-features -- -D warnings` failed on current clippy warnings including `unnecessary_sort_by`, `trim_split_whitespace`, `field_reassign_with_default`, `too_many_arguments`, identical if/else, obfuscated if/else, and one new test `len_zero` warning.
- **Slice 5 GREEN:** `cargo clippy --all-targets --all-features -- -D warnings` passed after local refactors and narrow helper-level `too_many_arguments` allows.
