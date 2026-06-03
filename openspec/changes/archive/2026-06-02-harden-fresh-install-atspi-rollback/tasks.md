## 1. Doctor AT-SPI probe TDD slices

- [x] 1.1 RED: add `tests/doctor_cli.rs` coverage for a fake AT-SPI collector candidate where `doctor --json` must report `accessibility.tree_available=true`, expected `candidate_count`, expected `match_outcome`, and no `atspi_tree_extraction_unavailable` false degradation.
- [x] 1.2 GREEN: implement a bounded lightweight AT-SPI probe seam reused from the accessibility collector path and wire `src/doctor.rs:gather_system_facts()` to populate `atspi_tree_available`, `atspi_match_outcome`, `atspi_candidate_count`, and `atspi_controlled_fixture_pass` from probe facts.
- [x] 1.3 RED: add doctor coverage for reachable bus plus `NO_AT_BRIDGE=1` where the result must remain `atspi_gtk_bridge_disabled_by_environment` and must not report tree availability.
- [x] 1.4 GREEN/REFACTOR: normalize AT-SPI probe outcomes so positive, bridge-disabled, bus-unavailable, no-match, ambiguous, and controlled-fixture-pass paths share one report mapping without duplicating correlation logic inside doctor.
- [x] 1.5 Evidence: record RED/GREEN commands for slices 1 and 2 in `test-plan.md` Evidence Log.

## 2. Standalone plugin install/accessibility manifest

- [x] 2.1 RED: add `tests/plugin_installer.rs` coverage for `scripts/install-codex-plugin.sh --activate-accessibility --dry-run --report-json` under fake `$CODEX_HOME`, fake gsettings, fake `systemctl --user`, and fake `dbus-update-activation-environment`, expecting manifest/report entries for plugin paths, `toolkit-accessibility`, `NO_AT_BRIDGE`, `GTK_MODULES`, and `QT_ACCESSIBILITY` with changed-vs-already-present classification.
- [x] 2.2 GREEN: extend `scripts/install-codex-plugin.sh` with fresh-install/accessibility setup planning, dry-run/report-json output, manifest writing, and fakeable command boundaries while preserving existing plugin install behavior and stale-install smoke expectations.
- [x] 2.3 RED: add standalone uninstall tests proving manifest-owned accessibility values are restored and already-present values are left untouched.
- [x] 2.4 GREEN: extend `scripts/uninstall-codex-plugin.sh` to read the manifest, restore only completed installer-changed plugin/accessibility entries, support dry-run/report-json, and remain idempotent when plugin state is absent or partial.
- [x] 2.5 RED/GREEN: add and satisfy a drift-blocker test for changed activation environment or gsettings current state that no longer matches manifest after-state.
- [x] 2.6 Evidence: record RED/GREEN commands for standalone install/uninstall slices in `test-plan.md` Evidence Log.

## 3. Source overlay, provider takeover, and live asset rollback manifest

- [x] 3.1 RED: extend `tests/source_overlay_scripts.rs` to require richer source/live manifest metadata: schema version, before/after state, `installer_changed`, `completed`, sha256, size, ownership, and mode while preserving legacy keys currently asserted by tests.
- [x] 3.2 GREEN: update `scripts/codex-source-overlay.py` manifest write paths for source overlay and live assets to record richer metadata before mutation and after successful mutation without breaking existing provider takeover reports.
- [x] 3.3 RED: extend rollback tests to require ownership/mode restoration where fakeable and blocker reporting when current source/live asset state drifts from manifest after-state.
- [x] 3.4 GREEN: update source/live rollback to compare current state to installer after-state, restore before-state bytes/metadata only when safe, and report drift/blockers in JSON.
- [x] 3.5 RED: add wrapper-level provider takeover install/uninstall tests proving cross-surface report-json linkage and preserving standalone plugin identity/bundled fallback per ADR 0010.
- [x] 3.6 GREEN: update `scripts/install-x11-provider-takeover.sh` and `scripts/uninstall-x11-provider-takeover.sh` to compose plugin/accessibility/source/live manifest reports, support selected live asset modes, and avoid removing standalone plugin state unless manifest-owned by the selected operation.
- [x] 3.7 Evidence: record RED/GREEN commands for source/live/provider takeover slices in `test-plan.md` Evidence Log.

## 4. Fake e2e smoke and live-safe checklist

- [x] 4.1 RED: add fake e2e coverage in `tests/e2e_harness_scripts.rs` or the fake smoke harness for fresh install → doctor AT-SPI ok → uninstall restored, using fake home, fake target, fake gsettings/env commands, fake live asset, and fake AT-SPI collector probe.
- [x] 4.2 GREEN: extend `scripts/e2e/codex-plugin-smoke.sh` and/or `scripts/e2e/codex-x11-e2e.py` fake mode to run the fresh-install/doctor/uninstall cycle and write safe evidence for manifest creation, doctor readiness, and restoration.
- [x] 4.3 RED: add docs/checklist test coverage requiring live-safe verification commands and rollback-first dry-run/report-json examples.
- [x] 4.4 GREEN: update docs/checklist content to include `x11_doctor`, `x11_get_app_state include_screenshot=true` with path/no-inline screenshot evidence, `x11_accessibility_tree` against a controlled fixture, provider takeover marker check, dry-run uninstall checks, and full uninstall restore/drift reporting.
- [x] 4.5 Evidence: record RED/GREEN commands for fake e2e and docs/checklist slices in `test-plan.md` Evidence Log.

## 5. Final verification and apply discipline

- [x] 5.1 Run `openspec validate harden-fresh-install-atspi-rollback` after implementation updates and before claiming apply complete.
- [x] 5.2 Run `make fmt` and record result.
- [x] 5.3 Run `make check` and record result.
- [x] 5.4 Run `make test` and record result.
- [x] 5.5 Run `scripts/e2e/codex-plugin-smoke.sh --fake` and record evidence path/output.
- [x] 5.6 Run relevant dry-run install/uninstall checks: standalone plugin install/uninstall dry-runs and provider takeover install/uninstall dry-runs when the configured target exists.
- [x] 5.7 If live Cinnamon/X11 validation is available, run the live-safe checklist and record non-secret evidence; otherwise record exact unavailable layers as limitations.
- [x] 5.8 Ensure `.secrets.local.env` and real secret values were not read, printed, staged, or committed.
- [x] 5.9 Show final `git status --short`, checkpoint the coherent apply group(s), and leave archive for a separate explicit verified archive step.
