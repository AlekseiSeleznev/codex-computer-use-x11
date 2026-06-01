## Context Read

- `CONSTITUTION.md` — required Rust/Cargo/Makefile stack, OpenSpec lifecycle gates, secret handling, verification rules, local target path policy, and safe automatic checkpoint discipline.
- `CONTEXT.md` — project glossary for `x11-ewmh`, app state, target window, overlay window, E2E harness, capability matrix evidence, Final DoD, and ADR ledger terms.
- `ARCHITECTURE.md` — in-force architecture snapshot, especially X11 root-coordinate model, final Cinnamon/X11 baseline, and provider takeover boundaries.
- `adr/README.md` and top-level ADRs, especially:
  - ADR 0008 — X11 root/global pixel coordinates and screenshot evidence constraints.
  - ADR 0009 — verified-focus input safety, no direct `xdotool --window` safety boundary, no arbitrary AT-SPI subtree on ambiguity/low confidence, pass/degraded capability evidence.
  - ADR 0010 — localized provider takeover shim only; no global plugin identity masquerade.
- Evidence files:
  - `target/e2e-logs/live-functional/acceptance-summary.md`.
  - `target/e2e-logs/live-functional/app-state-text.json`.
  - `target/e2e-logs/live-functional/safe_apps.py`.
  - `target/e2e-logs/live-functional/safe_apps.events.log`.
- Existing specs:
  - `openspec/specs/x11-targeted-input-safety/spec.md`.
  - `openspec/specs/x11-atspi-window-correlation/spec.md`.
  - `openspec/specs/x11-target-window-groups-overlays/spec.md`.
  - `openspec/specs/x11-get-app-state-integration/spec.md`.
  - `openspec/specs/codex-x11-e2e-test-harness/spec.md`.
- Relevant code:
  - `src/input.rs` — active-context `xdotool` backend, current lack of key alias normalization/stderr semantic failure detection, and current `xdotool type` route.
  - `src/accessibility.rs` — current scoring and `class_matches` substring behavior.
  - `src/list_windows.rs` — current disabled normal per-window xprop enrichment diagnostics.
  - `src/target_window.rs` and `tests/target_window_cli.rs` — current `no-overlay` warning behavior.
  - `src/app_state.rs` and `scripts/e2e/codex-x11-e2e.py` — app-state `diagnostics.layers` shape and harness validation.

## Plan Summary

- The proposal correctly scopes only planning for hardening gaps found on 2026-06-01; it states no implementation happens before the required OpenSpec artifacts are complete.
- Keyboard hardening preserves verified focus and active-context input while adding alias normalization, semantic `xdotool` stderr failure detection, Unicode keysym primary route, and recoverable clipboard fallback.
- AT-SPI hardening improves confidence evidence and positive fixture coverage without lowering the matcher to unsafe bounds-only behavior.
- Overlay hardening converts the current standalone `no-overlay` warning into a real provider contract while keeping overlay failure non-blocking for target save.
- Evidence/harness hardening fixes the `diagnostics.layers` summary path, avoids huge screenshot base64 in summaries, and makes pass/degraded capability rows concrete.

## Question Loop

### Question 1: Should the AT-SPI gate be relaxed so Tk/Tkinter windows pass by bounds-only evidence?

- Recommended answer: No.
- Rationale: ADR 0009 explicitly requires degraded/no arbitrary subtree on ambiguity or low confidence. The live evidence shows Tk windows are useful keyboard/pointer fixtures but not reliable AT-SPI-positive fixtures. Lowering to bounds-only would make a false positive likely and would conflict with the user's explicit scope.
- Resolution: Answered from user request, ADR 0009, and evidence. Specs keep bounds-only matching rejected and add a GTK-positive fixture instead.

### Question 2: Should direct `xdotool --window` or `ydotool` become the primary Unicode fix?

