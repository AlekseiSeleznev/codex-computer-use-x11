## 1. Installer public interface and dry-run

- [x] 1.1 Add RED fixture test for `scripts/install-codex-desktop-linux-x11-feature.sh --dry-run --report-json -` proving planned surfaces are reported and no files mutate.
- [x] 1.2 Add shell entrypoint and shared helper command parser with `install`, path resolution, dry-run, report-json, and no-secret report fields.
- [x] 1.3 Mark slice 1 evidence in `test-plan.md` after RED/GREEN commands run.

## 2. Feature/plugin install and manifest

- [x] 2.1 Add RED fixture test proving install copies the adapter scaffold, enables `x11-ewmh-computer-use`, stages `codex-computer-use-x11`, preserves bundled `computer-use`, and writes a rollback manifest.
- [x] 2.2 Implement manifest-backed install entries for target local feature, target feature config, plugin dir, marketplace file, update-builder feature/config, and optional app/webview surfaces.
- [x] 2.3 Invoke the copied adapter `stage.sh` with local binary/source staging and record changed vs already-acceptable state.
- [x] 2.4 Mark slice 2 evidence in `test-plan.md` after RED/GREEN commands run.

## 3. Uninstall and drift safety

- [x] 3.1 Add RED fixture test for uninstall dry-run plus clean rollback of an installed fixture.
- [x] 3.2 Add RED fixture test for drift blocker when an installer-owned after-state file changes after install.
- [x] 3.3 Add `scripts/uninstall-codex-desktop-linux-x11-feature.sh` and helper rollback logic that restores only completed entries, blocks on drift, and is idempotent when absent.
- [x] 3.4 Mark slices 3 and 4 evidence in `test-plan.md` after RED/GREEN commands run.

## 4. Documentation and verification

- [x] 4.1 Update docs with local manual install/uninstall commands for source, binary, dry-run, report-json, patch modes, manifest location, and root-owned install notes.
- [x] 4.2 Run `openspec validate add-codex-desktop-linux-feature-installers`.
- [x] 4.3 Run targeted tests: `cargo test --test codex_desktop_feature_installer` and any impacted existing script tests.
- [x] 4.4 Run full project checks (`make fmt`, `make check`, `make test`) or record exact blockers.
- [x] 4.5 Commit the coherent apply group and report final changed files, verification evidence, and any limitations.
