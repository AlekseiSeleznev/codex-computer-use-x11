## Context Read

- `openspec/changes/finalize-x11-computer-use-architecture-dod/proposal.md`
- `openspec/changes/finalize-x11-computer-use-architecture-dod/specs/x11-computer-use-architecture-dod/spec.md`
- `CONSTITUTION.md`
- `CONTEXT.md`
- `ARCHITECTURE.md`
- `adr/README.md`
- `adr/0008-adopt-x11-root-coordinate-model.md`
- `backlog/00-research-reuse-map.md`
- `backlog/13-final-architecture-dod.md`
- `README.md`
- `docs/integration-contract.md`
- `docs/e2e-harness.md`
- `docs/release-checklist.md`
- `docs/upstreaming.md`
- `docs/license-attribution.md`
- `scripts/e2e/codex-x11-e2e.py`
- `tests/e2e_harness_scripts.rs`
- `tests/packaging_docs.rs`
- Current target checkout: `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` branch/status and `computer-use-linux/src/{windowing,server.rs,diagnostics.rs,atspi_tree.rs,screenshot.rs,remote_desktop.rs}`

## Plan Summary

- The proposal adds a final architecture/DoD capability instead of another partial runtime feature.
- The spec requires a tracked architecture decision ledger, a fine-grained final capability matrix, and a machine-checkable validator.
- The checker must complement the existing coarse e2e matrix: e2e proves delivery-path smoke; final DoD proves v1 architecture and row-level evidence completeness.
- v1 claims remain scoped to the Cinnamon/X11 baseline; Cinnamon Wayland, unstable Cinnamon extension work, and unverified targeted input stay out of scope or degraded.
- No secrets or live GUI are required for the validator; live evidence remains optional and explicitly degraded when unavailable.

## Question Loop

### Question 1: Should the final gate require every optional/`should` row to pass live evidence before v1 is claimed?

Recommended answer: No. Keep optional or environment-dependent rows valid when they are either `pass` or explicitly `degraded` with evidence and a reason.

Rationale: The backlog and current docs define v1 as a safe Cinnamon/X11 baseline with degraded diagnostics when layers such as AT-SPI, screenshot, terminal context, or live source-overlay evidence are unavailable. Requiring unconditional live pass for optional rows would contradict the existing degraded-layer model and would make v1 dependent on local desktop conditions rather than tracked safety/evidence.

Resolution: Repository context resolves this without a user question. The spec requires pass/degraded status plus concrete degraded behavior. Design must make the validator reject missing/empty evidence while accepting documented degraded rows.

### Question 2: Should the final DoD create a new durable ADR or only a change-local ADR review?

Recommended answer: Create one durable ADR for the final v1 architecture/DoD baseline, because it consolidates hard-to-reverse safety, backend identity, and upstream strategy decisions that future maintainers will need after the OpenSpec change is archived.

Rationale: Earlier stages already made individual decisions, including ADR 0008 for coordinates, but the final DoD bundles the in-force answer to “is this a full Computer Use backend for Cinnamon/X11 v1?” That is a durable architecture/status decision with real trade-offs and future upstream consequences.

Resolution: No user question needed. Design and ADR artifacts must add a new append-only durable ADR under `adr/` and update `ARCHITECTURE.md` as the current snapshot.

### Question 3: Should the final checker directly run live Codex Desktop stock tools?

Recommended answer: No. The final checker should be deterministic and no-GUI; live Codex Desktop stock evidence remains in e2e/live/manual evidence and release checklist steps.

Rationale: Existing e2e harness fake mode is the deterministic CI boundary. Direct live tool invocation would depend on a running desktop, safe target windows, and local Codex Desktop state; the final gate should validate tracked evidence and docs without making unsafe input attempts.

Resolution: No user question needed. The checker will validate tracked matrix/docs and can be paired with fake e2e validation; optional live evidence is recorded as pass/degraded outside the checker.

## Resolved Terms

- `Final DoD` — added to `CONTEXT.md` as the final Definition-of-Done evidence gate for the documented Cinnamon/X11 v1 baseline.
- `Architecture decision ledger` — added to `CONTEXT.md` as the tracked decision-topic list that prevents final architecture choices from living only in chat/backlog text.

## Document Updates Applied

- Added `Final DoD` and `Architecture decision ledger` glossary entries to `CONTEXT.md`.
- No proposal/spec changes were required by the grill; the current spec already accepts degraded evidence and requires a deterministic validator.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- Durable ADR candidate: accept the final Cinnamon/X11 v1 architecture/DoD baseline and explicitly record the scoped readiness answer, including `x11-ewmh`, verified-input safety, source-overlay/upstream boundaries, shell-out/native thresholds, and degraded-layer policy.

## Open Questions

None.
