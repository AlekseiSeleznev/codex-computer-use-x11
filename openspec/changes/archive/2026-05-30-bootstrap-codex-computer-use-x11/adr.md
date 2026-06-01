## ADR Review

This ADR gate reviewed whether `bootstrap-codex-computer-use-x11` needs a durable top-level ADR in addition to the per-change OpenSpec artifacts.

## Existing In-Force ADRs

- No top-level durable ADR body files (`adr/NNNN-*.md`) are present in this checkout beyond `adr/README.md`, so no individual ADR body was available to evaluate.
- This is a repository source-of-truth gap outside the scope of `bootstrap-codex-computer-use-x11`: `ARCHITECTURE.md` and `adr/README.md` reference ADR 0001/0003/0005/0006/0007 as in-force, but the corresponding body files are absent. This ADR gate therefore treats the architecture snapshot and `adr/README.md` summaries as the available in-force constraints for this change, and does not attempt to reconstruct or rewrite missing accepted ADR history.
- `ARCHITECTURE.md` and `adr/README.md` list inherited/current in-force ADR decisions as project context:
  - ADR 0001 — Codex-native intent-driven OpenSpec overlay remains in force.
  - ADR 0003 — root `CONSTITUTION.md`, root `ARCHITECTURE.md`, OpenSpec bridge, ADR-derived architecture snapshots, and local-secret boundaries remain in force.
  - ADR 0005 — mandatory Matt grill/design-review gates and strict TDD discipline remain in force.
  - ADR 0006 — optional Claude artifact review as auxiliary reviewer evidence remains in force.
  - ADR 0007 — scoped automatic lifecycle checkpoints and session-scoped Claude review controls remain in force.
- Superseded context from `adr/README.md` was considered: ADR 0002 is superseded by ADR 0003; ADR 0004 is superseded by ADR 0005.

## Constitution / Architecture Rules Considered

- `CONSTITUTION.md` requires Rust 2021/Cargo as the default implementation stack for the standalone crate.
- `CONSTITUTION.md` requires the initial standalone Rust package/workspace at the repository root unless an accepted design/ADR introduces subcrates.
- `CONSTITUTION.md` requires root `Makefile` verification commands: `make fmt`, `make check`, and `make test` as thin Cargo wrappers.
- `CONSTITUTION.md` requires `CODEX_DESKTOP_LINUX_FULL_PATH` as the durable local integration-target variable; concrete local paths are machine defaults only.
- `CONSTITUTION.md` and `ARCHITECTURE.md` require secrets to remain outside Git-tracked artifacts and `.secrets.local.env` to be unread unless external access is needed. This change does not need external secret access.
- `ARCHITECTURE.md` requires OpenSpec artifact order, project context preflight, grill/design-review gates, TDD slices, Claude review as auxiliary evidence, and scoped Git checkpoints.
- `ARCHITECTURE.md` requires durable ADR history to be append-only and `ARCHITECTURE.md` updates only when a durable ADR changes current architecture.
- `adr/README.md` says each intent-driven change must record existing ADRs considered, grill/design-review findings considered, decisions evaluated, durable ADRs created, superseded ADRs, and rationale when no durable ADR is needed.
- Because the referenced durable ADR body files are missing, this review records that source-of-truth gap but does not block the bootstrap change: all relevant architecture constraints needed for stage 01 are available from `CONSTITUTION.md`, `ARCHITECTURE.md`, and `adr/README.md`. A separate maintenance change should restore or reconcile the missing durable ADR bodies if this repository is intended to carry them.

## Grill / Design-Review Findings Considered

- `grill.md` resolved the bootstrap doctor as a standalone smoke-test surface, not a strict subset of upstream target `doctor_report()`.
- `grill.md` resolved root package layout and `CODEX_DESKTOP_LINUX_FULL_PATH` portability.
- `grill.md` resolved `x11-ewmh`, Standalone plugin, and Source overlay terminology and updated `CONTEXT.md`.
- `grill.md` resolved standalone command-test seams as a design-owned property and source-overlay command style as target-style thin command wrappers unless a future design/ADR accepts a dependency-injection runner exception.
- `design-review.md` found no architecture/spec/glossary/external-system conflicts and no user-facing open questions.
- `design-review.md` resolved Claude review clarifications into `design.md`: stable stage-01 bootstrap check names, inline `x11_id` unit tests, compact JSON newline semantics, bootstrap readiness semantics, and manual README/integration-contract verification carry-forward.
- `design-review.md` explicitly concluded that no durable ADR candidate is needed for stage 01.

