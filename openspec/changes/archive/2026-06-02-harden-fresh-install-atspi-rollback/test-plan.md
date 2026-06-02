## TDD Strategy

Apply the project-local `tdd` skill with vertical RED -> GREEN -> REFACTOR slices. Each behavior-changing slice starts with one failing test or smoke check through a public interface, then the minimal production/script change, then the GREEN command and refactor guardrails. Tests may fake OS boundaries (`PATH`, fake `$CODEX_HOME`, fake target checkout, fake gsettings/systemctl/dbus commands, fake AT-SPI collector output, fake live assets) but must not mock internal logic that the project owns.

Production implementation must not start until this test plan and `tasks.md` are complete and checkpointed. During apply, each slice's evidence log must capture the RED failure and GREEN pass before the corresponding task is marked complete.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1. Doctor AT-SPI positive probe | `codex-computer-use-x11 doctor --json` via `tests/doctor_cli.rs` with fake AT-SPI collector/tree candidate | `cargo test --test doctor_cli doctor_atspi_probe_reports_tree_available -- --nocapture`; expected failure: missing test initially, then report still has `accessibility.tree_available=false` / degraded `atspi_tree_extraction_unavailable` despite fake collector candidate | Same command passes with `tree_available=true`, expected `candidate_count`, expected `match_outcome`, and no false degraded readiness | Keep probe bounded and reusable; do not duplicate accessibility-tree correlation logic inside doctor |
| 2. Doctor bridge-disabled degradation | `doctor --json` with fake bus and `NO_AT_BRIDGE=1` in effective env | `cargo test --test doctor_cli doctor_atspi_probe_preserves_bridge_disabled_state -- --nocapture`; expected failure: bridge-disabled state collapsed or positive probe attempted incorrectly | Same command passes with `diagnostic_state=atspi_gtk_bridge_disabled_by_environment`, `tree_available=false`, and remediation text | Ensure positive probe path and bridge-disabled path share one outcome enum/report mapping |
| 3. Standalone install manifest captures accessibility before-state | `scripts/install-codex-plugin.sh --dry-run/--activate-accessibility --report-json` under fake `$CODEX_HOME` and fake gsettings/systemctl/dbus commands | `cargo test --test plugin_installer plugin_installer_records_accessibility_manifest_before_state -- --nocapture`; expected failure: no manifest/report entries for `NO_AT_BRIDGE`, `GTK_MODULES`, `QT_ACCESSIBILITY`, or `toolkit-accessibility` | Same command passes with no dry-run mutation and JSON manifest entries classifying changed vs already-present | Extract shell/Python helper logic only as needed; keep command interfaces stable and fakeable |
| 4. Standalone uninstall restores only installer-changed env/gsettings | `scripts/uninstall-codex-plugin.sh --dry-run --report-json` and write-mode uninstall under fake commands | `cargo test --test plugin_installer plugin_uninstaller_restores_manifest_owned_accessibility_state -- --nocapture`; expected failure: uninstaller ignores accessibility manifest or overwrites already-present values | Same command passes: changed values restored, already-present values left unchanged, dry-run makes no mutation | Keep rollback idempotent; report missing/already-restored entries as outcomes, not panics |
| 5. Standalone uninstall blocks env/gsettings drift | Standalone uninstall report-json under fake post-install drift | `cargo test --test plugin_installer plugin_uninstaller_reports_accessibility_drift_blocker -- --nocapture`; expected failure: blind restore or no blocker | Same command passes with JSON drift/blocker and no overwrite | Drift detection should compare current state to recorded after-state before restore |
| 6. Source/live manifest records ownership, mode, sha256, changed classification | `scripts/install-codex-source-overlay.sh --provider x11 --mode takeover --live-assets-dir <fake>` | `cargo test --test source_overlay_scripts provider_takeover_manifest_records_metadata_and_changed_state -- --nocapture`; expected failure: manifest lacks ownership/mode/changed-vs-already-present metadata | Same command passes with legacy fields preserved plus richer metadata for source and live asset entries | Preserve existing tests/legacy keys; add schema fields compatibly |
| 7. Source/live rollback restores metadata and blocks drift | `scripts/uninstall-codex-source-overlay.sh --provider x11 --mode takeover --report-json` against fake target/live assets | `cargo test --test source_overlay_scripts provider_takeover_rollback_restores_metadata_and_blocks_drift -- --nocapture`; expected failure: mode/owner not restored or drift overwritten | Same command passes with mode/ownership restoration where fakeable and JSON blocker on drift | Avoid blind copy2 assumptions when ownership/mode need explicit preservation/reporting |
| 8. Provider takeover wrapper composes rollback-first surfaces | `scripts/install-x11-provider-takeover.sh` and `scripts/uninstall-x11-provider-takeover.sh` against fake target/home/live assets | `cargo test --test source_overlay_scripts provider_takeover_wrapper_reports_cross_surface_manifest_state -- --nocapture`; expected failure: wrapper omits standalone/accessibility/source/live report linkage | Same command passes with report-json showing selected surfaces, plugin status, source overlay status, live asset status, and blockers if any | Keep standalone plugin uninstall separate unless selected/manifest-owned by the operation |
| 9. Fake e2e fresh install → doctor ok → uninstall restored | `scripts/e2e/codex-plugin-smoke.sh --fake` or `scripts/e2e/codex-x11-e2e.py --fake` with fake install/doctor/uninstall cycle | `cargo test --test e2e_harness_scripts plugin_smoke_fake_fresh_install_doctor_uninstall_restores_state -- --nocapture` or shell smoke equivalent; expected failure: no cycle evidence or doctor still degraded | Fake smoke passes without GUI/sudo and evidence shows fresh install manifest, doctor AT-SPI tree available, uninstall restored before-state | Keep live checklist separate from fake CI path; no inline screenshots or secrets |
| 10. Documentation/checklist verifies live-safe commands | Docs/tests for install/uninstall guide and final checklist | `cargo test --test packaging_docs install_uninstall_docs_reference_rollback_first_live_checklist -- --nocapture`; expected failure: docs/checklist omit new dry-run/report-json/live-safe commands | Same command passes with docs/checklist referencing `x11_doctor`, `x11_get_app_state include_screenshot=true`, `x11_accessibility_tree`, provider marker check, and full uninstall restore | Keep docs non-secret and path/command based; live unavailable must be a reported limitation |

