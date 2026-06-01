## ADR Review

This per-change ADR review evaluates whether `add-x11-doctor-capability-detection` needs a new durable top-level ADR. It does not create or modify durable ADR files.

## Existing In-Force ADRs

- No `adr/NNNN-*.md` body files are present in this checkout, so no append-only durable ADR bodies could be read or followed for supersession links.
- `ARCHITECTURE.md` and `adr/README.md` both list the intended in-force ADR set: ADR 0001, ADR 0003, ADR 0005, ADR 0006, and ADR 0007; ADR 0002 and ADR 0004 are listed as superseded historical context. Because the body files are absent, this review treats the architecture snapshot and ADR README as the available project constraints, not as newly accepted ADR bodies.
- The absence of ADR body files is already noted in `design.md` and `design-review.md`; it does not block this scoped diagnostics change, but implementation, later gates, and archive work should not claim to have reviewed missing ADR bodies.
- Reconstructing, relocating, or otherwise repairing the missing historical ADR body files is out of scope for this doctor capability change. If needed, it should be handled by a separate project-context/ADR maintenance change rather than by this behavior-focused change.

## Constitution / Architecture Rules Considered

- `CONSTITUTION.md` requires Rust 2021/root Cargo/Makefile verification for implementation work, root-level standalone crate by default, OpenSpec validation for changed artifacts, safe local Git checkpoints, and no secret values in Git-tracked files.
- `CONSTITUTION.md` defines the local integration target via `CODEX_DESKTOP_LINUX_FULL_PATH` or the documented local default path and permits read-only comparison for source-overlay compatibility planning.
- `CONSTITUTION.md` says `.secrets.local.env` must be read only for workflows that actually need listed external systems. This change performs local host diagnostics only and does not require external systems or secret variables.
- `CONTEXT.md` fixes `x11-ewmh` as the canonical generic X11/EWMH backend label, distinct from Cinnamon-specific validation and from a window `client_type`.
- `CONTEXT.md` defines Standalone plugin and Source overlay as separate delivery paths. This change keeps standalone `doctor --json` primary and records source-overlay compatibility without patching the target checkout.
- `ARCHITECTURE.md` requires lifecycle order `proposal -> specs -> grill -> design -> design-review -> adr -> test-plan -> tasks -> apply -> verify -> archive`; this ADR review follows completed `design-review.md` and precedes `test-plan.md`/`tasks.md`.
- `ARCHITECTURE.md` keeps `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/`, and local secrets outside OpenSpec archive mechanics. This change does not alter those architecture boundaries.
- `adr/README.md` says every change gets a per-change ADR review, durable ADRs are append-only under top-level `adr/`, accepted ADR history is not rewritten, and a durable ADR is needed only when a concrete architectural decision must survive beyond a single change.

## Grill / Design-Review Findings Considered

- `grill.md` resolved that `readiness.ok` is blocker-based rather than serialization-based and that optional degradation uses additive `readiness.degraded_reasons`.
- `grill.md` resolved that `xdotool` availability alone does not satisfy upstream-shaped `can_send_development_input`; it remains an X11-native candidate fact unless a later accepted design maps it.
- `grill.md` resolved that source-overlay acceptance belongs in `x11-integration-contract` and later artifacts, not automatically in a durable ADR.
- `design-review.md` found no open questions and no document updates required before `adr.md`.
- `design-review.md` recommended no durable ADR unless this review concludes the source-overlay compatibility plan changes project architecture beyond scoped diagnostics behavior.
- `design-review.md` carries downstream test-plan/tasks requirements: pin EWMH/root probe behavior, enumerate and update `checks[]`, keep `source_overlay` static metadata, keep standalone `abs_pointer` derivation non-circular, treat `screencast`/`input_capture` as report-only unless later specs expand them, and preserve `capabilities.planned` handling.
- `design-review.md` records no open questions and no required document updates before `adr.md`. The auxiliary Claude report for `design-review.md`, when present in `reviews/design-review-claude-review.json`, should be consumed by later gates for reviewer carry-forward items; this ADR review does not rely on that auxiliary report for a durable-ADR decision.

## Decisions Evaluated

