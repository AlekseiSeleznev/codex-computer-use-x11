# 0009 — Adopt final Cinnamon/X11 v1 Computer Use DoD baseline

## Status

Accepted

## Date

2026-05-31

## Context

`codex-computer-use-x11` has accumulated the v1 X11/EWMH baseline across multiple OpenSpec changes: project bootstrap, doctor/readiness, EWMH listing, active/focus verification, standalone MCP plugin installation, source overlay, targeted keyboard and pointer safety, AT-SPI correlation, screenshot/root-coordinate model, `get_app_state`, target-window UX, e2e harness, and packaging/upstreaming documentation.

The final backlog stage needs a durable answer to whether the project is a full Computer Use backend for the supported scope. Without a final baseline decision, future maintainers would need to reconstruct hard safety and integration choices from archived OpenSpec artifacts, backlog notes, and chat history.

## Decision

Adopt a **final Cinnamon/X11 v1 Computer Use DoD baseline** for this repository:

- The canonical backend identity is `x11-ewmh`; do not use ambiguous backend id `x11` for `WindowInfo.backend`.
- The primary window model remains upstream-compatible `WindowInfo`; X11-specific raw source/provenance/reliability details remain in diagnostics/sidecars unless a later ADR changes the model.
- Standalone code may use command seams and fake `PATH` fixtures for deterministic tests; source-overlay/upstream code should follow the target repository style of thin command wrappers plus pure parser/normalizer tests unless a later ADR accepts a dependency-injection runner there.
- Shell-out through `wmctrl`, `xprop`, and `xdotool` is acceptable for v1 when strict diagnostics and tests cover failure/degraded cases. Switch to native X11 through a library such as `x11rb` if shell-out parsing becomes too brittle, per-window process spawning becomes too slow, window type/bounds cannot be made reliable, or upstream rejects shell-out.
- Diagnostics must use real target vocabulary and strict method/property checks. In particular, an empty RemoteDesktop portal introspection table is unavailable, screenshot readiness must be based on actual screenshot methods, and ydotool readiness requires a connectable socket.
- Targeted keyboard and pointer input require verified target focus and bounds as appropriate. `abs_pointer`, `ydotool`, and `xdotool` are global desktop injectors; direct `xdotool --window`/XSendEvent is not a trusted targeted-safety boundary.
- Pointer and keyboard routing should prefer existing Codex target stock tools and backends in source overlay (`activate_window`, `get_app_state`, `type_text`, `press_key`, `click`, `scroll`, `drag`) rather than adding parallel stock X11 tool names. Standalone MCP tools stay namespaced as `x11_*`.
- AT-SPI correlation remains confidence-scored and degraded on absence/ambiguity rather than returning arbitrary subtrees.
- ADR 0008 remains in force: X11 root/global pixels are canonical for bounds, pointer points, screenshot crops, and app-state composition.
- Source overlay is reversible staging evidence, not a long-lived fork. Upstream work must separate backend/windowing changes from Codex Desktop wrapper/package integration.
- Runtime command invocation is distinct from source copying/vendoring. GPL/AGPL/unclear sources remain copy-unsafe without separate review; compatible references still require attribution/license compliance if copied or vendored.
- Cinnamon Wayland and a Cinnamon/Muffin extension are outside v1. They require a future design/ADR if the project takes them on.

The final answer is precise: **yes** for the documented Cinnamon/X11 v1 baseline with listed evidence and degraded diagnostics; **no/unsupported** for Cinnamon Wayland, unstable Cinnamon extension work, and unsafe targeted input without verification.

## Considered Options

1. **Adopt a tracked final DoD baseline with machine validation** (chosen)
   - Makes final readiness discoverable after archive.
   - Keeps pass/degraded evidence explicit.
   - Gives release and upstream handoff a deterministic gate.

2. **Rely on existing archived OpenSpec changes and e2e evidence only**
   - Rejected because the final capability matrix and architecture answer would remain scattered across many changes and docs.

3. **Require all live capabilities to pass on the current desktop before v1**
   - Rejected because screenshot, AT-SPI, terminal context, and source-overlay live checks can be environment-dependent. v1 requires explicit degraded evidence rather than fabricated pass claims.

4. **Declare generic X11 complete beyond Cinnamon/X11**
   - Rejected because the first validated product target remains Linux Mint Cinnamon X11. Other X11 window managers may work by EWMH behavior but are not the v1 validation claim.

## Consequences

- Future work must update the final DoD matrix or create a new decision when adding/removing v1 capabilities.
- Release/archive verification includes the final DoD validator in addition to Cargo, OpenSpec, and e2e checks.
- The source-overlay and upstreaming posture remains conservative: reversible staging evidence first, backend and wrapper PRs separated.
- Degraded capability rows are acceptable only when they include concrete evidence and a reason; silent omissions are not acceptable.
- ADR 0008 remains the detailed coordinate-model decision and is not superseded.

## Evidence

- OpenSpec change: `openspec/changes/finalize-x11-computer-use-architecture-dod/`.
- Final DoD report: `docs/final-architecture-dod.md`.
- Validator: `scripts/validate-final-dod.py`.
- Existing evidence sources include `tests/*`, `scripts/e2e/codex-x11-e2e.py`, `docs/e2e-harness.md`, `docs/integration-contract.md`, `docs/upstreaming.md`, `docs/license-attribution.md`, and `docs/release-checklist.md`.
