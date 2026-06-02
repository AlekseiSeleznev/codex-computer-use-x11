## Context Read

- OpenSpec artifacts: `proposal.md`, `specs/x11-packaging-docs-upstreaming/spec.md`, `grill.md`, `design.md`.
- Project context: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, `adr/0008-adopt-x11-root-coordinate-model.md`.
- Existing docs: `README.md`, `docs/integration-contract.md`, `docs/e2e-harness.md`, `docs/intent-driven-lifecycle.md`, `docs/intent-driven-update-safety.md`.
- Public scripts and checks: plugin installer/uninstaller, source-overlay status/install/uninstall wrappers, e2e smoke wrappers, `Makefile`, `Cargo.toml`, existing integration tests under `tests/`.
- Target context from fresh research: clean `codex-desktop-linux-full` main checkout, target README/CHANGELOG/AGENTS, `linux-features/README.md`, and `computer-use-linux/src/` backend/windowing/diagnostics/server files.

## Design Summary

- The design adds docs and docs-check tests only; runtime CLI/MCP/source-overlay behavior remains unchanged.
- README becomes the entry point; deeper docs live in focused files under `docs/`.
- `tests/packaging_docs.rs` checks stable public contracts: headings, paths, script help/dry-run commands, license classifications, upstream matrix labels, rollback snippets, and release commands.
- Upstreaming docs keep backend/windowing ownership separate from Codex Desktop wrapper/packaging integration.
- License docs preserve a reference-first, no-copy posture for unclear/copyleft sources and distinguish runtime command invocation from source vendoring.

## Question Loop

### Q1 — Could docs checks accidentally execute live desktop or target mutations?

- **Recommended answer:** No. Automated docs tests should use `--help`, isolated `CODEX_HOME` dry-runs, and text assertions only; live e2e commands belong in release checklist/manual verification, not docs-check tests.
- **Rationale:** The constitution requires safe verification and no unintended external/system mutation. The source-overlay scripts can mutate real target checkouts, so docs tests must not run install/uninstall against the live target.
- **Resolution:** Answered from design and script behavior. Test-plan must explicitly keep live commands out of docs-check automation.

### Q2 — Should documentation introduce a new NOTICE file?

- **Recommended answer:** No, not for this change. Document attribution/NOTICE rules, but create a NOTICE only if code or bundled assets with NOTICE obligations are actually copied or vendored.
- **Rationale:** This change copies no external source code. Creating an empty or speculative NOTICE would imply reuse that did not occur.
- **Resolution:** Answered from license/reuse policy and scope. No ADR or design update required.

### Q3 — How should the docs handle `gh api` license refresh evidence?

- **Recommended answer:** Record observed SPDX statuses and the date, but state that maintainers must re-check before copying code or making upstream release claims.
- **Rationale:** GitHub license metadata can change or be incomplete. The license docs should be useful without becoming a stale legal guarantee.
- **Resolution:** Answered from proposal research refresh. Release checklist should require re-checking before copy/upstream claims.

### Q4 — Does the design need an architecture snapshot update?

- **Recommended answer:** No. The current architecture already captures the standalone/source-overlay split, Claude/session controls, and ADR 0008 coordinate model. This change applies those decisions to docs/tests.
- **Rationale:** Architecture changes require durable ADR/snapshot updates; docs-check tests and handoff docs do not alter runtime boundaries.
- **Resolution:** Answered from `ARCHITECTURE.md` and ADR review criteria. Per-change ADR should record no durable ADR needed unless implementation scope expands.

## Design Findings

- **Safety finding:** Docs tests must not run live source-overlay install/uninstall against the real target; use text/help checks and keep live smoke in the release checklist.
- **License finding:** License docs should avoid presenting GitHub SPDX data as legal advice or permanent truth; use observed-date language and require fresh review before copying.
- **Architecture finding:** The upstream matrix should reinforce, not change, the existing architecture boundary between backend lineage and wrapper integration.
- **Verification finding:** Public-interface tests can satisfy TDD for docs work if they verify concrete commands/headings/tables and avoid brittle prose snapshots.

## Document Updates Applied

None. The current design already includes the safety boundaries and no-copy policy found during review.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

None. The design review found no hard-to-reverse, surprising architecture decision requiring a durable ADR. The change documents existing boundaries and adds tests.

## Open Questions

None.
