## Context Read

- `openspec/changes/finalize-x11-computer-use-architecture-dod/proposal.md`
- `openspec/changes/finalize-x11-computer-use-architecture-dod/specs/x11-computer-use-architecture-dod/spec.md`
- `openspec/changes/finalize-x11-computer-use-architecture-dod/grill.md`
- `openspec/changes/finalize-x11-computer-use-architecture-dod/design.md`
- `CONSTITUTION.md`
- `CONTEXT.md`
- `ARCHITECTURE.md`
- `adr/README.md`
- `adr/0008-adopt-x11-root-coordinate-model.md`
- `README.md`
- `docs/e2e-harness.md`
- `docs/release-checklist.md`
- `docs/upstreaming.md`
- `docs/license-attribution.md`
- `scripts/e2e/codex-x11-e2e.py`
- `tests/e2e_harness_scripts.rs`
- `tests/packaging_docs.rs`
- Current target checkout `computer-use-linux/src/server.rs`, `diagnostics.rs`, `screenshot.rs`, `remote_desktop.rs`, `atspi_tree.rs`, and `windowing/`

## Design Summary

- The design creates `docs/final-architecture-dod.md` as the human report plus labeled machine-readable JSON blocks.
- `scripts/validate-final-dod.py` validates tracked decision and matrix completeness without GUI, sudo, target mutation, or secret reads.
- The final matrix is deliberately finer-grained than the existing e2e delivery-path groups and cites concrete project evidence for each row.
- ADR 0009 records the final Cinnamon/X11 v1 baseline; `ARCHITECTURE.md` and `adr/README.md` will point to it.
- The release checklist gains the validator while retaining existing project checks, fake e2e, OpenSpec validation, license, secret, archive, and push gates.

## Question Loop

### Question 1: Does embedding JSON in Markdown create too much validation fragility?

Recommended answer: Use embedded labeled JSON for this stage, but make the parser strict and tests cover incomplete fixtures.

Rationale: A separate JSON file would be easier for tooling but risks drift from the human final report. The final DoD is primarily release documentation, and deterministic parser tests can cover the machine-readable boundary.

Resolution: Repository context resolves this. Proceed with embedded labeled fenced JSON plus tests for missing rows/decisions.

### Question 2: Should the validator inspect `target/e2e-logs` from the latest run?

Recommended answer: No. It should validate tracked final evidence references and release documentation; actual e2e artifacts are produced by explicit smoke commands.

Rationale: `target/e2e-logs` is local generated evidence and should not be required or committed. The validator should not accidentally pass/fail based on stale local run directories.

Resolution: No user question needed. The validator will check tracked references; verify will separately run fake smoke and e2e matrix validation.

### Question 3: Should ADR 0009 supersede ADR 0008?

Recommended answer: No. ADR 0009 should cite ADR 0008 and incorporate it into the final baseline; ADR 0008 remains the detailed root-coordinate decision.

Rationale: ADR 0008 is still valid and more specific. Superseding it would obscure the precise coordinate rationale.

Resolution: No user question needed. ADR 0009 will not supersede ADR 0008.

## Design Findings

- **No conflict with constitution:** The implementation remains Rust/project-script based, uses tracked docs/tests, avoids secrets, and leaves external systems/target checkout untouched except existing smoke commands.
- **No glossary conflict:** `Final DoD` and `Architecture decision ledger` were added to `CONTEXT.md` during grill; design uses those terms consistently.
- **Architecture alignment:** The design updates ADR/architecture snapshot rather than relying only on an OpenSpec artifact that will be archived.
- **Verification feasibility:** The validator can run no-GUI; fake e2e remains available; live evidence is optional/degraded and not a hard deterministic test input.
- **Risk to watch:** The final matrix must include every backlog row, including terminal context selectors and AT-SPI action/value set, even if those are `should`/degraded rather than pass.

## Document Updates Applied

None. The design already states the validator must reject missing rows/evidence, avoid live local logs, and create ADR 0009 without superseding ADR 0008.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- Durable ADR 0009: adopt the final Cinnamon/X11 v1 architecture and DoD baseline. It is hard to reverse, surprising without context, and records real trade-offs around backend identity, global injector safety, shell-out/native thresholds, degraded evidence, and upstream/source-overlay boundaries.

## Open Questions

None.
