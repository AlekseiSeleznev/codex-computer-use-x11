## ADR Review

## Existing In-Force ADRs

- `adr/0008-adopt-x11-root-coordinate-model.md` — Accepted and in force. The e2e harness must keep pointer/bounds evidence in X11 root/global pixel coordinates and must not make the X11 backend own target screenshot capture.
- `ARCHITECTURE.md` snapshot references ADRs 0001, 0003, 0005, 0006, and 0007 as in force for OpenSpec overlay lifecycle, project context, Matt grill/TDD gates, optional Claude review, and automatic checkpoint/session controls. Their durable files are not present in the top-level `adr/` directory in this checkout, so this review applies their current snapshot constraints rather than rewriting history.

## Constitution / Architecture Rules Considered

- Required technologies: Rust 2021/Cargo for project implementation, Bash helper scripts under `scripts/`, Markdown OpenSpec artifacts, and root `Makefile` verification via `make fmt`, `make check`, and `make test`.
- Source-overlay target: use `CODEX_DESKTOP_LINUX_FULL_PATH` or the documented local default; any live source-overlay smoke must be reversible and return the target checkout to clean state.
- Secret handling: no real credentials or private values in tracked files, logs, artifacts, or chat; `.secrets.local.env` is not needed for this change and must not be read.
- Architecture boundaries: OpenSpec artifacts are source of truth; the harness is project-owned verification tooling, not an OpenSpec CLI fork and not installed target app mutation.
- Claude review: disabled for this session by explicit user request (`claude off`); no Claude artifact reports are required.
- Git/checkpoint discipline: safe lifecycle checkpoints may be automatic; archive and push are explicitly requested by the user for this run.

## Decisions Evaluated

- **Automated boundary for plugin smoke**: choose installed plugin metadata plus MCP stdio runner over Desktop UI automation. Rationale: stable, no-GUI compatible, and aligned with existing tests; Desktop UI remains manual/degraded when no stable runner exists.
- **Runner implementation language**: choose one stdlib Python runner behind thin Bash wrappers over duplicate Bash logic. Rationale: JSON evidence, JSON-RPC stdio, failure-safe logs, and matrix validation are safer in Python without adding dependencies.
- **Fake vs live mode split**: choose fake mode as deterministic archive-gate evidence and live mode as additional current-machine evidence. Rationale: desktop state and accessibility/screenshot/input availability are environmental.
- **Capability matrix semantics**: choose `pass` or explicit `degraded` per group/path and fail on missing evidence. Rationale: matches layer-degraded app-state language while preventing silent coverage holes.
- **Source-overlay live mutation boundary**: preserve existing reversible status/install/check/uninstall/final-clean boundary. Rationale: already established by source-overlay architecture and constitution/update-safety constraints.
- **Durable ADR need**: rejected. These decisions are local verification mechanics around existing architecture and do not alter backend identity, stock target tool surfaces, coordinate model, source-overlay ownership, or project lifecycle architecture.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None. The current architecture snapshot remains valid: this change adds verification harness scripts/docs/tests and does not change current architecture boundaries.

## No ADR Needed

- No durable ADR is needed because the selected harness boundary is reversible, unsurprising given existing MCP/server tests and source-overlay smoke precedent, and does not change a hard-to-reverse architectural decision. The rationale is recorded in this per-change ADR review, design, and design-review artifacts.
