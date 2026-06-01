## Why

The full documentation/code review found that the final v1 handoff is mostly green, but several tracked artifacts and runtime reports drifted from the implemented state: architecture docs reference missing durable ADR files, `doctor --json` still reports bootstrap-era focus/capability facts, the release checklist contains an archived-change validation command, README/source-overlay wording is stale, and strict linting exposes avoidable code-quality warnings.

## What Changes

- Restore durable ADR traceability so every in-force ADR referenced by `ARCHITECTURE.md` and `adr/README.md` exists as a tracked top-level ADR file, or reconcile the snapshot if restoration is impossible.
- Update `doctor --json` readiness/capability facts to reflect the finalized v1 X11/EWMH baseline while preserving additive/bootstrap-compatible field shapes.
- Clarify ydotool socket diagnostic privacy policy so env-derived local paths are either redacted or explicitly classified in docs/tests as allowed diagnostics.
- Refresh README and release checklist language to match the current standalone plugin plus reversible source-overlay handoff and only include validation commands that work after archive.
- Remove current strict clippy warnings without changing the required `make fmt`, `make check`, and `make test` verification surface.
- Convert illustrative skill-template links that confuse local Markdown link checks into non-link examples.

## Capabilities

- Modify `doctor-cli` to align finalized readiness and capability reporting with implemented v1 behavior and safe diagnostic privacy.
- Modify `x11-computer-use-architecture-dod` to require tracked architecture/ADR references and final DoD consistency after review-drift fixes.
- Modify `x11-packaging-docs-upstreaming` to keep README, release checklist, and documentation examples synchronized with executable project state.
- Modify `project-bootstrap` to keep the repeatable project verification surface clean under existing Rust quality tooling.

## Impact

- Affected code: `src/doctor.rs`, clippy-warning sites in `src/accessibility.rs`, `src/coordinates.rs`, `src/input.rs`, `src/list_windows.rs`, `src/pointer.rs`, and `src/target_window.rs` as needed.
- Affected tests: `tests/doctor_cli.rs`, `tests/final_dod.rs`, `tests/packaging_docs.rs`, and focused unit tests for doctor privacy/capability behavior.
- Affected docs: `README.md`, `docs/release-checklist.md`, `ARCHITECTURE.md`, `adr/README.md`, top-level `adr/*.md`, and `.codex/skills/grill-with-docs/CONTEXT-FORMAT.md` examples.
- Verification follows `CONSTITUTION.md`: Rust 2021/Cargo, root `Makefile` checks, OpenSpec validation, no secret files read or printed, and no external-system access. Optional strict clippy is used as a review remediation check, not as a new required constitution gate.
- Architecture impact: durable ADR history/architecture snapshot consistency is restored; no target checkout writes or source-overlay live mutation are required.
