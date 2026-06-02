## Context Read

- `CONSTITUTION.md` — Rust 2021/root Cargo/Makefile constraints, local target checkout rules, no-secret policy, OpenSpec validation rules, and automatic safe checkpoint discipline.
- `CONTEXT.md` — glossary terms for `x11-ewmh`, Standalone plugin, Source overlay, Grill gate, Design review gate, and TDD slice.
- `ARCHITECTURE.md` — lifecycle ordering, Codex/OpenSpec boundary, `grill-with-docs`/TDD gates, Claude review/session controls, local-secret boundaries, and checkpoint policy.
- `adr/README.md` — durable ADR process and in-force ADR list. No `adr/NNNN-*.md` ADR body files are present in this checkout, so only the snapshot and README rationale could be reviewed.
- `openspec/changes/add-x11-doctor-capability-detection/proposal.md` — research refresh, additive doctor JSON constraints, strict portal/screenshot facts, ydotool fallback requirements, source-overlay scope, and Claude proposal review disposition.
- `openspec/changes/add-x11-doctor-capability-detection/specs/doctor-cli/spec.md` — behavior deltas for additive JSON compatibility, readiness shape, degraded reasons, headless/no-display behavior, exit categories, ydotool socket probing, strict portal/screenshot facts, and fixture-backed parser coverage.
- `openspec/changes/add-x11-doctor-capability-detection/specs/x11-integration-contract/spec.md` — source-overlay deltas for diagnostics vocabulary compatibility, strict portal detection, screenshot provider mapping, targeted input gating, and source-overlay acceptance records.
- `openspec/changes/add-x11-doctor-capability-detection/grill.md` — pre-design resolutions: blocker-based readiness, `readiness.degraded_reasons`, xdotool as candidate-only, no durable ADR yet for source-overlay acceptance, and task-stage Purpose cleanup.
- `openspec/changes/add-x11-doctor-capability-detection/design.md` — additive report model, probe seams, read-only probe policy, readiness aggregation, fact sections, ydotool/socket order, strict parser design, X11/EWMH readiness, CLI exits, source-overlay plan, risks, and migration plan.
- `openspec/changes/add-x11-doctor-capability-detection/reviews/design-claude-review.json` — Claude review of `design.md`, verdict `pass` with no `mustFix` or questions and three downstream `shouldFix` clarifications for test-plan/tasks. The separate review of this `design-review.md` artifact is written after this artifact is created as `reviews/design-review-claude-review.json` and should be read by later gates when present.
- `src/doctor.rs`, `src/main.rs`, `tests/doctor_cli.rs` — current standalone bootstrap report, CLI behavior, and smoke tests that the additive design must preserve or intentionally update.
- `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/computer-use-linux/src/diagnostics.rs` — target `ReadinessReport`, `PortalReport`, `InputReport`, `WindowingReport`, `CapabilityMap`, ydotool candidate order, current success-by-exit portal check, current screenshot `gnome-shell --version` mapping, and recommended-next-step ordering.
- `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/computer-use-linux/src/server.rs` snippets — current `activate_window`, `list_windows`, `focused_window`, and input gating behavior requiring focus verification before targeted input.

## Design Summary

- The design keeps the standalone `DoctorReport` additive: existing bootstrap top-level fields remain while new sections (`environment`, `tools`, `accessibility`, `x11_ewmh`, `portals`, `screenshots`, `input`, `source_overlay`) carry machine-readable capability facts.
- Probes are intentionally read-only and testable through `ProbeContext` seams, fake command output, fixtures, and socket/filesystem probes; `.secrets.local.env` and the local target checkout are not touched.
- Readiness is blocker-based and aligned with the target diagnostics vocabulary: `can_query_windows`, `can_focus_apps`, `can_focus_windows`, `can_send_development_input`, `blockers`, `recommended_next_step`, and additive `degraded_reasons`.
- Development input readiness is the OR of verified `/dev/uinput` `abs_pointer`, ydotool with a connectable socket, or strict RemoteDesktop portal input; `xdotool` remains an X11-native candidate fact only.
- Source-overlay compatibility is scoped to future doctor/report diagnostics fixes: strict portal method/property checks, provider-aware screenshot facts, and preservation of existing desktop-specific backends.

