## Why

The 2026-06-01 full retest proved the installed standalone X11 Computer Use plugin is functional on Cinnamon/X11, but it also exposed two industrial-readiness gaps: `screenshot-crop` can report success-ish output when the provider returns false and no file exists, and live smoke evidence still marks fixture-dependent capabilities degraded because the harness does not orchestrate controlled live fixtures. This change hardens screenshot output correctness, live fixture-backed verification, safe input targeting, and evidence classification so production acceptance cannot be satisfied by ambiguous success or missing fixture setup.

## What Changes

- Harden screenshot crop output handling so `success=true` is impossible unless the requested output exists, is readable, is non-empty, and has a valid PNG signature after provider execution.
- Resolve relative screenshot crop output paths against the process current working directory before provider invocation, report the resolved absolute path, and return a structured error for invalid/unsafe output paths instead of passing ambiguous provider paths through as success.
- Treat provider false, provider missing output, empty output, inaccessible output, and non-PNG output as explicit FAIL outcomes with machine-readable `error_code`, diagnostics, and `success=false`.
- Extend the live standalone plugin E2E harness with controlled fixtures for Tk keyboard/pointer/focus/target/release, GTK AT-SPI with `GTK_MODULES=gail:atk-bridge NO_AT_BRIDGE=0`, optional Tk overlay with `CODEX_X11_ENABLE_TK_OVERLAY=1`, fixture-only screenshot crop, and fixture-only app-state checks.
- Strengthen live safety rules so keyboard, click, scroll, drag, screenshot, app-state, target-window, and overlay checks can only operate on uniquely identified project-owned fixture windows and never on real user applications.
- Improve machine-readable `evidence.json` and matrix validation so fixture-backed capabilities distinguish `PASS`, real environment `DEGRADED`, missing fixture setup, and actual code failure; missing fixture setup must not be accepted as an industrial pass.
- Update regression coverage for screenshot output validation, fixture lifecycle/cleanup, live matrix classification, controlled-target selection, and safe evidence/log serialization.
- Update docs/troubleshooting/release checklist to describe the industrial live verification boundary, accepted degraded cases, fixture dependencies, and safe evidence storage.
- No **BREAKING** changes to standalone `x11_*` tool names, backend identity, or source-overlay/provider-takeover architecture.

## Capabilities

Modified capabilities:

- `x11-screenshot-coordinate-model` — screenshot crop output path resolution/validation, provider false handling, output file existence/size/PNG verification, and structured failure semantics.
- `codex-x11-e2e-test-harness` — fixture-backed live plugin smoke for keyboard/pointer/focus/target/release, GTK AT-SPI, overlay enabled lifecycle, screenshot crop, app-state, evidence schema, and matrix validation classification.
- `x11-targeted-input-safety` — industrial live checks that prove input operations are limited to controlled fixture windows and never fall back to unverified real user app targeting.
- `x11-atspi-window-correlation` — GTK bridge fixture expectations as the semantic AT-SPI pass path while Tk remains an expected degraded path for AT-SPI.
- `x11-target-window-groups-overlays` — overlay enabled live fixture lifecycle and release cleanup evidence, without making overlay display a mandatory dependency when the environment intentionally disables it.
- `x11-get-app-state-integration` — fixture-scoped app-state live evidence and sanitized screenshot layer serialization for harness logs/evidence.
- `x11-computer-use-architecture-dod` — industrial acceptance semantics that reject missing fixture orchestration as acceptable pass evidence and preserve environment-only degraded classification.

## Impact

- Expected implementation areas include `src/screenshot.rs` or the screenshot crop command boundary, `src/mcp.rs` if MCP exposes crop/app-state semantics, `scripts/e2e/codex-plugin-smoke.sh`, `scripts/e2e/codex-x11-e2e.py`, fixture scripts under `scripts/e2e/` or test fixtures, matrix validation code, and relevant Rust/Python tests.
- Expected documentation updates include `docs/e2e-harness.md`, `docs/troubleshooting.md`, `docs/release-checklist.md`, and any final DoD/evidence guidance that describes industrial live acceptance.
- Verification must preserve `target/e2e-logs/<run-id>/` as the evidence directory convention and must store screenshots as files/evidence paths, not huge data URLs in ordinary logs or chat output.
- Live tests must use deterministic, uniquely titled/classed fixtures, timeouts, cleanup traps, and fixture ownership checks before any input/pointer operation.
- The change must preserve Rust 2021/root Cargo/Makefile verification, OpenSpec lifecycle gates, ADR 0008 X11 root-coordinate semantics, ADR 0009 verified-focus and no-arbitrary-AT-SPI-match safety, ADR 0010 standalone/source-overlay/provider-takeover boundaries, and the existing standalone `x11_*` namespace.
- No external credentials are required; `.secrets.local.env` must not be read or copied into artifacts.
- This is planning only. Apply must not start until proposal, specs, grill, design, design-review, adr, test-plan, and tasks are complete and checkpointed.
