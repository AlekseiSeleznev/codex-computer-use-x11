## Context Read

- `AGENTS.md` and root project instructions: OpenSpec is source of truth; no implementation before full planning gate; safe checkpoint commits are allowed after user confirmation; Claude review is off for this session.
- `CONSTITUTION.md`: Rust 2021/root Cargo/Makefile verification; no `.secrets.local.env` access; OpenSpec validation required; local target checkout is machine-specific and source-overlay live is not required when target is missing/dirty.
- `CONTEXT.md`: canonical terms used in this change include standalone plugin, source overlay, E2E harness, capability matrix evidence, target window, overlay window, app state, layer-degraded app state, X11 root coordinates, crop rectangle, and Final DoD.
- `ARCHITECTURE.md` and in-force ADRs `0005`, `0007`, `0008`, `0009`, `0010`: mandatory grill/TDD gates, automatic safe checkpoints, X11 root coordinates, verified-focus input safety, AT-SPI no-arbitrary-subtree rule, final Cinnamon/X11 v1 DoD semantics, and standalone/source-overlay/provider-takeover boundaries.
- Existing specs: `x11-screenshot-coordinate-model`, `codex-x11-e2e-test-harness`, `x11-targeted-input-safety`, `x11-atspi-window-correlation`, `x11-target-window-groups-overlays`, `x11-get-app-state-integration`, and `x11-computer-use-architecture-dod`.
- Prior archived change `openspec/changes/archive/2026-06-01-harden-live-x11-functional-gaps/`: already added Unicode/AT-SPI/overlay/app-state hardening and fixture expectations; this change focuses on industrial verification and screenshot output correctness after the full retest.
- Retest evidence under `target/e2e-logs/full-x11-retest-20260601T123839Z/`, especially `report.md`, `live-plugin-smoke.log`, `plugin-live/**/evidence.json`, `live-mcp/screenshot-and-bounds.log`, `live-mcp/screenshot-crop-absolute.log`, `live-mcp/overlay-enabled-cli.log`, `live-mcp/fixture-content.txt`, and `live-mcp/gtk-fixture-ready.json`.
- Relevant source/docs inspected: `src/coordinates.rs`, `src/cli.rs`, `scripts/e2e/codex-plugin-smoke.sh`, `scripts/e2e/codex-x11-e2e.py`, `docs/e2e-harness.md`, `docs/troubleshooting.md`, `docs/release-checklist.md`, and `docs/final-architecture-dod.md`.

## Plan Summary

- The plan closes the `screenshot-crop` ambiguity observed in retest: provider false or missing/invalid output can no longer be reported as success.
- Relative crop output paths are now planned to resolve against process cwd before provider invocation, with the resolved absolute path reported and validated.
- Live standalone plugin smoke will graduate from metadata/tools-only evidence to controlled fixture-backed checks for input, pointer, focus, target/release, screenshot, app-state, GTK AT-SPI, and optional overlay.
- Industrial matrix validation will distinguish `environment_limitation`, `missing_fixture_setup`, and `code_failure`; missing fixture setup is not acceptable pass/degraded evidence for industrial readiness.
- Safety remains stricter than convenience: live input/pointer/screenshot/app-state operations are fixture-only and must never target uncontrolled user applications.

## Question Loop

### Question 1: Should `screenshot-crop` require absolute paths or resolve relative paths?

- **Recommended answer:** Resolve relative paths against the process current working directory before provider invocation, report the resolved absolute path, and then verify the resulting file.
- **Rationale:** The retest showed a relative path reached the provider and produced `(false, 'relative/path')` with no file. Requiring absolute paths would be safe but less user-friendly for CLI/evidence runs. Resolving before the provider removes ambiguity while preserving existing CLI ergonomics and still enables strict output verification.
- **Resolution:** Resolved from repository context and user scope, no user question required. Updated `proposal.md` and `specs/x11-screenshot-coordinate-model/spec.md` to choose cwd-relative resolution.

### Question 2: Should live smoke default to fully industrial fixture-backed checks, or keep metadata-only behavior as acceptable?

