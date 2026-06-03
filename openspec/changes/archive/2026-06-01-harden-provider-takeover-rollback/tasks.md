## Implementation Tasks

### 1. Manifest metadata and install transaction

- [x] 1.1 RED: Add failing fake-target/live-asset test proving provider install manifest lacks complete original/installed checksum, size, and mode metadata for every changed source/live file.
- [x] 1.2 GREEN: Extend `scripts/codex-source-overlay.py` metadata helpers and provider install manifest writing for source and live asset backups.
- [x] 1.3 GREEN: Add current-transaction restore on provider install failure and failure report fields.
- [x] 1.4 REFACTOR: Deduplicate backup/metadata/report helpers while keeping existing source overlay behavior intact.

### 2. Manifest-backed provider uninstall

- [x] 2.1 RED: Add failing install → uninstall test proving source files and fake live assets are restored byte-for-byte and manifest removal happens after successful restore.
- [x] 2.2 GREEN: Implement manifest-backed source/live restore with checksum/marker drift checks in `scripts/codex-source-overlay.py`.
- [x] 2.3 GREEN: Implement safe no-op when provider takeover is absent from source, manifest, and scanned live assets.
- [x] 2.4 RED/GREEN: Add missing-manifest/missing-backup/drift tests that assert non-zero exit and no overwrite.

### 3. One-command rollback wrapper

- [x] 3.1 RED: Add failing wrapper test or script smoke for missing `scripts/uninstall-x11-provider-takeover.sh` behavior.
- [x] 3.2 GREEN: Add `scripts/uninstall-x11-provider-takeover.sh` with mirrored target, codex-home, live-assets, dry-run, and report options.
- [x] 3.3 GREEN: Aggregate plugin uninstall, provider uninstall, postcondition verification, and report JSON output.
- [x] 3.4 REFACTOR: Align help text and defaults with `scripts/install-x11-provider-takeover.sh`.

### 4. Install wrapper hardening

- [x] 4.1 RED: Add failing test for wrapper/source install failure that must not leave a claimed-success provider takeover or unreported residue.
- [x] 4.2 GREEN: Update install wrapper/source overlay behavior so partial failures attempt rollback and report plugin/source/live phase outcomes.
- [x] 4.3 GREEN: Ensure dry-run reports planned manifest/backup paths without writing files.

### 5. Documentation and verification

- [x] 5.1 RED: Add/update documentation regression test for provider takeover uninstall command, backup/restore behavior, missing-backup blocker, and restart guidance.
- [x] 5.2 GREEN: Update `README.md`, `INSTALL_CODEX.md`, and/or `docs/install-uninstall.md` with the new rollback command and semantics.
- [x] 5.3 VERIFY: Run `openspec validate harden-provider-takeover-rollback --strict`.
- [x] 5.4 VERIFY: Run `openspec validate --all --strict`.
- [x] 5.5 VERIFY: Run `make fmt`, `make check`, and `make test`.
- [x] 5.6 VERIFY: Confirm project repo and fake target test state are clean; do not mutate real `/opt` or user `$CODEX_HOME` during automated tests.


## Verification Evidence

- **Slice 1 RED:** `cargo test --test source_overlay_scripts provider_takeover_rollback_restores_source_and_live_asset_backup -- --nocapture` failed before implementation with `source backup should include before_sha256`.
- **Slice 1/2 GREEN:** Same targeted test passed after manifest metadata and source/live restore helper changes.
- **Slice 2/3 GREEN:** `cargo test --test source_overlay_scripts provider_takeover_rollback_refuses_live_asset_drift -- --nocapture` passed after checksum/marker drift guards.
- **Slice 3 RED:** `cargo test --test source_overlay_scripts provider_takeover_uninstall_wrapper_restores_overlay_live_asset_and_plugin_state -- --nocapture` failed with missing `scripts/uninstall-x11-provider-takeover.sh`.
- **Slice 3 GREEN:** Same wrapper test passed after adding the uninstaller wrapper and aggregate report.
- **Slice 4 RED:** `cargo test --test source_overlay_scripts provider_takeover_install_failure_restores_current_transaction_source_writes -- --nocapture` failed because failed live-asset install left provider source patched.
- **Slice 4 GREEN:** Same test passed after current-transaction restore was added to provider install failure handling.
- **Slice 5 RED:** `cargo test --test packaging_docs install_uninstall_docs_reference_real_scripts_and_safe_commands -- --nocapture` failed when docs did not mention provider takeover install/uninstall rollback semantics.
- **Slice 5 GREEN:** Same docs test passed after documentation updates.
- **Targeted regression:** `cargo test --test source_overlay_scripts -- --nocapture` passed: 15 tests.
- **Targeted docs regression:** `cargo test --test packaging_docs install_uninstall_docs_reference_real_scripts_and_safe_commands -- --nocapture` passed.
- **Change validation:** `openspec validate harden-provider-takeover-rollback --strict` passed.
- **Full verification:** `openspec validate --all --strict` passed: 19 items.
- **Full verification:** `make fmt` passed after `cargo fmt` applied Rust test formatting.
- **Full verification:** `make check` passed.
- **Full verification:** `make test` passed, including the expanded provider takeover source-overlay tests.
- **Cleanliness note:** Automated tests used fake targets, fake live asset directories, and fake `CODEX_HOME`; no real `/opt` or user `$CODEX_HOME` mutation was performed by tests.
