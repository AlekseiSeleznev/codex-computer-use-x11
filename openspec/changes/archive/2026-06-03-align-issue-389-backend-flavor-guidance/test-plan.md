## TDD Strategy

Use documentation checks as public-interface tests because this change changes maintainer-facing contract behavior, not runtime MCP behavior. Each slice starts with a failing Rust integration test in `tests/packaging_docs.rs`, then updates the minimum docs/spec-facing files needed for GREEN. No internal helpers will be mocked.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1 | Adapter contract document records `agent-sh/computer-use-linux` selectable backend/flavor as a separate future evaluation path while preserving current thin adapter default. | Add one focused test in `tests/packaging_docs.rs`, then run `cargo test --test packaging_docs adapter_contract_records_backend_flavor_guidance`; expect failure because current doc lacks the new guidance. | Update `docs/codex-desktop-linux-x11-ewmh-adapter.md`, rerun the same command; expect pass. | Keep assertions concept-based, not exact full-paragraph snapshots. |
| 2 | Copyable scaffold README separates current opt-in Linux Feature behavior from backend/flavor experiments and preserves no-default-behavior-change boundary. | Add one focused test in `tests/packaging_docs.rs`, then run `cargo test --test packaging_docs scaffold_readme_records_backend_flavor_guidance`; expect failure because current README lacks the new section. | Update `adapters/codex-desktop-linux/linux-features/x11-ewmh-computer-use/README.md`, rerun the same command; expect pass. | Do not change `feature.json`, `stage.sh`, or `patches.js` unless a test proves a behavior gap. |
| 3 | OpenSpec delta and canonical docs remain valid after docs/test changes. | Run `openspec validate align-issue-389-backend-flavor-guidance --strict` and targeted docs tests after slices; any failure blocks completion. | Run `openspec validate --all --strict`, `make fmt`, `make check`, and `make test`; expect pass or exact documented blocker. | Keep archive out of scope until the user explicitly approves archive. |

## Mocking / Boundary Policy

No mocks. The tests read repository-tracked documentation/scaffold files as public maintainer-facing interfaces. Network access and GitHub API are not part of test execution; issue #389 is captured as human-reviewed requirement context in OpenSpec artifacts.

## Required Checks

- `cargo test --test packaging_docs adapter_contract_records_backend_flavor_guidance`
- `cargo test --test packaging_docs scaffold_readme_records_backend_flavor_guidance`
- `openspec validate align-issue-389-backend-flavor-guidance --strict`
- `openspec validate --all --strict`
- `make fmt`
- `make check`
- `make test`
- `git diff --check`
- `git status --short --untracked-files=all`

## Evidence Log

- Slice 1 RED: `cargo test --test packaging_docs adapter_contract_records_backend_flavor_guidance` failed because `docs/codex-desktop-linux-x11-ewmh-adapter.md` did not contain `agent-sh/computer-use-linux`.
- Slice 1 GREEN: `cargo test --test packaging_docs adapter_contract_records_backend_flavor_guidance` passed after adding upstream path guidance to `docs/codex-desktop-linux-x11-ewmh-adapter.md`.
- Slice 2 RED: `cargo test --test packaging_docs scaffold_readme_records_backend_flavor_guidance` failed because the copyable scaffold README did not contain `Upstream alignment`.
- Slice 2 GREEN: `cargo test --test packaging_docs scaffold_readme_records_backend_flavor_guidance` passed after adding scaffold guidance that keeps backend/flavor experiments separate from feature behavior.
- Targeted verification: both targeted docs tests passed, and `openspec validate align-issue-389-backend-flavor-guidance --strict` passed.
- Full verification: initial `make fmt` found formatting drift in `tests/packaging_docs.rs`; after `cargo fmt`, `make fmt`, `make check`, `make test`, `openspec validate --all --strict`, and `git diff --check` passed.
- Boundary verification: `feature.json` still has `defaultEnabled=false`; no runtime files under `src/` and no scaffold behavior files `feature.json`, `stage.sh`, `patches.js`, or `test.js` changed.

## TDD Exceptions

None.
