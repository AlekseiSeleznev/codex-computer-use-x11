## TDD Strategy

Use vertical RED -> GREEN -> REFACTOR slices around the public rollback behavior, not a batch of all tests followed by all code. Each slice starts with a failing test or command that demonstrates the missing behavior, then the minimum implementation to pass, then cleanup while tests remain green.

Automated tests must use fake target checkouts, fake `CODEX_HOME`, and fake live asset directories. Do not mutate `/opt/codex-desktop` or real user plugin state during tests.

## Test Slices

### Slice 1: Provider install records complete backup metadata

- **RED:** Add/extend `tests/source_overlay_scripts.rs` to install provider takeover into a fake target and fake live asset dir, then assert manifest entries include original and installed checksum/size metadata and live asset backup paths.
- **GREEN:** Extend `scripts/codex-source-overlay.py` backup/manifest writing to record the required metadata.
- **REFACTOR:** Consolidate file metadata helpers for source and live assets.
- **Checks:** Targeted source overlay test, then `cargo test source_overlay_scripts` or project-equivalent test filter.

### Slice 2: Provider uninstall restores source and live asset bytes

- **RED:** Add a fake install → uninstall test that records original source/live asset bytes, runs provider uninstall, and asserts source files and live assets are restored byte-for-byte and manifest is removed only after restore.
- **GREEN:** Implement manifest-backed source/live restore in `codex-source-overlay.py`.
- **REFACTOR:** Share restore outcome/report formatting.
- **Checks:** Targeted test plus `openspec validate harden-provider-takeover-rollback --strict`.

### Slice 3: Provider uninstall refuses unsafe drift and missing backups

- **RED:** Add tests for (a) live asset marker present but backup missing and (b) current installed checksum differs from manifest. Assert non-zero exit and no file overwrite.
- **GREEN:** Add drift/missing-backup guards before restore.
- **REFACTOR:** Keep error messages stable and machine-readable in reports.
- **Checks:** Targeted tests.

### Slice 4: One-command provider takeover uninstall wrapper

- **RED:** Add a test that runs a fake provider takeover install with fake `CODEX_HOME`, then `scripts/uninstall-x11-provider-takeover.sh`, and asserts plugin state absent, overlay clean, live asset clean, and aggregate report written.
- **GREEN:** Add the wrapper and wire options to source overlay uninstall and standalone plugin uninstall.
- **REFACTOR:** Align option parsing/help text with install wrapper.
- **Checks:** Targeted wrapper test and shell `--help`/`--dry-run` smoke.

### Slice 5: Install wrapper failure cleanup/reporting

- **RED:** Add a fake failure test where source/live phase fails after plugin install or a partial file write, asserting current-transaction writes are restored and the report does not claim success.
- **GREEN:** Add transaction rollback behavior and wrapper failure reporting.
- **REFACTOR:** Minimize duplicated report JSON creation between install/uninstall wrappers.
- **Checks:** Targeted tests.

### Slice 6: Documentation and final verification

- **RED:** Add or update documentation tests/assertions that install/uninstall docs mention `uninstall-x11-provider-takeover.sh`, live asset backups, safe blockers, and restart guidance.
- **GREEN:** Update README/INSTALL/docs as needed.
- **REFACTOR:** Keep docs concise and command names executable.
- **Checks:** `openspec validate --all --strict`, `make fmt`, `make check`, `make test`, and final git status.

## Evidence to Record in Tasks

For each completed task, record:

- RED evidence: command/test that failed before implementation or explicit existing failing behavior when adding a test is not possible.
- GREEN evidence: command/test that passed after implementation.
- Any REFACTOR verification.
- Any limitations or skipped live-root checks.

## Stop Conditions

Stop and update artifacts before continuing if implementation reveals that safe rollback requires changing ADR 0010 boundaries, deleting live assets without backups, mutating bundled `openai-bundled/computer-use`, or requiring root-owned live assets in automated tests.
