## TDD Strategy

Use the project-local TDD discipline with vertical slices around the public script interfaces. Add tests that invoke `scripts/install-codex-desktop-linux-x11-feature.sh` and `scripts/uninstall-codex-desktop-linux-x11-feature.sh` against fake target/install directories. RED failures should be missing scripts/options/behavior; GREEN should implement the smallest script/helper behavior for each slice.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1 | Installer dry-run report does not mutate fixtures | `cargo test --test codex_desktop_feature_installer -- installer_dry_run_reports_plan_without_mutation` fails because the script/report is missing | Same command passes: report JSON includes target/install/feature/surfaces and fixture files are absent/unchanged | Keep path resolution and report construction pure enough for tests; no real `/opt` writes |
| 2 | Installer stages feature/plugin and preserves bundled `computer-use` | `cargo test --test codex_desktop_feature_installer -- installer_stages_feature_plugin_and_manifest` fails because install does not stage expected files | Same command passes with fake binary/source: feature config enabled, plugin staged, marketplace has both plugins, manifest records completed entries | Reuse adapter `stage.sh`; avoid duplicate plugin bundle semantics |
| 3 | Uninstaller dry-run and clean rollback | `cargo test --test codex_desktop_feature_installer -- uninstaller_restores_clean_install_and_supports_dry_run` fails because rollback is missing | Same command passes: dry-run leaves state intact, real uninstall restores marketplace/app/config/plugin to before-state | Rollback code must be entry-driven and idempotent |
| 4 | Uninstaller drift blocker | `cargo test --test codex_desktop_feature_installer -- uninstaller_blocks_on_drift` fails because drift is not detected | Same command passes: modified after-state causes non-zero exit and JSON blocker; drifted file is not overwritten | Drift errors should be structured and path-specific |
| 5 | Documentation/OpenSpec validation | `openspec validate add-codex-desktop-linux-feature-installers` and targeted docs tests fail before docs are updated | Validation and targeted tests pass after docs/scripts are aligned | Docs must label fake patch mode as test-only and avoid secrets |

## Mocking / Boundary Policy

- Mock only external filesystem/app boundaries by using temporary target/install fixtures.
- Use a fake executable binary for adapter staging; it may output minimal `doctor --json` data when invoked.
- Use `--patch-mode fake` only in tests to avoid real `app.asar`/Node tooling.
- Do not mock the script entrypoints, JSON report parsing, feature config editing, manifest creation, backup restore, or drift checks.

## Required Checks

- `openspec validate add-codex-desktop-linux-feature-installers`
- `cargo test --test codex_desktop_feature_installer`
- `cargo test --test source_overlay_scripts` if shared script behavior or docs interact with existing adapter/source-overlay tests
- `make fmt`, `make check`, and `make test` if full project checks are feasible; otherwise record exact blockers without claiming full pass
- `git status --short --untracked-files=all` before and after mutation/checkpoints

## Evidence Log

- RED slice 1-4: `cargo test --test codex_desktop_feature_installer -- --nocapture` failed because `scripts/install-codex-desktop-linux-x11-feature.sh` did not exist yet; all four planned fixture tests failed at the public script boundary.
- GREEN slice 1-4: `cargo test --test codex_desktop_feature_installer -- --nocapture` passed after adding the installer/uninstaller shell entrypoints, shared Python engine, manifest-backed install/rollback, fake patch fixture mode, and docs.

## TDD Exceptions

None

- VERIFY: `openspec validate add-codex-desktop-linux-feature-installers` passed.
- VERIFY: `cargo test --test codex_desktop_feature_installer` passed (4 tests).
- VERIFY: `cargo test --test source_overlay_scripts` passed (16 tests).
- VERIFY: `make fmt && make check && make test` passed; full test suite included 49 unit tests plus all integration/doc tests, including the new feature installer tests and release packaging tests.