- **Recommended answer:** Keep metadata/tools checks as a smoke sub-layer, but industrial acceptance must require fixture-backed rows or fail/not-ready classification.
- **Rationale:** Existing docs and evidence show plugin live mode validates metadata/tools only and marks fixture-dependent rows degraded. That is useful freshness evidence but not sufficient for production readiness because missing fixture orchestration can mask real code failures. Specs now introduce industrial matrix classification rather than deleting the lighter smoke path.
- **Resolution:** Resolved from retest findings and existing `docs/e2e-harness.md`. No proposal/spec update beyond the newly created industrial evidence requirements was needed.

### Question 3: Should Tk AT-SPI become a hard pass now that Tk works for input?

- **Recommended answer:** No. Keep Tk as keyboard/pointer/focus fixture evidence and use GTK with `GTK_MODULES=gail:atk-bridge NO_AT_BRIDGE=0` as the semantic AT-SPI pass path.
- **Rationale:** Retest evidence showed Tk fixture input is reliable, while Tk AT-SPI returns `NoAccessibilityMatch`. The GTK bridge fixture returned a high-confidence tree. ADR 0009 forbids lowering AT-SPI confidence or returning arbitrary subtrees.
- **Resolution:** Resolved from retest evidence and existing AT-SPI specs. The delta spec requires GTK bridge fixture evidence and keeps Tk no-match fixture-specific degraded evidence.

### Question 4: Should live fixture screenshots or app-state screenshots be embedded inline in evidence logs?

- **Recommended answer:** No. Store screenshots as files under `target/e2e-logs/<run-id>/` and retain paths/metadata/layer statuses in `evidence.json`.
- **Rationale:** Existing app-state can serialize a data URL, but the retest explicitly warned not to dump huge screenshots in chat/logs. Secret-safety and evidence quality are better served by file artifacts plus sanitized summaries.
- **Resolution:** Resolved from constitution secret-safety rules and existing `x11-get-app-state-integration` evidence-mode requirements. No user question required.

### Question 5: Does this change require a new durable top-level ADR?

- **Recommended answer:** Not during propose. Record decisions in per-change `adr.md`; create a durable ADR only if apply chooses a hard-to-reverse new industrial evidence contract beyond existing ADR 0009/DoD semantics or changes the accepted architecture snapshot.
- **Rationale:** The current plan hardens acceptance evidence and screenshot correctness within ADR 0008/0009/0010. It does not rewrite backend identity, coordinate model, safe input boundary, or provider takeover architecture.
- **Resolution:** Resolved from in-force ADRs. Per-change ADR should state no durable ADR is currently required.

## Resolved Terms

- **Industrial live verification**: fixture-backed live acceptance evidence for the supported Cinnamon/X11 scope, stronger than metadata/tools live smoke.
- **Missing fixture setup**: a harness failure/acceptance blocker where a capability is unproven because the controlled fixture was not orchestrated; it is not the same as an expected environment limitation.
- **Controlled fixture window**: a project-created or explicitly selected window with unique title/class, readiness signal, cleanup ownership, and an allowlisted role for live tests.
- **Screenshot output integrity**: the post-provider contract that success requires a readable non-empty PNG at the resolved output path.

`CONTEXT.md` was not updated because these are change-local acceptance terms that are fully defined in the OpenSpec artifacts and do not yet need durable glossary status.

## Document Updates Applied

- Created `proposal.md` for `harden-x11-industrial-live-verification`.
- Created delta specs for:
  - `x11-screenshot-coordinate-model`
  - `codex-x11-e2e-test-harness`
  - `x11-targeted-input-safety`
  - `x11-atspi-window-correlation`
  - `x11-target-window-groups-overlays`
  - `x11-get-app-state-integration`
  - `x11-computer-use-architecture-dod`
- Updated `proposal.md` and the screenshot delta spec to choose cwd-relative output path resolution before provider calls.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- No durable top-level ADR is required at this point. The industrial fixture-backed acceptance rules are important but are currently an extension of ADR 0009's explicit pass/degraded evidence posture, not a new architecture direction.
- Revisit durable ADR need during `adr.md` if design introduces a new hard-to-reverse harness mode, release-blocking evidence schema, or architecture snapshot change.

## Open Questions

None.