- Recommended answer: No.
- Rationale: ADR 0009 says `xdotool --window`/XSendEvent is not a trusted target-safety boundary, and the user explicitly ruled it out. `ydotool` is uinput/scancode/layout-bound, requires `ydotoold`, and does not directly solve exact Unicode fidelity. The safer sequence is verified focus -> X11 Unicode keysyms -> explicit recoverable clipboard fallback only if needed.
- Resolution: Answered from user request, ADR 0009, and current `src/input.rs`. Specs and proposal reject both shortcuts.

### Question 3: Should target-scoped `xprop` enrichment be added to normal list-windows for every window?

- Recommended answer: No.
- Rationale: The user explicitly forbids unbounded per-window `xprop` spawning in normal list-windows. Current `src/list_windows.rs` already reports per-window enrichment disabled. The needed enrichment is only for a resolved target during correlation, where exactly one target can be bounded and diagnosed.
- Resolution: Answered from user request and existing listing design. Specs require target-scoped enrichment only.

### Question 4: Should overlay provider failure block target save?

- Recommended answer: No.
- Rationale: Existing target-window spec already treats overlay display as optional/non-blocking, and live evidence shows target lifecycle can pass without overlay. The hardening is to implement a real provider and show `overlay.shown=true` when it works, not to make visual display part of target-state correctness.
- Resolution: Answered from existing spec and live evidence. Specs keep overlay failure as warning/degraded.

### Question 5: Is the RemoteDesktop portal gap a blocker for this X11/EWMH hardening change?

- Recommended answer: No.
- Rationale: ADR 0009 scopes v1 to Cinnamon/X11 `x11-ewmh`, and the live run passed X11/EWMH focus/input/pointer/screenshot enough to identify more specific degraded layers. Portal incompleteness remains report-only/optional for future portal work unless a later change changes the supported path.
- Resolution: Answered from ADR 0009 and acceptance summary. Specs require readiness wording to separate optional portal diagnostics from real blockers.

## Resolved Terms

- **Unicode keysym route**: The active-context X11 keyboard route that derives `Uxxxx` keysyms from Unicode scalar values after verified focus. It is a route label and implementation detail for this change, not a new glossary term requiring `CONTEXT.md` update.
- **Clipboard-paste fallback**: An explicit, diagnosable fallback route for non-ASCII text only after verified focus and only with restoration diagnostics. Existing glossary does not need an update because this is not durable domain vocabulary beyond the spec/design.
- **Target-scoped xprop enrichment**: A bounded enrichment step for one resolved target window during AT-SPI correlation. Existing `AT-SPI window correlation` glossary term remains sufficient.
- **No-screenshot-data evidence mode**: A harness/evidence mode that preserves screenshot metadata while omitting base64 bytes. Existing `App state` and `Capability matrix evidence` glossary terms remain sufficient.

## Document Updates Applied

- `proposal.md` already includes the unsafe shortcut rejections and affected capabilities.
- Spec deltas already encode:
  - keyboard aliases/stderr failure/Unicode route/recoverable clipboard fallback;
  - AT-SPI token matching, target-scoped xprop enrichment, score/missing-signal diagnostics, GTK fixture, and bounds-only rejection;
  - standalone overlay provider, overlay listing exclusion, and release/hide behavior;
  - `diagnostics.layers` extraction, no-screenshot-data evidence summaries, and optional portal wording;
  - live harness exact Cyrillic value, GTK AT-SPI, overlay lifecycle, and concrete capability matrix evidence.
- No `CONTEXT.md` update was applied because the new terms are route names or artifact-local evidence labels, not project glossary vocabulary.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- Per-change ADR required: Unicode route/fallback ordering and standalone overlay provider are architecturally significant enough to record in `openspec/changes/harden-live-x11-functional-gaps/adr.md`.
- Durable top-level ADR: likely not required yet if this change stays within ADR 0008/0009/0010 boundaries. A new durable ADR may be needed during apply only if implementation adopts a hard-to-reverse dependency or changes the supported Computer Use baseline beyond the current ADRs.

## Open Questions

None.