## Question Loop

No user-facing questions were necessary. I challenged the design against repository context and resolved the material branches from the available artifacts and code:

1. **Does the design over-couple standalone doctor JSON to the target `doctor_report()` JSON?**
   - Recommended answer: No; keep semantic vocabulary alignment without strict JSON shape coupling.
   - Rationale: `doctor-cli` explicitly says the standalone report is not required to be a strict subset of the target JSON, while `x11-integration-contract` requires compatibility with target concepts.
   - Resolution: The design's additive standalone model is consistent; no artifact update required.

2. **Does the design over-promise targeted input before focus verification exists?**
   - Recommended answer: No; keep `can_focus_apps` and `can_focus_windows` false in this stage and leave targeted-input explanation derived/report-only.
   - Rationale: Target `server.rs` gates targeted keyboard/pointer input on focus verification, and the specs forbid a top-level or upstream-required `can_send_targeted_input` field.
   - Resolution: Design is consistent with the source-overlay contract; no artifact update required.

3. **Should `can_query_windows` be pinned now to a single X11 command?**
   - Recommended answer: Not as a design rewrite before ADR. The design already makes `can_query_windows` deterministic, but the test plan should pin the exact root/EWMH probe command and its success criteria.
   - Rationale: Claude review identified ambiguity in the phrase “selected read-only EWMH/root probe.” The safest downstream resolution is to make `test-plan.md` choose one canonical fixture-backed command. Prefer `xprop -root _NET_SUPPORTING_WM_CHECK _NET_ACTIVE_WINDOW` as the canonical EWMH root probe because it directly exercises the X11 root properties needed for EWMH readiness; keep `wmctrl -m` as optional diagnostic detail only if tasks choose to add it.
   - Resolution: Carry to `test-plan.md` and `tasks.md`; not blocking ADR planning.

4. **Can the existing bootstrap `checks[]` entries remain unchanged?**
   - Recommended answer: Preserve the `checks[]` array shape, but do not preserve misleading check names/details unchanged when behavior changes.
   - Rationale: The current `no-live-x11-probes` check says stage 01 performs no live X11 probes. This change intentionally introduces read-only live host probes, so implementation/tests must replace or retire that exact check rather than keeping stale evidence.
   - Resolution: Carry to `test-plan.md` and `tasks.md`: enumerate expected `checks[]` entries for the expanded report, preserving `name`/`ok`/`detail` shape and at least one doctor-internal check, while replacing stale `no-live-x11-probes` with a truthful non-invasive/probe-policy check if needed.

5. **Are `source_overlay` fields runtime facts or static acceptance metadata?**
   - Recommended answer: Treat `source_overlay` as static/report-only acceptance metadata for this change; runtime portal/screenshot/provider facts belong in `portals`, `screenshots`, and `input` sections.
   - Rationale: `target_checkout_modified` is always false in this change, `target_vocabulary` and mapping names are known from the target report layers, and tests should avoid depending on prose-only notes.
   - Resolution: Carry to `test-plan.md`: assert stable fields such as `target_checkout_modified == false`, `strict_portal_required == true`, and target vocabulary/mapping names; do not treat `notes` as the runtime source of capability truth.

## Design Findings

