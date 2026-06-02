## Context Read

- `CONSTITUTION.md` — Rust 2021/root Cargo/Makefile constraints, local target path rules, no secret access, verification requirements for `doctor --json`, and automatic safe checkpoint discipline.
- `CONTEXT.md` — glossary terms: `x11-ewmh`, Standalone plugin, Source overlay, Grill gate, TDD slice.
- `ARCHITECTURE.md` — lifecycle gate ordering, Codex/OpenSpec boundary, Claude review/session state, and local-secret boundaries.
- `adr/README.md` — durable ADR process and in-force ADR list. No top-level `adr/NNNN-*.md` body files are present in this checkout, so no additional ADR bodies could be read.
- `openspec/changes/add-x11-doctor-capability-detection/proposal.md` — proposal, research refresh, compatibility constraints, and proposal Claude review disposition.
- `openspec/changes/add-x11-doctor-capability-detection/specs/doctor-cli/spec.md` — doctor CLI deltas for additive JSON compatibility, readiness, degraded reasons, headless behavior, exit behavior, ydotool socket probing, portal/screenshot facts, and fixture-backed probes.
- `openspec/changes/add-x11-doctor-capability-detection/specs/x11-integration-contract/spec.md` — source-overlay deltas for diagnostics vocabulary, strict portal detection, screenshot provider mapping, targeted input gating, and acceptance records.
- `src/doctor.rs`, `src/main.rs`, `src/x11_id.rs` — current standalone implementation baseline.
- Local target snippets from `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/computer-use-linux/src/diagnostics.rs`, `windowing/registry.rs`, and `windowing/types.rs` — current upstream-shaped diagnostics vocabulary, capability-map inputs, backend order, and focus-target behavior.

## Plan Summary

- The change expands standalone `codex-computer-use-x11 doctor --json` from a bootstrap identity report into an additive Cinnamon/X11 and generic X11/EWMH capability/readiness report.
- Existing bootstrap JSON paths stay compatible: `project`, `version`, `backend`, `readiness.ok`, `readiness.blockers`, `capabilities.implemented`, `capabilities.planned`, and `checks[*].name/ok/detail` remain present and type-stable.
- Readiness aligns with target Computer Use Linux vocabulary: `can_query_windows`, `can_focus_apps`, `can_focus_windows`, `can_send_development_input`, blockers, recommended next step, and additive `readiness.degraded_reasons`.
- Portal and screenshot detection must be strict and fixture-tested: empty RemoteDesktop introspection is unavailable; Screenshot version 2 with `Screenshot` is sufficient; Cinnamon-owned `org.gnome.Shell.Screenshot` is a distinct provider fact.
- Source-overlay work is constrained to doctor/report compatibility gaps and must preserve existing GNOME/COSMIC/KWin/Hyprland/i3 backend behavior unless later design/ADR explicitly changes scope.

## Question Loop

No user-facing questions were necessary. I challenged the plan against repository context and resolved the material branches as follows:

1. **Should `readiness.ok` mean “JSON was produced” or “desktop-control baseline is ready”?**
   - Recommended answer: `readiness.ok` should be blocker-based, not serialization-based.
   - Rationale: `doctor --json` must exit 0 in degraded/no-display cases when it can emit a report, while readiness must still communicate whether the current supported target is usable. The specs already state successful JSON production alone is insufficient.
   - Resolution: Covered by `doctor-cli` / `Aggregate readiness ok from blockers`; no artifact update required.

2. **Should optional degradation be encoded only in prose or in a stable field?**
   - Recommended answer: use additive `readiness.degraded_reasons: string[]` while keeping blockers for readiness-blocking failures.
   - Rationale: Downstream automation needs a stable machine-readable distinction between optional degraded facts and hard blockers, and this remains additive to bootstrap JSON.
   - Resolution: Covered by `doctor-cli` / `Doctor capability detection report` and `Report degraded reasons separately from blockers`; no artifact update required.

3. **Should `xdotool` alone satisfy upstream-shaped development input readiness?**
   - Recommended answer: no for this stage; report `xdotool` as an X11-native candidate fact, while `can_send_development_input` is satisfied by `abs_pointer` via read/write `/dev/uinput`, ydotool with a connectable socket, or portal RemoteDesktop with concrete methods/properties.
   - Rationale: This mirrors the target diagnostics concepts and avoids inventing a new upstream readiness contract before focus verification and input backend design.
   - Resolution: Covered by `doctor-cli` / `Report accessibility and input backend facts`; no artifact update required.

4. **Should source-overlay acceptance require a durable ADR now?**
   - Recommended answer: no; keep acceptance in the `x11-integration-contract` delta and carry it into design/tasks. Revisit in `adr.md` only if design introduces a hard-to-reverse architecture decision.
   - Rationale: The current work is a scoped diagnostics/reporting contract, not a new persistent architecture style. The repository already has the top-level ADR process, but the referenced ADR body files are absent in this checkout.
   - Resolution: Covered by `x11-integration-contract` / `Source-overlay acceptance record`; no artifact update required.

5. **Should canonical spec `Purpose` TBD cleanup block design?**
   - Recommended answer: no, but it must not be lost.
   - Rationale: The TBD Purpose text is pre-existing canonical-spec maintenance debt. Delta specs do not directly update canonical Purpose text before archive/spec-sync, and it does not alter doctor behavior. Specs already require a later `tasks.md` task or archive note.
   - Resolution: Task-stage follow-up is recorded in `doctor-cli` / `Claude Specs Review Disposition`; no artifact update required before design.

## Resolved Terms

- `x11-ewmh` remains the canonical backend label for generic X11/EWMH work; no glossary update needed.
- “Standalone plugin” and “Source overlay” are already defined in `CONTEXT.md`; no glossary update needed.
- “Development input” is used in the upstream-shaped readiness sense (`can_send_development_input`), while “targeted input” remains derived/report-only until focus verification defines a concrete contract; no glossary update needed because this is a spec behavior distinction, not a new domain term.
- “Degraded reasons” are now a spec-level JSON field shape (`readiness.degraded_reasons`) rather than a glossary term; no `CONTEXT.md` update needed.

## Document Updates Applied

None during this grill gate. The proposal and spec deltas already contain the required resolutions before design:

- bootstrap compatibility table and `project-bootstrap` out-of-scope rationale;
- headless/no-display behavior;
- strict portal and screenshot provider facts;
- ydotool socket ordering and fallback behavior;
- fixture-backed DBus/parser requirements;
- source-overlay acceptance location;
- task-stage follow-ups for canonical Purpose cleanup and planned-capabilities enumeration.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

None at this gate.

Rationale: the current decisions are scoped to the change’s doctor/report contract and are either additive JSON contract details or source-overlay acceptance constraints already captured in specs. The later ADR artifact should revisit whether design introduces a durable architecture decision, especially if it changes source-overlay boundaries, target repo report shapes, or backend ordering.

## Open Questions

None.
