## ADR Review

This per-change ADR review records architecture and verification decisions for `harden-x11-industrial-live-verification`. It creates no durable top-level ADR because the change strengthens acceptance evidence and screenshot correctness inside the existing ADR 0008/0009/0010 architecture boundaries.

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — Accepted; remains in force for OpenSpec-as-source-of-truth workflow.
- `adr/0003-formalize-project-context-entrypoints.md` — Accepted; remains in force for `CONSTITUTION.md`, `ARCHITECTURE.md`, ADR history, and local-secret boundaries.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — Accepted; remains in force for mandatory `grill.md`, `design-review.md`, and TDD apply discipline.
- `adr/0006-adopt-claude-artifact-review.md` — Accepted; remains in force, but Claude artifact review is disabled/off for this session and change run.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — Accepted; remains in force for scoped automatic local lifecycle checkpoint commits, which the user explicitly authorized for this propose run.
- `adr/0008-adopt-x11-root-coordinate-model.md` — Accepted; remains in force. Screenshot crop rectangles and fixture bounds remain X11 root/global pixels.
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — Accepted; remains in force. This change sharpens the evidence posture so missing fixture setup is not accepted as industrial readiness.
- `adr/0010-adopt-x11-provider-takeover-shim.md` — Accepted; remains in force. This change does not rename the standalone plugin, remove `x11_*` tools, or change provider takeover scope.
- Superseded ADRs `0002` and `0004` remain historical only and are not revived.

## Constitution / Architecture Rules Considered

- Rust 2021 and root `Makefile` verification remain the implementation and verification baseline.
- OpenSpec lifecycle order remains mandatory; no apply/implementation before proposal/specs/grill/design/design-review/adr/test-plan/tasks are complete and checkpointed.
- No `.secrets.local.env` access is needed; no secrets may be printed, staged, committed, archived, or copied into evidence.
- Runtime fixtures and evidence must stay under project-local scripts/log directories and avoid uncontrolled external systems.
- `target/e2e-logs/<run-id>/` remains the evidence convention.
- X11 root/global coordinates remain canonical for crop rectangles, target bounds, pointer points, overlays, and app-state screenshot context.
- Verified focus and bounds remain the input/pointer safety boundary; live harness fixture allowlisting augments but does not replace tool-level safety.
- AT-SPI correlation remains confidence-scored and degraded on absence/ambiguity; GTK bridge fixture is the semantic pass path, Tk no-match stays fixture-specific degraded evidence.
- Standalone plugin identity, `x11-ewmh` backend identity, and `x11_*` MCP namespace remain unchanged.

## Decisions Evaluated

- **Decision: Resolve relative crop output paths before provider invocation.**
  - Accepted: resolve relative paths against process cwd, pass the resolved absolute path to the provider, and report that resolved path in JSON.
  - Rationale: This preserves CLI ergonomics while removing the ambiguous provider false/no-file behavior observed during retest.
  - Rejected: require absolute paths only. It is safe but less useful for scripts, and postflight verification handles the real safety requirement.
  - Consequence: Tests must cover relative path resolution, invalid parents, provider false, missing output, empty output, non-PNG output, and valid PNG success.

- **Decision: Screenshot success requires verified PNG output.**
  - Accepted: `success=true` requires provider success and readable non-empty PNG signature at the resolved output path.
  - Rationale: Provider status alone was insufficient in retest. Industrial correctness requires output artifact verification.
  - Consequence: `src/coordinates.rs`/CLI behavior will gain structured output error codes and diagnostics.

- **Decision: Industrial live verification extends, not replaces, existing smoke.**
  - Accepted: keep metadata/tools smoke and fake mode, but add an explicit industrial acceptance profile/mode that requires fixture-backed rows for fixture-dependent capabilities.
  - Rationale: Existing fake and metadata smoke are useful and should remain backward compatible, but production readiness cannot be based on missing fixture orchestration.
  - Consequence: `validate-matrix` should preserve legacy behavior while adding stricter industrial validation.

- **Decision: Fixture allowlisting is required before live desktop mutation/capture.**
  - Accepted: live input, pointer, screenshot, app-state, target-window, and overlay checks must target only controlled fixture windows with unique run-scoped identity.
  - Rationale: Tool-level focus/bounds checks are necessary, but the harness must prevent accidental selection of real user applications before invoking tools.
  - Rejected: fallback to currently focused or first listed app when fixture resolution fails.
  - Consequence: Missing/ambiguous fixture target is `missing_fixture_setup` or `unsafe_target_selection`, not a pass.

- **Decision: GTK bridge fixture is the AT-SPI acceptance path.**
  - Accepted: industrial AT-SPI pass requires GTK bridge fixture or equivalent accessible app evidence; Tk AT-SPI no-match is allowed as Tk-specific degraded evidence only.
  - Rationale: Retest proved GTK bridge fixture can return a high-confidence tree, while Tk reliably exercises input but not semantic accessibility.
  - Rejected: lower matcher thresholds or use bounds-only matching to make Tk pass.

- **Decision: No durable top-level ADR now.**
  - Accepted: record all above decisions in this per-change ADR review.
  - Rationale: The decisions are scoped to hardening an existing accepted Cinnamon/X11 baseline and do not supersede ADR 0008, ADR 0009, or ADR 0010.
  - Future trigger: create a durable ADR if industrial profile becomes the project-wide replacement for existing release gates, if final DoD architecture changes, or if a new hard-to-reverse external fixture/test infrastructure is adopted.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None required for this planning change.
- `ARCHITECTURE.md` already captures the relevant current architecture via ADR 0008, ADR 0009, and ADR 0010.
- Add an implementation task only if apply changes the current architecture snapshot or creates a durable ADR.

## No ADR Needed

- No durable ADR is needed because the design preserves existing backend identity, coordinate model, verified input safety, AT-SPI confidence posture, standalone/source-overlay boundaries, and provider takeover architecture.
- The stricter industrial evidence profile is an implementation of ADR 0009's explicit pass/degraded evidence principle rather than a replacement architecture decision.
