# Grill — fix-review-drift-and-doctor-readiness

## Context Read

- `CONSTITUTION.md` — Rust/Cargo root package, `make fmt`/`make check`/`make test`, no secrets, OpenSpec as source of truth, automatic safe checkpoints.
- `CONTEXT.md` — project glossary for `x11-ewmh`, source overlay, focus verification, runtime command dependency, release checklist, final DoD, and architecture decision ledger.
- `ARCHITECTURE.md` — current architecture snapshot, including in-force ADR references and final Cinnamon/X11 v1 baseline.
- `adr/README.md` — ADR index and append-only/supersession rules.
- `openspec/changes/fix-review-drift-and-doctor-readiness/proposal.md` and `specs/**/spec.md` — remediation scope and normative deltas.
- Relevant tracked docs/tests/code named by the proposal: `README.md`, `docs/release-checklist.md`, `.codex/skills/grill-with-docs/CONTEXT-FORMAT.md`, `src/doctor.rs`, and final DoD/packaging/doctor tests.

## Plan Summary

Fix the post-review drift without changing the accepted v1 architecture: restore durable ADR traceability, align `doctor --json` readiness/capability facts with implemented X11/EWMH v1 behavior, redact environment-derived ydotool socket paths in serialized diagnostics, refresh README/release-checklist/docs examples, and remove strict clippy warnings while preserving the constitution's required verification surface.

## Question Loop

### Q1 — Should missing ADR references be removed or restored?

- **Material risk:** `ARCHITECTURE.md` and `adr/README.md` currently reference ADRs that are not tracked. Removing references would make validation green but would erase the durable rationale claimed by the snapshot.
- **Recommended answer:** Restore or reconstruct all referenced top-level ADR files `adr/0001` through `adr/0007`, including superseded ADRs 0002 and 0004, using the existing architecture/index text as the source of truth.
- **Resolution:** Repository context answers this: the snapshot/index intentionally present ADRs 0001/0003/0005/0006/0007 as in-force and 0002/0004 as historical context. The remediation will restore tracked files and add validator/test coverage for missing references rather than deleting the references.

### Q2 — Should `doctor --json` expose real ydotool socket paths from environment variables?

- **Material risk:** Real `YDOTOOL_SOCKET` and `XDG_RUNTIME_DIR` values can contain private user names, runtime IDs, or local topology. The command is explicitly a shareable smoke-test surface.
- **Recommended answer:** Keep internal connection probes against real paths, but serialize environment-derived candidates as stable labels such as `env:YDOTOOL_SOCKET` and `env:XDG_RUNTIME_DIR/.ydotool_socket`; keep `/tmp/.ydotool_socket` literal because it is a documented public fallback path.
- **Resolution:** Specs adopt redaction-by-label for environment-derived socket candidates. No user question is needed because it follows the constitution's no-secret/no-private-value posture and preserves diagnostic usefulness.

### Q3 — Should `can_focus_apps` and `can_focus_windows` both become true?

- **Material risk:** The implementation verifies X11 window activation. App-level focus is only safe when it maps to verified window activation, and overclaiming app focus could conflict with upstream-compatible semantics.
- **Recommended answer:** Set `can_focus_windows` from verified X11/EWMH focus prerequisites. Set `can_focus_apps` true only when the report explicitly maps app focus to verified X11 window activation; otherwise keep it distinct and explain the difference without making `can_focus_windows` stale/false.
- **Resolution:** Specs require truthful distinction. Design/apply will use the available implementation model and tests to avoid stale false window focus and avoid unsupported app-focus claims.

### Q4 — Should strict clippy become a new default Makefile gate?

- **Material risk:** The constitution currently requires `make fmt`, `make check`, and `make test`. Adding a default gate would be a project-wide verification policy change, not just review cleanup.
- **Recommended answer:** Use `cargo clippy --all-targets --all-features -- -D warnings` as review-remediation evidence and clean current warnings, but do not add it to the default Makefile gate without a later constitution/spec decision.
- **Resolution:** Specs keep clippy as an explicit maintainer quality check for this change and preserve the constitution verification surface.

### Q5 — Should the release checklist keep active-change validation commands after archive?

- **Material risk:** A release checklist that tells users to validate an archived change by active change name fails after archive and undermines handoff reliability.
- **Recommended answer:** Replace active archived-change validation with durable commands that remain valid after archive, especially `openspec validate --all --strict`, plus existing project/e2e/final-DoD validators.
- **Resolution:** Specs require post-archive-valid release commands and tests will reject stale archived-change validation.

## Resolved Terms and CONTEXT.md Updates

- No new glossary terms were required. Existing terms `Source overlay`, `Final DoD`, `Architecture decision ledger`, `Release checklist`, and `Runtime command dependency` already cover this change.
- `CONTEXT.md` does not need an update for this remediation.

## OpenSpec Artifact Updates Applied or Required

- Applied: spec deltas for `doctor-cli`, `x11-computer-use-architecture-dod`, `x11-packaging-docs-upstreaming`, and `project-bootstrap`.
- Required next: `design.md` must define concrete implementation boundaries for ADR restoration, doctor capability/readiness, ydotool redaction, documentation tests, and clippy cleanup.

## ADR Candidates

- No new architecture decision is currently required. The main architecture action is traceability restoration for decisions already listed by `ARCHITECTURE.md`/`adr/README.md`.
- The per-change ADR review should still record that no durable ADR is added unless design discovers a new hard-to-reverse decision.

## Open Questions

None.
