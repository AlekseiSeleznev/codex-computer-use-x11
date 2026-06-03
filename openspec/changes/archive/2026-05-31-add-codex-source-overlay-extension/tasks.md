## 1. Overlay script public interface TDD

- [x] 1.1 RED: Add `tests/source_overlay_scripts.rs::install_refuses_missing_target_structure`; run focused cargo test and record the expected failure in `test-plan.md`.
- [x] 1.2 GREEN: Add executable shell wrappers and initial overlay engine target preflight so missing structures fail without mutation.
- [x] 1.3 RED/GREEN: Add clean status/default target resolution tests and implement `status` target resolution plus `state=clean` reporting.

## 2. Install/idempotence/status TDD

- [x] 2.1 RED: Add fake-target install test proving generated backend file and owned marker blocks are required; record failing evidence.
- [x] 2.2 GREEN: Implement generated `x11_ewmh.rs` template and marker-block patching for target backend module, registry, windowing module tests/exports, and diagnostics strict portal check.
- [x] 2.3 RED/GREEN: Add repeated-install idempotence test and make install avoid duplicate markers/files.
- [x] 2.4 RED/GREEN: Add applied/drifted status test and implement marker/content drift detection with non-zero exit for drift.

## 3. Uninstall and conflict safety TDD

- [x] 3.1 RED: Add uninstall test proving owned markers/backend are removed and unrelated target content is preserved; record failing evidence.
- [x] 3.2 GREEN: Implement uninstall marker removal and owned generated backend deletion with idempotent clean behavior.
- [x] 3.3 RED/GREEN: Add unowned native X11 backend conflict test and implement install refusal without overwriting unowned files/registrations.

## 4. Documentation and integration guidance

- [x] 4.1 RED: Run grep/docs checks showing source-overlay command guidance is absent; record evidence.
- [x] 4.2 GREEN: Update `README.md` and `docs/integration-contract.md` with install/status/uninstall usage, experimental/reversible warning, target cleanliness, and stock tool boundary.

## 5. Real target smoke, verification, and safety checks

- [x] 5.1 Run focused fake-target test suite and full project checks: `make fmt`, `make check`, `make test`; fix issues and record evidence.
- [x] 5.2 Run real target status/install/target cargo tests/uninstall/final clean smoke against `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` or configured target; record exact evidence and uninstall even on failure.
- [x] 5.3 Run `openspec validate add-codex-source-overlay-extension --strict` and record evidence.
- [x] 5.4 Verify local repo status, target repo status, and no `.secrets.local.env` tracking/staging; record evidence.
- [x] 5.5 Mark tasks complete only after matching evidence is present in `test-plan.md`.
