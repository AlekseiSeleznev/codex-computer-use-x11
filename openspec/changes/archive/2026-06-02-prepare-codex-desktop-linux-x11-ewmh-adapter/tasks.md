## 1. Release package TDD slices

- [x] 1.1 Add RED Rust release-package test for versioned tarball and SHA256 sidecar through `scripts/package-release.sh`.
- [x] 1.2 Implement shared plugin bundle writer and `scripts/package-release.sh` until artifact/checksum test is GREEN.
- [x] 1.3 Add RED release-package test for extracted plugin bundle manifests, executable binary, icon, and `RELEASE-METADATA.json`.
- [x] 1.4 Extend bundle writer/package script until extracted bundle metadata test is GREEN and installer metadata remains compatible.
- [x] 1.5 Add RED release-package test for forbidden path exclusion and artifact filename/version consistency.
- [x] 1.6 Harden package script staging/check mode until forbidden-file and checksum checks are GREEN.

## 2. Adapter contract docs and scaffold TDD slices

- [x] 2.1 Add RED docs/scaffold consistency tests for adapter contract doc links and safe status wording in `README.md`, `INSTALL_CODEX.md`, and `CHANGELOG.md`.
- [x] 2.2 Add `docs/codex-desktop-linux-x11-ewmh-adapter.md` and minimal docs cross-links/changelog wording until docs tests are GREEN.
- [x] 2.3 Add RED scaffold consistency tests for required scaffold files, `feature.json` id/defaultEnabled/entrypoints, README non-goals, staging modes, and exposed `x11_*` tool list.
- [x] 2.4 Add inert scaffold files under `adapters/codex-desktop-linux/linux-features/x11-ewmh-computer-use/` until scaffold consistency tests are GREEN.

## 3. Scaffold stage and patch behavior TDD slices

- [x] 3.1 Add RED self-contained Node tests for disabled/enabled Linux Feature discovery using upstream `scripts/lib/linux-features.js` through env/default lookup.
- [x] 3.2 Implement scaffold `feature.json`, `patches.js`, and test helper setup until disabled/enabled hook and descriptor tests are GREEN.
- [x] 3.3 Add RED Node test for `stage.sh` with fake executable binary and temporary install/work directories.
- [x] 3.4 Implement `stage.sh` direct binary/source/tarball/download mode handling, plugin staging, and marketplace update until fake binary staging test is GREEN.
- [x] 3.5 Add RED Node tests for preserving an existing `computer-use` fixture and plugin gate patch idempotence/narrowness.
- [x] 3.6 Harden `stage.sh` and `patches.js` until existing bundled plugin preservation and idempotent patch tests are GREEN.

## 4. Integration, docs polish, and verification

- [x] 4.1 Refactor shared metadata/constants to remove duplication while keeping all release/scaffold/docs tests GREEN.
- [x] 4.2 Run `node adapters/codex-desktop-linux/linux-features/x11-ewmh-computer-use/test.js` and record evidence in `test-plan.md`.
- [x] 4.3 Run `scripts/package-release.sh --check`, extract the tarball, run extracted `doctor --json`, and record JSON/version/backend/checksum/forbidden-file evidence in `test-plan.md`.
- [x] 4.4 Run `make fmt`, `make check`, `make test`, `openspec validate --all --strict`, and `git diff --check`; fix any failures.
- [x] 4.5 Update `test-plan.md` Evidence Log with RED/GREEN/verification evidence and mark completed tasks only after evidence exists.
- [x] 4.6 Prepare final summary with deliverables, verification results, no-release/no-archive status, and next upstream PR plan.