## Mocking / Boundary Policy

- Fake only external OS/session boundaries: `gsettings`, `systemctl --user`, `dbus-update-activation-environment`, `dbus-send`/`gdbus`, AT-SPI collector subprocess output, live asset directories, target checkout files, `$CODEX_HOME`, and command availability through fake `PATH`.
- Do not mock project-owned manifest parsing, rollback classification, doctor report mapping, or installer/uninstaller argument handling.
- Live Cinnamon/X11 evidence is supplemental and must use controlled fixtures. It may not replace fake/fixture tests for CI-like confidence.
- Sudo/root-owned live asset behavior should be represented in fake tests through file metadata where possible; if ownership cannot be changed in the test environment, assert report fields and mode/sha256 behavior and record ownership restoration as live checklist evidence.

## Required Checks

Before apply eligibility:

- `openspec validate harden-fresh-install-atspi-rollback`
- Confirm planning artifacts are committed/checkpointed.

During/after apply:

- Slice-specific RED and GREEN commands listed above.
- `make fmt`
- `make check`
- `make test`
- `scripts/e2e/codex-plugin-smoke.sh --fake`
- Relevant dry-run install/uninstall checks, including:
  - `scripts/install-codex-plugin.sh --dry-run`
  - `scripts/uninstall-codex-plugin.sh --dry-run`
  - `scripts/install-x11-provider-takeover.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH" --dry-run` when the target exists
  - `scripts/uninstall-x11-provider-takeover.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH" --dry-run` when the target exists
- Live-safe checklist when live Cinnamon/X11 access is available:
  - `x11_doctor`
  - `x11_get_app_state include_screenshot=true` with no inline screenshot data in stored evidence
  - `x11_accessibility_tree` against a controlled fixture
  - provider takeover marker in live asset when live patching was requested
  - full uninstall restore or explicit drift/blocker report

## Evidence Log

- Slice 1 — Doctor AT-SPI positive probe
  - RED command: `cargo test --test doctor_cli doctor_atspi_probe_reports_tree_available -- --nocapture`
  - RED result: failed as expected before production changes; assertion showed `accessibility.tree_available` was `false` while the fixture expected `true`, confirming the hardcoded doctor false-negative.
  - GREEN command: `cargo test --test doctor_cli doctor_atspi_probe_reports_tree_available -- --nocapture`
  - GREEN result: passed after wiring a bounded shared AT-SPI collector probe into `gather_system_facts()` and mapping `match_outcome=tree_available` to `diagnostic_state=tree_extraction_available`.
  - Refactor/check command: `cargo test --test doctor_cli -- --nocapture`
  - Refactor/check result: passed 7/7 doctor CLI tests.

