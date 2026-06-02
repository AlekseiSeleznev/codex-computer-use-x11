## ADR Review

## Existing In-Force ADRs

- `adr/0008-adopt-x11-root-coordinate-model.md` — Accepted and remains in force for X11 root/global coordinate semantics, bounds provenance, pointer points, screenshot crops, and app-state composition.
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — Created by this change; Accepted final Cinnamon/X11 v1 DoD baseline.

Note: `ARCHITECTURE.md` and `adr/README.md` reference earlier ADRs from the broader intent-driven overlay history. In this repository checkout, the currently present top-level durable ADR files are ADR 0008 and ADR 0009; this change does not rewrite or repair historical ADR inventory.

## Constitution / Architecture Rules Considered

- `CONSTITUTION.md` requires Rust 2021/Cargo, root Makefile verification, OpenSpec validation, automatic safe checkpoint discipline, no secret values in tracked files, and no target checkout mutation unless an OpenSpec task explicitly owns it.
- `ARCHITECTURE.md` requires OpenSpec lifecycle artifacts, Matt grill gates, TDD, ADR append-only history, and architecture snapshot updates when durable decisions change the current snapshot.
- `CONTEXT.md` terms considered: `x11-ewmh`, `App state`, `Capability matrix evidence`, `Upstream target matrix`, `Runtime command dependency`, `Release checklist`, `Final DoD`, and `Architecture decision ledger`.
- ADR 0008 requires X11 root/global coordinates and bounds provenance for bounds, pointer, screenshot crop, and app-state composition.

## Decisions Evaluated

- Whether final v1 readiness should be captured only in OpenSpec artifacts or in a durable ADR as well: durable ADR chosen because future maintainers need this baseline after archive.
- Whether ADR 0009 should supersede ADR 0008: rejected; ADR 0008 remains the detailed coordinate-model decision and ADR 0009 cites it.
- Whether final v1 should require every live capability to pass unconditionally: rejected; environment-dependent layers may be degraded only with evidence and reasons.
- Whether the final checker should run live desktop/Codex tool calls: rejected; live checks remain explicit e2e/release steps, while final DoD validation is deterministic and no-GUI.

## New Durable ADRs Created

- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — Accepted; captures the final Cinnamon/X11 v1 Computer Use DoD baseline, scoped readiness answer, safety invariants, source-overlay/upstream boundaries, license posture, and degraded-evidence policy.

## Superseded ADRs

- None. ADR 0008 remains in force and is not superseded.

## Architecture Snapshot Updates

- `ARCHITECTURE.md` must be updated to list ADR 0009 as in force and summarize the final Cinnamon/X11 v1 DoD baseline.
- `adr/README.md` must be updated to list ADR 0009 in the current state.

## No ADR Needed

- N/A.
