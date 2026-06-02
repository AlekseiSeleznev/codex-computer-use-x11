## TDD Strategy

Use the project-local `tdd` skill for documentation work by treating documentation checks as the public interface. Each slice starts with one failing Rust integration test in `tests/packaging_docs.rs`, then adds the minimum README/docs content to pass, and refactors only while the slice is green. Tests should verify stable observable contracts: section headings, script paths, command `--help`/`--dry-run` behavior, required matrix/table entries, and safety language. They must not execute live source-overlay mutation or rely on a real desktop session.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1 | README v1 quick start and required docs links | `cargo test --test packaging_docs readme_v1_quick_start_links_required_docs` fails because README lacks the new quick-start/doc-link contract | Same command passes after README describes v1 posture, standalone plugin/source overlay paths, out-of-scope Wayland/extension/native packaging, and links required docs | Keep README concise; move deep detail to docs files |
| 2 | Install/uninstall docs and executable command snippets | `cargo test --test packaging_docs install_uninstall_docs_reference_real_scripts_and_safe_commands` fails because `docs/install-uninstall.md` is missing or lacks required commands | Same command passes after docs show plugin dry-run/install/uninstall, source-overlay status/install/test/uninstall/final-clean commands, and tests execute supported `--help`/isolated `--dry-run` snippets | Do not run live target install/uninstall in tests; use isolated `CODEX_HOME` for plugin dry-runs |
| 3 | License/attribution docs classify references and runtime command dependencies | `cargo test --test packaging_docs license_attribution_docs_classify_references_and_commands` fails because license docs/table are missing | Same command passes after `docs/license-attribution.md` contains observed-date license table, copy-safe/copy-unsafe policy, runtime command dependency distinction, and no-copy statement | Avoid legal advice; say observed metadata must be rechecked before copying |
| 4 | Upstream target matrix separates backend and wrapper targets | `cargo test --test packaging_docs upstreaming_docs_separate_backend_and_wrapper_targets` fails because upstreaming matrix is missing | Same command passes after `docs/upstreaming.md` maps backend/windowing work to Computer Use Linux lineage and packaging/wrapper work to Codex Desktop Linux lineage | Matrix labels should be stable; prose can evolve |
| 5 | Troubleshooting docs cover degraded layers and source-overlay drift | `cargo test --test packaging_docs troubleshooting_docs_cover_degraded_layers_and_drift` fails because troubleshooting docs are missing | Same command passes after `docs/troubleshooting.md` covers doctor layers, command dependencies, strict RemoteDesktop false positives, screenshot/AT-SPI degradation, plugin issues, source-overlay drift, and e2e logs | Keep commands safe; no secrets or private paths beyond variable names |
| 6 | Release checklist gates handoff evidence and secret safety | `cargo test --test packaging_docs release_checklist_requires_validation_evidence_and_secret_safety` fails because release checklist is missing | Same command passes after `docs/release-checklist.md` includes OpenSpec strict validation, `make fmt/check/test`, fake e2e, optional live smoke, rollback, license refresh, and git status checks | Checklist should be actionable without requiring secrets |
| 7 | Full docs/test integration | `cargo test --test packaging_docs` plus `make test` pass after all slices | Full docs test and project test suite pass with no live mutation | Refactor duplicated assertion helpers only after all docs slices are green |

## Mocking / Boundary Policy

- Use no internal mocks for docs content.
- For command snippets, execute only safe public boundaries:
  - `--help` for source-overlay and e2e wrappers.
  - Plugin install/uninstall `--dry-run` with an isolated temporary `CODEX_HOME` created under `std::env::temp_dir()`.
- Do not run live source-overlay install/uninstall, live input, or real desktop mutation from docs tests.
- Do not read `.secrets.local.env`; tests may assert docs mention variable names such as `CODEX_DESKTOP_LINUX_FULL_PATH` only.

## Required Checks

- `openspec validate prepare-x11-backend-packaging-docs-upstreaming --type change --strict` during artifacts/apply.
- `cargo test --test packaging_docs` for docs slices.
- `make fmt`.
- `make check`.
- `make test`.
- `openspec validate --all --strict` before archive.
- `git status --short` in the project before archive and push.
- Optional live target sanity only if the target checkout is clean and available: status/fake or live source-overlay smoke as documented in release checklist; if skipped, record the reason.

## Evidence Log


- Slice 1 RED: `cargo test --test packaging_docs readme_v1_quick_start_links_required_docs` failed because `README.md` lacked `## Quick start` and required v1 doc-link/scope phrases.
- Slice 1 GREEN: same command passed after adding README quick start and documentation links for the v1 standalone plugin/source-overlay handoff.
- Slice 2 RED: `cargo test --test packaging_docs install_uninstall_docs_reference_real_scripts_and_safe_commands` failed because `docs/install-uninstall.md` did not exist.
- Slice 2 GREEN: same command passed after adding `docs/install-uninstall.md` with plugin dry-run/install/uninstall, source-overlay status/install/test/uninstall/final-clean, fake/live smoke, and drift guidance.
- Slice 3 RED: `cargo test --test packaging_docs license_attribution_docs_classify_references_and_commands` failed because `docs/license-attribution.md` did not exist.
- Slice 3 GREEN: same command passed after adding observed-date license/reference table, runtime command dependency policy, copy-safe/copy-unsafe classifications, and no-copy statement.
- Slice 4 RED: `cargo test --test packaging_docs upstreaming_docs_separate_backend_and_wrapper_targets` failed because `docs/upstreaming.md` did not exist.
- Slice 4 GREEN: same command passed after adding upstream target matrix that separates backend-upstream and wrapper-integration work and preserves source-overlay-as-staging guidance.
- Slice 5 RED: `cargo test --test packaging_docs troubleshooting_docs_cover_degraded_layers_and_drift` failed because `docs/troubleshooting.md` did not exist.
- Slice 5 GREEN: same command passed after adding troubleshooting coverage for doctor/session layers, strict RemoteDesktop false positives, screenshot/AT-SPI degradation, standalone plugin issues, source-overlay drift, and e2e logs.
- Slice 6 RED: `cargo test --test packaging_docs release_checklist_requires_validation_evidence_and_secret_safety` failed because `docs/release-checklist.md` did not exist.
- Slice 6 GREEN: same command passed after adding release checklist coverage for OpenSpec/project/docs/e2e/optional-live/rollback/license/git and secret-safety evidence.
- Slice 7 GREEN: `cargo test --test packaging_docs` passed with 6 docs-check tests covering README quick start, install/uninstall, license/attribution, upstreaming, troubleshooting, and release checklist.
- Verification GREEN: initial `make fmt` failed on `tests/packaging_docs.rs` formatting; `cargo fmt` fixed it. Then `make fmt`, `make check`, and `make test` all passed.
- OpenSpec GREEN: `openspec validate prepare-x11-backend-packaging-docs-upstreaming --type change --strict` passed; `openspec validate --all --strict` passed with 15 items.
- Git/secret safety: `git status --short` showed only expected apply evidence updates before checkpoint; `git ls-files` showed no tracked `.secrets.local.env`; `.secrets.example.env` is the tracked empty/example file.

## TDD Exceptions

None.