- **Additive standalone doctor JSON vs strict target JSON coupling.** Decision: keep the standalone `DoctorReport` additive over existing bootstrap fields and align vocabulary semantically with the target diagnostics model without requiring a strict subset of target `doctor_report()` JSON. ADR result: no durable ADR needed; this is a change-local API compatibility decision captured by `doctor-cli` specs and design.
- **Read-only local probe architecture with `ProbeContext` seams.** Decision: use local env/tool/DBus/socket/filesystem probes through test seams and fixtures; do not mutate focus/input state, read secrets, or patch the target checkout. ADR result: no durable ADR needed; this is an implementation/testability design for one standalone crate and is reversible without changing project architecture.
- **Upstream-shaped readiness vocabulary without a targeted-input field.** Decision: expose `can_query_windows`, `can_focus_apps`, `can_focus_windows`, `can_send_development_input`, blockers, recommendation, and `degraded_reasons`; do not add a top-level/upstream-required `can_send_targeted_input`. ADR result: no durable ADR needed; this is normative behavior in spec deltas and preserves the target vocabulary rather than introducing a new project-wide architecture rule.
- **Stable public readiness JSON contract.** Decision: treat the expanded readiness vocabulary as a stable additive doctor JSON behavior governed by the `doctor-cli` spec delta and compatibility table, not as a project-wide architecture decision. ADR result: no durable ADR needed while the contract remains strictly additive and scoped to `doctor --json`; reopen durable ADR review if a later change proposes a breaking JSON contract, strict target JSON coupling, or a new top-level readiness field that upstream consumers must honor.
- **Development input backend aggregation.** Decision: `can_send_development_input` is true when at least one supported backend is verified: local `/dev/uinput`/`abs_pointer`, ydotool with connectable socket, or strict RemoteDesktop portal input. `xdotool` remains candidate-only. ADR result: no durable ADR needed; this is a report/readiness rule captured in specs and design.
- **Strict portal/screenshot provider detection.** Decision: empty RemoteDesktop introspection is unavailable, Screenshot version 2 with `Screenshot` is sufficient, and Cinnamon-owned `org.gnome.Shell.Screenshot` is a separate provider fact; future source-overlay work should replace target success-by-exit checks with strict method/property parsing. ADR result: no durable ADR needed now; the acceptance is in `x11-integration-contract`, and no target architecture or backend priority is changed by this planning artifact.
- **Source-overlay scope.** Decision: do not patch `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` in this change; preserve existing GNOME/COSMIC/KWin/Hyprland/i3 backend behavior; keep `x11-ewmh` as a later/future fallback if a window backend is added later. ADR result: no durable ADR needed; the current decision narrows this change's scope and avoids architecture changes.
- **Architecture snapshot and ADR body absence.** Decision: record the absent ADR body files as a review limitation and rely on `ARCHITECTURE.md`/`adr/README.md` as available constraints. ADR result: no durable ADR needed for this diagnostics change; creating a durable ADR solely to document missing historical bodies would be unrelated maintenance.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None.
- This change does not alter the current project architecture snapshot: it keeps the root Rust crate, OpenSpec lifecycle, context/secret boundaries, top-level ADR process, and standalone/source-overlay boundary intact.
- No `ARCHITECTURE.md` update task is required by this change unless a later artifact expands scope into a durable source-overlay architecture change.

## No ADR Needed

- No new durable ADR is needed because the evaluated decisions are scoped to this OpenSpec change's doctor/report behavior and testability plan.
- The decisions are already captured normatively in `doctor-cli` and `x11-integration-contract` spec deltas and operationally in `design.md` / `design-review.md`.
- The upstream-shaped readiness JSON fields are durable public behavior, but in this change they are strictly additive to the bootstrap report and governed by the OpenSpec spec deltas. A durable ADR is not required merely to restate an additive feature contract; the ADR gate should reopen if a later artifact makes the contract breaking, project-wide, or coupled strictly to the target repo JSON.
- The additive guarantee is verifiable through the `doctor-cli` compatibility table: existing bootstrap paths and types for `project`, `version`, `backend`, `readiness.ok`, `readiness.blockers`, `capabilities.implemented`, `capabilities.planned`, `checks`, and each check's `name`/`ok`/`detail` are preserved while new readiness fields are added.
- The design is additive and reversible within the standalone crate: it does not create a new subcrate, alter project-wide lifecycle rules, change target backend ordering, couple standalone JSON strictly to the target JSON, introduce a new required upstream readiness field, patch the local integration target, or change secret/external-system boundaries.
- Future durable ADR review should be reopened only if implementation or a later source-overlay change makes a hard-to-reverse project-wide decision, such as changing backend priority, making the standalone report strictly target-shaped, introducing a persistent source-overlay architecture, or adding a new top-level readiness contract that upstream consumers must honor.
