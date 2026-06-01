## 1. Validator TDD

- [x] 1.1 Add RED integration tests for `scripts/validate-final-dod.py` that reject missing capability rows, missing decision topics, empty evidence, and degraded rows without reasons.
- [x] 1.2 Implement the minimal final DoD validator script with labeled Markdown JSON parsing, required decision-topic checks, required capability-row checks, and deterministic no-GUI/no-secret behavior.
- [x] 1.3 Add the complete tracked `docs/final-architecture-dod.md` report with research refresh, decision ledger, final capability matrix, final readiness answer, validation commands, and license/upstream summary.

## 2. Documentation and ADR Integration

- [x] 2.1 Update README and release documentation to expose the final DoD report and require `scripts/validate-final-dod.py` in v1 handoff checks.
- [x] 2.2 Complete durable architecture integration for ADR 0009 by ensuring `ARCHITECTURE.md` and `adr/README.md` list the final Cinnamon/X11 v1 baseline and do not supersede ADR 0008.
- [x] 2.3 Add/adjust docs tests so final DoD validator, final report, ADR 0009, and architecture snapshot are covered through public files.

## 3. Evidence and Verification

- [x] 3.1 Run the final DoD validator and targeted TDD/doc tests; record RED/GREEN evidence in `test-plan.md`.
- [x] 3.2 Run project checks: `make fmt`, `make check`, and `make test`.
- [x] 3.3 Run fake standalone plugin and fake source-overlay e2e smoke plus `validate-matrix` on their generated evidence files.
- [x] 3.4 Run OpenSpec validation for this change and all specs in strict mode.
- [x] 3.5 Confirm project git status is clean after checkpointing and no secret/local evidence files are staged.