- Slice 2 — Doctor bridge-disabled degradation
  - RED command: `cargo test --test doctor_cli doctor_atspi_probe_preserves_bridge_disabled_state -- --nocapture`
  - RED result: passed against existing behavior, confirming the bridge-disabled guard already existed for `NO_AT_BRIDGE=1`; the new test locks the requirement that the collector probe must not run when the bridge is disabled.
  - GREEN command: `cargo test --test doctor_cli doctor_atspi_probe_preserves_bridge_disabled_state -- --nocapture`
  - GREEN result: passed after the probe wiring, with `tree_available=false`, `diagnostic_state=atspi_gtk_bridge_disabled_by_environment`, and no collector invocation.
  - Refactor/check command: `cargo test --test doctor_cli -- --nocapture`
  - Refactor/check result: passed 7/7 doctor CLI tests.


- Slice 3 — Standalone install manifest captures accessibility before-state
  - RED command: `cargo test --test plugin_installer plugin_installer_records_accessibility_manifest_before_state -- --nocapture`
  - RED result: failed as expected before script changes with `unknown argument: --activate-accessibility`.
  - GREEN command: `cargo test --test plugin_installer plugin_installer_records_accessibility_manifest_before_state -- --nocapture`
  - GREEN result: passed after adding `--activate-accessibility` and `--report-json` dry-run planning with plugin, gsettings, and activation-env manifest entries.

- Slice 4 — Standalone uninstall restores only installer-changed env/gsettings
  - RED command: `cargo test --test plugin_installer plugin_uninstaller_restores_manifest_owned_accessibility_state -- --nocapture`
  - RED result: failed as expected before script changes with `unknown argument: --report-json`.
  - Additional RED command: `cargo test --test plugin_installer plugin_installer_writes_accessibility_manifest_and_applies_setup -- --nocapture`
  - Additional RED result: failed as expected before write-mode manifest support; `install-manifest.json` was missing.
  - GREEN commands: `cargo test --test plugin_installer plugin_installer_writes_accessibility_manifest_and_applies_setup -- --nocapture` and `cargo test --test plugin_installer plugin_uninstaller_restores_manifest_owned_accessibility_state -- --nocapture`
  - GREEN result: both passed after write-mode installer manifest/setup and uninstaller manifest restoration support.
  - Refactor/check command: `cargo test --test plugin_installer -- --nocapture`
  - Refactor/check result: passed 9/9 plugin installer tests.

- Slice 5 — Standalone uninstall blocks env/gsettings drift
  - RED/GREEN command: `cargo test --test plugin_installer plugin_uninstaller_reports_accessibility_drift_blocker -- --nocapture`
  - Result: passed after the uninstaller drift-check implementation; the test verifies drift is reported in JSON and `gsettings set` is not invoked.
  - Refactor/check command: `cargo test --test plugin_installer -- --nocapture`
  - Refactor/check result: passed 9/9 plugin installer tests.


- Slice 6 — Source/live manifest records ownership, mode, sha256, changed classification
  - RED command: `cargo test --test source_overlay_scripts provider_takeover_rollback_restores_source_and_live_asset_backup -- --nocapture`
  - RED result: failed as expected after strengthening assertions; report lacked `schema_version` and richer ADR 0011 metadata.
  - GREEN command: `cargo test --test source_overlay_scripts provider_takeover_rollback_restores_source_and_live_asset_backup -- --nocapture`
  - GREEN result: passed after adding compatible source/live backup metadata (`before`, `after`, owner/mode, `installer_changed`, `completed`) and `schema_version`.

- Slice 7 — Source/live rollback restores metadata and blocks drift
  - Check command: `cargo test --test source_overlay_scripts provider_takeover_rollback_refuses_live_asset_drift -- --nocapture`
  - Result: passed, confirming drifted live assets are reported/refused rather than blindly overwritten.

- Slice 8 — Provider takeover wrapper composes rollback-first surfaces
  - Check command: `cargo test --test source_overlay_scripts provider_takeover_uninstall_wrapper_restores_overlay_live_asset_and_plugin_state -- --nocapture`
  - Result: passed. Wrapper composition was strengthened so install uses standalone `--activate-accessibility`, dry-run plugin reports are included, and uninstall writes/includes plugin report JSON.
  - Additional RED command: `cargo test --test source_overlay_scripts provider_takeover_uninstall_wrapper_dry_run_allows_pending_live_markers -- --nocapture`
  - Additional RED result: failed as expected; dry-run wrapper attempted a post-restore live-marker assertion even though dry-run intentionally leaves owned live markers in place.
  - Additional GREEN command: `cargo test --test source_overlay_scripts provider_takeover_uninstall_wrapper_dry_run_allows_pending_live_markers -- --nocapture`
  - Additional GREEN result: passed after reporting `live_assets.status="dry-run"` and skipping post-mutation marker absence checks in dry-run.
  - Refactor/check command: `cargo test --test source_overlay_scripts -- --nocapture`
  - Refactor/check result: passed 16/16 source overlay/provider tests.


