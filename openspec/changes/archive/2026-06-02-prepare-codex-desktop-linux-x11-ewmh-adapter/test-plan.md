## TDD Strategy

Use the project-local `tdd` skill with vertical behavior slices. Each slice starts with one public-interface test/check that fails for the current repository state, then adds the minimum production/docs/scaffold code for GREEN, then refactors only while the relevant checks remain green. Tests should verify observable behavior through script outputs, tarball contents, JSON manifests, docs text, or scaffold stage/patch behavior rather than private implementation details.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1. Release package command exists and emits artifact/checksum | `cargo test --test release_package package_release_script_creates_versioned_tarball_and_checksum` | Fails because `scripts/package-release.sh` and/or release artifact output do not exist. | Same test passes: tarball and `.sha256` are created in a temp output directory, filename includes `VERSION`, and checksum verifies. | Keep script options small; no repo-root tar; no unrelated build-system changes. |
| 2. Release bundle contains executable plugin manifests and metadata | `cargo test --test release_package release_tarball_contains_ready_plugin_bundle` | Fails because extracted tarball lacks `.mcp.json`, `.codex-plugin/plugin.json`, executable `bin/codex-computer-use-x11`, icon, or `RELEASE-METADATA.json`. | Same test passes and verifies command `./bin/codex-computer-use-x11`, args `["mcp"]`, plugin version equals `VERSION`, and metadata includes baseline/source/release/checksum fields. | Share manifest/bundle generation with installer where practical; preserve installer rollback/config code. |
| 3. Release artifact excludes forbidden paths | `cargo test --test release_package release_tarball_excludes_forbidden_files` | Fails until package script stages an explicit minimal bundle and tests inspect tar listing. | Same test passes with no `.git/`, `target/`, `.codex/session/`, `.secrets*`, local env, or backup files in tarball. | Keep forbidden pattern list centralized in test/script if useful; do not broaden artifact contents speculatively. |
| 4. Adapter contract docs and top-level docs are linked | `cargo test --test packaging_docs adapter_contract_docs_are_linked_and_status_safe` | Fails because `docs/codex-desktop-linux-x11-ewmh-adapter.md` and README/INSTALL/CHANGELOG links/wording are missing. | Same test passes and proves docs say prepared contract only, not upstream merged/default enabled. | Keep docs wording precise; no release-published claims. |
| 5. Scaffold files and manifest contract exist | `cargo test --test packaging_docs downstream_adapter_scaffold_matches_linux_feature_contract` | Fails because scaffold files are missing. | Same test passes for `feature.json` id/defaultEnabled/entrypoints, README non-goals, stage/patch/test file presence, and tool list. | Keep scaffold under `adapters/` only; no upstream checkout writes. |
| 6. Scaffold stage hook and patch behavior | `node adapters/codex-desktop-linux/linux-features/x11-ewmh-computer-use/test.js` | Fails because scaffold test/stage/patch behavior is missing or cannot locate upstream Linux Feature helper. | Node tests pass: disabled feature exposes no hooks/descriptors, enabled feature exposes hook/descriptors, fake binary staging writes plugin and marketplace, existing `computer-use` fixture is untouched, patch is idempotent/narrow. | Keep test.js runnable locally through upstream env/default lookup and runnable after upstream copy through relative lookup. |
| 7. Package script self-check and extracted doctor JSON | `scripts/package-release.sh --check` | Fails until the script can build/package/verify/extract and run extracted doctor JSON. | Command passes or reports only structured environment blockers while still validating JSON, version, backend, checksum, and forbidden paths. | Avoid introducing live X11 dependence into deterministic checks; accept structured doctor degraded/blocker JSON when environment lacks X11. |

## Mocking / Boundary Policy

- Mock only filesystem/app-staging boundaries with temporary directories and fake executable binaries.
- Do not mock the package script internals; tests should invoke `scripts/package-release.sh` as the public interface.
- Do not mutate the real upstream checkout. Scaffold Node tests may read upstream `scripts/lib/linux-features.js` from `CODEX_DESKTOP_LINUX_REPO`, `CODEX_DESKTOP_LINUX_FULL_PATH`, or the documented local default, but all feature/stage files and install roots must be temporary.
- Do not read `.secrets.local.env`; no credentials are needed.
- Do not target real user applications for tests; extracted `doctor --json` is allowed to report structured environment blockers.

## Required Checks

- Slice-level RED/GREEN commands listed above.
- `make fmt`
- `make check`
- `make test`
- `openspec validate --all --strict`
- `git diff --check`
- `scripts/package-release.sh --check`
- Extract generated tarball and run `./codex-computer-use-x11/bin/codex-computer-use-x11 doctor --json`; verify valid JSON, `version` equals `VERSION`, `backend` equals `x11-ewmh`, and readiness is either `ok=true` or a structured blockers/degraded report.
- Verify tarball SHA256 matches and forbidden paths are absent.

## Evidence Log

- **Slice 1 RED**: `cargo test --test release_package package_release_script_creates_versioned_tarball_and_checksum -- --nocapture` failed because `scripts/package-release.sh` did not exist.
- **Slices 1-3 GREEN**: `cargo test --test release_package -- --nocapture` passed 3/3 tests after adding `scripts/lib/plugin-bundle.py`, `scripts/package-release.sh`, and release artifact tests. The tests verify versioned tarball creation, `.sha256` verification, extracted executable bundle, `.mcp.json`, plugin manifest version/display metadata, `RELEASE-METADATA.json`, and forbidden path exclusion.
- **Slice 4 RED**: `cargo test --test packaging_docs adapter_contract_docs_are_linked_and_status_safe -- --nocapture` failed because `docs/codex-desktop-linux-x11-ewmh-adapter.md` was missing.
- **Slices 4-5 GREEN**: `cargo test --test packaging_docs -- --nocapture` passed 11/11 tests after adding adapter contract docs, README/INSTALL/CHANGELOG links, scaffold files, and docs/scaffold consistency tests.
- **Slice 6 GREEN**: `node adapters/codex-desktop-linux/linux-features/x11-ewmh-computer-use/test.js` passed 3/3 tests, covering disabled/enabled Linux Feature discovery, plugin gate idempotence/narrowness, fake binary staging, marketplace entry insertion, and preservation of an existing `computer-use` fixture.
- **Slice 7 / verification RED**: initial `make fmt` failed on formatting diffs in `tests/packaging_docs.rs` and `tests/release_package.rs`; `cargo fmt` fixed the formatting.
- **Slice 7 GREEN / final verification**: `make fmt`, `make check`, `make test`, `node adapters/codex-desktop-linux/linux-features/x11-ewmh-computer-use/test.js`, `openspec validate --all --strict`, `git diff --check`, and `scripts/package-release.sh --check` all passed. The package check built the release binary, verified the sidecar SHA256, extracted the tarball, validated manifests, and ran extracted `doctor --json`.
- **Extracted doctor evidence**: manual extraction of `dist/release/codex-computer-use-x11-v0.1.2-x86_64-unknown-linux-gnu.tar.gz` produced valid JSON with `project=codex-computer-use-x11`, `version=0.1.2`, `backend=x11-ewmh`, and `readiness.ok=true` on the current Cinnamon/X11 machine. Manual sidecar verification from `dist/release` reported `OK`. Generated release artifacts were removed after verification because publishing/release asset retention requires separate approval.

## TDD Exceptions

None.
