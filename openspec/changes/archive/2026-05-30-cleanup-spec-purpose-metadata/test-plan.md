## TDD Strategy

This is documentation/spec-metadata work, so the project-local TDD discipline is applied with command-based observable checks rather than Rust unit tests. Each slice starts with a failing text/OpenSpec check against the current canonical specs, then applies the smallest metadata edit needed to pass. No Rust code should change.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1. `doctor-cli` Purpose is non-placeholder | Canonical `openspec/specs/doctor-cli/spec.md` Purpose section | `python3 - <<'PY' ...` check that extracts the `## Purpose` block and fails while it contains `TBD` or `bootstrap-codex-computer-use-x11` | Same command passes after replacing Purpose with concise doctor CLI JSON/capability-readiness prose | Only Purpose prose changes; requirements/scenarios stay semantically unchanged. |
| 2. `x11-integration-contract` Purpose is non-placeholder | Canonical `openspec/specs/x11-integration-contract/spec.md` Purpose section | `python3 - <<'PY' ...` check that extracts the `## Purpose` block and fails while it contains `TBD` or `bootstrap-codex-computer-use-x11` | Same command passes after replacing Purpose with concise X11/EWMH/source-overlay contract prose | Only Purpose prose changes; requirements/scenarios stay semantically unchanged. |
| 3. Repository/OpenSpec validation remains clean | OpenSpec validation and git diff scope | `openspec validate cleanup-spec-purpose-metadata --type change --json` plus `openspec validate --all --strict` after edits would fail if metadata breaks spec parsing; `git diff --name-only` fails if unexpected files changed | Both OpenSpec commands pass and diff scope is limited to the two canonical specs plus OpenSpec change artifacts | If Rust files change, stop and add `make fmt`, `make check`, and `make test` before apply completion. |
| 4. Archive preserves canonical Purpose text | Post-archive canonical specs | After archive, re-run the placeholder absence check; failure means archive sync regressed metadata | Placeholder absence check passes after archive, and `openspec validate --all --strict` passes | Do not archive if Purpose placeholders reappear. |

## Mocking / Boundary Policy

No mocks are required. Boundaries are the filesystem and OpenSpec CLI. Checks read only Git-tracked Markdown files and do not access `.secrets.local.env`, external systems, or the local integration target checkout.

## Required Checks

Before marking apply complete:

- Placeholder absence check for `openspec/specs/doctor-cli/spec.md`.
- Placeholder absence check for `openspec/specs/x11-integration-contract/spec.md`.
- `openspec validate cleanup-spec-purpose-metadata --type change --json`.
- `openspec validate --all --strict`.
- `git diff --name-only` confirms no Rust files, target-checkout paths, `.secrets.local.env`, or unrelated files changed.

Before archive:

- Re-run all apply-complete checks.
- Confirm `tasks.md` records evidence for each documentation slice.
- After archive/spec sync, re-run placeholder absence checks and `openspec validate --all --strict`.

## Evidence Log

## Apply Notes

- `.secrets.local.env`, external systems, Rust code, and the local integration target checkout were not needed for this metadata-only change.
- Rust checks (`make fmt`, `make check`, `make test`) are not applicable because no Rust files changed.

Fill during apply/verify.

| Slice | RED evidence (command / exit status / excerpt or log) | GREEN evidence (command / exit status / excerpt or log) | Refactor/check evidence (command / exit status / excerpt or log) | Notes |
| --- | --- | --- | --- | --- |
| 1. `doctor-cli` Purpose is non-placeholder | `python3 /tmp/check_purpose.py openspec/specs/doctor-cli/spec.md` / exit 1 / `placeholder purpose remains: TBD - created by archiving change bootstrap-codex-computer-use-x11...` | `python3 /tmp/check_purpose.py openspec/specs/doctor-cli/spec.md` / exit 0 / `purpose is non-placeholder: This specification defines the codex-computer-use-x11 doctor --json command...` | `git diff --name-only` / exit 0 / scope includes only canonical spec metadata plus change-local evidence/tasks before checkpoint. | Only canonical `## Purpose` prose changed. |
| 2. `x11-integration-contract` Purpose is non-placeholder | `python3 /tmp/check_purpose.py openspec/specs/x11-integration-contract/spec.md; echo status=$?` / status 1 / `placeholder purpose remains: TBD - created by archiving change bootstrap-codex-computer-use-x11...` | `python3 /tmp/check_purpose.py openspec/specs/x11-integration-contract/spec.md` / exit 0 / `purpose is non-placeholder: This specification defines the X11/EWMH integration contract...` | `git diff --name-only` / exit 0 / scope includes only canonical spec metadata plus change-local evidence/tasks before checkpoint. | Only canonical `## Purpose` prose changed. |
| 3. Repository/OpenSpec validation remains clean | N/A — validation is a post-edit guard for this docs-only slice. | `openspec validate cleanup-spec-purpose-metadata --type change --json` / exit 0 / change valid; `openspec validate --all --strict` / exit 0 / 4 items passed. | `git diff --name-only` / exit 0 / `openspec/specs/doctor-cli/spec.md`, `openspec/specs/x11-integration-contract/spec.md`; no Rust files. | Rust checks not applicable because no Rust files changed. |
| 4. Archive preserves canonical Purpose text | `openspec archive cleanup-spec-purpose-metadata -y` / warning before archive: `15/16 tasks`; post-archive checks required because this slice is archive-dependent. | `python3 /tmp/check_purpose.py openspec/specs/doctor-cli/spec.md` and `python3 /tmp/check_purpose.py openspec/specs/x11-integration-contract/spec.md` / exit 0 / both canonical Purpose sections remained non-placeholder after archive/spec sync. | `openspec validate --all --strict` / exit 0 / 3 canonical specs passed after archive. | Archive sync added the metadata regression requirements and preserved direct Purpose prose edits. |

## TDD Exceptions

None. Documentation-only TDD is feasible with text checks and OpenSpec validation.
