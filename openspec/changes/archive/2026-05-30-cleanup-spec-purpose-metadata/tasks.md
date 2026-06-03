## 1. Apply Preconditions

- [x] 1.1 Reconfirm `git status --short` is clean before apply and no unrelated work is present.
- [x] 1.2 Re-run `openspec validate cleanup-spec-purpose-metadata --type change --json`; treat failure as a blocker.
- [x] 1.3 Record in `test-plan.md` that `.secrets.local.env`, external systems, Rust code, and the local integration target checkout are not needed for this metadata-only change.

## 2. RED Text Checks

- [x] 2.1 RED Slice 1: run the `doctor-cli` Purpose placeholder absence check from `test-plan.md` and record the expected failure while canonical `openspec/specs/doctor-cli/spec.md` still contains `TBD` / bootstrap archive placeholder text.
- [x] 2.2 RED Slice 2: run the `x11-integration-contract` Purpose placeholder absence check from `test-plan.md` and record the expected failure while canonical `openspec/specs/x11-integration-contract/spec.md` still contains `TBD` / bootstrap archive placeholder text.

## 3. GREEN Metadata Edits

- [x] 3.1 GREEN Slice 1: replace only the `doctor-cli` canonical `## Purpose` prose with a concise description of the `doctor --json` smoke-test and capability/readiness report contract; do not change existing requirements/scenarios.
- [x] 3.2 GREEN Slice 2: replace only the `x11-integration-contract` canonical `## Purpose` prose with a concise description of X11/EWMH backend identity, source-overlay compatibility, and integration constraints; do not change existing requirements/scenarios.
- [x] 3.3 Re-run both placeholder absence checks and record passing GREEN evidence in `test-plan.md`.

## 4. Apply Verification

- [x] 4.1 Run `git diff --name-only` and confirm the implementation diff is limited to `openspec/specs/doctor-cli/spec.md`, `openspec/specs/x11-integration-contract/spec.md`, and change-local evidence/tasks files.
- [x] 4.2 Run `openspec validate cleanup-spec-purpose-metadata --type change --json`.
- [x] 4.3 Run `openspec validate --all --strict`.
- [x] 4.4 If any Rust file changed unexpectedly, stop and run `make fmt`, `make check`, and `make test` before marking apply complete; otherwise record that Rust checks are not applicable.

## 5. Archive Readiness

- [x] 5.1 Ensure `test-plan.md` evidence log is complete for all four slices.
- [x] 5.2 Run the canonical verification workflow or produce an equivalent verification report with no CRITICAL issues before archive.
- [x] 5.3 Before archive, confirm `git status --short` is clean except intended archive/spec-sync changes.
- [x] 5.4 After archive/spec sync, re-run the placeholder absence checks and `openspec validate --all --strict`; do not consider the archive complete if `TBD` or the bootstrap archive placeholder reappears in either canonical Purpose section.
