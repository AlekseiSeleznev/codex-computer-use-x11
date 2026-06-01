## 1. Docs-check TDD harness

- [x] 1.1 Add the first `tests/packaging_docs.rs` RED check for README v1 quick-start posture, required docs links, and v1 out-of-scope statements.
- [x] 1.2 Add minimal README/doc-link updates to satisfy the README quick-start check, then record RED/GREEN evidence in `test-plan.md`.

## 2. Install/uninstall documentation

- [x] 2.1 Add RED docs checks for `docs/install-uninstall.md`, real script paths, safe plugin `--dry-run`, source-overlay `--help`, e2e `--help`, and source-overlay status/install/test/uninstall/final-clean snippets.
- [x] 2.2 Add `docs/install-uninstall.md` and README links/content to pass install/uninstall docs checks, then record RED/GREEN evidence.

## 3. License and upstreaming documentation

- [x] 3.1 Add RED docs checks requiring `docs/license-attribution.md` to classify refreshed references, copy-safe/copy-unsafe sources, runtime command dependencies, and no-copy posture.
- [x] 3.2 Add `docs/license-attribution.md` to pass the license/attribution checks, then record RED/GREEN evidence.
- [x] 3.3 Add RED docs checks requiring `docs/upstreaming.md` to separate backend-upstream and wrapper-integration targets.
- [x] 3.4 Add `docs/upstreaming.md` to pass the upstream target matrix checks, then record RED/GREEN evidence.

## 4. Troubleshooting and release checklist documentation

- [x] 4.1 Add RED docs checks requiring `docs/troubleshooting.md` to cover degraded capability layers, strict RemoteDesktop false positives, plugin issues, source-overlay drift, and e2e logs.
- [x] 4.2 Add `docs/troubleshooting.md` to pass troubleshooting checks, then record RED/GREEN evidence.
- [x] 4.3 Add RED docs checks requiring `docs/release-checklist.md` to gate OpenSpec/project/docs/e2e/rollback/license/git/secret-safety evidence.
- [x] 4.4 Add `docs/release-checklist.md` to pass release checklist checks, then record RED/GREEN evidence.

## 5. Verification and handoff

- [x] 5.1 Run `cargo test --test packaging_docs` and refactor docs/test helpers only while green.
- [x] 5.2 Run `make fmt`, `make check`, and `make test`; fix any docs/test regressions and keep evidence in `test-plan.md`.
- [x] 5.3 Run `openspec validate prepare-x11-backend-packaging-docs-upstreaming --type change --strict` and `openspec validate --all --strict`.
- [x] 5.4 Confirm project `git status --short` is clean except expected checkpointed changes and confirm no real secret files are staged or tracked.