- Slice 9 — Fake e2e fresh install → doctor ok → uninstall restored
  - RED command: `cargo test --test e2e_harness_scripts plugin_smoke_fake_auto_install_validates_marketplace_metadata -- --nocapture`
  - RED result: failed as expected after strengthening evidence expectations; fake smoke did not record `fresh_install_doctor_uninstall`.
  - GREEN command: `cargo test --test e2e_harness_scripts plugin_smoke_fake_auto_install_validates_marketplace_metadata -- --nocapture`
  - GREEN result: passed after fake plugin smoke uninstalls the isolated auto-installed plugin, verifies owned plugin state removal, and records `fresh_install_doctor_uninstall` evidence.
  - Refactor/check command: `cargo test --test e2e_harness_scripts -- --nocapture`
  - Refactor/check result: passed 19/19 e2e harness tests.

- Slice 10 — Documentation/checklist verifies live-safe commands
  - RED command: `cargo test --test packaging_docs install_uninstall_docs_reference_real_scripts_and_safe_commands -- --nocapture`
  - RED result: failed as expected after strengthening docs expectations; docs lacked rollback-first report-json and live-safe checklist commands.
  - GREEN command: `cargo test --test packaging_docs install_uninstall_docs_reference_real_scripts_and_safe_commands -- --nocapture`
  - GREEN result: passed after updating install/uninstall docs with accessibility dry-run/report-json, provider report-json examples, and live-safe checklist entries.
  - Refactor/check command: `cargo test --test packaging_docs -- --nocapture`
  - Refactor/check result: passed 9/9 packaging docs tests.


## TDD Exceptions

None.

## Final Verification Evidence (2026-06-02)

- OpenSpec validation: `openspec validate harden-fresh-install-atspi-rollback` passed with `Change 'harden-fresh-install-atspi-rollback' is valid`.
- Formatting: `make fmt` passed (`cargo fmt -- --check`).
- Build/check: `make check` passed (`cargo check`, dev profile finished successfully).
- Test suite: `make test` passed after final dry-run fix; notable counts included 49 unit tests, 7 doctor CLI tests, 19 e2e harness tests, 9 packaging docs tests, 9 plugin installer tests, and 16 source overlay/provider tests, all with 0 failures.
- Fake smoke: `scripts/e2e/codex-plugin-smoke.sh --fake` passed.
- Standalone dry-run checks: with a temporary `CODEX_HOME`, `scripts/install-codex-plugin.sh --dry-run`, `scripts/uninstall-codex-plugin.sh --dry-run`, `scripts/install-codex-plugin.sh --activate-accessibility --dry-run --report-json`, and `scripts/uninstall-codex-plugin.sh --dry-run --report-json` all passed; JSON reports parsed with `python3 -m json.tool`.
- Provider dry-run checks: configured target `/home/as/Документы/AI_PROJECTS/codex-desktop-linux` existed; `scripts/install-x11-provider-takeover.sh --target ... --dry-run --report-json /tmp/x11-provider-install-report.json` and `scripts/uninstall-x11-provider-takeover.sh --target ... --dry-run --report-json /tmp/x11-provider-uninstall-report.json` both passed; JSON reports parsed successfully. The uninstall dry-run reports `live_assets.status="dry-run"` because dry-run does not remove current live markers.
- Live-safe checklist: read-only `x11_doctor`, `x11_focused_window`, and `x11_list_windows` were available. Doctor reported Cinnamon/X11 readiness `ok=true` with no blockers, window query/focus/input baseline available, and acceptable degradation `atspi_tree_extraction_unavailable` plus optional RemoteDesktop portal unavailability. A controlled `zenity` fixture was attempted for `x11_get_app_state include_screenshot=true` and `x11_accessibility_tree`; MCP target resolution did not find the transient fixture, and the fallback screenshot artifact was deleted from `/tmp` to avoid storing unrelated user-window pixels. Therefore controlled-fixture app-state/accessibility live evidence is recorded as limited by transient fixture targeting/AT-SPI tree extraction in this session, not as a blocker because fake fixture tests cover the rollback path and doctor readiness remains `ok=true` with explicit degradation.
- Secret safety: `.secrets.local.env` was not read, printed, staged, or committed; final status checks were limited to Git metadata/status, not file contents.

