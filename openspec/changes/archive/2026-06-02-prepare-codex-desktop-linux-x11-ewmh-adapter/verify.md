# Verification Report: prepare-codex-desktop-linux-x11-ewmh-adapter

## Summary

| Dimension | Status |
| --- | --- |
| Completeness | 22/22 tasks complete; 8/8 planning artifacts complete |
| Correctness | Delta requirements covered by release package tests, docs/scaffold tests, Node scaffold tests, package `--check`, and OpenSpec validation |
| Coherence | Follows design, design-review, test-plan, ADR 0009/0010/0011 constraints, and no-release/no-upstream-mutation boundary |

## Checks Run

- `make fmt` — passed after formatting RED evidence was resolved with `cargo fmt`.
- `make check` — passed.
- `make test` — passed, including `tests/release_package.rs` and new adapter docs/scaffold assertions in `tests/packaging_docs.rs`.
- `node adapters/codex-desktop-linux/linux-features/x11-ewmh-computer-use/test.js` — passed 3/3 tests.
- `openspec validate --all --strict` — passed 20/20 items.
- `git diff --check` — passed.
- `scripts/package-release.sh --check` — passed; built release binary, generated tarball and `.sha256`, verified checksum, extracted bundle, validated manifests, and ran extracted `doctor --json`.
- Manual extraction/doctor evidence recorded in `test-plan.md`: extracted binary emitted valid JSON with `project=codex-computer-use-x11`, `version=0.1.2`, `backend=x11-ewmh`, and `readiness.ok=true` on the current Cinnamon/X11 machine.

## Requirement Coverage

- Release artifact and checksum: covered by `tests/release_package.rs` and `scripts/package-release.sh --check`.
- Ready Codex plugin bundle: covered by extracted `.mcp.json`, `.codex-plugin/plugin.json`, icon, binary executable, and metadata assertions.
- Forbidden file exclusion: covered by tar listing tests and package script check.
- Adapter contract documentation: covered by `tests/packaging_docs.rs` and docs cross-links.
- Copyable disabled-by-default scaffold: covered by Rust docs/scaffold checks and Node Linux Feature discovery tests.
- Stage hook and marketplace safety: covered by Node fake-binary staging test preserving existing `computer-use` fixture.
- Plugin gate patch idempotence/narrowness: covered by Node patch test.

## Issues

### CRITICAL

- None.

### WARNING

- None.

### SUGGESTION

- Generated release tarball assets are intentionally not committed in this change. Publishing a GitHub release and retaining/uploading assets should be done only after explicit user approval.

## Final Assessment

All checks passed. The change is verify-ready. Do not archive, publish, push, or open an upstream PR without separate explicit approval.