- **No constitution/architecture conflict found.** The design remains within Rust 2021/root crate constraints, does not require external systems, does not read `.secrets.local.env`, and does not patch the local target checkout.
- **Lifecycle gate alignment is good.** The design consumes proposal/spec/grill findings and leaves implementation to later TDD slices; it does not bypass `adr.md`, `test-plan.md`, or `tasks.md`.
- **Readiness strictness is intentional and safe.** Keeping focus booleans false until verified focus exists may produce blocked/degraded reports on machines where manual focus works, but this matches the target input-safety model and avoids unsafe targeted-input claims.
- **X11/EWMH probe selection must be made test-explicit downstream.** The design gives deterministic prerequisites but leaves the exact selected root probe open. `test-plan.md` should pin one canonical command, recommended: `xprop -root _NET_SUPPORTING_WM_CHECK _NET_ACTIVE_WINDOW`, with fake-output fixtures for success, missing `DISPLAY`, missing tool, non-zero exit, and absent EWMH properties.
- **Expanded `checks[]` needs an explicit expectation list downstream.** The JSON shape is preserved, but current bootstrap check text includes `no-live-x11-probes`, which becomes false/stale once read-only probes are added. `test-plan.md`/`tasks.md` should define the expected check entries and update CLI tests accordingly.
- **`source_overlay` should remain static acceptance metadata.** This avoids duplicating runtime facts in prose fields and keeps source-overlay tests meaningful. Runtime portal/screenshot/input availability must be asserted in their dedicated sections.
- **Standalone `abs_pointer` must be derived locally, not circularly from the target report.** The design intentionally uses the target vocabulary name `abs_pointer`, but the standalone fact should be derived from read/write `/dev/uinput` access and local probe facts. It must not require calling or embedding the target repo's `CapabilityMap` to prove the standalone report.
- **Portal `screencast` and `input_capture` facts are additive report-only unless a later spec expands them.** The normative strict-portal scenarios for this change cover Screenshot and RemoteDesktop because those are the capability gaps needed for screenshot and development-input readiness. `screencast` and `input_capture` may be present for target-vocabulary completeness, but test-plan assertions should treat them as shape/presence facts rather than readiness gates unless a later spec adds behavior.
- **Fixture-first verification remains mandatory.** Live Cinnamon/X11 smoke is useful only after fake command output, parser fixtures, and socket/filesystem seam tests cover the behavioral branches.
- **Durable ADR body gap is already acknowledged.** The design review could not inspect the ADR bodies named by `ARCHITECTURE.md`/`adr/README.md` because they are absent. This is not a blocker for this scoped diagnostics design, but `adr.md` must record the limitation and decide whether a durable ADR is needed.

## Document Updates Applied

None.

No proposal/spec/grill/design text changes were required before ADR planning. The findings above are concrete carry-forward inputs for `adr.md`, `test-plan.md`, and `tasks.md`.

## Document Updates Required Before Next Gate

None before `adr.md`.

Carry-forward requirements for later gates:

- `adr.md` must record the absent top-level ADR body files and explain whether this scoped doctor/report design needs a new durable ADR. Current review recommendation: no new durable ADR unless ADR review decides the source-overlay compatibility plan changes project architecture beyond scoped diagnostics behavior.
- `test-plan.md` must pin the exact X11/EWMH root probe command and success criteria for `can_query_windows`; recommended canonical probe is `xprop -root _NET_SUPPORTING_WM_CHECK _NET_ACTIVE_WINDOW`. Success criteria should be fixture-testable and include: command exits 0, output contains both requested property names without an absent-property response, `_NET_SUPPORTING_WM_CHECK` has a parseable non-zero window id, and missing `DISPLAY`, missing `xprop`, non-zero exit, or absent EWMH properties make `can_query_windows` false.
- `test-plan.md`/`tasks.md` must enumerate expected `checks[]` entries and replace/retire the stale bootstrap `no-live-x11-probes` check detail.
- `test-plan.md` must clarify that `source_overlay` is static acceptance metadata, while runtime facts live in `portals`, `screenshots`, and `input`.
- `test-plan.md`/`tasks.md` must keep the standalone `abs_pointer` capability fact non-circular: derive it from `/dev/uinput` read/write access and local input facts while using `abs_pointer` only as target-vocabulary-compatible naming.
- `test-plan.md` should note that `portals.screencast` and `portals.input_capture` are additive report-only target-vocabulary facts in this change, not normative readiness gates, unless a later accepted spec expands their behavior.
- `tasks.md` must still include the previously recorded canonical `Purpose` cleanup/archive note and planned-capabilities enumeration before allowing `capabilities.planned` to become empty.

## ADR Candidates

None recommended at this design-review gate.

Rationale: the reviewed choices are additive report-shape and diagnostics/probe decisions scoped to this change. They are testable and reversible within the standalone crate, and the future source-overlay acceptance is already captured in `x11-integration-contract`. A durable ADR would become appropriate only if `adr.md` concludes that this change establishes a project-wide architecture rule, changes target backend priority, couples the standalone doctor JSON strictly to the target JSON, or introduces a long-lived source-overlay architecture decision.

## Open Questions

None.
