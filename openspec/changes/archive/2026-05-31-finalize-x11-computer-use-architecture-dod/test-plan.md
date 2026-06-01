## TDD Strategy

Use the project-local `tdd` discipline with small vertical RED -> GREEN -> REFACTOR slices through public interfaces:

- public script interface: `scripts/validate-final-dod.py`;
- public docs interface: `docs/final-architecture-dod.md`, `docs/release-checklist.md`, `README.md`, `ARCHITECTURE.md`, and `adr/README.md`;
- public project verification commands: `cargo test`, e2e fake scripts, and OpenSpec validation.

Tests will exercise behavior through script invocation and documentation assertions, not private parser functions. The validator may parse Markdown internals, but tests call it as a process against real and fixture documents.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1. Missing DoD rows fail | `scripts/validate-final-dod.py --document <fixture>` rejects incomplete matrix data | Add `tests/final_dod.rs` fixture test and run `cargo test --test final_dod final_dod_validator_rejects_missing_capability_rows`; expected failure because script/docs do not exist or validator does not reject row | Implement minimal validator missing-row checks and incomplete fixture; command passes by observing non-zero validator exit and named missing row | Parser stays deterministic, no GUI/sudo/secrets, failure messages name row/field |
| 2. Missing decisions/evidence fail | Validator rejects missing decision topics, empty evidence, and degraded rows without reason | Run `cargo test --test final_dod final_dod_validator_rejects_missing_decision_and_evidence`; expected failure before checks exist | Extend validator field checks and fixture coverage; command passes | Keep schema simple and readable; do not overfit to one fixture path |
| 3. Complete final DoD passes | `scripts/validate-final-dod.py` validates tracked `docs/final-architecture-dod.md` | Run `cargo test --test final_dod final_dod_validator_accepts_tracked_final_report`; expected failure until report/matrix complete | Add full final report with decision JSON and capability matrix; command passes | Human table and JSON blocks agree enough for maintainers; no generated local logs required |
| 4. Release/readme docs expose gate | Docs tests require final DoD report and validator in README/release checklist | Run `cargo test --test packaging_docs`; expected failure until docs mention final DoD validator | Update README and release checklist; command passes | Existing packaging docs assertions remain intact; no stale change name in release commands |
| 5. ADR/architecture snapshot record final baseline | Tracked ADR/architecture references show ADR 0009 and final scope answer | Run `cargo test --test final_dod final_dod_docs_record_adr_and_architecture_snapshot`; expected failure until assertions/docs align | Complete ADR 0009, `ARCHITECTURE.md`, and `adr/README.md`; command passes | ADR 0008 remains in force and is not superseded |
| 6. Existing e2e evidence remains valid | Fake e2e smoke and matrix validation still pass after DoD additions | Run fake e2e plugin/source-overlay smoke and `validate-matrix`; expected failure only if final changes regress existing harness | Fix regressions only; final validator does not replace e2e validation | Generated evidence remains under `target/e2e-logs` and is not committed |

## Mocking / Boundary Policy

- Use fixture Markdown documents in tests to exercise validator failure modes.
- Do not mock internal Rust code; call the validator as a subprocess.
- Do not use live GUI, real Codex Desktop input, sudo, or `.secrets.local.env` in tests.
- E2E fake mode may use the existing fake command fixtures in `scripts/e2e/codex-x11-e2e.py`.

## Required Checks

Before apply completion:

```bash
cargo test --test final_dod
cargo test --test packaging_docs
scripts/validate-final-dod.py
make fmt
make check
make test
scripts/e2e/codex-plugin-smoke.sh --fake --log-dir target/e2e-logs/final-dod-plugin-fake
scripts/e2e/codex-source-overlay-smoke.sh --fake --log-dir target/e2e-logs/final-dod-source-overlay-fake
scripts/e2e/codex-x11-e2e.py validate-matrix --evidence target/e2e-logs/final-dod-plugin-fake/<run>/evidence.json
scripts/e2e/codex-x11-e2e.py validate-matrix --evidence target/e2e-logs/final-dod-source-overlay-fake/<run>/evidence.json
openspec validate finalize-x11-computer-use-architecture-dod --type change --strict
openspec validate --all --strict
git status --short
```

Before archive: rerun or confirm fresh `openspec validate finalize-x11-computer-use-architecture-dod --type change --strict`, `openspec validate --all --strict`, and clean git status after implementation commits.

## Evidence Log

- RED slice 1: `cargo test --test final_dod final_dod_validator_rejects_missing_capability_rows` failed because `scripts/validate-final-dod.py` did not exist.
- GREEN slices 1-2: `cargo test --test final_dod final_dod_validator_rejects_missing` passed after adding the validator and failure fixtures.
- RED slice 3: `cargo test --test final_dod final_dod_validator_accepts_tracked_final_report` failed because `docs/final-architecture-dod.md` did not exist, then failed once for a missing required readiness phrase.
- GREEN slice 3: `cargo test --test final_dod final_dod_validator_accepts_tracked_final_report` passed after adding the final report and exact readiness wording.
- RED/GREEN slice 4: `cargo test --test packaging_docs readme_v1_quick_start_links_required_docs` failed after adding final DoD doc expectations, then `cargo test --test packaging_docs` passed after updating README and release checklist.
- RED/GREEN slice 5: `cargo test --test final_dod final_dod_docs_record_adr_and_architecture_snapshot` failed until `ARCHITECTURE.md` included the ADR 0009 filename; it passed after updating the architecture snapshot.
- GREEN targeted suite: `cargo test --test final_dod` passed (4 tests).
- GREEN validator: `scripts/validate-final-dod.py` passed.
- GREEN project checks: first `make fmt` failed on Rust formatting in `tests/final_dod.rs`; after `cargo fmt`, `make fmt`, `make check`, and `make test` passed.
- GREEN e2e: fake plugin smoke and fake source-overlay smoke passed; `scripts/e2e/codex-x11-e2e.py validate-matrix` passed for `target/e2e-logs/final-dod-plugin-fake/.../evidence.json` and `target/e2e-logs/final-dod-source-overlay-fake/.../evidence.json`.
- GREEN OpenSpec: `openspec validate finalize-x11-computer-use-architecture-dod --type change --strict` passed; `openspec validate --all --strict` passed with 16 items.

- GREEN git hygiene: `git status --short` was clean before marking task 3.5 complete; generated e2e logs stayed under ignored `target/` and were not staged.

## TDD Exceptions

None.