## Decisions Evaluated

- **Root Rust package and Makefile wrappers** — accepted for this change, but no durable ADR needed. This follows `CONSTITUTION.md` defaults and remains reversible if a later design/ADR introduces subcrates or workspace structure.
- **Standalone `doctor --json` smoke-test surface** — accepted for this change, but no durable ADR needed. It is scoped to bootstrap validation and does not couple future source-overlay architecture to upstream `doctor_report()`.
- **`x11-ewmh` backend identity and upstream `WindowInfo` primary model** — accepted and captured in specs/design/docs. No durable ADR needed now because no source-overlay code is added; a later change that modifies upstream `WindowInfo` or registry strategy may need a durable ADR.
- **Sidecar/report default for X11-only diagnostics and `WindowObservationMeta` sketch** — accepted as documentation and future-design boundary, but no durable ADR needed until real backend/source-overlay code depends on the shape or expands upstream data models.
- **Source-overlay command style default** — accepted as future guidance only. No durable ADR needed because this change does not patch `${CODEX_DESKTOP_LINUX_FULL_PATH}` or add a command runner to the target repo.
- **No MSRV pin, compact JSON output, stable bootstrap check names, and bootstrap readiness semantics** — accepted as per-change implementation/test-plan details rather than durable architecture decisions.
- **No code copy from external projects** — accepted as current license posture and already documented in proposal/design; no durable ADR needed because stage 01 does not vendor or copy external code.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None.

`ARCHITECTURE.md` does not need an update from this change because stage 01 does not change the current Codex/OpenSpec overlay architecture or introduce a durable project-wide architecture decision. Future backend/source-overlay changes may need to update `ARCHITECTURE.md` if they create or supersede durable ADRs.

## Claude ADR Review Disposition

The rerun of Claude review for this `adr` stage returned `warn`, no `mustFix`, and findings only about the already-acknowledged missing durable ADR body files. No user participation is required for the current bootstrap ADR gate.

- The durable ADR body files are genuinely absent from this checkout: `find adr -maxdepth 1 -type f -name "*.md" ! -name README.md` returns no files. The review bundle did not omit them.
- No follow-up maintenance change has been opened in this run because `/opsx:continue` creates exactly one lifecycle artifact and the missing ADR bodies are outside `bootstrap-codex-computer-use-x11` scope.
- Concrete follow-up to open separately: OpenSpec change `restore-durable-adr-bodies` should restore or reconcile the referenced ADR 0001/0003/0005/0006/0007 body files, or update `ARCHITECTURE.md` / `adr/README.md` if this repository intentionally keeps only summaries.
- Verify follow-up disposition: this current OpenSpec artifact is the tracked follow-up record for the bootstrap verify gate. A separate `restore-durable-adr-bodies` change is intentionally not opened during verify because that would be an out-of-scope lifecycle state change; open it explicitly after this change is verified/archived if the repository should carry durable ADR body files.
- Until that maintenance change is done, future ADR gates should treat `CONSTITUTION.md`, `ARCHITECTURE.md`, and `adr/README.md` as the available in-force architecture constraints and explicitly note the missing ADR body limitation.

## No ADR Needed

- No durable ADR is needed for stage 01 because the accepted decisions are either:
  - already required by `CONSTITUTION.md`, `ARCHITECTURE.md`, and the intent-driven OpenSpec workflow;
  - scoped to this bootstrap change and recorded in proposal/specs/grill/design/design-review;
  - reversible local implementation/documentation choices; or
  - future source-overlay constraints that do not yet modify the target repo or upstream data model.
- The change deliberately avoids hard-to-reverse architecture moves: no target checkout patch, no upstream `WindowInfo` expansion, no new target command-runner abstraction, no live X11 backend, no copied external code, and no durable registry reordering.
- ADR review should be revisited in a later change if source-overlay code is introduced, upstream `WindowInfo` changes, the target registry order changes, a dependency-injection runner is proposed for the target repo, or external code is copied/vendored.
